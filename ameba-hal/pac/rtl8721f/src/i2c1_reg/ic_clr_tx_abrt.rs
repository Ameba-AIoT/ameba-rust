#[doc = "Register `IC_CLR_TX_ABRT` reader"]
pub type R = crate::R<IcClrTxAbrtSpec>;
#[doc = "Field `CLR_TX_ABRT` reader - Read this register to clear the TX_ABRT interrupt (bit 6) of IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This is also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO."]
pub type ClrTxAbrtR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the TX_ABRT interrupt (bit 6) of IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This is also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO."]
    #[inline(always)]
    pub fn clr_tx_abrt(&self) -> ClrTxAbrtR {
        ClrTxAbrtR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear TX_ABRT Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_tx_abrt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrTxAbrtSpec;
impl crate::RegisterSpec for IcClrTxAbrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_tx_abrt::R`](R) reader structure"]
impl crate::Readable for IcClrTxAbrtSpec {}
#[doc = "`reset()` method sets IC_CLR_TX_ABRT to value 0"]
impl crate::Resettable for IcClrTxAbrtSpec {}
