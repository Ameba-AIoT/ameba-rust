#[doc = "Register `REG_ADC_DUMMY_CNT` reader"]
pub type R = crate::R<RegAdcDummyCntSpec>;
#[doc = "Register `REG_ADC_DUMMY_CNT` writer"]
pub type W = crate::W<RegAdcDummyCntSpec>;
#[doc = "Field `bit_adc_dummy_chn_scnt` reader - dummy channel time counter. in auto mode, adc share the dummy channel for captouch. this reg controls the time of dummy channel."]
pub type BitAdcDummyChnScntR = crate::FieldReader<u32>;
#[doc = "Field `bit_adc_dummy_chn_scnt` writer - dummy channel time counter. in auto mode, adc share the dummy channel for captouch. this reg controls the time of dummy channel."]
pub type BitAdcDummyChnScntW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - dummy channel time counter. in auto mode, adc share the dummy channel for captouch. this reg controls the time of dummy channel."]
    #[inline(always)]
    pub fn bit_adc_dummy_chn_scnt(&self) -> BitAdcDummyChnScntR {
        BitAdcDummyChnScntR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - dummy channel time counter. in auto mode, adc share the dummy channel for captouch. this reg controls the time of dummy channel."]
    #[inline(always)]
    pub fn bit_adc_dummy_chn_scnt(&mut self) -> BitAdcDummyChnScntW<'_, RegAdcDummyCntSpec> {
        BitAdcDummyChnScntW::new(self, 0)
    }
}
#[doc = "ADC Dummy Cnt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_dummy_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_dummy_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcDummyCntSpec;
impl crate::RegisterSpec for RegAdcDummyCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_dummy_cnt::R`](R) reader structure"]
impl crate::Readable for RegAdcDummyCntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_dummy_cnt::W`](W) writer structure"]
impl crate::Writable for RegAdcDummyCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_DUMMY_CNT to value 0"]
impl crate::Resettable for RegAdcDummyCntSpec {}
