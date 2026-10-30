use super::{Spi, SpiInstance};
use crate::mode::{Async, Blocking};

#[derive(Debug)]
pub struct SpiError;

impl embedded_hal::spi::Error for SpiError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        embedded_hal::spi::ErrorKind::Other
    }
}

// ---------------------------------------------------------------------------
// Blocking SpiBus for Spi
// ---------------------------------------------------------------------------

impl<P: SpiInstance> embedded_hal::spi::ErrorType for Spi<P, Blocking> {
    type Error = SpiError;
}

impl<P: SpiInstance> embedded_hal::spi::SpiBus<u8> for Spi<P, Blocking> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for w in words.iter_mut() {
            while !self.is_tx_not_full() {}
            self.write_data(0);
            while !self.is_rx_not_empty() {}
            *w = self.read_data() as u8;
        }
        Ok(())
    }
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        for &w in words {
            while !self.is_tx_not_full() {}
            self.write_data(w as u32);
        }
        while self.is_rx_not_empty() {
            let _ = self.read_data();
        }
        Ok(())
    }
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let len = read.len().max(write.len());
        for i in 0..len {
            let tx = write.get(i).copied().unwrap_or(0);
            while !self.is_tx_not_full() {}
            self.write_data(tx as u32);
            while !self.is_rx_not_empty() {}
            let rx = self.read_data() as u8;
            if i < read.len() {
                read[i] = rx;
            }
        }
        Ok(())
    }
    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for w in words.iter_mut() {
            while !self.is_tx_not_full() {}
            self.write_data(*w as u32);
            while !self.is_rx_not_empty() {}
            *w = self.read_data() as u8;
        }
        Ok(())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        while self.is_busy() {}
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Async SpiBus for Spi<P, Async>
// ---------------------------------------------------------------------------

impl<P: SpiInstance> embedded_hal::spi::ErrorType for Spi<P, Async> {
    type Error = SpiError;
}

impl<P: SpiInstance> embedded_hal_async::spi::SpiBus<u8> for Spi<P, Async> {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.read_async(words).await;
        Ok(())
    }
    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.write_async(words).await;
        Ok(())
    }
    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.transfer_async(read, write).await;
        Ok(())
    }
    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.transfer_in_place_async(words).await;
        Ok(())
    }
    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush_async().await;
        Ok(())
    }
}
