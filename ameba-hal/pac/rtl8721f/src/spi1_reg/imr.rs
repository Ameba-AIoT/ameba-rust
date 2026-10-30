#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `TXEIM` reader - Transmit FIFO Empty Interrupt Mask. 0: ssi_txe_intr interrupt is masked 1: ssi_txe_intr interrupt is not masked"]
pub type TxeimR = crate::BitReader;
#[doc = "Field `TXEIM` writer - Transmit FIFO Empty Interrupt Mask. 0: ssi_txe_intr interrupt is masked 1: ssi_txe_intr interrupt is not masked"]
pub type TxeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOIM` reader - Transmit FIFO Overflow Interrupt Mask. 0: ssi_txo_intr interrupt is masked 1: ssi_txo_intr interrupt is not masked"]
pub type TxoimR = crate::BitReader;
#[doc = "Field `TXOIM` writer - Transmit FIFO Overflow Interrupt Mask. 0: ssi_txo_intr interrupt is masked 1: ssi_txo_intr interrupt is not masked"]
pub type TxoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUIM` reader - Receive FIFO Underflow Interrupt Mask. 0: ssi_rxu_intr interrupt is masked 1: ssi_rxu_intr interrupt is not masked"]
pub type RxuimR = crate::BitReader;
#[doc = "Field `RXUIM` writer - Receive FIFO Underflow Interrupt Mask. 0: ssi_rxu_intr interrupt is masked 1: ssi_rxu_intr interrupt is not masked"]
pub type RxuimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOIM` reader - Receive FIFO Overflow Interrupt Mask. 0: ssi_rxo_intr interrupt is masked 1: ssi_rxo_intr interrupt is not masked"]
pub type RxoimR = crate::BitReader;
#[doc = "Field `RXOIM` writer - Receive FIFO Overflow Interrupt Mask. 0: ssi_rxo_intr interrupt is masked 1: ssi_rxo_intr interrupt is not masked"]
pub type RxoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIM` reader - Receive FIFO Full Interrupt Mask. 0: ssi_rxf_intr interrupt is masked 1: ssi_rxf_intr interrupt is not masked"]
pub type RxfimR = crate::BitReader;
#[doc = "Field `RXFIM` writer - Receive FIFO Full Interrupt Mask. 0: ssi_rxf_intr interrupt is masked 1: ssi_rxf_intr interrupt is not masked"]
pub type RxfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTIM_FAEIM` reader - When SPI is configured as serial-master, this bit field is present as Multi-Master Contention Interrupt Mask. 0: ssi_mst_intr interrupt is masked 1: ssi_mst_intr interrupt is not masked When SPI is configured as serial-slave, this bit field is present as Frame Alignment Interrupt Mask. 0: ssi_fae_intr interrupt is masked 1: ssi_fae_intr interrupt is not masked"]
pub type MstimFaeimR = crate::BitReader;
#[doc = "Field `MSTIM_FAEIM` writer - When SPI is configured as serial-master, this bit field is present as Multi-Master Contention Interrupt Mask. 0: ssi_mst_intr interrupt is masked 1: ssi_mst_intr interrupt is not masked When SPI is configured as serial-slave, this bit field is present as Frame Alignment Interrupt Mask. 0: ssi_fae_intr interrupt is masked 1: ssi_fae_intr interrupt is not masked"]
pub type MstimFaeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUIM` reader - Transmit FIFO Underflow Interrupt Mask. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_txu_intr is masked 1: ssi_txu_intr is not masked"]
pub type TxuimR = crate::BitReader;
#[doc = "Field `TXUIM` writer - Transmit FIFO Underflow Interrupt Mask. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_txu_intr is masked 1: ssi_txu_intr is not masked"]
pub type TxuimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSRIM` reader - SS_N Rising Edge Detect Interrupt Mask. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_ssr_intr is masked 1: ssi_ssr_intr is not masked"]
pub type SsrimR = crate::BitReader;
#[doc = "Field `SSRIM` writer - SS_N Rising Edge Detect Interrupt Mask. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_ssr_intr is masked 1: ssi_ssr_intr is not masked"]
pub type SsrimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Empty Interrupt Mask. 0: ssi_txe_intr interrupt is masked 1: ssi_txe_intr interrupt is not masked"]
    #[inline(always)]
    pub fn txeim(&self) -> TxeimR {
        TxeimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Mask. 0: ssi_txo_intr interrupt is masked 1: ssi_txo_intr interrupt is not masked"]
    #[inline(always)]
    pub fn txoim(&self) -> TxoimR {
        TxoimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Interrupt Mask. 0: ssi_rxu_intr interrupt is masked 1: ssi_rxu_intr interrupt is not masked"]
    #[inline(always)]
    pub fn rxuim(&self) -> RxuimR {
        RxuimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Interrupt Mask. 0: ssi_rxo_intr interrupt is masked 1: ssi_rxo_intr interrupt is not masked"]
    #[inline(always)]
    pub fn rxoim(&self) -> RxoimR {
        RxoimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full Interrupt Mask. 0: ssi_rxf_intr interrupt is masked 1: ssi_rxf_intr interrupt is not masked"]
    #[inline(always)]
    pub fn rxfim(&self) -> RxfimR {
        RxfimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When SPI is configured as serial-master, this bit field is present as Multi-Master Contention Interrupt Mask. 0: ssi_mst_intr interrupt is masked 1: ssi_mst_intr interrupt is not masked When SPI is configured as serial-slave, this bit field is present as Frame Alignment Interrupt Mask. 0: ssi_fae_intr interrupt is masked 1: ssi_fae_intr interrupt is not masked"]
    #[inline(always)]
    pub fn mstim_faeim(&self) -> MstimFaeimR {
        MstimFaeimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO Underflow Interrupt Mask. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_txu_intr is masked 1: ssi_txu_intr is not masked"]
    #[inline(always)]
    pub fn txuim(&self) -> TxuimR {
        TxuimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SS_N Rising Edge Detect Interrupt Mask. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_ssr_intr is masked 1: ssi_ssr_intr is not masked"]
    #[inline(always)]
    pub fn ssrim(&self) -> SsrimR {
        SsrimR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Empty Interrupt Mask. 0: ssi_txe_intr interrupt is masked 1: ssi_txe_intr interrupt is not masked"]
    #[inline(always)]
    pub fn txeim(&mut self) -> TxeimW<'_, ImrSpec> {
        TxeimW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Mask. 0: ssi_txo_intr interrupt is masked 1: ssi_txo_intr interrupt is not masked"]
    #[inline(always)]
    pub fn txoim(&mut self) -> TxoimW<'_, ImrSpec> {
        TxoimW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Interrupt Mask. 0: ssi_rxu_intr interrupt is masked 1: ssi_rxu_intr interrupt is not masked"]
    #[inline(always)]
    pub fn rxuim(&mut self) -> RxuimW<'_, ImrSpec> {
        RxuimW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Interrupt Mask. 0: ssi_rxo_intr interrupt is masked 1: ssi_rxo_intr interrupt is not masked"]
    #[inline(always)]
    pub fn rxoim(&mut self) -> RxoimW<'_, ImrSpec> {
        RxoimW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO Full Interrupt Mask. 0: ssi_rxf_intr interrupt is masked 1: ssi_rxf_intr interrupt is not masked"]
    #[inline(always)]
    pub fn rxfim(&mut self) -> RxfimW<'_, ImrSpec> {
        RxfimW::new(self, 4)
    }
    #[doc = "Bit 5 - When SPI is configured as serial-master, this bit field is present as Multi-Master Contention Interrupt Mask. 0: ssi_mst_intr interrupt is masked 1: ssi_mst_intr interrupt is not masked When SPI is configured as serial-slave, this bit field is present as Frame Alignment Interrupt Mask. 0: ssi_fae_intr interrupt is masked 1: ssi_fae_intr interrupt is not masked"]
    #[inline(always)]
    pub fn mstim_faeim(&mut self) -> MstimFaeimW<'_, ImrSpec> {
        MstimFaeimW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit FIFO Underflow Interrupt Mask. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_txu_intr is masked 1: ssi_txu_intr is not masked"]
    #[inline(always)]
    pub fn txuim(&mut self) -> TxuimW<'_, ImrSpec> {
        TxuimW::new(self, 6)
    }
    #[doc = "Bit 7 - SS_N Rising Edge Detect Interrupt Mask. This bit field is present only if the SPI is configured as a serial-slave device. 0: ssi_ssr_intr is masked 1: ssi_ssr_intr is not masked"]
    #[inline(always)]
    pub fn ssrim(&mut self) -> SsrimW<'_, ImrSpec> {
        SsrimW::new(self, 7)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR to value 0xff"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0xff;
}
