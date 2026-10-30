#[doc = "Register `REG_KEY_RSIP_CTR_1` reader"]
pub type R = crate::R<RegKeyRsipCtr1Spec>;
#[doc = "Field `key_RSIP_CTR_1` reader - "]
pub type KeyRsipCtr1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ctr_1(&self) -> KeyRsipCtr1R {
        KeyRsipCtr1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ctr_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipCtr1Spec;
impl crate::RegisterSpec for RegKeyRsipCtr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ctr_1::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipCtr1Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_CTR_1 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipCtr1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
