#[doc = "Register `IC_CLR_RX_OVER` reader"]
pub type R = crate::R<IcClrRxOverSpec>;
#[doc = "Field `CLR_RX_OVER` reader - Read this register to clear the RX_OVER interrupt (bit 1) of IC_RAW_INTR_STAT register."]
pub type ClrRxOverR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_OVER interrupt (bit 1) of IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn clr_rx_over(&self) -> ClrRxOverR {
        ClrRxOverR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_OVER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrRxOverSpec;
impl crate::RegisterSpec for IcClrRxOverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_rx_over::R`](R) reader structure"]
impl crate::Readable for IcClrRxOverSpec {}
#[doc = "`reset()` method sets IC_CLR_RX_OVER to value 0"]
impl crate::Resettable for IcClrRxOverSpec {}
