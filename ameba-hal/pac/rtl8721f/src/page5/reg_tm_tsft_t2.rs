#[doc = "Register `REG_TM_TSFT_T2` reader"]
pub type R = crate::R<RegTmTsftT2Spec>;
#[doc = "Field `BIT_TM_TSFT_T2` reader - Latch tsft timer of Rx TM frame, unit: 1us"]
pub type BitTmTsftT2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Latch tsft timer of Rx TM frame, unit: 1us"]
    #[inline(always)]
    pub fn bit_tm_tsft_t2(&self) -> BitTmTsftT2R {
        BitTmTsftT2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tm_tsft_t2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTmTsftT2Spec;
impl crate::RegisterSpec for RegTmTsftT2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tm_tsft_t2::R`](R) reader structure"]
impl crate::Readable for RegTmTsftT2Spec {}
#[doc = "`reset()` method sets REG_TM_TSFT_T2 to value 0"]
impl crate::Resettable for RegTmTsftT2Spec {}
