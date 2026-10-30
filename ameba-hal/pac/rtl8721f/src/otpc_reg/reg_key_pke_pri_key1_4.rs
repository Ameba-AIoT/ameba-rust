#[doc = "Register `REG_KEY_PKE_PRI_KEY1_4` reader"]
pub type R = crate::R<RegKeyPkePriKey1_4Spec>;
#[doc = "Field `key_PKE_PRI_KEY1_4` reader - "]
pub type KeyPkePriKey1_4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_pke_pri_key1_4(&self) -> KeyPkePriKey1_4R {
        KeyPkePriKey1_4R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_pke_pri_key1_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyPkePriKey1_4Spec;
impl crate::RegisterSpec for RegKeyPkePriKey1_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_pke_pri_key1_4::R`](R) reader structure"]
impl crate::Readable for RegKeyPkePriKey1_4Spec {}
#[doc = "`reset()` method sets REG_KEY_PKE_PRI_KEY1_4 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyPkePriKey1_4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
