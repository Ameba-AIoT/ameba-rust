#[doc = "Register `REG_TM_TSFT_T1` reader"]
pub type R = crate::R<RegTmTsftT1Spec>;
#[doc = "Field `BIT_TM_TSFT_T1` reader - Latch tsft timer of Tx TM frame, unit: 1us"]
pub type BitTmTsftT1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Latch tsft timer of Tx TM frame, unit: 1us"]
    #[inline(always)]
    pub fn bit_tm_tsft_t1(&self) -> BitTmTsftT1R {
        BitTmTsftT1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tm_tsft_t1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTmTsftT1Spec;
impl crate::RegisterSpec for RegTmTsftT1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tm_tsft_t1::R`](R) reader structure"]
impl crate::Readable for RegTmTsftT1Spec {}
#[doc = "`reset()` method sets REG_TM_TSFT_T1 to value 0"]
impl crate::Resettable for RegTmTsftT1Spec {}
