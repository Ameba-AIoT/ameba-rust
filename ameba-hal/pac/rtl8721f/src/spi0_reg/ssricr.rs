#[doc = "Register `SSRICR` reader"]
pub type R = crate::R<SsricrSpec>;
#[doc = "Field `SSRICR` reader - When SPI is configured as serial-slave, this register is used to Clear SS_N Rinsing Edge Detect Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_ssr_intr interrupt; writing has no effect."]
pub type SsricrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When SPI is configured as serial-slave, this register is used to Clear SS_N Rinsing Edge Detect Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_ssr_intr interrupt; writing has no effect."]
    #[inline(always)]
    pub fn ssricr(&self) -> SsricrR {
        SsricrR::new((self.bits & 1) != 0)
    }
}
#[doc = "SS_N Rising Edge Detect Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssricr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsricrSpec;
impl crate::RegisterSpec for SsricrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssricr::R`](R) reader structure"]
impl crate::Readable for SsricrSpec {}
#[doc = "`reset()` method sets SSRICR to value 0"]
impl crate::Resettable for SsricrSpec {}
