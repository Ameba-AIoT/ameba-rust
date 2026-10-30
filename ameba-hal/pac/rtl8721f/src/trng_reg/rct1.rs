#[doc = "Register `RCT1` reader"]
pub type R = crate::R<Rct1Spec>;
#[doc = "Register `RCT1` writer"]
pub type W = crate::W<Rct1Spec>;
#[doc = "Field `RCT1_ENABLE` reader - Test enable"]
pub type Rct1EnableR = crate::BitReader;
#[doc = "Field `RCT1_ENABLE` writer - Test enable"]
pub type Rct1EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCT1_SAMPLE_SIZE` reader - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
pub type Rct1SampleSizeR = crate::FieldReader;
#[doc = "Field `RCT1_SAMPLE_SIZE` writer - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
pub type Rct1SampleSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RCT1_CUTOFF_VALUE` reader - Cutoff value"]
pub type Rct1CutoffValueR = crate::FieldReader;
#[doc = "Field `RCT1_CUTOFF_VALUE` writer - Cutoff value"]
pub type Rct1CutoffValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Test enable"]
    #[inline(always)]
    pub fn rct1_enable(&self) -> Rct1EnableR {
        Rct1EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
    #[inline(always)]
    pub fn rct1_sample_size(&self) -> Rct1SampleSizeR {
        Rct1SampleSizeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Cutoff value"]
    #[inline(always)]
    pub fn rct1_cutoff_value(&self) -> Rct1CutoffValueR {
        Rct1CutoffValueR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Test enable"]
    #[inline(always)]
    pub fn rct1_enable(&mut self) -> Rct1EnableW<'_, Rct1Spec> {
        Rct1EnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Number of bit per test sample. The sample size is the value add 1. 0: 1-bit 1: 2-bit ... 7: 8-bit"]
    #[inline(always)]
    pub fn rct1_sample_size(&mut self) -> Rct1SampleSizeW<'_, Rct1Spec> {
        Rct1SampleSizeW::new(self, 1)
    }
    #[doc = "Bits 8:15 - Cutoff value"]
    #[inline(always)]
    pub fn rct1_cutoff_value(&mut self) -> Rct1CutoffValueW<'_, Rct1Spec> {
        Rct1CutoffValueW::new(self, 8)
    }
}
#[doc = "Repetition Count Test 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rct1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rct1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rct1Spec;
impl crate::RegisterSpec for Rct1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rct1::R`](R) reader structure"]
impl crate::Readable for Rct1Spec {}
#[doc = "`write(|w| ..)` method takes [`rct1::W`](W) writer structure"]
impl crate::Writable for Rct1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCT1 to value 0x050f"]
impl crate::Resettable for Rct1Spec {
    const RESET_VALUE: u32 = 0x050f;
}
