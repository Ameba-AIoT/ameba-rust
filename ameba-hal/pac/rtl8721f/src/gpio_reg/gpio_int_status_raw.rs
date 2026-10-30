#[doc = "Register `GPIO_INT_STATUS_RAW` reader"]
pub type R = crate::R<GpioIntStatusRawSpec>;
#[doc = "Field `GPIO_RAW_INT_STATUS` reader - Raw interrupt status of Portx (premasking bits)."]
pub type GpioRawIntStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Raw interrupt status of Portx (premasking bits)."]
    #[inline(always)]
    pub fn gpio_raw_int_status(&self) -> GpioRawIntStatusR {
        GpioRawIntStatusR::new(self.bits)
    }
}
#[doc = "Port raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_status_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntStatusRawSpec;
impl crate::RegisterSpec for GpioIntStatusRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_status_raw::R`](R) reader structure"]
impl crate::Readable for GpioIntStatusRawSpec {}
#[doc = "`reset()` method sets GPIO_INT_STATUS_RAW to value 0"]
impl crate::Resettable for GpioIntStatusRawSpec {}
