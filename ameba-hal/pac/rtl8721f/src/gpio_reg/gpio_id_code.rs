#[doc = "Register `GPIO_ID_CODE` reader"]
pub type R = crate::R<GpioIdCodeSpec>;
#[doc = "Field `GPIO_ID_CODE` reader - This register is available only if Portx is configured to generate interrupts (GPIO_PORTx_INTR = Include (1)) and interrupt detection is configured to generate on both rising and falling edges of external input signal (GPIO_INT_BOTH_EDGE=Include (1))."]
pub type GpioIdCodeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register is available only if Portx is configured to generate interrupts (GPIO_PORTx_INTR = Include (1)) and interrupt detection is configured to generate on both rising and falling edges of external input signal (GPIO_INT_BOTH_EDGE=Include (1))."]
    #[inline(always)]
    pub fn gpio_id_code(&self) -> GpioIdCodeR {
        GpioIdCodeR::new(self.bits)
    }
}
#[doc = "Port GPIO ID code register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_id_code::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIdCodeSpec;
impl crate::RegisterSpec for GpioIdCodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_id_code::R`](R) reader structure"]
impl crate::Readable for GpioIdCodeSpec {}
#[doc = "`reset()` method sets GPIO_ID_CODE to value 0x01"]
impl crate::Resettable for GpioIdCodeSpec {
    const RESET_VALUE: u32 = 0x01;
}
