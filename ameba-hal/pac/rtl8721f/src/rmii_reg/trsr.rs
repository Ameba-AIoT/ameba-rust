#[doc = "Register `TRSR` reader"]
pub type R = crate::R<TrsrSpec>;
#[doc = "Field `RBU` reader - Rx buff is un-available, rx full"]
pub type RbuR = crate::BitReader;
#[doc = "Field `RxFIFOEmpty` reader - Rx Fifo is Empty."]
pub type RxFifoemptyR = crate::BitReader;
#[doc = "Field `TXUNDER` reader - Transmit FIFO Underrun: Set to 1 if the Tx FIFO was exhausted during the transmission of a packet."]
pub type TxunderR = crate::BitReader;
#[doc = "Field `TOK` reader - Transmit OK: Set to 1 indicates that the transmission of a packet was completed successfully and no transmit underrun occurs."]
pub type TokR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Rx buff is un-available, rx full"]
    #[inline(always)]
    pub fn rbu(&self) -> RbuR {
        RbuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx Fifo is Empty."]
    #[inline(always)]
    pub fn rx_fifoempty(&self) -> RxFifoemptyR {
        RxFifoemptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Underrun: Set to 1 if the Tx FIFO was exhausted during the transmission of a packet."]
    #[inline(always)]
    pub fn txunder(&self) -> TxunderR {
        TxunderR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit OK: Set to 1 indicates that the transmission of a packet was completed successfully and no transmit underrun occurs."]
    #[inline(always)]
    pub fn tok(&self) -> TokR {
        TokR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`trsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrsrSpec;
impl crate::RegisterSpec for TrsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trsr::R`](R) reader structure"]
impl crate::Readable for TrsrSpec {}
#[doc = "`reset()` method sets TRSR to value 0"]
impl crate::Resettable for TrsrSpec {}
