#[doc = "Register `RVE` reader"]
pub type R = crate::R<RveSpec>;
#[doc = "Field `REV` reader - The design reversion of RXI-300."]
pub type RevR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The design reversion of RXI-300."]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new(self.bits)
    }
}
#[doc = "RXI300 design reversion register\n\nYou can [`read`](crate::Reg::read) this register and get [`rve::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RveSpec;
impl crate::RegisterSpec for RveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rve::R`](R) reader structure"]
impl crate::Readable for RveSpec {}
#[doc = "`reset()` method sets RVE to value 0x01"]
impl crate::Resettable for RveSpec {
    const RESET_VALUE: u32 = 0x01;
}
