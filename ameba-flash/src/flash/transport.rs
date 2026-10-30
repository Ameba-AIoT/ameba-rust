//! Serial transport + state machine driving the ROM-stage and floader-stage
//! codecs in [`rom`] / [`floader`].
//!
//! [`rom`] and [`floader`] are pure codecs: they only encode requests and
//! decode responses, with no notion of retries, timeouts, or an actual
//! serial port. This module owns all of that — the `Transport` trait
//! abstracts the serial port (so the retry/timeout logic can be exercised
//! with a scripted mock, with no hardware attached), and the functions
//! below port the reference implementation's `send_request`/`handshake`/
//! `sense` control flow on top of it.

use std::time::{Duration, Instant};

use super::proto::FlashError;
use super::{floader, proto, rom};

/// Minimal serial-port surface the state machine needs. A blanket impl
/// below satisfies this for any `serialport::SerialPort`, so real callers
/// just pass the `Box<dyn SerialPort>` from `serialport::new(..).open()`.
/// Tests use a scripted in-memory mock instead.
pub trait Transport {
    fn write_all(&mut self, data: &[u8]) -> std::io::Result<()>;
    /// One read attempt, bounded by `timeout`. Returns the number of bytes
    /// actually read (0 on timeout) — never blocks past `timeout`.
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
    fn set_timeout(&mut self, timeout: Duration) -> std::io::Result<()>;
    fn set_baud_rate(&mut self, baud: u32) -> std::io::Result<()>;
    fn set_dtr(&mut self, level: bool) -> std::io::Result<()>;
    fn set_rts(&mut self, level: bool) -> std::io::Result<()>;
    fn clear_input(&mut self) -> std::io::Result<()>;
}

impl<S: serialport::SerialPort + ?Sized> Transport for S {
    fn write_all(&mut self, data: &[u8]) -> std::io::Result<()> {
        <Self as std::io::Write>::write_all(self, data)
    }

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        <Self as std::io::Read>::read(self, buf)
    }

    fn set_timeout(&mut self, timeout: Duration) -> std::io::Result<()> {
        serialport::SerialPort::set_timeout(self, timeout).map_err(to_io_error)
    }

    fn set_baud_rate(&mut self, baud: u32) -> std::io::Result<()> {
        serialport::SerialPort::set_baud_rate(self, baud).map_err(to_io_error)
    }

    fn set_dtr(&mut self, level: bool) -> std::io::Result<()> {
        serialport::SerialPort::write_data_terminal_ready(self, level).map_err(to_io_error)
    }

    fn set_rts(&mut self, level: bool) -> std::io::Result<()> {
        serialport::SerialPort::write_request_to_send(self, level).map_err(to_io_error)
    }

    fn clear_input(&mut self) -> std::io::Result<()> {
        serialport::SerialPort::clear(self, serialport::ClearBuffer::Input).map_err(to_io_error)
    }
}

/// Collapse an I/O failure into `SysIo`, printing the cause first —
/// `FlashError` is `Copy` and cannot carry it (unplugged, permission, ...).
fn io_fault(what: &str, e: std::io::Error) -> FlashError {
    eprintln!("serial I/O error ({what}): {e}");
    FlashError::SysIo
}

fn to_io_error(e: serialport::Error) -> std::io::Error {
    std::io::Error::other(e)
}

/// Open and configure the serial port used for flashing: baud set to the
/// shared handshake rate, no flow control (the protocol does its own).
pub fn open_port(path: &str) -> serialport::Result<Box<dyn serialport::SerialPort>> {
    serialport::new(path, proto::HANDSHAKE_BAUDRATE)
        .timeout(Duration::from_millis(proto::SYNC_RESPONSE_TIMEOUT_MS))
        .open()
}

// ── low-level timed reads ──────────────────────────────────────────────
// Both ROM-stage and floader-stage requests are read back a few bytes at a
// time against a caller-supplied deadline, so these two helpers are shared.

/// One bounded read attempt. 0 means "nothing arrived before `timeout`",
/// not an error — callers decide whether that's retryable.
fn read_once<T: Transport + ?Sized>(
    t: &mut T,
    buf: &mut [u8],
    timeout: Duration,
) -> Result<usize, FlashError> {
    t.set_timeout(timeout)
        .map_err(|e| io_fault("set timeout", e))?;
    match t.read(buf) {
        Ok(n) => Ok(n),
        Err(e)
            if matches!(
                e.kind(),
                std::io::ErrorKind::TimedOut
                    | std::io::ErrorKind::WouldBlock
                    | std::io::ErrorKind::Interrupted
            ) =>
        {
            Ok(0)
        }
        Err(e) => Err(io_fault("read", e)),
    }
}

