//! ROM-stage wire format: the xmodem-like protocol the chip's boot ROM
//! speaks before any floader is running. Used only to load the floader
//! binary into RAM and hand off to it.
//!
//! Pure codec layer only — no serial I/O, no retry/timeout logic. The
//! transport layer (Phase 2's `Transport` trait + state machine) drives
//! these encoders/decoders and owns the actual read/write/retry sequencing
//! (`handshake()`, `check_alive()`, `send_request()` in the reference
//! implementation).

// ── flow-control bytes ───────────────────────────────────────────────────

/// Begin a data-transfer packet.
pub const STX: u8 = 0x02;
/// End of all transfers for the current session.
pub const EOT: u8 = 0x04;
/// Handshake: host proposes a baud-rate table index.
pub const BAUDSET: u8 = 0x05;
/// Handshake: host confirms the baud switch took effect (UART only).
pub const BAUDCHK: u8 = 0x07;
/// Positive acknowledge.
pub const ACK: u8 = 0x06;
/// Negative acknowledge.
pub const NAK: u8 = 0x15;
/// Device-initiated cancel (UART only).
pub const CAN: u8 = 0x18;
/// Host-initiated cancel.
pub const ESC: u8 = 0x1B;

/// Bytes of fixed STX packet overhead before the data payload: `STX,
/// packet_no, ~packet_no` (3) + 4-byte address + checksum (1) = 8.
pub const STX_BASE_LEN: usize = 8;
/// STX data payload size per packet over UART.
pub const STX_UART_DATA_LEN: usize = 1024;
/// STX data payload size per packet over USB.
pub const STX_USB_DATA_LEN: usize = 2048;
/// Largest floader binary this stage will transfer.
pub const FLOADER_SIZE_LIMIT: usize = 1024 * 1024;

/// ROM's built-in baud-rate table: `BAUDSET`'s argument is an index into
/// this table, not the raw rate. Rates absent from the table (including any
/// the ROM doesn't support) fall back to 115200's index.
pub fn baudrate_table_index(rate: u32) -> u8 {
    match rate {
        110 => 0,
        300 => 1,
        600 => 2,
        1200 => 3,
        2400 => 4,
        4800 => 5,
        9600 => 6,
        14400 => 7,
        19200 => 8,
        28800 => 9,
        38400 => 10,
        57600 => 11,
        76800 => 12,
        115200 => 13,
        128000 => 14,
        153600 => 15,
        230400 => 16,
        380400 => 17,
        460800 => 18,
        500000 => 19,
        921600 => 20,
        1_000_000 => 21,
        1_382_400 => 22,
        1_444_400 => 23,
        1_500_000 => 24,
        1_843_200 => 25,
        2_000_000 => 26,
        2_100_000 => 27,
        2_764_800 => 28,
        3_000_000 => 29,
        3_250_000 => 30,
        3_692_300 => 31,
        3_750_000 => 32,
        4_000_000 => 33,
        6_000_000 => 34,
        _ => 13,
    }
}

/// Build the `BAUDSET` request payload for `rate`.
pub fn encode_baudset(rate: u32) -> [u8; 2] {
    [BAUDSET, baudrate_table_index(rate)]
}

// ── STX data packets ─────────────────────────────────────────────────────

/// Build one STX packet: `[STX, seq, !seq] + address(4B LE) + data +
/// checksum`. `checksum` is the unsigned byte sum (mod 256) of everything
/// from the address field onward — the sequence header is excluded.
/// `seq` wraps at 256 and is never 0 (the first packet in a session is 1).
pub fn encode_stx_packet(seq: u8, address: u32, data: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(STX_BASE_LEN + data.len());
    packet.push(STX);
    packet.push(seq);
    packet.push(!seq);
    packet.extend_from_slice(&address.to_le_bytes());
    packet.extend_from_slice(data);
    packet.push(super::proto::checksum8(&packet[3..]));
    packet
}

// ── floader payload preparation ─────────────────────────────────────────

/// Round `size` up to the next multiple of `page_size` (no-op if already
/// aligned).
pub fn page_aligned_size(size: usize, page_size: usize) -> usize {
    size.div_ceil(page_size) * page_size
}

/// Build the exact byte buffer the ROM stage transfers to load the
/// floader: the floader image with the host's current baud rate appended
/// as 4 little-endian bytes (the floader reads this trailing word to know
/// what baud its own handshake will run at), then zero-padded to a whole
/// number of STX data pages.
///
/// Padding must happen on the *combined* floader+baudrate buffer, not on
/// the floader alone: if the floader's own size already lands on a page
/// boundary, padding it first and appending the baud rate afterward would
/// leave those 4 bytes in a page that never gets transferred, so the
/// device reads garbage for its handshake baud rate and times out.
pub fn prepare_floader_payload(floader: &[u8], baudrate: u32, page_size: usize) -> Vec<u8> {
    let mut payload = Vec::with_capacity(floader.len() + 4);
    payload.extend_from_slice(floader);
    payload.extend_from_slice(&baudrate.to_le_bytes());
    let aligned = page_aligned_size(payload.len(), page_size);
    payload.resize(aligned, 0x00);
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baud_table_known_and_unknown_rates() {
        assert_eq!(baudrate_table_index(1_500_000), 24);
        assert_eq!(baudrate_table_index(115200), 13);
        assert_eq!(baudrate_table_index(999_999_999), 13);
    }

    #[test]
    fn stx_packet_layout_and_checksum() {
        let packet = encode_stx_packet(1, 0x3007_0000, &[0xAA, 0xBB]);
        assert_eq!(packet[0], STX);
        assert_eq!(packet[1], 1);
        assert_eq!(packet[2], !1u8);
        assert_eq!(&packet[3..7], &0x3007_0000u32.to_le_bytes());
        assert_eq!(&packet[7..9], &[0xAA, 0xBB]);
        let checksum = *packet.last().unwrap();
        assert_eq!(
            super::super::proto::checksum8(&packet[3..packet.len() - 1]),
            checksum
        );
    }

    #[test]
    fn page_aligned_size_rounds_up() {
        assert_eq!(page_aligned_size(1024, 1024), 1024);
        assert_eq!(page_aligned_size(1025, 1024), 2048);
        assert_eq!(page_aligned_size(0, 1024), 0);
    }

    #[test]
    fn floader_payload_pads_combined_buffer() {
        // Floader exactly one page long: padding only the floader (not
        // floader+baud) would strand the baud bytes in an untransferred
        // second page. Combined-buffer padding must produce two full pages
        // instead, with the baud rate inside the transferred range.
        let floader = vec![0x11u8; 1024];
        let payload = prepare_floader_payload(&floader, 1_500_000, 1024);
        assert_eq!(payload.len(), 2048);
        assert_eq!(&payload[1024..1028], &1_500_000u32.to_le_bytes());
        assert!(payload[1028..].iter().all(|&b| b == 0x00));
    }

    #[test]
    fn floader_payload_already_aligned_with_room_for_baud() {
        let floader = vec![0x22u8; 1020];
        let payload = prepare_floader_payload(&floader, 115200, 1024);
        assert_eq!(payload.len(), 1024);
        assert_eq!(&payload[1020..1024], &115200u32.to_le_bytes());
    }
}
