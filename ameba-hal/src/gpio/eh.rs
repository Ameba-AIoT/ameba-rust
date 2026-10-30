use super::{GpioPin, Input, Output, hw};

impl embedded_hal::digital::ErrorType for GpioPin<Input> {
    type Error = core::convert::Infallible;
}

impl embedded_hal_async::digital::Wait for GpioPin<Input> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        // Arm interrupt BEFORE sampling to prevent missed-edge race:
        // if the edge occurs after is_high() but before arming, we would wait forever.
        // GpioEdgeFuture stores only pin: u32 (no borrow of self), so is_high() is valid after.
        let fut = self.wait_for_rising_edge();
        if embedded_hal::digital::InputPin::is_high(self)? {
            drop(fut); // Drop impl disarms interrupt and clears ASYNC_PENDING
            return Ok(());
        }
        fut.await;
        Ok(())
    }
    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        let fut = self.wait_for_falling_edge();
        if embedded_hal::digital::InputPin::is_low(self)? {
            drop(fut);
            return Ok(());
        }
        fut.await;
        Ok(())
    }
    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        // Calls the inherent method (inherent takes priority over trait in method resolution).
        self.wait_for_rising_edge().await;
        Ok(())
    }
    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_falling_edge().await;
        Ok(())
    }
    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_any_edge().await;
        Ok(())
    }
}

impl embedded_hal::digital::InputPin for GpioPin<Input> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(hw::read_bit(self.pin))
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!hw::read_bit(self.pin))
    }
}

impl embedded_hal::digital::ErrorType for GpioPin<Output> {
    type Error = core::convert::Infallible;
}

impl embedded_hal::digital::OutputPin for GpioPin<Output> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        hw::write_bit(self.pin, true);
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        hw::write_bit(self.pin, false);
        Ok(())
    }
}

impl embedded_hal::digital::StatefulOutputPin for GpioPin<Output> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(hw::read_bit(self.pin))
    }
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!hw::read_bit(self.pin))
    }
}
