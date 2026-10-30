use rtl8721f_pac::{UartlogReg, uartlog_reg};

// LOGUART register block at 0x4081_0000
fn regs() -> &'static uartlog_reg::RegisterBlock {
    unsafe { &*UartlogReg::PTR }
}

// ── Blocking character output via PAC (replaces ROM LOGUART_PutChar) ──────

/// Write one byte to LOGUART path 1 (AP/KM4TZ, THR0).
///
/// Polls LSR.tp1f_not_full, then writes to THR0.  LF (`\n`) is followed by
/// CR (`\r`) — matching SDK `LOGUART_PutChar`.
fn putchar(c: u8) {
    while !regs().uart_lsr().read().tp1f_not_full().bit() {}
    regs().uart_thr0().write(|w| unsafe { w.bits(c as u32) });
    if c == b'\n' {
        while !regs().uart_lsr().read().tp1f_not_full().bit() {}
        regs()
            .uart_thr0()
            .write(|w| unsafe { w.bits(b'\r' as u32) });
    }
}

/// Diagnostic: bounded-spin variant of `putchar` used only to test whether
/// an unbounded FIFO-full wait is the actual cause of an observed system
/// freeze. Drops the byte (and reports it) instead of blocking forever if
/// `tp1f_not_full` never asserts within `SPIN_LIMIT` polls.
fn putchar_bounded(c: u8) -> bool {
    const SPIN_LIMIT: u32 = 200_000;
    let mut i = 0;
    while !regs().uart_lsr().read().tp1f_not_full().bit() {
        i += 1;
        if i >= SPIN_LIMIT {
            return false;
        }
    }
    regs().uart_thr0().write(|w| unsafe { w.bits(c as u32) });
    if c == b'\n' {
        let mut i = 0;
        while !regs().uart_lsr().read().tp1f_not_full().bit() {
            i += 1;
            if i >= SPIN_LIMIT {
                return false;
            }
        }
        regs()
            .uart_thr0()
            .write(|w| unsafe { w.bits(b'\r' as u32) });
    }
    true
}

/// Diagnostic: write a byte via the bounded putchar, returning `false` (and
/// dropping the byte) instead of hanging if the LOGUART TX FIFO never frees.
pub fn write_byte_bounded(c: u8) -> bool {
    putchar_bounded(c)
}

/// Diagnostic: write a string via the bounded putchar. Stops (returns
/// `false`) at the first byte that would have blocked, rather than hanging.
pub fn write_str_bounded(s: &str) -> bool {
    for &b in s.as_bytes() {
        if !putchar_bounded(b) {
            return false;
        }
    }
    true
}

/// Write a string slice to LOGUART.
pub fn write_str(s: &str) {
    for &b in s.as_bytes() {
        putchar(b);
    }
}

/// Zero-sized writer for `core::fmt::Write`.
pub struct LogWriter;

impl core::fmt::Write for LogWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write_str(s);
        Ok(())
    }
}

/// Diagnostic: bounded counterpart of [`LogWriter`]. Used to test whether an
/// unbounded FIFO-full wait inside an ISR (e.g. `api_isr`, which prints on
/// every N2A IPC event with no synchronisation against task-context writers
/// of the same LOGUART) is the cause of an observed freeze. Aborts the
/// `write!` (returning `Err`, silently discarded by callers using `let _ =`)
/// at the first byte that would have blocked, instead of hanging forever.
pub struct BoundedLogWriter;

impl core::fmt::Write for BoundedLogWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if write_str_bounded(s) {
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}

/// Prints a message followed by a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = writeln!($crate::loguart::LogWriter, $($arg)*);
    }};
}

/// Prints a message (no trailing newline).
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::loguart::LogWriter, $($arg)*);
    }};
}

/// Diagnostic: bounded counterpart of [`print!`] — never blocks forever on a
/// wedged LOGUART TX FIFO. See [`BoundedLogWriter`].
#[macro_export]
macro_rules! print_bounded {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::loguart::BoundedLogWriter, $($arg)*);
    }};
}

// ── `#[no_mangle]` stubs for ameba-rt panic handler ───────────────────────

/// # Safety
///
/// `c` is written directly to the LOGUART THR0 register.  No synchronisation
/// is performed; the caller must ensure exclusive access to LOGUART output.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loguart_panic_putchar(c: u8) {
    putchar(c);
}

/// # Safety
///
/// Reads `len` bytes from `s` and writes each to the LOGUART THR0 register.
/// `s` must point to at least `len` readable bytes.  No synchronisation is
/// performed; the caller must ensure exclusive access to LOGUART output.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loguart_panic_puts(s: *const u8, len: u32) {
    unsafe {
        for i in 0..len as usize {
            putchar(core::ptr::read_volatile(s.add(i)));
        }
    }
}

// ── LOGUART DMA configuration driver ──────────────────────────────────────

/// LOGUART DMA configuration driver.
///
/// Only the DMA-configuration subset of the LOGUART API is covered here.
pub struct LogUart(());

impl LogUart {
    /// # Safety
    /// Only one `LogUart` instance may exist at a time.
    pub unsafe fn steal() -> Self {
        LogUart(())
    }

    /// Set the TX DMA burst size (max 16). Mirrors `LOGUART_TXDMAConfig`.
    pub fn set_tx_dma_burst_size(&self, size: u8) {
        regs()
            .uart_miscr()
            .modify(|_, w| unsafe { w.txdma_burstsize().bits(size) });
    }

    /// Set the RX DMA burst size (max 16). Mirrors `LOGUART_RXDMAConfig`.
    pub fn set_rx_dma_burst_size(&self, size: u8) {
        regs()
            .uart_miscr()
            .modify(|_, w| unsafe { w.rxdma_burstsize().bits(size) });
    }

    /// Enable or disable TX DMA. Mirrors `LOGUART_TXDMACmd`.
    pub fn set_tx_dma_enabled(&self, enabled: bool) {
        regs().uart_miscr().modify(|_, w| w.txdma_en().bit(enabled));
    }

    /// Enable or disable RX DMA. Mirrors `LOGUART_RXDMACmd`.
    pub fn set_rx_dma_enabled(&self, enabled: bool) {
        regs().uart_miscr().modify(|_, w| w.rxdma_en().bit(enabled));
    }

    /// Set RX DMA flow control ownership.
    ///
    /// `true` → LOGUART acts as flow controller (GDMA peripheral-controlled transfer).
    /// `false` → GDMA is flow controller.
    pub fn set_rx_dma_owner(&self, loguart_is_owner: bool) {
        regs()
            .uart_miscr()
            .modify(|_, w| w.rxdma_owner().bit(loguart_is_owner));
    }
}
