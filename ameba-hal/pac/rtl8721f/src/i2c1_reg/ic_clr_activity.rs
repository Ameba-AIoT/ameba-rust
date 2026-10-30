#[doc = "Register `IC_CLR_ACTIVITY` reader"]
pub type R = crate::R<IcClrActivitySpec>;
#[doc = "Field `CLR_ACTIVITY` reader - Read this register to clear the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register."]
pub type ClrActivityR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn clr_activity(&self) -> ClrActivityR {
        ClrActivityR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear ACTIVITY Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_activity::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrActivitySpec;
impl crate::RegisterSpec for IcClrActivitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_activity::R`](R) reader structure"]
impl crate::Readable for IcClrActivitySpec {}
#[doc = "`reset()` method sets IC_CLR_ACTIVITY to value 0"]
impl crate::Resettable for IcClrActivitySpec {}
