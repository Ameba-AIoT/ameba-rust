#[doc = "Register `REG_KEY_RSIP_CTR_7` reader"]
pub type R = crate::R<RegKeyRsipCtr7Spec>;
#[doc = "Field `key_RSIP_CTR_7` reader - "]
pub type KeyRsipCtr7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ctr_7(&self) -> KeyRsipCtr7R {
        KeyRsipCtr7R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ctr_7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipCtr7Spec;
impl crate::RegisterSpec for RegKeyRsipCtr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ctr_7::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipCtr7Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_CTR_7 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipCtr7Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
