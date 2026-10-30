#[doc = "Register `REG_KEY_RSIP_CTR_4` reader"]
pub type R = crate::R<RegKeyRsipCtr4Spec>;
#[doc = "Field `key_RSIP_CTR_4` reader - "]
pub type KeyRsipCtr4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rsip_ctr_4(&self) -> KeyRsipCtr4R {
        KeyRsipCtr4R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_rsip_ctr_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyRsipCtr4Spec;
impl crate::RegisterSpec for RegKeyRsipCtr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_rsip_ctr_4::R`](R) reader structure"]
impl crate::Readable for RegKeyRsipCtr4Spec {}
#[doc = "`reset()` method sets REG_KEY_RSIP_CTR_4 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyRsipCtr4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
