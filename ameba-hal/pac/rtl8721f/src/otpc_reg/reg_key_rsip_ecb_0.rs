#[doc = "Register `REG_KEY_RSIP_ECB_0` reader"]
pub type R = crate::R<RegKeyRsipEcb0Spec>;
#[doc = "Field `key_RSIP_ECB_0` reader - "]
pub type KeyRsipEcb0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ecb_0(&self) -> KeyRsipEcb0R {
        KeyRsipEcb0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ecb_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipEcb0Spec;
impl crate::RegisterSpec for RegKeyRsipEcb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ecb_0::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipEcb0Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_ECB_0 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipEcb0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