/// Fill `buf` completely, retrying short reads until `timeout` elapses in
/// total. Real I/O faults propagate immediately rather than being retried.
fn read_exact_timeout<T: Transport + ?Sized>(
    t: &mut T,
    buf: &mut [u8],
    timeout: Duration,
) -> Result<(), FlashError> {
    let deadline = Instant::now() + timeout;
    let mut filled = 0;
    while filled < buf.len() {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(FlashError::SysTimeout);
        }
        filled += read_once(t, &mut buf[filled..], remaining)?;
    }
    Ok(())
}

// ── ROM stage ────────────────────────────────────────────────────────────

/// Poll for the ROM's liveness byte (`NAK`), used before the first
/// handshake attempt on UART (the reference implementation also absorbs
/// and logs any crash-dump text the device sends instead; that's a
/// diagnostic nicety this port skips — only the alive/not-alive result
/// matters to the state machine).
pub fn rom_check_alive<T: Transport + ?Sized>(
    t: &mut T,
    timeout: Duration,
) -> Result<(), FlashError> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        let mut byte = [0u8; 1];
        let slice = deadline
            .saturating_duration_since(Instant::now())
            .min(Duration::from_millis(50));
        if read_once(t, &mut byte, slice)? == 1 && byte[0] == rom::NAK {
            return Ok(());
        }
    }
    Err(FlashError::SysProto)
}

/// Send one ROM-stage request and wait for its single-byte reply, with the
/// reference implementation's two-attempt / three-read-per-attempt retry
/// shape. `DevLength` and `Cancel` are returned immediately (they mean "not
/// in ROM download mode" / "device cancelled"), neither is retryable.
fn rom_send_request<T: Transport + ?Sized>(
    t: &mut T,
    request: &[u8],
    timeout: Duration,
) -> Result<(), FlashError> {
    let mut last_err = FlashError::SysTimeout;
    for _ in 0..2 {
        t.clear_input().map_err(|e| io_fault("clear input", e))?;
        t.write_all(request).map_err(|e| io_fault("write", e))?;

        let mut result = Err(FlashError::SysTimeout);
        for _ in 0..3 {
            let mut byte = [0u8; 1];
            match read_once(t, &mut byte, timeout)? {
                0 => {
                    result = Err(FlashError::SysTimeout);
                    break;
                }
                _ => match byte[0] {
                    rom::ACK => {
                        result = Ok(());
                        break;
                    }
                    rom::NAK => {
                        result = Err(FlashError::SysNak);
                        continue;
                    }
                    rom::CAN => {
                        result = Err(FlashError::SysCancel);
                        break;
                    }
                    b if FlashError::from_dev_byte(b) == Some(FlashError::DevLength) => {
                        result = Err(FlashError::DevLength);
                        break;
                    }
                    _ => {
                        result = Err(FlashError::SysProto);
                        continue;
                    }
                },
            }
        }

        match result {
            Ok(()) => return Ok(()),
            Err(e @ (FlashError::SysCancel | FlashError::DevLength)) => return Err(e),
            Err(e) => {
                last_err = e;
                std::thread::sleep(Duration::from_millis(proto::REQUEST_RETRY_INTERVAL_MS));
            }
        }
    }
    Err(last_err)
}

/// ROM-stage handshake: on UART, confirm the device is alive first, then
/// propose the target baud rate (`BAUDSET`), switch the host's own port to
/// it, and confirm with `BAUDCHK`.
pub fn rom_handshake<T: Transport + ?Sized>(
    t: &mut T,
    baudrate: u32,
    is_usb: bool,
) -> Result<(), FlashError> {
    if !is_usb {
        rom_check_alive(t, Duration::from_millis(proto::ROM_CHECK_ALIVE_TIMEOUT_MS))?;
    }

    let cmd = rom::encode_baudset(baudrate);
    for _ in 0..2 {
        match rom_send_request(t, &cmd, Duration::from_millis(500)) {
            Ok(()) => {}
            Err(FlashError::DevLength) => return Err(FlashError::DevLength),
            Err(_) => continue,
        }
        if is_usb {
            return Ok(());
        }

        t.set_baud_rate(baudrate)
            .map_err(|e| io_fault("set baud rate", e))?;
        std::thread::sleep(Duration::from_millis(proto::BAUDRATE_SWITCH_DELAY_MS));

        if rom_send_request(t, &[rom::BAUDCHK], Duration::from_millis(500)).is_ok() {
            return Ok(());
        }
    }
    Err(FlashError::SysTimeout)
}

/// Transfer one STX data packet at `address`, retrying per `rom_send_request`.
fn rom_transfer<T: Transport + ?Sized>(
    t: &mut T,
    seq: u8,
    address: u32,
    data: &[u8],
) -> Result<(), FlashError> {
    let packet = rom::encode_stx_packet(seq, address, data);
    rom_send_request(t, &packet, Duration::from_secs(1))
}

