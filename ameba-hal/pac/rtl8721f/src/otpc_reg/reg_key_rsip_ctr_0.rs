#[doc = "Register `REG_KEY_RSIP_CTR_0` reader"]
pub type R = crate::R<RegKeyRsipCtr0Spec>;
#[doc = "Field `key_RSIP_CTR_0` reader - "]
pub type KeyRsipCtr0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ctr_0(&self) -> KeyRsipCtr0R {
        KeyRsipCtr0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ctr_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipCtr0Spec;
impl crate::RegisterSpec for RegKeyRsipCtr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ctr_0::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipCtr0Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_CTR_0 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipCtr0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
