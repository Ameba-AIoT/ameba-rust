#[doc = "Register `REG_DTIM_COUNT_0_3` reader"]
pub type R = crate::R<RegDtimCount0_3Spec>;
#[doc = "Register `REG_DTIM_COUNT_0_3` writer"]
pub type W = crate::W<RegDtimCount0_3Spec>;
#[doc = "Field `BIT_DTIM_COUNT_ROOT` reader - "]
pub type BitDtimCountRootR = crate::FieldReader;
#[doc = "Field `BIT_DTIM_COUNT_ROOT` writer - "]
pub type BitDtimCountRootW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_DTIM_COUNT_VAP1` reader - "]
pub type BitDtimCountVap1R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_COUNT_VAP1` writer - "]
pub type BitDtimCountVap1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_DTIM_COUNT_VAP2` reader - "]
pub type BitDtimCountVap2R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_COUNT_VAP2` writer - "]
pub type BitDtimCountVap2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_DTIM_COUNT_VAP3` reader - "]
pub type BitDtimCountVap3R = crate::FieldReader;
#[doc = "Field `BIT_DTIM_COUNT_VAP3` writer - "]
pub type BitDtimCountVap3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bit_dtim_count_root(&self) -> BitDtimCountRootR {
        BitDtimCountRootR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn bit_dtim_count_vap1(&self) -> BitDtimCountVap1R {
        BitDtimCountVap1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn bit_dtim_count_vap2(&self) -> BitDtimCountVap2R {
        BitDtimCountVap2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn bit_dtim_count_vap3(&self) -> BitDtimCountVap3R {
        BitDtimCountVap3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bit_dtim_count_root(&mut self) -> BitDtimCountRootW<'_, RegDtimCount0_3Spec> {
        BitDtimCountRootW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn bit_dtim_count_vap1(&mut self) -> BitDtimCountVap1W<'_, RegDtimCount0_3Spec> {
        BitDtimCountVap1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn bit_dtim_count_vap2(&mut self) -> BitDtimCountVap2W<'_, RegDtimCount0_3Spec> {
        BitDtimCountVap2W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn bit_dtim_count_vap3(&mut self) -> BitDtimCountVap3W<'_, RegDtimCount0_3Spec> {
        BitDtimCountVap3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dtim_count_0_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dtim_count_0_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDtimCount0_3Spec;
impl crate::RegisterSpec for RegDtimCount0_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dtim_count_0_3::R`](R) reader structure"]
impl crate::Readable for RegDtimCount0_3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_dtim_count_0_3::W`](W) writer structure"]
impl crate::Writable for RegDtimCount0_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DTIM_COUNT_0_3 to value 0"]
impl crate::Resettable for RegDtimCount0_3Spec {}
