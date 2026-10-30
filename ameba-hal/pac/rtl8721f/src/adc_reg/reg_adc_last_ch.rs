#[doc = "Register `REG_ADC_LAST_CH` reader"]
pub type R = crate::R<RegAdcLastChSpec>;
#[doc = "Field `bit_adc_last_ch` reader - This field reflects the last used channel"]
pub type BitAdcLastChR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - This field reflects the last used channel"]
    #[inline(always)]
    pub fn bit_adc_last_ch(&self) -> BitAdcLastChR {
        BitAdcLastChR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "ADC Last Channel Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_last_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcLastChSpec;
impl crate::RegisterSpec for RegAdcLastChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_last_ch::R`](R) reader structure"]
impl crate::Readable for RegAdcLastChSpec {}
#[doc = "`reset()` method sets REG_ADC_LAST_CH to value 0"]
impl crate::Resettable for RegAdcLastChSpec {}
