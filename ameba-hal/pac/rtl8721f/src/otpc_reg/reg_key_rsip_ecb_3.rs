#[doc = "Register `REG_KEY_RSIP_ECB_3` reader"]
pub type R = crate::R<RegKeyRsipEcb3Spec>;
#[doc = "Field `key_RSIP_ECB_3` reader - "]
pub type KeyRsipEcb3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ecb_3(&self) -> KeyRsipEcb3R {
        KeyRsipEcb3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ecb_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipEcb3Spec;
impl crate::RegisterSpec for RegKeyRsipEcb3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ecb_3::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipEcb3Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_ECB_3 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipEcb3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
