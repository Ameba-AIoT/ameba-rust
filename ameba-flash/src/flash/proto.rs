//! Shared protocol primitives for the UART flashing protocol: error codes,
//! timing/retry constants, DTR/RTS reset-timing tables, and the fixed device
//! profile for this chip + NOR flash combination.
//!
//! Pure data and pure functions only — no serial I/O. [`rom`](super::rom) and
//! [`floader`](super::floader) build on top of these.

// ── error codes ──────────────────────────────────────────────────────────

/// Negative-response codes. Device-side codes (`Dev*`) are parsed from a
/// response's first byte (`0xE1..=0xE8` — the NAND-only range `0xE9..=0xED`
/// is intentionally unmapped, this port never talks to NAND); `Sys*` codes
/// are constructed directly by the transport/state-machine layer for
/// host-side failures (timeout, I/O, framing) that never arrive as a byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashError {
    DevBusy,
    DevTimeout,
    DevFull,
    DevInvalid,
    DevLength,
    DevChecksum,
    DevAddress,
    SysTimeout,
    SysParameter,
    SysIo,
    SysNak,
    SysProto,
    SysChecksum,
    SysOverrange,
    SysCancel,
    /// Block-protect bits in the NOR status register are set: some region of
    /// flash is write-protected.
    FlashLocked,
    /// The device's eFuse-configured SPIC addressing mode doesn't match what
    /// this port supports.
    UnsupportedFlashSize,
}

impl FlashError {
    /// Map a device response's first byte to an error, if it is one.
    /// Non-error opcodes and ack bytes are below `0xE0` and return `None`.
    pub fn from_dev_byte(b: u8) -> Option<Self> {
        Some(match b {
            0xE1 => Self::DevBusy,
            0xE2 => Self::DevTimeout,
            0xE4 => Self::DevFull,
            0xE5 => Self::DevInvalid,
            0xE6 => Self::DevLength,
            0xE7 => Self::DevChecksum,
            0xE8 => Self::DevAddress,
            _ => return None,
        })
    }
}

impl std::fmt::Display for FlashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for FlashError {}

// ── checksum ──────────────────────────────────────────────────────────────

/// Checksum shared by ROM-stage STX packets and floader frames: the
/// unsigned byte sum of `data`, truncated to 8 bits.
pub fn checksum8(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
}

// ── timing & retry constants ───────────────────────────────────────────────
// Protocol-level timeouts and retry counts; comments describe only the
// protocol step each applies to.

/// Request retries before a command gives up.
pub const REQUEST_RETRY_COUNT: u32 = 3;
/// Delay between retries of a failed request.
pub const REQUEST_RETRY_INTERVAL_MS: u64 = 10;
/// Timeout waiting for an async command's single-byte ack.
pub const ASYNC_RESPONSE_TIMEOUT_MS: u64 = 1000;
/// Timeout waiting for a sync command's full framed response.
pub const SYNC_RESPONSE_TIMEOUT_MS: u64 = 1000;
/// Settle time after changing the serial port's baud rate.
pub const BAUDRATE_SWITCH_DELAY_MS: u64 = 200;
/// Delay after entering ROM download mode, before the first handshake byte.
pub const ROM_BOOT_DELAY_MS: u64 = 100;
/// Delay after switching to the floader's init baud, before its handshake.
pub const FLOADER_BOOT_DELAY_MS: u64 = 1000;
/// Timeout for a WRITE command's async ack (longer than the generic async
/// timeout: a page write plus an optional periodic SENSE can take longer).
pub const WRITE_RESPONSE_TIMEOUT_MS: u64 = 2000;
/// Timeout polling for the ROM stage's NAK liveness byte.
pub const ROM_CHECK_ALIVE_TIMEOUT_MS: u64 = 1000;
/// Timeout for an OTP map read — longer than other sync responses because a
/// physical-map read takes up to ~170 ms on real hardware.
pub const OTP_READ_TIMEOUT_MS: u64 = 10_000;
/// How many WRITE pages between periodic SENSE acknowledgements.
pub const SENSE_PACKET_COUNT: u32 = 32;

// ── DTR/RTS reset timing ─────────────────────────────────────────────────

/// One step of a DTR/RTS reset sequence: set a line level, or hold for a
/// delay. A full sequence must save the port's original DTR/RTS state
/// before running and restore it afterward — the steps below only cover the
/// pulse itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DtrRtsStep {
    Dtr(bool),
    Rts(bool),
    DelayMs(u32),
}

