#[doc = "Register `GPIO_VER_ID_CODE` reader"]
pub type R = crate::R<GpioVerIdCodeSpec>;
#[doc = "Field `GPIO_VER_ID_CODE` reader - ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*. Reset Value: See the releases table in the Release Notes."]
pub type GpioVerIdCodeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*. Reset Value: See the releases table in the Release Notes."]
    #[inline(always)]
    pub fn gpio_ver_id_code(&self) -> GpioVerIdCodeR {
        GpioVerIdCodeR::new(self.bits)
    }
}
#[doc = "Port GPIO component version register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ver_id_code::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioVerIdCodeSpec;
impl crate::RegisterSpec for GpioVerIdCodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ver_id_code::R`](R) reader structure"]
impl crate::Readable for GpioVerIdCodeSpec {}
#[doc = "`reset()` method sets GPIO_VER_ID_CODE to value 0x3231_302a"]
impl crate::Resettable for GpioVerIdCodeSpec {
    const RESET_VALUE: u32 = 0x3231_302a;
}
