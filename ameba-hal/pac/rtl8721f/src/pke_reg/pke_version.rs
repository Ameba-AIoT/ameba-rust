#[doc = "Register `PKE_Version` reader"]
pub type R = crate::R<PkeVersionSpec>;
#[doc = "Field `VERSION` reader - 32-bit code update date or version number , such as 32'h20211015"]
pub type VersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit code update date or version number , such as 32'h20211015"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(self.bits)
    }
}
#[doc = "Version Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pke_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkeVersionSpec;
impl crate::RegisterSpec for PkeVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pke_version::R`](R) reader structure"]
impl crate::Readable for PkeVersionSpec {}
#[doc = "`reset()` method sets PKE_Version to value 0"]
impl crate::Resettable for PkeVersionSpec {}
