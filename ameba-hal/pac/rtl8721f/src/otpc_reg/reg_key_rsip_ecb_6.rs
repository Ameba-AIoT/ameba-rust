#[doc = "Register `REG_KEY_RSIP_ECB_6` reader"]
pub type R = crate::R<RegKeyRsipEcb6Spec>;
#[doc = "Field `key_RSIP_ECB_6` reader - "]
pub type KeyRsipEcb6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ecb_6(&self) -> KeyRsipEcb6R {
        KeyRsipEcb6R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ecb_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipEcb6Spec;
impl crate::RegisterSpec for RegKeyRsipEcb6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ecb_6::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipEcb6Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_ECB_6 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipEcb6Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
