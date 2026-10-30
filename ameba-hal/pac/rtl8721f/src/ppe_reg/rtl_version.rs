#[doc = "Register `RTL_VERSION` reader"]
pub type R = crate::R<RtlVersionSpec>;
#[doc = "Field `RTL_VERSION` reader - RTL version"]
pub type RtlVersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RTL version"]
    #[inline(always)]
    pub fn rtl_version(&self) -> RtlVersionR {
        RtlVersionR::new(self.bits)
    }
}
#[doc = "RTL version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtl_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtlVersionSpec;
impl crate::RegisterSpec for RtlVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtl_version::R`](R) reader structure"]
impl crate::Readable for RtlVersionSpec {}
#[doc = "`reset()` method sets RTL_VERSION to value 0"]
impl crate::Resettable for RtlVersionSpec {}
