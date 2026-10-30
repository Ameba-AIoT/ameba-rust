#[doc = "Register `REG_ADC_FLR` reader"]
pub type R = crate::R<RegAdcFlrSpec>;
#[doc = "Field `bit_adc_flr` reader - This field records the current ADC FIFO entry number."]
pub type BitAdcFlrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - This field records the current ADC FIFO entry number."]
    #[inline(always)]
    pub fn bit_adc_flr(&self) -> BitAdcFlrR {
        BitAdcFlrR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "ADC FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_flr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcFlrSpec;
impl crate::RegisterSpec for RegAdcFlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_flr::R`](R) reader structure"]
impl crate::Readable for RegAdcFlrSpec {}
#[doc = "`reset()` method sets REG_ADC_FLR to value 0"]
impl crate::Resettable for RegAdcFlrSpec {}