/// Reset into normal boot after flashing is complete.
pub const RESET_TIMING: &[DtrRtsStep] = &[
    DtrRtsStep::Dtr(false),
    DtrRtsStep::Rts(true),
    DtrRtsStep::DelayMs(200),
    DtrRtsStep::Rts(false),
    DtrRtsStep::Dtr(false),
];

// ── device profile (NOR) ───────────────────────────────────────────────────
// Fixed values for this chip/flash combination.

/// Device ID the floader's QUERY response must report.
pub const DEVICE_ID: u16 = 0x7005;
/// Flash memory type QUERY must report, and the type byte WRITE/FS_ERASE/
/// CHKSM use to select NOR flash.
pub const MEMORY_TYPE_NOR: u8 = 1;
/// RAM load address for the floader binary during the ROM-stage STX
/// transfer.
pub const FLOADER_ADDRESS: u32 = 0x3007_0000;
/// Baud rate for the ROM-stage and floader handshakes.
pub const HANDSHAKE_BAUDRATE: u32 = 1_500_000;
/// eFuse logical map size read by `OTP_RMAP`.
pub const LOGICAL_EFUSE_LEN: u32 = 1024;

/// Base of flash in raw flash-bus addressing; flash ends at this plus the
/// capacity QUERY reports.
pub const FLASH_START_ADDRESS: u32 = 0x0800_0000;
/// `boot.bin`'s flash range (raw flash-bus addressing, end-exclusive).
pub const IMAGE_BOOT_RANGE: (u32, u32) = (0x0800_0000, 0x0804_0000);
/// `app.bin`'s flash range (raw flash-bus addressing, end-exclusive).
pub const IMAGE_APP_RANGE: (u32, u32) = (0x0804_0000, 0x081C_0000);

// ── NOR flash status / erase-block constants ──────────────────────────────

/// `FS_RDSTS` command byte for "read status register 1".
pub const NOR_CMD_READ_STATUS_REG1: u8 = 0x05;
/// Block-protect bitfield within status register 1. Non-zero means some
/// region of flash is write-protected.
pub const NOR_STATUS_REG1_BP_MASK: u8 = 0x3C;
/// NOR page size in bytes — the WRITE command's per-page chunk size.
pub const NOR_PAGE_SIZE: u32 = 1024;

/// Pick the largest erase-block size the current address and remaining
/// image length both support: a bigger block erases faster, but needs the
/// address aligned to it with enough image left to fill it. Falls back to
/// 4 KiB (always aligned, since `addr` is itself page-aligned).
pub fn erase_block_size(addr: u32, remaining: u32) -> u32 {
    let page = NOR_PAGE_SIZE;
    if addr.is_multiple_of(64 * page) && remaining >= 64 * page {
        64 * page
    } else if addr.is_multiple_of(32 * page) && remaining >= 32 * page {
        32 * page
    } else {
        4 * page
    }
}

// ── size-scaled operation timeouts ─────────────────────────────────────────
// Erase and write take longer the bigger the block, so (unlike the fixed
// per-command timeouts above) these scale with the erase-block size chosen
// by `erase_block_size`. Only the three block sizes that function can ever
// return are handled — this port never issues an erase of any other size.

/// Per-page NOR program time budgeted into the WRITE-then-SENSE wait.
pub const NOR_PAGE_PROGRAM_TIMEOUT_MS: u64 = 5;
/// Raw (pre-floor) erase time for a 4 KiB sector.
const NOR_4K_SECTOR_ERASE_RAW_MS: u64 = 800;
/// Raw (pre-floor) erase time for a 32 KiB block.
const NOR_32K_BLOCK_ERASE_RAW_MS: u64 = 2000;
/// Raw (pre-floor) erase time for a 64 KiB block.
const NOR_64K_BLOCK_ERASE_RAW_MS: u64 = 2500;

/// Ceiling-divide, matching the reference's `divide_then_round_up`.
pub fn divide_then_round_up(dividend: u64, divisor: u64) -> u64 {
    dividend.div_ceil(divisor)
}

