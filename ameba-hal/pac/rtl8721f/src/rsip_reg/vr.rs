#[doc = "Register `VR` reader"]
pub type R = crate::R<VrSpec>;
#[doc = "Field `version` reader - Version of code generation"]
pub type VersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Version of code generation"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(self.bits)
    }
}
#[doc = "Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrSpec;
impl crate::RegisterSpec for VrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vr::R`](R) reader structure"]
impl crate::Readable for VrSpec {}
#[doc = "`reset()` method sets VR to value 0"]
impl crate::Resettable for VrSpec {}
