#[doc = "Register `silence_inform` reader"]
pub type R = crate::R<SilenceInformSpec>;
#[doc = "Field `adc_0_silence_det_o` reader - adc_0_silencedata status (result of silence detection) 1'b0: not adc_0_silencedata 1'b1: adc_0_silenceis detected"]
pub type Adc0SilenceDetOR = crate::BitReader;
#[doc = "Field `adc_0_silence_det_status` reader - Ongoing status of adc_0_silencedetection 1'b0: adc_0_silencedetection is resting (clock is gating) 1'b1: adc_0_silencedetection is working"]
pub type Adc0SilenceDetStatusR = crate::BitReader;
#[doc = "Field `adc_1_silence_det_o` reader - adc_1_silencedata status (result of silence detection) 1'b0: not adc_1_silencedata 1'b1: adc_1_silenceis detected"]
pub type Adc1SilenceDetOR = crate::BitReader;
#[doc = "Field `adc_1_silence_det_status` reader - Ongoing status of adc_1_silencedetection 1'b0: adc_1_silencedetection is resting (clock is gating) 1'b1: adc_1_silencedetection is working"]
pub type Adc1SilenceDetStatusR = crate::BitReader;
impl R {
    #[doc = "Bit 8 - adc_0_silencedata status (result of silence detection) 1'b0: not adc_0_silencedata 1'b1: adc_0_silenceis detected"]
    #[inline(always)]
    pub fn adc_0_silence_det_o(&self) -> Adc0SilenceDetOR {
        Adc0SilenceDetOR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ongoing status of adc_0_silencedetection 1'b0: adc_0_silencedetection is resting (clock is gating) 1'b1: adc_0_silencedetection is working"]
    #[inline(always)]
    pub fn adc_0_silence_det_status(&self) -> Adc0SilenceDetStatusR {
        Adc0SilenceDetStatusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - adc_1_silencedata status (result of silence detection) 1'b0: not adc_1_silencedata 1'b1: adc_1_silenceis detected"]
    #[inline(always)]
    pub fn adc_1_silence_det_o(&self) -> Adc1SilenceDetOR {
        Adc1SilenceDetOR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Ongoing status of adc_1_silencedetection 1'b0: adc_1_silencedetection is resting (clock is gating) 1'b1: adc_1_silencedetection is working"]
    #[inline(always)]
    pub fn adc_1_silence_det_status(&self) -> Adc1SilenceDetStatusR {
        Adc1SilenceDetStatusR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Silence Inform Register\n\nYou can [`read`](crate::Reg::read) this register and get [`silence_inform::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SilenceInformSpec;
impl crate::RegisterSpec for SilenceInformSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`silence_inform::R`](R) reader structure"]
impl crate::Readable for SilenceInformSpec {}
#[doc = "`reset()` method sets silence_inform to value 0"]
impl crate::Resettable for SilenceInformSpec {}
