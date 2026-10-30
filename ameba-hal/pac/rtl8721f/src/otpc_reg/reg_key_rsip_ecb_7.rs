#[doc = "Register `REG_KEY_RSIP_ECB_7` reader"]
pub type R = crate::R<RegKeyRsipEcb7Spec>;
#[doc = "Field `key_RSIP_ECB_7` reader - "]
pub type KeyRsipEcb7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ecb_7(&self) -> KeyRsipEcb7R {
        KeyRsipEcb7R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ecb_7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipEcb7Spec;
impl crate::RegisterSpec for RegKeyRsipEcb7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ecb_7::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipEcb7Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_ECB_7 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipEcb7Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
