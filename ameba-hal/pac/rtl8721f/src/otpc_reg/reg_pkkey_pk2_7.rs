#[doc = "Register `REG_PKKEY_PK2_7` reader"]
pub type R = crate::R<RegPkkeyPk2_7Spec>;
#[doc = "Field `key_PK2_7` reader - "]
pub type KeyPk2_7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_pk2_7(&self) -> KeyPk2_7R {
        KeyPk2_7R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pkkey_pk2_7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPkkeyPk2_7Spec;
impl crate::RegisterSpec for RegPkkeyPk2_7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pkkey_pk2_7::R`](R) reader structure"]
impl crate::Readable for RegPkkeyPk2_7Spec {}
#[doc = "`reset()` method sets REG_PKKEY_PK2_7 to value 0xffff_ffff"]
impl crate::Resettable for RegPkkeyPk2_7Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
