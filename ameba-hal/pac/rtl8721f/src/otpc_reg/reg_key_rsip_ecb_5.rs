#[doc = "Register `REG_KEY_RSIP_ECB_5` reader"]
pub type R = crate::R<RegKeyRsipEcb5Spec>;
#[doc = "Field `key_RSIP_ECB_5` reader - "]
pub type KeyRsipEcb5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ecb_5(&self) -> KeyRsipEcb5R {
        KeyRsipEcb5R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ecb_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipEcb5Spec;
impl crate::RegisterSpec for RegKeyRsipEcb5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ecb_5::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipEcb5Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_ECB_5 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipEcb5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
