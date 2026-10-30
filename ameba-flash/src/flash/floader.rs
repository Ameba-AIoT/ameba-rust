//! Floader-stage wire format: opcodes, frame encode/decode, and response
//! parsing for the small RAM-resident program the ROM stage loads before any
//! erase/program/read happens.
//!
//! Pure codec layer only — no serial I/O. Building a request and parsing a
//! response are separate steps so the transport layer
//! ([`transport`](super::transport)) can own retries and timeouts without this
//! module knowing about either.

use super::proto::checksum8;

// ── opcodes ─────────────────────────────────────────────────────────────

pub const OP_QUERY: u8 = 0x02;
pub const OP_WRITE: u8 = 0x84;
pub const OP_READ: u8 = 0x05;
pub const OP_CHKSM: u8 = 0x06;
pub const OP_SENSE: u8 = 0x07;
pub const OP_NEXTOP: u8 = 0x08;

pub const OP_FS_ERASE: u8 = 0xA0;
pub const OP_FS_RDSTS: u8 = 0x21;

pub const OP_OTP_RMAP: u8 = 0x42;

/// Frame start byte for every floader request.
pub const SOF: u8 = 0xA5;

/// Single-byte async acks (as opposed to the `0xE0..` error range).
pub const ACK_BUF_EMPTY: u8 = 0xB0;
pub const ACK_BUF_FULL: u8 = 0xB1;
/// Queue-full retry signal: NOT a success ack despite being below the error
/// range — the caller must resend, not advance.
pub const ACK_RETRY: u8 = 0xE4;

/// `NEXTOP` operand selecting what the floader does after the current
/// session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextOp {
    Reset = 2,
}

// ── frame encode ───────────────────────────────────────────────────────

/// Build a complete request frame: `SOF, len_lo, len_hi, len_lo^len_hi,
/// payload..., checksum`. `payload` is the opcode byte followed by that
/// command's arguments.
pub fn encode_request(payload: &[u8]) -> Vec<u8> {
    let len = payload.len() as u16;
    let (len_lo, len_hi) = (len as u8, (len >> 8) as u8);
    let mut frame = Vec::with_capacity(4 + payload.len() + 1);
    frame.push(SOF);
    frame.push(len_lo);
    frame.push(len_hi);
    frame.push(len_lo ^ len_hi);
    frame.extend_from_slice(payload);
    frame.push(checksum8(payload));
    frame
}

// ── sync response decode ───────────────────────────────────────────────

/// Validate a sync response's 3-byte length header (`len_lo, len_hi,
/// len_lo^len_hi` immediately after `SOF`) and return the declared payload
/// length.
pub fn decode_length_header(header: &[u8; 3]) -> Result<u16, super::proto::FlashError> {
    let (len_lo, len_hi, xor) = (header[0], header[1], header[2]);
    if len_lo ^ len_hi != xor {
        return Err(super::proto::FlashError::SysProto);
    }
    Ok(u16::from(len_lo) | (u16::from(len_hi) << 8))
}

/// Validate and strip a sync response's trailing checksum byte, returning
/// the payload. `body` is `payload... + checksum` (everything after the
/// length header).
pub fn decode_payload(body: &[u8]) -> Result<&[u8], super::proto::FlashError> {
    let (&checksum, rest) = body
        .split_last()
        .ok_or(super::proto::FlashError::SysProto)?;
    if checksum8(rest) != checksum {
        return Err(super::proto::FlashError::SysChecksum);
    }
    Ok(rest)
}

/// Classify a single async-command ack byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncAck {
    Success,
    Retry,
    Error(super::proto::FlashError),
}

pub fn decode_async_ack(byte: u8) -> AsyncAck {
    match byte {
        ACK_BUF_EMPTY | ACK_BUF_FULL => AsyncAck::Success,
        ACK_RETRY => AsyncAck::Retry,
        b => match super::proto::FlashError::from_dev_byte(b) {
            Some(e) => AsyncAck::Error(e),
            None => AsyncAck::Error(super::proto::FlashError::SysProto),
        },
    }
}

// ── SENSE response ─────────────────────────────────────────────────────

/// Parsed `SENSE` response: which opcode it reports on, that opcode's
/// completion status, and an opcode-specific data word (e.g. the address a
/// WRITE or FS_ERASE just completed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SenseResult {
    pub op_code: u8,
    pub status: u8,
    pub data: u32,
}

/// Parse a 7-byte SENSE response: `[echo, op_code, status, data_le_u32]`.
/// `echo` (`resp[0]`) is not returned — callers that need to detect dropped
/// frames compare the caller-supplied `expect_op`/`expect_data` against the
/// parsed fields instead.
pub fn parse_sense_response(resp: &[u8]) -> Result<SenseResult, super::proto::FlashError> {
    if resp.len() < 7 {
        return Err(super::proto::FlashError::SysProto);
    }
    Ok(SenseResult {
        op_code: resp[1],
        status: resp[2],
        data: u32::from_le_bytes([resp[3], resp[4], resp[5], resp[6]]),
    })
}

