#[doc = "Register `IDR` reader"]
pub type R = crate::R<IdrSpec>;
#[doc = "Field `IDCODE` reader - Contain the decimal value of SPIC version."]
pub type IdcodeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Contain the decimal value of SPIC version."]
    #[inline(always)]
    pub fn idcode(&self) -> IdcodeR {
        IdcodeR::new(self.bits)
    }
}
#[doc = "Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IdrSpec {}
#[doc = "`reset()` method sets IDR to value 0x0400_ff01"]
impl crate::Resettable for IdrSpec {
    const RESET_VALUE: u32 = 0x0400_ff01;
}
