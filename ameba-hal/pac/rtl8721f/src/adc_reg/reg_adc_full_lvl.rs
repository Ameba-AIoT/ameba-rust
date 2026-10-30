#[doc = "Register `REG_ADC_FULL_LVL` reader"]
pub type R = crate::R<RegAdcFullLvlSpec>;
#[doc = "Register `REG_ADC_FULL_LVL` writer"]
pub type W = crate::W<RegAdcFullLvlSpec>;
#[doc = "Field `bit_adc_full_lvl` reader - This field defines the FIFO full entry thredshold level and it's used for FIFO full interrupt. It should be set according to ADC FIFO depth. A value of 0 sets the threshold for 1 entry, and a value of 15 sets the threshold for 16 entries."]
pub type BitAdcFullLvlR = crate::FieldReader;
#[doc = "Field `bit_adc_full_lvl` writer - This field defines the FIFO full entry thredshold level and it's used for FIFO full interrupt. It should be set according to ADC FIFO depth. A value of 0 sets the threshold for 1 entry, and a value of 15 sets the threshold for 16 entries."]
pub type BitAdcFullLvlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - This field defines the FIFO full entry thredshold level and it's used for FIFO full interrupt. It should be set according to ADC FIFO depth. A value of 0 sets the threshold for 1 entry, and a value of 15 sets the threshold for 16 entries."]
    #[inline(always)]
    pub fn bit_adc_full_lvl(&self) -> BitAdcFullLvlR {
        BitAdcFullLvlR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - This field defines the FIFO full entry thredshold level and it's used for FIFO full interrupt. It should be set according to ADC FIFO depth. A value of 0 sets the threshold for 1 entry, and a value of 15 sets the threshold for 16 entries."]
    #[inline(always)]
    pub fn bit_adc_full_lvl(&mut self) -> BitAdcFullLvlW<'_, RegAdcFullLvlSpec> {
        BitAdcFullLvlW::new(self, 0)
    }
}
#[doc = "ADC FIFO Full Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_full_lvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_full_lvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcFullLvlSpec;
impl crate::RegisterSpec for RegAdcFullLvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_full_lvl::R`](R) reader structure"]
impl crate::Readable for RegAdcFullLvlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_full_lvl::W`](W) writer structure"]
impl crate::Writable for RegAdcFullLvlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_FULL_LVL to value 0"]
impl crate::Resettable for RegAdcFullLvlSpec {}
