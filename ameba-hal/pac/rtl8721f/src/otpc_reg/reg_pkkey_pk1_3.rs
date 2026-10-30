#[doc = "Register `REG_PKKEY_PK1_3` reader"]
pub type R = crate::R<RegPkkeyPk1_3Spec>;
#[doc = "Field `key_PK1_3` reader - "]
pub type KeyPk1_3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_pk1_3(&self) -> KeyPk1_3R {
        KeyPk1_3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pkkey_pk1_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPkkeyPk1_3Spec;
impl crate::RegisterSpec for RegPkkeyPk1_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pkkey_pk1_3::R`](R) reader structure"]
impl crate::Readable for RegPkkeyPk1_3Spec {}
#[doc = "`reset()` method sets REG_PKKEY_PK1_3 to value 0xffff_ffff"]
impl crate::Resettable for RegPkkeyPk1_3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