fn rom_end_transfer<T: Transport + ?Sized>(t: &mut T) -> Result<(), FlashError> {
    rom_send_request(t, &[rom::EOT], Duration::from_millis(500))
}

/// Host-initiated abort of an in-progress ROM-stage transfer. Best-effort:
/// callers are already bailing out, so a failed `ESC` doesn't change the
/// outcome.
pub fn rom_abort<T: Transport + ?Sized>(t: &mut T) {
    let _ = rom_send_request(t, &[rom::ESC], Duration::from_millis(500));
}

/// Load the floader into RAM over the ROM stage: builds the
/// floader+baudrate payload (see `rom::prepare_floader_payload`), transfers
/// it one page at a time at increasing addresses starting from
/// `proto::FLOADER_ADDRESS`, then ends the session with `EOT`.
pub fn rom_download_floader<T: Transport + ?Sized>(
    t: &mut T,
    floader_bin: &[u8],
    baudrate: u32,
    is_usb: bool,
) -> Result<(), FlashError> {
    if floader_bin.len() > rom::FLOADER_SIZE_LIMIT {
        return Err(FlashError::SysOverrange);
    }
    let page_size = if is_usb {
        rom::STX_USB_DATA_LEN
    } else {
        rom::STX_UART_DATA_LEN
    };
    let payload = rom::prepare_floader_payload(floader_bin, baudrate, page_size);

    let mut address = proto::FLOADER_ADDRESS;
    let mut seq: u8 = 1;
    for chunk in payload.chunks(page_size) {
        rom_transfer(t, seq, address, chunk)?;
        seq = seq.wrapping_add(1);
        address += page_size as u32;
    }
    rom_end_transfer(t)
}

// ── floader stage ─────────────────────────────────────────────────────────

/// Send one floader request and parse its response, with the reference
/// implementation's retry shape. The two command kinds diverge in how a
/// reply is read, per the frame layer's sync/async split (see [`floader`]):
///
/// - sync: body is `SOF` + 3-byte length header + payload + checksum.
/// - async: body is a single ack byte. `0xE4` means the frame was dropped
///   by a full UART FIFO and was never processed by the device — it must
///   be resent, not treated as success, even though it isn't in the
///   negative-response range checked by the generic error path.
///
/// Returns the decoded payload bytes (empty for a successful async ack).
pub fn floader_send_request<T: Transport + ?Sized>(
    t: &mut T,
    payload: &[u8],
    timeout: Duration,
    is_sync: bool,
) -> Result<Vec<u8>, FlashError> {
    let frame = floader::encode_request(payload);
    let mut last_err = FlashError::SysTimeout;
    for _ in 0..proto::REQUEST_RETRY_COUNT {
        t.clear_input().map_err(|e| io_fault("clear input", e))?;
        t.write_all(&frame).map_err(|e| io_fault("write", e))?;

        match floader_read_response(t, timeout, is_sync) {
            Ok(data) => return Ok(data),
            Err(e @ FlashError::SysCancel) => return Err(e),
            Err(FlashError::SysIo) => return Err(FlashError::SysIo),
            Err(e) => {
                last_err = e;
                std::thread::sleep(Duration::from_millis(proto::REQUEST_RETRY_INTERVAL_MS));
            }
        }
    }
    Err(last_err)
}

fn floader_read_response<T: Transport + ?Sized>(
    t: &mut T,
    timeout: Duration,
    is_sync: bool,
) -> Result<Vec<u8>, FlashError> {
    let mut first = [0u8; 1];
    read_exact_timeout(t, &mut first, timeout)?;

    if !is_sync {
        return match floader::decode_async_ack(first[0]) {
            floader::AsyncAck::Success => Ok(Vec::new()),
            floader::AsyncAck::Retry => Err(FlashError::DevFull),
            floader::AsyncAck::Error(e) => Err(e),
        };
    }

    if first[0] != floader::SOF {
        return Err(FlashError::from_dev_byte(first[0]).unwrap_or(FlashError::SysProto));
    }

    let mut header = [0u8; 3];
    read_exact_timeout(t, &mut header, timeout)?;
    let len = floader::decode_length_header(&header)? as usize;

    // The reference implementation reads the payload against the generic
    // async-ack timeout regardless of which command is in flight, not the
    // caller-supplied `timeout` used for the length header above.
    let mut body = vec![0u8; len + 1];
    read_exact_timeout(
        t,
        &mut body,
        Duration::from_millis(proto::ASYNC_RESPONSE_TIMEOUT_MS),
    )?;
    floader::decode_payload(&body).map(|p| p.to_vec())
}

