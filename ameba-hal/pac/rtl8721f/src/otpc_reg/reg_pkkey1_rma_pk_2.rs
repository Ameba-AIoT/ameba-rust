#[doc = "Register `REG_PKKEY1_RMA_PK_2` reader"]
pub type R = crate::R<RegPkkey1RmaPk2Spec>;
#[doc = "Field `key_RMA_PK_2` reader - "]
pub type KeyRmaPk2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rma_pk_2(&self) -> KeyRmaPk2R {
        KeyRmaPk2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pkkey1_rma_pk_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPkkey1RmaPk2Spec;
impl crate::RegisterSpec for RegPkkey1RmaPk2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pkkey1_rma_pk_2::R`](R) reader structure"]
impl crate::Readable for RegPkkey1RmaPk2Spec {}
#[doc = "`reset()` method sets REG_PKKEY1_RMA_PK_2 to value 0xffff_ffff"]
impl crate::Resettable for RegPkkey1RmaPk2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
