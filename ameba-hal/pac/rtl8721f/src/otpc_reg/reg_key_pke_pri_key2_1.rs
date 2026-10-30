#[doc = "Register `REG_KEY_PKE_PRI_KEY2_1` reader"]
pub type R = crate::R<RegKeyPkePriKey2_1Spec>;
#[doc = "Field `key_PKE_PRI_KEY2_1` reader - "]
pub type KeyPkePriKey2_1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_pke_pri_key2_1(&self) -> KeyPkePriKey2_1R {
        KeyPkePriKey2_1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_pke_pri_key2_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyPkePriKey2_1Spec;
impl crate::RegisterSpec for RegKeyPkePriKey2_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_pke_pri_key2_1::R`](R) reader structure"]
impl crate::Readable for RegKeyPkePriKey2_1Spec {}
#[doc = "`reset()` method sets REG_KEY_PKE_PRI_KEY2_1 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyPkePriKey2_1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
