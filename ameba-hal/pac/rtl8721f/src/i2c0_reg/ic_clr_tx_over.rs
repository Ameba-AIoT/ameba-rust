#[doc = "Register `IC_CLR_TX_OVER` reader"]
pub type R = crate::R<IcClrTxOverSpec>;
#[doc = "Field `CLR_TX_OVER` reader - Read this register to clear the TX_OVER interrupt (bit 3) of IC_RAW_INTR_STAT register."]
pub type ClrTxOverR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the TX_OVER interrupt (bit 3) of IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn clr_tx_over(&self) -> ClrTxOverR {
        ClrTxOverR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_OVER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_tx_over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrTxOverSpec;
impl crate::RegisterSpec for IcClrTxOverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_tx_over::R`](R) reader structure"]
impl crate::Readable for IcClrTxOverSpec {}
#[doc = "`reset()` method sets IC_CLR_TX_OVER to value 0"]
impl crate::Resettable for IcClrTxOverSpec {}
