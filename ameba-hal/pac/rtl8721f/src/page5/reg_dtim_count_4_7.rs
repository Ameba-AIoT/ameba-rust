#[doc = "Register `REG_DTIM_COUNT_4_7` reader"]
pub type R = crate::R<RegDtimCount4_7Spec>;
#[doc = "Register `REG_DTIM_COUNT_4_7` writer"]
pub type W = crate::W<RegDtimCount4_7Spec>;
#[doc = "Field `BIT_DTIM_COUNT_VAP4` reader - "]
pub type BitDtimCountVap4R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_COUNT_VAP4` writer - "]
pub type BitDtimCountVap4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_DTIM_COUNT_VAP5` reader - "]
pub type BitDtimCountVap5R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_COUNT_VAP5` writer - "]
pub type BitDtimCountVap5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_DTIM_COUNT_VAP6` reader - "]
pub type BitDtimCountVap6R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_COUNT_VAP6` writer - "]
pub type BitDtimCountVap6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_DTIM_COUNT_VAP7` reader - "]
pub type BitDtimCountVap7R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_COUNT_VAP7` writer - "]
pub type BitDtimCountVap7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bit_dtim_count_vap4(&self) -> BitDtimCountVap4R {
        BitDtimCountVap4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn bit_dtim_count_vap5(&self) -> BitDtimCountVap5R {
        BitDtimCountVap5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn bit_dtim_count_vap6(&self) -> BitDtimCountVap6R {
        BitDtimCountVap6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn bit_dtim_count_vap7(&self) -> BitDtimCountVap7R {
        BitDtimCountVap7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bit_dtim_count_vap4(&mut self) -> BitDtimCountVap4W<'_, RegDtimCount4_7Spec> {
        BitDtimCountVap4W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn bit_dtim_count_vap5(&mut self) -> BitDtimCountVap5W<'_, RegDtimCount4_7Spec> {
        BitDtimCountVap5W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn bit_dtim_count_vap6(&mut self) -> BitDtimCountVap6W<'_, RegDtimCount4_7Spec> {
        BitDtimCountVap6W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn bit_dtim_count_vap7(&mut self) -> BitDtimCountVap7W<'_, RegDtimCount4_7Spec> {
        BitDtimCountVap7W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dtim_count_4_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dtim_count_4_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDtimCount4_7Spec;
impl crate::RegisterSpec for RegDtimCount4_7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dtim_count_4_7::R`](R) reader structure"]
impl crate::Readable for RegDtimCount4_7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_dtim_count_4_7::W`](W) writer structure"]
impl crate::Writable for RegDtimCount4_7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DTIM_COUNT_4_7 to value 0"]
impl crate::Resettable for RegDtimCount4_7Spec {}
