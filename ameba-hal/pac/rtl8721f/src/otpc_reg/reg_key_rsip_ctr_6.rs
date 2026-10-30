#[doc = "Register `REG_KEY_RSIP_CTR_6` reader"]
pub type R = crate::R<RegKeyRsipCtr6Spec>;
#[doc = "Field `key_RSIP_CTR_6` reader - "]
pub type KeyRsipCtr6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ctr_6(&self) -> KeyRsipCtr6R {
        KeyRsipCtr6R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ctr_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipCtr6Spec;
impl crate::RegisterSpec for RegKeyRsipCtr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ctr_6::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipCtr6Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_CTR_6 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipCtr6Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
