#[doc = "Register `SPIC_VERSION` reader"]
pub type R = crate::R<SpicVersionSpec>;
#[doc = "Field `SPIC_VERSION` reader - Contain the decimal value of SPIC version. (After 201612: SVN_NUM changes to Git Counter)"]
pub type SpicVersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Contain the decimal value of SPIC version. (After 201612: SVN_NUM changes to Git Counter)"]
    #[inline(always)]
    pub fn spic_version(&self) -> SpicVersionR {
        SpicVersionR::new(self.bits)
    }
}
#[doc = "SPIC version ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spic_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpicVersionSpec;
impl crate::RegisterSpec for SpicVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spic_version::R`](R) reader structure"]
impl crate::Readable for SpicVersionSpec {}
#[doc = "`reset()` method sets SPIC_VERSION to value 0"]
impl crate::Resettable for SpicVersionSpec {}
