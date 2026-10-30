#[doc = "Register `REG_KEY_RSIP_CTR_2` reader"]
pub type R = crate::R<RegKeyRsipCtr2Spec>;
#[doc = "Field `key_RSIP_CTR_2` reader - "]
pub type KeyRsipCtr2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ctr_2(&self) -> KeyRsipCtr2R {
        KeyRsipCtr2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ctr_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipCtr2Spec;
impl crate::RegisterSpec for RegKeyRsipCtr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ctr_2::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipCtr2Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_CTR_2 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipCtr2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
