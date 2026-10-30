#[doc = "Register `REG_KEY_RSIP_ECB_4` reader"]
pub type R = crate::R<RegKeyRsipEcb4Spec>;
#[doc = "Field `key_RSIP_ECB_4` reader - "]
pub type KeyRsipEcb4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ecb_4(&self) -> KeyRsipEcb4R {
        KeyRsipEcb4R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ecb_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipEcb4Spec;
impl crate::RegisterSpec for RegKeyRsipEcb4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ecb_4::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipEcb4Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_ECB_4 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipEcb4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
