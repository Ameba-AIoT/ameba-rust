#[doc = "Register `REG_PKKEY_PK1_5` reader"]
pub type R = crate::R<RegPkkeyPk1_5Spec>;
#[doc = "Field `key_PK1_5` reader - "]
pub type KeyPk1_5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_pk1_5(&self) -> KeyPk1_5R {
        KeyPk1_5R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pkkey_pk1_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPkkeyPk1_5Spec;
impl crate::RegisterSpec for RegPkkeyPk1_5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pkkey_pk1_5::R`](R) reader structure"]
impl crate::Readable for RegPkkeyPk1_5Spec {}
#[doc = "`reset()` method sets REG_PKKEY_PK1_5 to value 0xffff_ffff"]
impl crate::Resettable for RegPkkeyPk1_5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
