#[doc = "Register `APT` reader"]
pub type R = crate::R<AptSpec>;
#[doc = "Register `APT` writer"]
pub type W = crate::W<AptSpec>;
#[doc = "Field `APT0_ENABLE` reader - Test enable"]
pub type Apt0EnableR = crate::BitReader;
#[doc = "Field `APT0_ENABLE` writer - Test enable"]
pub type Apt0EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APT0_WINDOW_SIZE` reader - 0: 512 bits 1: 1024 bits"]
pub type Apt0WindowSizeR = crate::BitReader;
#[doc = "Field `APT0_WINDOW_SIZE` writer - 0: 512 bits 1: 1024 bits"]
pub type Apt0WindowSizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APT0_SAMPLE_SIZE` reader - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
pub type Apt0SampleSizeR = crate::FieldReader;
#[doc = "Field `APT0_SAMPLE_SIZE` writer - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
pub type Apt0SampleSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `APT0_CUTOFF_VALUE` reader - Cutoff value"]
pub type Apt0CutoffValueR = crate::FieldReader<u16>;
#[doc = "Field `APT0_CUTOFF_VALUE` writer - Cutoff value"]
pub type Apt0CutoffValueW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `APT0_DONE_THRESHOLD` reader - Assert test done after how many test pass"]
pub type Apt0DoneThresholdR = crate::FieldReader;
#[doc = "Field `APT0_DONE_THRESHOLD` writer - Assert test done after how many test pass"]
pub type Apt0DoneThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Test enable"]
    #[inline(always)]
    pub fn apt0_enable(&self) -> Apt0EnableR {
        Apt0EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: 512 bits 1: 1024 bits"]
    #[inline(always)]
    pub fn apt0_window_size(&self) -> Apt0WindowSizeR {
        Apt0WindowSizeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
    #[inline(always)]
    pub fn apt0_sample_size(&self) -> Apt0SampleSizeR {
        Apt0SampleSizeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - Cutoff value"]
    #[inline(always)]
    pub fn apt0_cutoff_value(&self) -> Apt0CutoffValueR {
        Apt0CutoffValueR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - Assert test done after how many test pass"]
    #[inline(always)]
    pub fn apt0_done_threshold(&self) -> Apt0DoneThresholdR {
        Apt0DoneThresholdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Test enable"]
    #[inline(always)]
    pub fn apt0_enable(&mut self) -> Apt0EnableW<'_, AptSpec> {
        Apt0EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: 512 bits 1: 1024 bits"]
    #[inline(always)]
    pub fn apt0_window_size(&mut self) -> Apt0WindowSizeW<'_, AptSpec> {
        Apt0WindowSizeW::new(self, 1)
    }
    #[doc = "Bits 8:10 - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
    #[inline(always)]
    pub fn apt0_sample_size(&mut self) -> Apt0SampleSizeW<'_, AptSpec> {
        Apt0SampleSizeW::new(self, 8)
    }
    #[doc = "Bits 16:25 - Cutoff value"]
    #[inline(always)]
    pub fn apt0_cutoff_value(&mut self) -> Apt0CutoffValueW<'_, AptSpec> {
        Apt0CutoffValueW::new(self, 16)
    }
    #[doc = "Bits 28:31 - Assert test done after how many test pass"]
    #[inline(always)]
    pub fn apt0_done_threshold(&mut self) -> Apt0DoneThresholdW<'_, AptSpec> {
        Apt0DoneThresholdW::new(self, 28)
    }
}
#[doc = "Adaptive Proportion Test Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AptSpec;
impl crate::RegisterSpec for AptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apt::R`](R) reader structure"]
impl crate::Readable for AptSpec {}
#[doc = "`write(|w| ..)` method takes [`apt::W`](W) writer structure"]
impl crate::Writable for AptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APT to value 0x124d_0003"]
impl crate::Resettable for AptSpec {
    const RESET_VALUE: u32 = 0x124d_0003;
}
