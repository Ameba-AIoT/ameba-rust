#[doc = "Register `ELR_PLD1_S` reader"]
pub type R = crate::R<ElrPld1SSpec>;
#[doc = "Field `ELR_SIZE_S` reader - Size of the error request."]
pub type ElrSizeSR = crate::FieldReader;
#[doc = "Field `ElR_PROT_S` reader - AxProt of the error request."]
pub type ElRProtSR = crate::FieldReader;
#[doc = "Field `ELR_CACHE_S` reader - AxCache of the error request."]
pub type ElrCacheSR = crate::FieldReader;
#[doc = "Field `ELR_LOCK_S` reader - AxLock of the error request."]
pub type ElrLockSR = crate::FieldReader;
impl R {
    #[doc = "Bits 16:18 - Size of the error request."]
    #[inline(always)]
    pub fn elr_size_s(&self) -> ElrSizeSR {
        ElrSizeSR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 23:25 - AxProt of the error request."]
    #[inline(always)]
    pub fn el_r_prot_s(&self) -> ElRProtSR {
        ElRProtSR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:29 - AxCache of the error request."]
    #[inline(always)]
    pub fn elr_cache_s(&self) -> ElrCacheSR {
        ElrCacheSR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - AxLock of the error request."]
    #[inline(always)]
    pub fn elr_lock_s(&self) -> ElrLockSR {
        ElrLockSR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Error log payload register1\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_pld1_s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrPld1SSpec;
impl crate::RegisterSpec for ElrPld1SSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_pld1_s::R`](R) reader structure"]
impl crate::Readable for ElrPld1SSpec {}
#[doc = "`reset()` method sets ELR_PLD1_S to value 0"]
impl crate::Resettable for ElrPld1SSpec {}