// ── QUERY response ──────────────────────────────────────────────────────

/// Byte offsets into a 129-byte QUERY response. Multi-byte fields are
/// little-endian; every field (per the ROM's response framing) is read one
/// byte past its nominal offset.
pub mod query_offset {
    pub const DEVICE_ID: usize = 1;
    pub const MEMORY_TYPE: usize = 6 + 1;
    pub const FLASH_PAGE_SIZE: usize = 42 + 1;
    pub const FLASH_PAGES_PER_BLOCK: usize = 48 + 1;
    pub const FLASH_BLOCKS_PER_LUN: usize = 52 + 1;
    pub const FLASH_CAPACITY: usize = 61 + 1;
    pub const WIFI_MAC: usize = 71 + 1;
}

/// Minimum QUERY response length these parsers assume (`WIFI_MAC` at offset 71+1
/// plus its 6 bytes is the last field).
pub const QUERY_RESPONSE_LEN: usize = 129;

fn le_u16(resp: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([resp[off], resp[off + 1]])
}

fn le_u32(resp: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([resp[off], resp[off + 1], resp[off + 2], resp[off + 3]])
}

/// Device identity and NOR geometry parsed out of a 129-byte QUERY
/// response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub device_id: u16,
    pub memory_type: u8,
    pub flash_page_size: u32,
    pub flash_pages_per_block: u32,
    pub flash_blocks_per_lun: u32,
    pub flash_capacity: u32,
    pub wifi_mac: [u8; 6],
}

pub fn parse_query_response(resp: &[u8]) -> Result<DeviceInfo, super::proto::FlashError> {
    if resp.len() < QUERY_RESPONSE_LEN {
        return Err(super::proto::FlashError::SysProto);
    }
    let mut wifi_mac = [0u8; 6];
    wifi_mac.copy_from_slice(&resp[query_offset::WIFI_MAC..query_offset::WIFI_MAC + 6]);
    Ok(DeviceInfo {
        device_id: le_u16(resp, query_offset::DEVICE_ID),
        memory_type: resp[query_offset::MEMORY_TYPE],
        flash_page_size: le_u32(resp, query_offset::FLASH_PAGE_SIZE),
        flash_pages_per_block: le_u32(resp, query_offset::FLASH_PAGES_PER_BLOCK),
        flash_blocks_per_lun: le_u32(resp, query_offset::FLASH_BLOCKS_PER_LUN),
        flash_capacity: le_u32(resp, query_offset::FLASH_CAPACITY),
        wifi_mac,
    })
}

// ── command payload builders ────────────────────────────────────────────
// Each builds the payload (opcode + arguments) that `encode_request` wraps
// into a frame. Response parsing for the sync commands lives below.

pub fn payload_query() -> Vec<u8> {
    vec![OP_QUERY]
}

pub fn payload_next_op(op: NextOp, operand: u32) -> Vec<u8> {
    let mut p = Vec::with_capacity(6);
    p.push(OP_NEXTOP);
    p.push(op as u8);
    p.extend_from_slice(&operand.to_le_bytes());
    p
}

pub fn payload_write(mem_type: u8, addr: u32, data: &[u8]) -> Vec<u8> {
    let mut p = Vec::with_capacity(5 + data.len());
    p.push(OP_WRITE);
    p.push(mem_type);
    p.extend_from_slice(&addr.to_le_bytes());
    p.extend_from_slice(data);
    p
}

pub fn payload_read(mem_type: u8, addr: u32, size: u32) -> Vec<u8> {
    let mut p = Vec::with_capacity(9);
    p.push(OP_READ);
    p.push(mem_type);
    p.extend_from_slice(&addr.to_le_bytes());
    p.extend_from_slice(&size.to_le_bytes());
    p
}

/// Parse a READ response: validates the echoed opcode and extracts the
/// `size`-byte payload.
pub fn parse_read_response(resp: &[u8], size: usize) -> Result<&[u8], super::proto::FlashError> {
    if resp.len() < 1 + size || resp[0] != OP_READ {
        return Err(super::proto::FlashError::SysProto);
    }
    Ok(&resp[1..1 + size])
}

pub fn payload_checksum(mem_type: u8, start_addr: u32, end_addr: u32, size: u32) -> Vec<u8> {
    let mut p = Vec::with_capacity(13);
    p.push(OP_CHKSM);
    p.push(mem_type);
    p.extend_from_slice(&start_addr.to_le_bytes());
    p.extend_from_slice(&end_addr.to_le_bytes());
    p.extend_from_slice(&size.to_le_bytes());
    p
}

/// Parse a CHKSM response: validates the echoed opcode and extracts the
/// little-endian checksum word.
pub fn parse_checksum_response(resp: &[u8]) -> Result<u32, super::proto::FlashError> {
    if resp.len() < 5 || resp[0] != OP_CHKSM {
        return Err(super::proto::FlashError::SysProto);
    }
    Ok(le_u32(resp, 1))
}

