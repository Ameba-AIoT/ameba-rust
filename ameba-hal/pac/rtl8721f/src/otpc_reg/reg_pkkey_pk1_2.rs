#[doc = "Register `REG_PKKEY_PK1_2` reader"]
pub type R = crate::R<RegPkkeyPk1_2Spec>;
#[doc = "Field `key_PK1_2` reader - "]
pub type KeyPk1_2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_pk1_2(&self) -> KeyPk1_2R {
        KeyPk1_2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pkkey_pk1_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPkkeyPk1_2Spec;
impl crate::RegisterSpec for RegPkkeyPk1_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pkkey_pk1_2::R`](R) reader structure"]
impl crate::Readable for RegPkkeyPk1_2Spec {}
#[doc = "`reset()` method sets REG_PKKEY_PK1_2 to value 0xffff_ffff"]
impl crate::Resettable for RegPkkeyPk1_2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
