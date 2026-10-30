#[doc = "Register `ELR_PLD1` reader"]
pub type R = crate::R<ElrPld1Spec>;
#[doc = "Field `ELR_SIZE` reader - Size of the error request."]
pub type ElrSizeR = crate::FieldReader;
#[doc = "Field `ElR_PROT` reader - AxProt of the error request."]
pub type ElRProtR = crate::FieldReader;
#[doc = "Field `ELR_CACHE` reader - AxCache of the error request."]
pub type ElrCacheR = crate::FieldReader;
#[doc = "Field `ELR_LOCK` reader - AxLock of the error request."]
pub type ElrLockR = crate::FieldReader;
impl R {
    #[doc = "Bits 16:18 - Size of the error request."]
    #[inline(always)]
    pub fn elr_size(&self) -> ElrSizeR {
        ElrSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 23:25 - AxProt of the error request."]
    #[inline(always)]
    pub fn el_r_prot(&self) -> ElRProtR {
        ElRProtR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:29 - AxCache of the error request."]
    #[inline(always)]
    pub fn elr_cache(&self) -> ElrCacheR {
        ElrCacheR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - AxLock of the error request."]
    #[inline(always)]
    pub fn elr_lock(&self) -> ElrLockR {
        ElrLockR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Error log payload register1\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_pld1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrPld1Spec;
impl crate::RegisterSpec for ElrPld1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_pld1::R`](R) reader structure"]
impl crate::Readable for ElrPld1Spec {}
#[doc = "`reset()` method sets ELR_PLD1 to value 0"]
impl crate::Resettable for ElrPld1Spec {}
