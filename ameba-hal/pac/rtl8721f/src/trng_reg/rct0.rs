#[doc = "Register `RCT0` reader"]
pub type R = crate::R<Rct0Spec>;
#[doc = "Register `RCT0` writer"]
pub type W = crate::W<Rct0Spec>;
#[doc = "Field `RCT0_ENABLE` reader - Test enable"]
pub type Rct0EnableR = crate::BitReader;
#[doc = "Field `RCT0_ENABLE` writer - Test enable"]
pub type Rct0EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCT0_SAMPLE_SIZE` reader - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
pub type Rct0SampleSizeR = crate::FieldReader;
#[doc = "Field `RCT0_SAMPLE_SIZE` writer - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
pub type Rct0SampleSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RCT0_CUTOFF_VALUE` reader - Cutoff value"]
pub type Rct0CutoffValueR = crate::FieldReader;
#[doc = "Field `RCT0_CUTOFF_VALUE` writer - Cutoff value"]
pub type Rct0CutoffValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Test enable"]
    #[inline(always)]
    pub fn rct0_enable(&self) -> Rct0EnableR {
        Rct0EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
    #[inline(always)]
    pub fn rct0_sample_size(&self) -> Rct0SampleSizeR {
        Rct0SampleSizeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Cutoff value"]
    #[inline(always)]
    pub fn rct0_cutoff_value(&self) -> Rct0CutoffValueR {
        Rct0CutoffValueR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Test enable"]
    #[inline(always)]
    pub fn rct0_enable(&mut self) -> Rct0EnableW<'_, Rct0Spec> {
        Rct0EnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
    #[inline(always)]
    pub fn rct0_sample_size(&mut self) -> Rct0SampleSizeW<'_, Rct0Spec> {
        Rct0SampleSizeW::new(self, 1)
    }
    #[doc = "Bits 8:15 - Cutoff value"]
    #[inline(always)]
    pub fn rct0_cutoff_value(&mut self) -> Rct0CutoffValueW<'_, Rct0Spec> {
        Rct0CutoffValueW::new(self, 8)
    }
}
#[doc = "Repetition Count Test 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rct0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rct0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rct0Spec;
impl crate::RegisterSpec for Rct0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rct0::R`](R) reader structure"]
impl crate::Readable for Rct0Spec {}
#[doc = "`write(|w| ..)` method takes [`rct0::W`](W) writer structure"]
impl crate::Writable for Rct0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCT0 to value 0x1f01"]
impl crate::Resettable for Rct0Spec {
    const RESET_VALUE: u32 = 0x1f01;
}