/// `SENSE`: poll the floader's last-completed command. `expect`, when
/// given, is `(op_code, data)` the caller believes it just drove to
/// completion (e.g. the address a WRITE just finished). Mismatches are
/// treated as protocol errors — see the data-mismatch comment below.
pub fn floader_sense<T: Transport + ?Sized>(
    t: &mut T,
    timeout: Duration,
    expect: Option<(u8, u32)>,
) -> Result<floader::SenseResult, FlashError> {
    let resp = floader_send_request(t, &[floader::OP_SENSE], timeout, true)?;
    let sense = floader::parse_sense_response(&resp)?;

    if sense.status != 0 {
        return Err(FlashError::from_dev_byte(sense.status).unwrap_or(FlashError::SysProto));
    }
    if let Some((expect_op, expect_data)) = expect {
        if sense.op_code != expect_op {
            return Err(FlashError::SysProto);
        }
        if sense.data != expect_data {
            // Silent-loss guard: if a WRITE frame was dropped (e.g. UART FIFO
            // overflow on the device), the floader's last-processed address
            // lags behind what the host believes it already sent. Without
            // this check the download loop finishes "successfully" with
            // flash missing pages, and that only surfaces later at CHKSM.
            return Err(FlashError::SysProto);
        }
    }
    Ok(sense)
}

/// Floader-stage handshake: poll `SENSE` until it succeeds (device is up
/// and answering) or the retry budget is exhausted. This only implements
/// the `SWITCH_BAUDRATE_AT_FLOADER = false` branch — see [`rom_handshake`]
/// — so there is no mid-handshake `BAUDSET`/baud switch here; the floader
/// already starts at the same baud the ROM stage ended on.
pub fn floader_handshake<T: Transport + ?Sized>(t: &mut T) -> Result<(), FlashError> {
    let mut last_err = FlashError::SysTimeout;
    for _ in 0..3 {
        match floader_sense(
            t,
            Duration::from_millis(proto::SYNC_RESPONSE_TIMEOUT_MS),
            None,
        ) {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_err = e;
                std::thread::sleep(Duration::from_millis(proto::REQUEST_RETRY_INTERVAL_MS));
            }
        }
    }
    Err(last_err)
}

// ── floader command layer ───────────────────────────────────────────────
// Thin wrappers pairing each payload builder in [`floader`] with the
// send/parse call it needs; the download loop (erase/write/verify
// orchestration) is a later layer built on top of these.

/// Also validates the reported device ID against `proto::DEVICE_ID` and the
/// memory type against `proto::MEMORY_TYPE_NOR` — a mismatch means a
/// different chip or a NAND part, neither of which the rest of the download
/// sequence supports.
pub fn floader_query<T: Transport + ?Sized>(t: &mut T) -> Result<floader::DeviceInfo, FlashError> {
    let resp = floader_send_request(
        t,
        &floader::payload_query(),
        Duration::from_millis(proto::SYNC_RESPONSE_TIMEOUT_MS),
        true,
    )?;
    let info = floader::parse_query_response(&resp)?;
    if info.device_id != proto::DEVICE_ID || info.memory_type != proto::MEMORY_TYPE_NOR {
        return Err(FlashError::SysParameter);
    }
    Ok(info)
}

pub fn floader_next_op<T: Transport + ?Sized>(
    t: &mut T,
    op: floader::NextOp,
    operand: u32,
) -> Result<(), FlashError> {
    floader_send_request(
        t,
        &floader::payload_next_op(op, operand),
        Duration::from_millis(proto::ASYNC_RESPONSE_TIMEOUT_MS),
        false,
    )
    .map(|_| ())
}

/// `WRITE` is async: a successful ack only means the floader accepted the
/// frame into its queue, not that the page landed in flash. `need_sense`
/// mirrors the reference implementation's own internal coupling — when set,
/// this additionally waits on `SENSE` for `(OP_WRITE, addr)` before
/// returning, which is what actually confirms completion and triggers the
/// silent-loss guard in [`floader_sense`]. `sense_timeout` scales with the
/// current erase block (see `proto::nor_write_timeout_ms`) — a block-ending
/// WRITE's SENSE must outlast every page since the block's own erase.
pub fn floader_write_page<T: Transport + ?Sized>(
    t: &mut T,
    mem_type: u8,
    addr: u32,
    data: &[u8],
    need_sense: bool,
    sense_timeout: Duration,
) -> Result<(), FlashError> {
    floader_send_request(
        t,
        &floader::payload_write(mem_type, addr, data),
        Duration::from_millis(proto::WRITE_RESPONSE_TIMEOUT_MS),
        false,
    )?;
    if need_sense {
        floader_sense(t, sense_timeout, Some((floader::OP_WRITE, addr)))?;
    }
    Ok(())
}

