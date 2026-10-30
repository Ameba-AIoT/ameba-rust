#[doc = "Register `IC_COMP_VERSION` reader"]
pub type R = crate::R<IcCompVersionSpec>;
#[doc = "Field `IC_COMP_VERSION` reader - I2C version number"]
pub type IcCompVersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - I2C version number"]
    #[inline(always)]
    pub fn ic_comp_version(&self) -> IcCompVersionR {
        IcCompVersionR::new(self.bits)
    }
}
#[doc = "I2C Component Version Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_comp_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcCompVersionSpec;
impl crate::RegisterSpec for IcCompVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_comp_version::R`](R) reader structure"]
impl crate::Readable for IcCompVersionSpec {}
#[doc = "`reset()` method sets IC_COMP_VERSION to value 0x2020_0702"]
impl crate::Resettable for IcCompVersionSpec {
    const RESET_VALUE: u32 = 0x2020_0702;
}