/// Timeout for erasing `block_size` bytes: the reference computes this in
/// floating-point seconds and truncates to a whole number of seconds (not
/// milliseconds) before using it, so e.g. the 64 KiB block's raw 2.5 s
/// floors to 2 s, not 2.5 s — this floors the same way, in the millisecond
/// domain, before converting back to milliseconds.
pub fn nor_erase_timeout_ms(block_size: u32) -> u64 {
    let raw_ms = if block_size == 4 * NOR_PAGE_SIZE {
        NOR_4K_SECTOR_ERASE_RAW_MS
    } else if block_size == 32 * NOR_PAGE_SIZE {
        NOR_32K_BLOCK_ERASE_RAW_MS
    } else {
        NOR_64K_BLOCK_ERASE_RAW_MS
    };
    (raw_ms / 1000).max(1) * 1000
}

/// Timeout for a WRITE's post-completion SENSE wait: a per-page program
/// budget (scaled by the larger of the sense interval or the whole block,
/// since a block-ending WRITE's SENSE must outlast every page since the
/// last one) plus the block's own erase timeout, since a block's first
/// WRITE can land while that block's erase is still finishing.
pub fn nor_write_timeout_ms(pages_per_block: u32, block_size: u32) -> u64 {
    let page_component =
        NOR_PAGE_PROGRAM_TIMEOUT_MS * u64::from(SENSE_PACKET_COUNT.max(pages_per_block));
    page_component + nor_erase_timeout_ms(block_size)
}

/// Checksum compute time per KiB, in milliseconds (the reference's 0.001 s
/// per KiB).
const NOR_1KIB_CHECKSUM_MS: u64 = 1;

/// Timeout for checksumming/reading `size_in_byte` bytes, floored to whole
/// seconds the same way as [`nor_erase_timeout_ms`] (the reference's
/// `int(max(.., MinFlashProcessTimeoutInSecond))` truncates a sub-second
/// float down to 0, then floors that up to the 1-second minimum).
pub fn nor_checksum_timeout_ms(size_in_byte: u32) -> u64 {
    let kib = divide_then_round_up(u64::from(size_in_byte), 1024);
    let raw_s = (kib * NOR_1KIB_CHECKSUM_MS) / 1000;
    raw_s.max(1) * 1000
}

// ── OTP SPIC address-mode check ────────────────────────────────────────────

/// Byte offset within the eFuse logical map holding the SPIC address-mode
/// bit.
pub const OTP_SPIC_ADDR_MODE_BYTE: usize = 0x02;
/// Bitmask isolating the address-mode bit within that byte.
pub const OTP_SPIC_ADDR_MODE_MASK: u8 = 0x02;
/// Masked value meaning "4-byte addressing" (flash > 16 MiB). Any other
/// masked value, including the erased `0xFF` default, means 3-byte.
pub const OTP_SPIC_ADDR_MODE_4BYTE: u8 = 0x02;

/// True if the eFuse logical-map byte at `OTP_SPIC_ADDR_MODE_BYTE` selects
/// 4-byte SPIC addressing.
pub fn is_4byte_addr_mode(efuse_byte: u8) -> bool {
    efuse_byte != 0xFF && (efuse_byte & OTP_SPIC_ADDR_MODE_MASK) == OTP_SPIC_ADDR_MODE_4BYTE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum8_wraps() {
        assert_eq!(checksum8(&[0x01, 0x02, 0xFF, 0xFF]), 0x01);
        assert_eq!(checksum8(&[]), 0);
    }

    #[test]
    fn dev_byte_mapping() {
        assert_eq!(FlashError::from_dev_byte(0xE4), Some(FlashError::DevFull));
        assert_eq!(FlashError::from_dev_byte(0xE0), None);
        assert_eq!(FlashError::from_dev_byte(0x07), None);
    }

    #[test]
    fn erase_block_size_picks_largest_aligned() {
        assert_eq!(erase_block_size(0, 100 * NOR_PAGE_SIZE), 64 * NOR_PAGE_SIZE);
        assert_eq!(
            erase_block_size(64 * NOR_PAGE_SIZE, 40 * NOR_PAGE_SIZE),
            32 * NOR_PAGE_SIZE
        );
        assert_eq!(
            erase_block_size(NOR_PAGE_SIZE, 100 * NOR_PAGE_SIZE),
            4 * NOR_PAGE_SIZE
        );
    }

    #[test]
    fn addr_mode_detection() {
        assert!(!is_4byte_addr_mode(0xFF));
        assert!(!is_4byte_addr_mode(0x00));
        assert!(is_4byte_addr_mode(OTP_SPIC_ADDR_MODE_4BYTE));
    }
}
