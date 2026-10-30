#[doc = "Register `REG_PKKEY_PK1_1` reader"]
pub type R = crate::R<RegPkkeyPk1_1Spec>;
#[doc = "Field `key_PK1_1` reader - "]
pub type KeyPk1_1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_pk1_1(&self) -> KeyPk1_1R {
        KeyPk1_1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pkkey_pk1_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPkkeyPk1_1Spec;
impl crate::RegisterSpec for RegPkkeyPk1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pkkey_pk1_1::R`](R) reader structure"]
impl crate::Readable for RegPkkeyPk1_1Spec {}
#[doc = "`reset()` method sets REG_PKKEY_PK1_1 to value 0xffff_ffff"]
impl crate::Resettable for RegPkkeyPk1_1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
