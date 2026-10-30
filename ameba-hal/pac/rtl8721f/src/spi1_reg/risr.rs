#[doc = "Register `RISR` reader"]
pub type R = crate::R<RisrSpec>;
#[doc = "Field `TXEIR` reader - Transmit FIFO Empty Raw Interrupt Status. 0: ssi_txe_intr interrupt is not active prior to masking 1: ssi_txe_intr interrupt is active prior to masking"]
pub type TxeirR = crate::BitReader;
#[doc = "Field `TXOIR` reader - Transmit FIFO Overflow Raw Interrupt Status. 0: ssi_txo_intr interrupt is not active prior to masking 1: ssi_txo_intr interrupt is active prior to masking"]
pub type TxoirR = crate::BitReader;
#[doc = "Field `RXUIR` reader - Receive FIFO Underflow Raw Interrupt Status. 0: ssi_rxu_intr interrupt is not active prior to masking 1: ssi_rxu_intr interrupt is active prior to masking"]
pub type RxuirR = crate::BitReader;
#[doc = "Field `RXOIR` reader - Receive FIFO Overflow Raw Interrupt Status. 0: ssi_rxo_intr interrupt is not active prior to masking 1: ssi_rxo_intr interrupt is active prior to masking"]
pub type RxoirR = crate::BitReader;
#[doc = "Field `RXFIR` reader - Receive FIFO Full Raw Interrupt Status. 0: ssi_rxf_intr interrupt is not active prior to masking 1: ssi_rxf_intr interrupt is active prior to masking"]
pub type RxfirR = crate::BitReader;
#[doc = "Field `MSTIR_FAEIR` reader - When SPI is configured as serial-master, this bit field is present as Multi-Master Contention Raw Interrupt Status. 0: ssi_mst_intr interrupt is not active prior to masking 1: ssi_mst_intr interrupt is active prior to masking When SPI is configured as serial-slave, this bit field is present as Frame Alignment Error Raw Interrupt Status. 0: ssi_fae_intr interrupt not active prior to masking 1: ssi_fae_intr interrupt is active prior to masking"]
pub type MstirFaeirR = crate::BitReader;
#[doc = "Field `TXUIR` reader - Transmit FIFO Under Flow Raw Interrupt Status. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_txu_intr interrupt is not active prior to masking 1: ssi_txu_intr interrupt is active prior to masking"]
pub type TxuirR = crate::BitReader;
#[doc = "Field `SSRIR` reader - SS_N Rising Edge Detect Raw Interrupt Status. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_ssr_intr interrupt is not active prior to masking 1: ssi_ssr_intr interrupt is active prior to masking"]
pub type SsrirR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Empty Raw Interrupt Status. 0: ssi_txe_intr interrupt is not active prior to masking 1: ssi_txe_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn txeir(&self) -> TxeirR {
        TxeirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Raw Interrupt Status. 0: ssi_txo_intr interrupt is not active prior to masking 1: ssi_txo_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn txoir(&self) -> TxoirR {
        TxoirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Raw Interrupt Status. 0: ssi_rxu_intr interrupt is not active prior to masking 1: ssi_rxu_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn rxuir(&self) -> RxuirR {
        RxuirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Raw Interrupt Status. 0: ssi_rxo_intr interrupt is not active prior to masking 1: ssi_rxo_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn rxoir(&self) -> RxoirR {
        RxoirR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full Raw Interrupt Status. 0: ssi_rxf_intr interrupt is not active prior to masking 1: ssi_rxf_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn rxfir(&self) -> RxfirR {
        RxfirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When SPI is configured as serial-master, this bit field is present as Multi-Master Contention Raw Interrupt Status. 0: ssi_mst_intr interrupt is not active prior to masking 1: ssi_mst_intr interrupt is active prior to masking When SPI is configured as serial-slave, this bit field is present as Frame Alignment Error Raw Interrupt Status. 0: ssi_fae_intr interrupt not active prior to masking 1: ssi_fae_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn mstir_faeir(&self) -> MstirFaeirR {
        MstirFaeirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO Under Flow Raw Interrupt Status. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_txu_intr interrupt is not active prior to masking 1: ssi_txu_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn txuir(&self) -> TxuirR {
        TxuirR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SS_N Rising Edge Detect Raw Interrupt Status. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_ssr_intr interrupt is not active prior to masking 1: ssi_ssr_intr interrupt is active prior to masking"]
    #[inline(always)]
    pub fn ssrir(&self) -> SsrirR {
        SsrirR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`risr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisrSpec;
impl crate::RegisterSpec for RisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`risr::R`](R) reader structure"]
impl crate::Readable for RisrSpec {}
#[doc = "`reset()` method sets RISR to value 0"]
impl crate::Resettable for RisrSpec {}
