#[doc = "Register `REG_GPIO_WAKE_ISR` reader"]
pub type R = crate::R<RegGpioWakeIsrSpec>;
#[doc = "Field `GPIO_PIN0_WAKE_ISR` reader - Wake pin0 interrupt status 0:no interrupt 1:interrupt occurs"]
pub type GpioPin0WakeIsrR = crate::BitReader;
#[doc = "Field `GPIO_PIN1_WAKE_ISR` reader - Wake pin1 interrupt status 0:no interrupt 1:interrupt occurs"]
pub type GpioPin1WakeIsrR = crate::BitReader;
#[doc = "Field `GPIO_PIN2_WAKE_ISR` reader - Wake pin2 interrupt status 0:no interrupt 1:interrupt occurs"]
pub type GpioPin2WakeIsrR = crate::BitReader;
#[doc = "Field `GPIO_PIN3_WAKE_ISR` reader - Wake pin3 interrupt status 0:no interrupt 1:interrupt occurs"]
pub type GpioPin3WakeIsrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Wake pin0 interrupt status 0:no interrupt 1:interrupt occurs"]
    #[inline(always)]
    pub fn gpio_pin0_wake_isr(&self) -> GpioPin0WakeIsrR {
        GpioPin0WakeIsrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake pin1 interrupt status 0:no interrupt 1:interrupt occurs"]
    #[inline(always)]
    pub fn gpio_pin1_wake_isr(&self) -> GpioPin1WakeIsrR {
        GpioPin1WakeIsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake pin2 interrupt status 0:no interrupt 1:interrupt occurs"]
    #[inline(always)]
    pub fn gpio_pin2_wake_isr(&self) -> GpioPin2WakeIsrR {
        GpioPin2WakeIsrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake pin3 interrupt status 0:no interrupt 1:interrupt occurs"]
    #[inline(always)]
    pub fn gpio_pin3_wake_isr(&self) -> GpioPin3WakeIsrR {
        GpioPin3WakeIsrR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_gpio_wake_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGpioWakeIsrSpec;
impl crate::RegisterSpec for RegGpioWakeIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_gpio_wake_isr::R`](R) reader structure"]
impl crate::Readable for RegGpioWakeIsrSpec {}
#[doc = "`reset()` method sets REG_GPIO_WAKE_ISR to value 0"]
impl crate::Resettable for RegGpioWakeIsrSpec {}
