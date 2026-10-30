#[doc = "Register `Version` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Field `Engine_Version` reader - "]
pub type EngineVersionR = crate::FieldReader<u16>;
#[doc = "Field `SVN_number` reader - "]
pub type SvnNumberR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn engine_version(&self) -> EngineVersionR {
        EngineVersionR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn svn_number(&self) -> SvnNumberR {
        SvnNumberR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Version number register\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`reset()` method sets Version to value 0"]
impl crate::Resettable for VersionSpec {}
