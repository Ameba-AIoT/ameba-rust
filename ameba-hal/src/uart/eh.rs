use super::{Uart, UartInstance, uart_char_get, uart_char_put, uart_readable, uart_writable};
use crate::mode::{Async, Blocking};

#[derive(Debug)]
pub struct UartError;

impl core::fmt::Display for UartError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UartError")
    }
}

impl core::error::Error for UartError {}

impl embedded_io::Error for UartError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl<P: UartInstance> embedded_io::ErrorType for Uart<P, Blocking> {
    type Error = UartError;
}

impl<P: UartInstance> embedded_io::Read for Uart<P, Blocking> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        let rb = self.regs();
        while !uart_readable(rb) {}
        buf[0] = uart_char_get(rb);
        let mut count = 1usize;
        while count < buf.len() {
            if !uart_readable(rb) {
                break;
            }
            buf[count] = uart_char_get(rb);
            count += 1;
        }
        Ok(count)
    }
}

impl<P: UartInstance> embedded_io::Write for Uart<P, Blocking> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let rb = self.regs();
        for &b in buf {
            while !uart_writable(rb) {}
            uart_char_put(rb, b);
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        // Poll RUART_BIT_TX_EMPTY (LSR bit 5) until TX FIFO drains.
        // RTL8721F has no TEMT bit; shift register may hold the last byte briefly after.
        // SDK: ameba_uart.h : RUART_BIT_TX_EMPTY, ameba_uart_rom.c : UART_WaitBusy
        while !self.regs().uart_lsr().read().tx_empty().bit_is_set() {}
        Ok(())
    }
}

impl<P: UartInstance> embedded_io::ErrorType for Uart<P, Async> {
    type Error = UartError;
}

impl<P: UartInstance> embedded_io_async::Read for Uart<P, Async> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            self.read_async().await;
            let rb = self.regs();
            let mut count = 0;
            while count < buf.len() {
                if !uart_readable(rb) {
                    break;
                }
                buf[count] = uart_char_get(rb);
                count += 1;
            }
            if count > 0 {
                return Ok(count);
            }
            // Spurious interrupt (timeout with no data); re-arm and wait.
        }
    }
}

impl<P: UartInstance> embedded_io_async::Write for Uart<P, Async> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.write_async(buf).await;
        Ok(buf.len())
    }
    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush_async().await;
        Ok(())
    }
}
