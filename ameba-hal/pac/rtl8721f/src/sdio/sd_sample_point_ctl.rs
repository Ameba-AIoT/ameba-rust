#[doc = "Register `SD_SAMPLE_POINT_CTL` reader"]
pub type R = crate::R<SdSamplePointCtlSpec>;
#[doc = "Register `SD_SAMPLE_POINT_CTL` writer"]
pub type W = crate::W<SdSamplePointCtlSpec>;
#[doc = "Field `sd20_samp_point` reader - Sample Point Selection for SD20 CMD / DATA 0: sample at rising edge of SDCLK 1: sample point is delayed by 1/4 SDCLK period"]
pub type Sd20SampPointR = crate::BitReader;
#[doc = "Field `sd20_samp_point` writer - Sample Point Selection for SD20 CMD / DATA 0: sample at rising edge of SDCLK 1: sample point is delayed by 1/4 SDCLK period"]
pub type Sd20SampPointW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Sample Point Selection for SD20 CMD / DATA 0: sample at rising edge of SDCLK 1: sample point is delayed by 1/4 SDCLK period"]
    #[inline(always)]
    pub fn sd20_samp_point(&self) -> Sd20SampPointR {
        Sd20SampPointR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Sample Point Selection for SD20 CMD / DATA 0: sample at rising edge of SDCLK 1: sample point is delayed by 1/4 SDCLK period"]
    #[inline(always)]
    pub fn sd20_samp_point(&mut self) -> Sd20SampPointW<'_, SdSamplePointCtlSpec> {
        Sd20SampPointW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_sample_point_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_sample_point_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdSamplePointCtlSpec;
impl crate::RegisterSpec for SdSamplePointCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sd_sample_point_ctl::R`](R) reader structure"]
impl crate::Readable for SdSamplePointCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_sample_point_ctl::W`](W) writer structure"]
impl crate::Writable for SdSamplePointCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_SAMPLE_POINT_CTL to value 0"]
impl crate::Resettable for SdSamplePointCtlSpec {}