/// `FS_ERASE` is async, with the same completion/sense coupling as
/// [`floader_write_page`]: `need_sense` set waits on `SENSE` for
/// `(OP_FS_ERASE, start_addr)` before returning. `sense_timeout` scales with
/// the block size being erased (see `proto::nor_erase_timeout_ms`). This
/// port never issues a forced erase (the reference's `force` flag, used for
/// overriding bad-block/lock checks) — always unforced.
pub fn floader_erase<T: Transport + ?Sized>(
    t: &mut T,
    mem_type: u8,
    start_addr: u32,
    end_addr: u32,
    size: u32,
    need_sense: bool,
    sense_timeout: Duration,
) -> Result<(), FlashError> {
    floader_send_request(
        t,
        &floader::payload_erase_flash(mem_type, false, start_addr, end_addr, size),
        Duration::from_millis(proto::ASYNC_RESPONSE_TIMEOUT_MS),
        false,
    )?;
    if need_sense {
        floader_sense(t, sense_timeout, Some((floader::OP_FS_ERASE, start_addr)))?;
    }
    Ok(())
}

/// `READ` is sync, unlike WRITE/FS_ERASE — the device replies with the data
/// directly rather than a queued-ack-plus-SENSE pair.
pub fn floader_read<T: Transport + ?Sized>(
    t: &mut T,
    mem_type: u8,
    addr: u32,
    size: u32,
    timeout: Duration,
) -> Result<Vec<u8>, FlashError> {
    let resp = floader_send_request(
        t,
        &floader::payload_read(mem_type, addr, size),
        timeout,
        true,
    )?;
    floader::parse_read_response(&resp, size as usize).map(|p| p.to_vec())
}

/// `timeout` scales with `size` (see `proto::nor_checksum_timeout_ms`) — a
/// whole-image checksum over a large flash range takes longer than the
/// fixed per-command sync timeout allows.
pub fn floader_checksum<T: Transport + ?Sized>(
    t: &mut T,
    mem_type: u8,
    start_addr: u32,
    end_addr: u32,
    size: u32,
    timeout: Duration,
) -> Result<u32, FlashError> {
    let resp = floader_send_request(
        t,
        &floader::payload_checksum(mem_type, start_addr, end_addr, size),
        timeout,
        true,
    )?;
    floader::parse_checksum_response(&resp)
}

pub fn floader_read_status_register<T: Transport + ?Sized>(
    t: &mut T,
    cmd: u8,
    address: u8,
) -> Result<u8, FlashError> {
    let resp = floader_send_request(
        t,
        &floader::payload_read_status_register(cmd, address),
        Duration::from_millis(proto::SYNC_RESPONSE_TIMEOUT_MS),
        true,
    )?;
    floader::parse_read_status_register_response(&resp)
}

pub fn floader_otp_read_logical_map<T: Transport + ?Sized>(
    t: &mut T,
    address: u32,
    size: u32,
) -> Result<Vec<u8>, FlashError> {
    let resp = floader_send_request(
        t,
        &floader::payload_otp_read_logical_map(address, size),
        Duration::from_millis(proto::OTP_READ_TIMEOUT_MS),
        true,
    )?;
    floader::parse_otp_read_map_response(&resp).map(|s| s.to_vec())
}

/// Replay a DTR/RTS reset-pulse sequence, e.g. [`proto::RESET_TIMING`].
pub fn run_dtr_rts_timing<T: Transport + ?Sized>(
    t: &mut T,
    steps: &[proto::DtrRtsStep],
) -> Result<(), FlashError> {
    for step in steps {
        match *step {
            proto::DtrRtsStep::Dtr(level) => {
                t.set_dtr(level).map_err(|e| io_fault("set DTR", e))?
            }
            proto::DtrRtsStep::Rts(level) => {
                t.set_rts(level).map_err(|e| io_fault("set RTS", e))?
            }
            proto::DtrRtsStep::DelayMs(ms) => std::thread::sleep(Duration::from_millis(ms as u64)),
        }
    }
    Ok(())
}

// ── pre-flight checks ───────────────────────────────────────────────────
// Read-only; neither check writes to the status register or OTP. A mismatch
// bails out rather than attempting any auto-correction.

/// Confirm none of the NOR status register's block-protect bits are set.
pub fn check_flash_not_locked<T: Transport + ?Sized>(t: &mut T) -> Result<(), FlashError> {
    let status = floader_read_status_register(t, proto::NOR_CMD_READ_STATUS_REG1, 0)?;
    if status & proto::NOR_STATUS_REG1_BP_MASK != 0 {
        return Err(FlashError::FlashLocked);
    }
    Ok(())
}

