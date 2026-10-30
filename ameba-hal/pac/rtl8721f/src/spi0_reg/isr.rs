#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `TXEIS` reader - Transmit FIFO Empty Interrupt Status. 0: ssi_txe_intr interrupt is not active after masking 1: ssi_txe_intr interrupt is active after masking"]
pub type TxeisR = crate::BitReader;
#[doc = "Field `TXOIS` reader - Transmit FIFO Overflow Interrupt Status. 0: ssi_txo_intr interrupt is not active after masking 1: ssi_txo_intr interrupt is active after masking"]
pub type TxoisR = crate::BitReader;
#[doc = "Field `RXUIS` reader - Receive FIFO Underflow Interrupt Status. 0: ssi_rxu_intr interrupt is not active after masking 1: ssi_rxu_intr interrupt is active after masking"]
pub type RxuisR = crate::BitReader;
#[doc = "Field `RXOIS` reader - Receive FIFO Overflow Interrupt Status. 0: ssi_rxo_intr interrupt is not active after masking 1: ssi_rxo_intr interrupt is active after masking"]
pub type RxoisR = crate::BitReader;
#[doc = "Field `RXFIS` reader - Receive FIFO Full Interrupt Status. 0: ssi_rxf_intr interrupt is not active after masking 1: ssi_rxf_intr interrupt is full after masking"]
pub type RxfisR = crate::BitReader;
#[doc = "Field `MSTIS_FAEIS` reader - When SPI is configured as serial-master, this bit field is present as Multi-Master Contention Interrupt Status. 0: ssi_mst_intr interrupt is not active after masking 1: ssi_mst_intr interrupt is active after masking When SPI is configured as serial-slave, this bit field is present as Frame Alignment Interrupt Status. 0: ssi_fae_intr interrupt not active after masking 1: ssi_fae_intr interrupt is active after masking"]
pub type MstisFaeisR = crate::BitReader;
#[doc = "Field `TXUIS` reader - Transmit FIFO Under Flow Interrupt Status. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_txu_intr interrupt is not active after masking 1: ssi_txu_intr interrupt is active after masking"]
pub type TxuisR = crate::BitReader;
#[doc = "Field `SSRIS` reader - SS_N Rising Edge Detect Interrupt Status. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_ssr_intr interrupt is not active after masking 1: ssi_ssr_intr interrupt is active after masking"]
pub type SsrisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Empty Interrupt Status. 0: ssi_txe_intr interrupt is not active after masking 1: ssi_txe_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn txeis(&self) -> TxeisR {
        TxeisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Status. 0: ssi_txo_intr interrupt is not active after masking 1: ssi_txo_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn txois(&self) -> TxoisR {
        TxoisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Interrupt Status. 0: ssi_rxu_intr interrupt is not active after masking 1: ssi_rxu_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn rxuis(&self) -> RxuisR {
        RxuisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Interrupt Status. 0: ssi_rxo_intr interrupt is not active after masking 1: ssi_rxo_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn rxois(&self) -> RxoisR {
        RxoisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full Interrupt Status. 0: ssi_rxf_intr interrupt is not active after masking 1: ssi_rxf_intr interrupt is full after masking"]
    #[inline(always)]
    pub fn rxfis(&self) -> RxfisR {
        RxfisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When SPI is configured as serial-master, this bit field is present as Multi-Master Contention Interrupt Status. 0: ssi_mst_intr interrupt is not active after masking 1: ssi_mst_intr interrupt is active after masking When SPI is configured as serial-slave, this bit field is present as Frame Alignment Interrupt Status. 0: ssi_fae_intr interrupt not active after masking 1: ssi_fae_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn mstis_faeis(&self) -> MstisFaeisR {
        MstisFaeisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO Under Flow Interrupt Status. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_txu_intr interrupt is not active after masking 1: ssi_txu_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn txuis(&self) -> TxuisR {
        TxuisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SS_N Rising Edge Detect Interrupt Status. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_ssr_intr interrupt is not active after masking 1: ssi_ssr_intr interrupt is active after masking"]
    #[inline(always)]
    pub fn ssris(&self) -> SsrisR {
        SsrisR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