pub fn payload_erase_flash(
    mem_type: u8,
    force: bool,
    start_addr: u32,
    end_addr: u32,
    size: u32,
) -> Vec<u8> {
    let mut p = Vec::with_capacity(14);
    p.push(OP_FS_ERASE);
    p.push(mem_type);
    p.push(force as u8);
    p.extend_from_slice(&start_addr.to_le_bytes());
    p.extend_from_slice(&end_addr.to_le_bytes());
    p.extend_from_slice(&size.to_le_bytes());
    p
}

pub fn payload_read_status_register(cmd: u8, address: u8) -> Vec<u8> {
    vec![OP_FS_RDSTS, cmd, address]
}

/// Parse an FS_RDSTS response: validates the echoed opcode and extracts the
/// status byte.
pub fn parse_read_status_register_response(resp: &[u8]) -> Result<u8, super::proto::FlashError> {
    if resp.len() < 2 || resp[0] != OP_FS_RDSTS {
        return Err(super::proto::FlashError::SysProto);
    }
    Ok(resp[1])
}

pub fn payload_otp_read_logical_map(address: u32, size: u32) -> Vec<u8> {
    let mut p = Vec::with_capacity(9);
    p.push(OP_OTP_RMAP);
    p.extend_from_slice(&address.to_le_bytes());
    p.extend_from_slice(&size.to_le_bytes());
    p
}

/// Parse an OTP_RMAP response: validates the echoed opcode and strips it,
/// returning the raw map bytes.
pub fn parse_otp_read_map_response(resp: &[u8]) -> Result<&[u8], super::proto::FlashError> {
    let (&op, rest) = resp
        .split_first()
        .ok_or(super::proto::FlashError::SysProto)?;
    if op != OP_OTP_RMAP {
        return Err(super::proto::FlashError::SysProto);
    }
    Ok(rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_request_roundtrip() {
        let payload = payload_query();
        let frame = encode_request(&payload);
        assert_eq!(frame[0], SOF);
        let len = decode_length_header(&[frame[1], frame[2], frame[3]]).unwrap();
        assert_eq!(len as usize, payload.len());
        let body = &frame[4..];
        let decoded = decode_payload(body).unwrap();
        assert_eq!(decoded, payload.as_slice());
    }

    #[test]
    fn length_header_rejects_bad_xor() {
        assert!(decode_length_header(&[0x01, 0x00, 0xFF]).is_err());
        assert!(decode_length_header(&[0x01, 0x00, 0x01]).is_ok());
    }

    #[test]
    fn payload_checksum_rejects_corruption() {
        let mut body = encode_request(&payload_query())[4..].to_vec();
        let last = body.len() - 1;
        body[last] ^= 0xFF;
        assert_eq!(
            decode_payload(&body),
            Err(super::super::proto::FlashError::SysChecksum)
        );
    }

    #[test]
    fn async_ack_classification() {
        assert_eq!(decode_async_ack(ACK_BUF_FULL), AsyncAck::Success);
        assert_eq!(decode_async_ack(ACK_BUF_EMPTY), AsyncAck::Success);
        assert_eq!(decode_async_ack(ACK_RETRY), AsyncAck::Retry);
        assert_eq!(
            decode_async_ack(0xE2),
            AsyncAck::Error(super::super::proto::FlashError::DevTimeout)
        );
    }

    #[test]
    fn sense_response_parse() {
        let resp = [0xAA, OP_WRITE, 0x00, 0x00, 0x00, 0x00, 0x08];
        let sense = parse_sense_response(&resp).unwrap();
        assert_eq!(sense.op_code, OP_WRITE);
        assert_eq!(sense.status, 0x00);
        assert_eq!(sense.data, 0x0800_0000);
    }

    #[test]
    fn sense_response_short_is_error() {
        let resp = [0xAA, OP_WRITE, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(
            parse_sense_response(&resp),
            Err(super::super::proto::FlashError::SysProto)
        );
    }

    #[test]
    fn checksum_response_parse() {
        let mut resp = vec![OP_CHKSM];
        resp.extend_from_slice(&0x1234_5678u32.to_le_bytes());
        assert_eq!(parse_checksum_response(&resp).unwrap(), 0x1234_5678);
    }

    #[test]
    fn query_response_parse() {
        let mut resp = vec![0u8; QUERY_RESPONSE_LEN];
        resp[query_offset::DEVICE_ID..query_offset::DEVICE_ID + 2]
            .copy_from_slice(&0x7005u16.to_le_bytes());
        resp[query_offset::MEMORY_TYPE] = 1;
        resp[query_offset::FLASH_PAGE_SIZE..query_offset::FLASH_PAGE_SIZE + 4]
            .copy_from_slice(&1024u32.to_le_bytes());
        let info = parse_query_response(&resp).unwrap();
        assert_eq!(info.device_id, 0x7005);
        assert_eq!(info.memory_type, 1);
        assert_eq!(info.flash_page_size, 1024);
    }
}
