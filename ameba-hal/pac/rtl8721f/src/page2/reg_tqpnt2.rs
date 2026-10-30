#[doc = "Register `REG_TQPNT2` reader"]
pub type R = crate::R<RegTqpnt2Spec>;
#[doc = "Register `REG_TQPNT2` writer"]
pub type W = crate::W<RegTqpnt2Spec>;
#[doc = "Field `BIT_LPQ_LOW_TH` reader - For SDIO, Low Priority Queue Page Number Low Threshold"]
pub type BitLpqLowThR = crate::FieldReader;
#[doc = "Field `BIT_LPQ_LOW_TH` writer - For SDIO, Low Priority Queue Page Number Low Threshold"]
pub type BitLpqLowThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_LPQ_HIGH_TH` reader - For SDIO, Low Priority Queue Page Number High Threshold"]
pub type BitLpqHighThR = crate::FieldReader;
#[doc = "Field `BIT_LPQ_HIGH_TH` writer - For SDIO, Low Priority Queue Page Number High Threshold"]
pub type BitLpqHighThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EXQ_LOW_TH` reader - For SDIO, Extra Priority Queue Page Number Low Threshold"]
pub type BitExqLowThR = crate::FieldReader;
#[doc = "Field `BIT_EXQ_LOW_TH` writer - For SDIO, Extra Priority Queue Page Number Low Threshold"]
pub type BitExqLowThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EXQ_HIGH_TH` reader - For SDIO, Extra Priority Queue Page Number High Threshold"]
pub type BitExqHighThR = crate::FieldReader;
#[doc = "Field `BIT_EXQ_HIGH_TH` writer - For SDIO, Extra Priority Queue Page Number High Threshold"]
pub type BitExqHighThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - For SDIO, Low Priority Queue Page Number Low Threshold"]
    #[inline(always)]
    pub fn bit_lpq_low_th(&self) -> BitLpqLowThR {
        BitLpqLowThR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - For SDIO, Low Priority Queue Page Number High Threshold"]
    #[inline(always)]
    pub fn bit_lpq_high_th(&self) -> BitLpqHighThR {
        BitLpqHighThR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - For SDIO, Extra Priority Queue Page Number Low Threshold"]
    #[inline(always)]
    pub fn bit_exq_low_th(&self) -> BitExqLowThR {
        BitExqLowThR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - For SDIO, Extra Priority Queue Page Number High Threshold"]
    #[inline(always)]
    pub fn bit_exq_high_th(&self) -> BitExqHighThR {
        BitExqHighThR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - For SDIO, Low Priority Queue Page Number Low Threshold"]
    #[inline(always)]
    pub fn bit_lpq_low_th(&mut self) -> BitLpqLowThW<'_, RegTqpnt2Spec> {
        BitLpqLowThW::new(self, 0)
    }
    #[doc = "Bits 8:15 - For SDIO, Low Priority Queue Page Number High Threshold"]
    #[inline(always)]
    pub fn bit_lpq_high_th(&mut self) -> BitLpqHighThW<'_, RegTqpnt2Spec> {
        BitLpqHighThW::new(self, 8)
    }
    #[doc = "Bits 16:23 - For SDIO, Extra Priority Queue Page Number Low Threshold"]
    #[inline(always)]
    pub fn bit_exq_low_th(&mut self) -> BitExqLowThW<'_, RegTqpnt2Spec> {
        BitExqLowThW::new(self, 16)
    }
    #[doc = "Bits 24:31 - For SDIO, Extra Priority Queue Page Number High Threshold"]
    #[inline(always)]
    pub fn bit_exq_high_th(&mut self) -> BitExqHighThW<'_, RegTqpnt2Spec> {
        BitExqHighThW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tqpnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tqpnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTqpnt2Spec;
impl crate::RegisterSpec for RegTqpnt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tqpnt2::R`](R) reader structure"]
impl crate::Readable for RegTqpnt2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_tqpnt2::W`](W) writer structure"]
impl crate::Writable for RegTqpnt2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TQPNT2 to value 0"]
impl crate::Resettable for RegTqpnt2Spec {}
