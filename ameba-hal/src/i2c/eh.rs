use super::{Enabled, I2c, I2cInstance};
use crate::mode::{Async, Blocking};

#[derive(Debug, Clone, Copy)]
pub struct I2cError(pub embedded_hal::i2c::ErrorKind);

impl embedded_hal::i2c::Error for I2cError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        self.0
    }
}

impl<P: I2cInstance> embedded_hal::i2c::ErrorType for I2c<P, Enabled, Blocking> {
    type Error = I2cError;
}

impl<P: I2cInstance> embedded_hal::i2c::I2c for I2c<P, Enabled, Blocking> {
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.set_slave_address(address as u16);
        let n = self.master_read(read);
        if n == read.len() {
            Ok(())
        } else {
            Err(I2cError(embedded_hal::i2c::ErrorKind::Other))
        }
    }
    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.set_slave_address(address as u16);
        let n = self.master_write(write);
        if n == write.len() {
            Ok(())
        } else {
            Err(I2cError(embedded_hal::i2c::ErrorKind::Other))
        }
    }
    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.set_slave_address(address as u16);
        let n = self.master_repeat_read(write, read);
        if n == read.len() {
            Ok(())
        } else {
            Err(I2cError(embedded_hal::i2c::ErrorKind::Other))
        }
    }
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        use embedded_hal::i2c::Operation;
        self.set_slave_address(address as u16);
        let mut i = 0;
        while i < operations.len() {
            if i + 1 < operations.len()
                && matches!(&operations[i], Operation::Write(_))
                && matches!(&operations[i + 1], Operation::Read(_))
            {
                // Write followed by Read: use Repeated START, not STOP+START.
                let (left, right) = operations.split_at_mut(i + 1);
                let w = if let Operation::Write(w) = &left[i] {
                    *w
                } else {
                    unreachable!()
                };
                let r = if let Operation::Read(r) = &mut right[0] {
                    r
                } else {
                    unreachable!()
                };
                let n = self.master_repeat_read(w, r);
                if n != r.len() {
                    return Err(I2cError(embedded_hal::i2c::ErrorKind::Other));
                }
                i += 2;
            } else {
                match &mut operations[i] {
                    Operation::Write(buf) => {
                        let n = self.master_write(buf);
                        if n != buf.len() {
                            return Err(I2cError(embedded_hal::i2c::ErrorKind::Other));
                        }
                    }
                    Operation::Read(buf) => {
                        let n = self.master_read(buf);
                        if n != buf.len() {
                            return Err(I2cError(embedded_hal::i2c::ErrorKind::Other));
                        }
                    }
                }
                i += 1;
            }
        }
        Ok(())
    }
}

impl<P: I2cInstance> embedded_hal::i2c::ErrorType for I2c<P, Enabled, Async> {
    type Error = I2cError;
}

impl<P: I2cInstance> embedded_hal_async::i2c::I2c for I2c<P, Enabled, Async> {
    async fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        self.set_target(address as u16);
        let intr = self.read_async(read).await;
        // TX_ABRT means NACK from slave; treat as error unless buffer is empty
        if super::IntMask::from_bits_truncate(intr).contains(super::IntMask::TX_ABRT)
            && !read.is_empty()
        {
            Err(I2cError(embedded_hal::i2c::ErrorKind::NoAcknowledge(
                embedded_hal::i2c::NoAcknowledgeSource::Unknown,
            )))
        } else {
            Ok(())
        }
    }

    async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.set_target(address as u16);
        let intr = self.write_async(write).await;
        if super::IntMask::from_bits_truncate(intr).contains(super::IntMask::TX_ABRT) {
            Err(I2cError(embedded_hal::i2c::ErrorKind::NoAcknowledge(
                embedded_hal::i2c::NoAcknowledgeSource::Unknown,
            )))
        } else {
            Ok(())
        }
    }

    async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.set_target(address as u16);
        let mut ops = [
            embedded_hal::i2c::Operation::Write(write),
            embedded_hal::i2c::Operation::Read(read),
        ];
        let intr = self.transaction_async(&mut ops).await;
        if super::IntMask::from_bits_truncate(intr).contains(super::IntMask::TX_ABRT) {
            Err(I2cError(embedded_hal::i2c::ErrorKind::NoAcknowledge(
                embedded_hal::i2c::NoAcknowledgeSource::Unknown,
            )))
        } else {
            Ok(())
        }
    }

    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.set_target(address as u16);
        let intr = self.transaction_async(operations).await;
        if super::IntMask::from_bits_truncate(intr).contains(super::IntMask::TX_ABRT) {
            Err(I2cError(embedded_hal::i2c::ErrorKind::NoAcknowledge(
                embedded_hal::i2c::NoAcknowledgeSource::Unknown,
            )))
        } else {
            Ok(())
        }
    }
}
