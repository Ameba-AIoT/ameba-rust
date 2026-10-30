#[doc = "Register `REG_TQPNT1` reader"]
pub type R = crate::R<RegTqpnt1Spec>;
#[doc = "Register `REG_TQPNT1` writer"]
pub type W = crate::W<RegTqpnt1Spec>;
#[doc = "Field `BIT_HPQ_LOW_TH` reader - For SDIO, High Priority Queue Page Number Low Threshold"]
pub type BitHpqLowThR = crate::FieldReader;
#[doc = "Field `BIT_HPQ_LOW_TH` writer - For SDIO, High Priority Queue Page Number Low Threshold"]
pub type BitHpqLowThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_HPQ_HIGH_TH` reader - For SDIO, High Priority Queue Page Number High Threshold"]
pub type BitHpqHighThR = crate::FieldReader;
#[doc = "Field `BIT_HPQ_HIGH_TH` writer - For SDIO, High Priority Queue Page Number High Threshold"]
pub type BitHpqHighThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_NPQ_LOW_TH` reader - For SDIO, Normal Priority Queue Page Number Low Threshold"]
pub type BitNpqLowThR = crate::FieldReader;
#[doc = "Field `BIT_NPQ_LOW_TH` writer - For SDIO, Normal Priority Queue Page Number Low Threshold"]
pub type BitNpqLowThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_NPQ_HIGH_TH` reader - For SDIO, Normal Priority Queue Page Number High Threshold"]
pub type BitNpqHighThR = crate::FieldReader;
#[doc = "Field `BIT_NPQ_HIGH_TH` writer - For SDIO, Normal Priority Queue Page Number High Threshold"]
pub type BitNpqHighThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - For SDIO, High Priority Queue Page Number Low Threshold"]
    #[inline(always)]
    pub fn bit_hpq_low_th(&self) -> BitHpqLowThR {
        BitHpqLowThR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - For SDIO, High Priority Queue Page Number High Threshold"]
    #[inline(always)]
    pub fn bit_hpq_high_th(&self) -> BitHpqHighThR {
        BitHpqHighThR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - For SDIO, Normal Priority Queue Page Number Low Threshold"]
    #[inline(always)]
    pub fn bit_npq_low_th(&self) -> BitNpqLowThR {
        BitNpqLowThR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - For SDIO, Normal Priority Queue Page Number High Threshold"]
    #[inline(always)]
    pub fn bit_npq_high_th(&self) -> BitNpqHighThR {
        BitNpqHighThR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - For SDIO, High Priority Queue Page Number Low Threshold"]
    #[inline(always)]
    pub fn bit_hpq_low_th(&mut self) -> BitHpqLowThW<'_, RegTqpnt1Spec> {
        BitHpqLowThW::new(self, 0)
    }
    #[doc = "Bits 8:15 - For SDIO, High Priority Queue Page Number High Threshold"]
    #[inline(always)]
    pub fn bit_hpq_high_th(&mut self) -> BitHpqHighThW<'_, RegTqpnt1Spec> {
        BitHpqHighThW::new(self, 8)
    }
    #[doc = "Bits 16:23 - For SDIO, Normal Priority Queue Page Number Low Threshold"]
    #[inline(always)]
    pub fn bit_npq_low_th(&mut self) -> BitNpqLowThW<'_, RegTqpnt1Spec> {
        BitNpqLowThW::new(self, 16)
    }
    #[doc = "Bits 24:31 - For SDIO, Normal Priority Queue Page Number High Threshold"]
    #[inline(always)]
    pub fn bit_npq_high_th(&mut self) -> BitNpqHighThW<'_, RegTqpnt1Spec> {
        BitNpqHighThW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tqpnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tqpnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTqpnt1Spec;
impl crate::RegisterSpec for RegTqpnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tqpnt1::R`](R) reader structure"]
impl crate::Readable for RegTqpnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_tqpnt1::W`](W) writer structure"]
impl crate::Writable for RegTqpnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TQPNT1 to value 0"]
impl crate::Resettable for RegTqpnt1Spec {}
