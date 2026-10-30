#[doc = "Register `REG_TM_TSFT_T3` reader"]
pub type R = crate::R<RegTmTsftT3Spec>;
#[doc = "Field `BIT_TM_TSFT_T3` reader - Latch tsft timer of Tx ACK frame, unit: 1us"]
pub type BitTmTsftT3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Latch tsft timer of Tx ACK frame, unit: 1us"]
    #[inline(always)]
    pub fn bit_tm_tsft_t3(&self) -> BitTmTsftT3R {
        BitTmTsftT3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tm_tsft_t3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTmTsftT3Spec;
impl crate::RegisterSpec for RegTmTsftT3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tm_tsft_t3::R`](R) reader structure"]
impl crate::Readable for RegTmTsftT3Spec {}
#[doc = "`reset()` method sets REG_TM_TSFT_T3 to value 0"]
impl crate::Resettable for RegTmTsftT3Spec {}