/// Confirm the device's eFuse-configured SPIC addressing mode is 3-byte
/// (flash size <= 16 MiB) — the only mode this port supports.
pub fn check_supported_flash_size<T: Transport + ?Sized>(t: &mut T) -> Result<(), FlashError> {
    let map = floader_otp_read_logical_map(t, 0, proto::LOGICAL_EFUSE_LEN)?;
    let byte = *map
        .get(proto::OTP_SPIC_ADDR_MODE_BYTE)
        .ok_or(FlashError::SysProto)?;
    if proto::is_4byte_addr_mode(byte) {
        return Err(FlashError::UnsupportedFlashSize);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    /// Scripted in-memory stand-in for a serial port: `rx` is drained byte
    /// by byte by `read`, `tx` records everything written for assertions.
    /// Every read either returns at least one byte or `TimedOut` — there is
    /// no simulated latency, so tests run at full speed regardless of the
    /// timeouts passed in.
    struct MockPort {
        rx: VecDeque<u8>,
        tx: Vec<u8>,
        baud: u32,
    }

    impl MockPort {
        fn new(script: &[u8]) -> Self {
            Self {
                rx: script.iter().copied().collect(),
                tx: Vec::new(),
                baud: 0,
            }
        }
    }

    impl Transport for MockPort {
        fn write_all(&mut self, data: &[u8]) -> std::io::Result<()> {
            self.tx.extend_from_slice(data);
            Ok(())
        }

        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let mut n = 0;
            while n < buf.len() {
                match self.rx.pop_front() {
                    Some(b) => {
                        buf[n] = b;
                        n += 1;
                    }
                    None => break,
                }
            }
            if n == 0 {
                Err(std::io::Error::from(std::io::ErrorKind::TimedOut))
            } else {
                Ok(n)
            }
        }

        fn set_timeout(&mut self, _timeout: Duration) -> std::io::Result<()> {
            Ok(())
        }

        fn set_baud_rate(&mut self, baud: u32) -> std::io::Result<()> {
            self.baud = baud;
            Ok(())
        }

        fn set_dtr(&mut self, _level: bool) -> std::io::Result<()> {
            Ok(())
        }

        fn set_rts(&mut self, _level: bool) -> std::io::Result<()> {
            Ok(())
        }

        fn clear_input(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn rom_check_alive_detects_nak() {
        let mut port = MockPort::new(&[rom::NAK]);
        assert!(rom_check_alive(&mut port, Duration::from_millis(100)).is_ok());
    }

    #[test]
    fn rom_check_alive_times_out_without_nak() {
        let mut port = MockPort::new(&[]);
        assert_eq!(
            rom_check_alive(&mut port, Duration::from_millis(60)),
            Err(FlashError::SysProto)
        );
    }

    #[test]
    fn rom_handshake_switches_baud_on_ack() {
        // check_alive (non-USB) first waits for NAK, then BAUDSET -> ACK,
        // then BAUDCHK (after the host switches baud) -> ACK.
        let mut port = MockPort::new(&[rom::NAK, rom::ACK, rom::ACK]);
        assert!(rom_handshake(&mut port, 1_500_000, false).is_ok());
        assert_eq!(port.baud, 1_500_000);
    }

    #[test]
    fn rom_handshake_reports_not_in_download_mode() {
        let mut port = MockPort::new(&[rom::NAK, 0xE6]); // alive, then DevLength
        assert_eq!(
            rom_handshake(&mut port, 1_500_000, false),
            Err(FlashError::DevLength)
        );
    }

    #[test]
    fn floader_async_overflow_byte_is_retried_not_accepted() {
        // First attempt: 0xE4 (FIFO overflow, frame dropped) — must retry.
        // Second attempt: ACK_BUF_EMPTY — now it succeeds.
        let mut port = MockPort::new(&[floader::ACK_RETRY, floader::ACK_BUF_EMPTY]);
        let result = floader_send_request(
            &mut port,
            &[floader::OP_WRITE],
            Duration::from_millis(100),
            false,
        );
        assert_eq!(result, Ok(Vec::new()));
        // Two full frames were sent — the dropped one, then the resend.
        assert_eq!(
            port.tx.len(),
            floader::encode_request(&[floader::OP_WRITE]).len() * 2
        );
    }

    #[test]
    fn floader_sense_data_mismatch_is_silent_loss_guard() {
        // SENSE response claims the last WRITE landed at 0x0800_0000, but
        // the caller believes it just finished a WRITE at 0x0804_0000 — a
        // dropped frame scenario the guard must catch.
        let mut resp = vec![floader::SOF];
        let payload = [
            floader::OP_SENSE,
            floader::OP_WRITE,
            0x00,
            0x00,
            0x00,
            0x00,
            0x08,
        ];
        let len = payload.len() as u16;
        resp.push(len as u8);
        resp.push((len >> 8) as u8);
        resp.push((len as u8) ^ (len >> 8) as u8);
        resp.extend_from_slice(&payload);
        resp.push(super::proto::checksum8(&payload));

        let mut port = MockPort::new(&resp);
        let result = floader_sense(
            &mut port,
            Duration::from_millis(100),
            Some((floader::OP_WRITE, 0x0804_0000)),
        );
        assert_eq!(result, Err(FlashError::SysProto));
    }

    #[test]
    fn floader_sense_matching_expectation_succeeds() {
        let mut resp = vec![floader::SOF];
        let payload = [
            floader::OP_SENSE,
            floader::OP_WRITE,
            0x00,
            0x00,
            0x00,
            0x00,
            0x08,
        ];
        let len = payload.len() as u16;
        resp.push(len as u8);
        resp.push((len >> 8) as u8);
        resp.push((len as u8) ^ (len >> 8) as u8);
        resp.extend_from_slice(&payload);
        resp.push(super::proto::checksum8(&payload));

        let mut port = MockPort::new(&resp);
        let result = floader_sense(
            &mut port,
            Duration::from_millis(100),
            Some((floader::OP_WRITE, 0x0800_0000)),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn floader_write_page_without_sense_skips_sense_request() {
        let mut port = MockPort::new(&[floader::ACK_BUF_EMPTY]);
        assert!(
            floader_write_page(
                &mut port,
                proto::MEMORY_TYPE_NOR,
                0x0800_0000,
                &[0xAA],
                false,
                Duration::from_millis(100)
            )
            .is_ok()
        );
    }

    #[test]
    fn floader_write_page_with_sense_confirms_matching_address() {
        let mut resp = vec![floader::ACK_BUF_EMPTY, floader::SOF];
        let payload = [
            floader::OP_SENSE,
            floader::OP_WRITE,
            0x00,
            0x00,
            0x00,
            0x00,
            0x08,
        ];
        let len = payload.len() as u16;
        resp.push(len as u8);
        resp.push((len >> 8) as u8);
        resp.push((len as u8) ^ (len >> 8) as u8);
        resp.extend_from_slice(&payload);
        resp.push(super::proto::checksum8(&payload));

        let mut port = MockPort::new(&resp);
        assert!(
            floader_write_page(
                &mut port,
                proto::MEMORY_TYPE_NOR,
                0x0800_0000,
                &[0xAA],
                true,
                Duration::from_millis(100)
            )
            .is_ok()
        );
    }

    #[test]
    fn floader_write_page_with_sense_catches_dropped_frame() {
        // SENSE reports the last WRITE landed at 0x0804_0000, but this call
        // believes it just wrote 0x0800_0000 — the silent-loss guard must
        // surface as an error rather than a silent "success".
        let mut resp = vec![floader::ACK_BUF_EMPTY, floader::SOF];
        let payload = [
            floader::OP_SENSE,
            floader::OP_WRITE,
            0x00,
            0x00,
            0x00,
            0x04,
            0x08,
        ];
        let len = payload.len() as u16;
        resp.push(len as u8);
        resp.push((len >> 8) as u8);
        resp.push((len as u8) ^ (len >> 8) as u8);
        resp.extend_from_slice(&payload);
        resp.push(super::proto::checksum8(&payload));

        let mut port = MockPort::new(&resp);
        assert_eq!(
            floader_write_page(
                &mut port,
                proto::MEMORY_TYPE_NOR,
                0x0800_0000,
                &[0xAA],
                true,
                Duration::from_millis(100)
            ),
            Err(FlashError::SysProto)
        );
    }

    #[test]
    fn floader_erase_with_sense_confirms_matching_start_addr() {
        let mut resp = vec![floader::ACK_BUF_EMPTY, floader::SOF];
        let payload = [
            floader::OP_SENSE,
            floader::OP_FS_ERASE,
            0x00,
            0x00,
            0x00,
            0x00,
            0x08,
        ];
        let len = payload.len() as u16;
        resp.push(len as u8);
        resp.push((len >> 8) as u8);
        resp.push((len as u8) ^ (len >> 8) as u8);
        resp.extend_from_slice(&payload);
        resp.push(super::proto::checksum8(&payload));

        let mut port = MockPort::new(&resp);
        assert!(
            floader_erase(
                &mut port,
                proto::MEMORY_TYPE_NOR,
                0x0800_0000,
                0x0801_0000,
                0x0001_0000,
                true,
                Duration::from_millis(100)
            )
            .is_ok()
        );
    }

    #[test]
    fn check_flash_not_locked_passes_when_bp_bits_clear() {
        let mut resp = vec![floader::SOF];
        let payload = [floader::OP_FS_RDSTS, 0x00];
        let len = payload.len() as u16;
        resp.push(len as u8);
        resp.push((len >> 8) as u8);
        resp.push((len as u8) ^ (len >> 8) as u8);
        resp.extend_from_slice(&payload);
        resp.push(super::proto::checksum8(&payload));

        let mut port = MockPort::new(&resp);
        assert!(check_flash_not_locked(&mut port).is_ok());
    }

    #[test]
    fn check_flash_not_locked_fails_when_bp_bits_set() {
        let mut resp = vec![floader::SOF];
        let payload = [floader::OP_FS_RDSTS, proto::NOR_STATUS_REG1_BP_MASK];
        let len = payload.len() as u16;
        resp.push(len as u8);
        resp.push((len >> 8) as u8);
        resp.push((len as u8) ^ (len >> 8) as u8);
        resp.extend_from_slice(&payload);
        resp.push(super::proto::checksum8(&payload));

        let mut port = MockPort::new(&resp);
        assert_eq!(
            check_flash_not_locked(&mut port),
            Err(FlashError::FlashLocked)
        );
    }

    #[test]
    fn check_supported_flash_size_passes_for_3byte_mode() {
        let mut map = vec![0xFFu8; proto::LOGICAL_EFUSE_LEN as usize];
        map[proto::OTP_SPIC_ADDR_MODE_BYTE] = 0x00;

        let mut resp = vec![floader::SOF];
        let mut payload = vec![floader::OP_OTP_RMAP];
        payload.extend_from_slice(&map);
        let len = payload.len() as u16;
        resp.push(len as u8);
        resp.push((len >> 8) as u8);
        resp.push((len as u8) ^ (len >> 8) as u8);
        resp.extend_from_slice(&payload);
        resp.push(super::proto::checksum8(&payload));

        let mut port = MockPort::new(&resp);
        assert!(check_supported_flash_size(&mut port).is_ok());
    }

    #[test]
    fn check_supported_flash_size_fails_for_4byte_mode() {
        let mut map = vec![0xFFu8; proto::LOGICAL_EFUSE_LEN as usize];
        map[proto::OTP_SPIC_ADDR_MODE_BYTE] = proto::OTP_SPIC_ADDR_MODE_4BYTE;

        let mut resp = vec![floader::SOF];
        let mut payload = vec![floader::OP_OTP_RMAP];
        payload.extend_from_slice(&map);
        let len = payload.len() as u16;
        resp.push(len as u8);
        resp.push((len >> 8) as u8);
        resp.push((len as u8) ^ (len >> 8) as u8);
        resp.extend_from_slice(&payload);
        resp.push(super::proto::checksum8(&payload));

        let mut port = MockPort::new(&resp);
        assert_eq!(
            check_supported_flash_size(&mut port),
            Err(FlashError::UnsupportedFlashSize)
        );
    }

    /// Framed QUERY response reporting `device_id` and `memory_type`.
    fn query_response(device_id: u16, memory_type: u8) -> Vec<u8> {
        let mut payload = vec![0u8; floader::QUERY_RESPONSE_LEN];
        payload[0] = floader::OP_QUERY;
        payload[floader::query_offset::DEVICE_ID..floader::query_offset::DEVICE_ID + 2]
            .copy_from_slice(&device_id.to_le_bytes());
        payload[floader::query_offset::MEMORY_TYPE] = memory_type;

        let mut resp = vec![floader::SOF];
        let len = payload.len() as u16;
        resp.push(len as u8);
        resp.push((len >> 8) as u8);
        resp.push((len as u8) ^ (len >> 8) as u8);
        resp.extend_from_slice(&payload);
        resp.push(super::proto::checksum8(&payload));
        resp
    }

    #[test]
    fn floader_query_accepts_expected_nor_device() {
        let mut port = MockPort::new(&query_response(proto::DEVICE_ID, proto::MEMORY_TYPE_NOR));
        assert!(floader_query(&mut port).is_ok());
    }

    #[test]
    fn floader_query_rejects_nand() {
        let mut port = MockPort::new(&query_response(proto::DEVICE_ID, 2));
        assert_eq!(floader_query(&mut port), Err(FlashError::SysParameter));
    }

    #[test]
    fn floader_query_rejects_other_chip() {
        let mut port = MockPort::new(&query_response(0x1234, proto::MEMORY_TYPE_NOR));
        assert_eq!(floader_query(&mut port), Err(FlashError::SysParameter));
    }

    #[test]
    fn floader_handshake_gives_up_after_retry_budget() {
        let mut port = MockPort::new(&[]);
        assert!(floader_handshake(&mut port).is_err());
    }

    #[test]
    fn rom_download_floader_sends_whole_payload_as_acked_pages() {
        // Floader exactly 2 pages (page_size=1024) once the 4-byte baud is
        // appended and the combined buffer is padded — see rom::tests for
        // the padding-boundary behavior itself; this only checks that the
        // transport layer ACKs every page plus the final EOT.
        let floader_bin = vec![0x11u8; 1024];
        let acks = vec![rom::ACK; 3]; // 2 STX pages + EOT
        let mut port = MockPort::new(&acks);
        assert!(rom_download_floader(&mut port, &floader_bin, 1_500_000, false).is_ok());
    }
}
