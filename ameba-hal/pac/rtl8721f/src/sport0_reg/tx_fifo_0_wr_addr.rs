#[doc = "Register `TX_FIFO_0_WR_ADDR` writer"]
pub type W = crate::W<TxFifo0WrAddrSpec>;
#[doc = "Field `TX_FIFO_0_WR_ADDR` writer - TX_FIFO_0_WR_ADDR"]
pub type TxFifo0WrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - TX_FIFO_0_WR_ADDR"]
    #[inline(always)]
    pub fn tx_fifo_0_wr_addr(&mut self) -> TxFifo0WrAddrW<'_, TxFifo0WrAddrSpec> {
        TxFifo0WrAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fifo_0_wr_addr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFifo0WrAddrSpec;
impl crate::RegisterSpec for TxFifo0WrAddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_fifo_0_wr_addr::W`](W) writer structure"]
impl crate::Writable for TxFifo0WrAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_FIFO_0_WR_ADDR to value 0"]
impl crate::Resettable for TxFifo0WrAddrSpec {}
