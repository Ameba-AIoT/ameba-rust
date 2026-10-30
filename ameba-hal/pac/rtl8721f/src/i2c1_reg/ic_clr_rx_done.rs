#[doc = "Register `IC_CLR_RX_DONE` reader"]
pub type R = crate::R<IcClrRxDoneSpec>;
#[doc = "Field `CLR_RX_DONE` reader - Read this register to clear the RX_DONE interrupt (bit 7) of IC_RAW_INTR_STAT register."]
pub type ClrRxDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RX_DONE interrupt (bit 7) of IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn clr_rx_done(&self) -> ClrRxDoneR {
        ClrRxDoneR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RX_DONE Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_done::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrRxDoneSpec;
impl crate::RegisterSpec for IcClrRxDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_rx_done::R`](R) reader structure"]
impl crate::Readable for IcClrRxDoneSpec {}
#[doc = "`reset()` method sets IC_CLR_RX_DONE to value 0"]
impl crate::Resettable for IcClrRxDoneSpec {}
