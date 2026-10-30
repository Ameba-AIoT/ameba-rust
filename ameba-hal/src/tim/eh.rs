use super::{Channel, Pwm, PwmTimInst, Tim};

pub struct PwmChannel<'a, P: PwmTimInst> {
    tim: &'a Tim<P, Pwm>,
    channel: Channel,
}

impl<'a, P: PwmTimInst> PwmChannel<'a, P> {
    pub fn new(tim: &'a Tim<P, Pwm>, channel: Channel) -> Self {
        Self { tim, channel }
    }
}

impl<P: PwmTimInst> embedded_hal::pwm::ErrorType for PwmChannel<'_, P> {
    type Error = core::convert::Infallible;
}

impl<P: PwmTimInst> embedded_hal::pwm::SetDutyCycle for PwmChannel<'_, P> {
    fn max_duty_cycle(&self) -> u16 {
        self.tim.regs().tim_arr().read().arr().bits()
    }
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.tim.set_compare(self.channel, duty);
        Ok(())
    }
}
