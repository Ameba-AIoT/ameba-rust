#[doc = "Register `REG_KEY_RSIP_CTR_3` reader"]
pub type R = crate::R<RegKeyRsipCtr3Spec>;
#[doc = "Field `key_RSIP_CTR_3` reader - "]
pub type KeyRsipCtr3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ctr_3(&self) -> KeyRsipCtr3R {
        KeyRsipCtr3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ctr_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipCtr3Spec;
impl crate::RegisterSpec for RegKeyRsipCtr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ctr_3::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipCtr3Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_CTR_3 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipCtr3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
