#[doc = "Register `REG_TM_TSFT_T4` reader"]
pub type R = crate::R<RegTmTsftT4Spec>;
#[doc = "Field `BIT_TM_TSFT_T4` reader - Latch tsft timer of Rx ACK frame, unit: 1us"]
pub type BitTmTsftT4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Latch tsft timer of Rx ACK frame, unit: 1us"]
    #[inline(always)]
    pub fn bit_tm_tsft_t4(&self) -> BitTmTsftT4R {
        BitTmTsftT4R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tm_tsft_t4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTmTsftT4Spec;
impl crate::RegisterSpec for RegTmTsftT4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tm_tsft_t4::R`](R) reader structure"]
impl crate::Readable for RegTmTsftT4Spec {}
#[doc = "`reset()` method sets REG_TM_TSFT_T4 to value 0"]
impl crate::Resettable for RegTmTsftT4Spec {}
