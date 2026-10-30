#[doc = "Register `TIM_ISR` reader"]
pub type R = crate::R<TimIsrSpec>;
#[doc = "Field `isr_timer0` reader - Timer interrupt status 0: no interrupt 1: interrupt occurs Write 1 to clear interrupts in interrupt handler."]
pub type IsrTimer0R = crate::BitReader;
#[doc = "Field `isr_timer1` reader - Timer interrupt status 0: no interrupt 1: interrupt occurs Write 1 to clear interrupts in interrupt handler."]
pub type IsrTimer1R = crate::BitReader;
#[doc = "Field `isr_timer2` reader - Timer interrupt status 0: no interrupt 1: interrupt occurs Write 1 to clear interrupts in interrupt handler."]
pub type IsrTimer2R = crate::BitReader;
#[doc = "Field `isr_timer3` reader - Timer interrupt status 0: no interrupt 1: interrupt occurs Write 1 to clear interrupts in interrupt handler."]
pub type IsrTimer3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer interrupt status 0: no interrupt 1: interrupt occurs Write 1 to clear interrupts in interrupt handler."]
    #[inline(always)]
    pub fn isr_timer0(&self) -> IsrTimer0R {
        IsrTimer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer interrupt status 0: no interrupt 1: interrupt occurs Write 1 to clear interrupts in interrupt handler."]
    #[inline(always)]
    pub fn isr_timer1(&self) -> IsrTimer1R {
        IsrTimer1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer interrupt status 0: no interrupt 1: interrupt occurs Write 1 to clear interrupts in interrupt handler."]
    #[inline(always)]
    pub fn isr_timer2(&self) -> IsrTimer2R {
        IsrTimer2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer interrupt status 0: no interrupt 1: interrupt occurs Write 1 to clear interrupts in interrupt handler."]
    #[inline(always)]
    pub fn isr_timer3(&self) -> IsrTimer3R {
        IsrTimer3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "TIM interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimIsrSpec;
impl crate::RegisterSpec for TimIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_isr::R`](R) reader structure"]
impl crate::Readable for TimIsrSpec {}
#[doc = "`reset()` method sets TIM_ISR to value 0"]
impl crate::Resettable for TimIsrSpec {}
