#[doc = "Register `IC_CLR_RX_UNDER` reader"]
pub type R = crate::R<IcClrRxUnderSpec>;
#[doc = "Field `CLR_RX_UNDER` reader - Read this register to clear the RX_UNDER interrupt (bit 0) of IC_RAW_INTR_STAT register."]
pub type ClrRxUnderR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_UNDER interrupt (bit 0) of IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn clr_rx_under(&self) -> ClrRxUnderR {
        ClrRxUnderR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_UNDER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_under::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrRxUnderSpec;
impl crate::RegisterSpec for IcClrRxUnderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_rx_under::R`](R) reader structure"]
impl crate::Readable for IcClrRxUnderSpec {}
#[doc = "`reset()` method sets IC_CLR_RX_UNDER to value 0"]
impl crate::Resettable for IcClrRxUnderSpec {}
