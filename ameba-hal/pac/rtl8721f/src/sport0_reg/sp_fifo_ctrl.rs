#[doc = "Register `SP_FIFO_CTRL` reader"]
pub type R = crate::R<SpFifoCtrlSpec>;
#[doc = "Field `sp_ready_to_tx_0` reader - 1: It is ready to send data out (SPK path)"]
pub type SpReadyToTx0R = crate::BitReader;
#[doc = "Field `sp_ready_to_rx_0` reader - 1: It is ready to receive data (MIC path)"]
pub type SpReadyToRx0R = crate::BitReader;
#[doc = "Field `tx_fifo_full_intr_0` reader - 1: TX FIFO_0 is full (SPK path)"]
pub type TxFifoFullIntr0R = crate::BitReader;
#[doc = "Field `rx_fifo_full_intr_0` reader - 1: RX FIFO_0 is full (MIC path)"]
pub type RxFifoFullIntr0R = crate::BitReader;
#[doc = "Field `tx_fifo_empty_intr_0` reader - 1: TX FIFO_0 is empty (SPK path)"]
pub type TxFifoEmptyIntr0R = crate::BitReader;
#[doc = "Field `rx_fifo_empty_intr_0` reader - 1: RX FIFO_0 is empty (MIC path)"]
pub type RxFifoEmptyIntr0R = crate::BitReader;
#[doc = "Field `tx_i2s_idle_0` reader - 1: TX is working but FIFO_0 is empty."]
pub type TxI2sIdle0R = crate::BitReader;
#[doc = "Field `sp_ready_to_tx_1` reader - 1: It is ready to send data out (SPK path)"]
pub type SpReadyToTx1R = crate::BitReader;
#[doc = "Field `sp_ready_to_rx_1` reader - 1: It is ready to receive data (MIC path)"]
pub type SpReadyToRx1R = crate::BitReader;
#[doc = "Field `tx_fifo_full_intr_1` reader - 1: TX FIFO_1 is full (SPK path)"]
pub type TxFifoFullIntr1R = crate::BitReader;
#[doc = "Field `rx_fifo_full_intr_1` reader - 1: RX FIFO_1 is full (MIC path)"]
pub type RxFifoFullIntr1R = crate::BitReader;
#[doc = "Field `tx_fifo_empty_intr_1` reader - 1: TX FIFO_1 is empty (SPK path)"]
pub type TxFifoEmptyIntr1R = crate::BitReader;
#[doc = "Field `rx_fifo_empty_intr_1` reader - 1: RX FIFO_1 is empty (MIC path)"]
pub type RxFifoEmptyIntr1R = crate::BitReader;
#[doc = "Field `tx_i2s_idle_1` reader - 1: TX is working but FIFO_1 is empty."]
pub type TxI2sIdle1R = crate::BitReader;
#[doc = "Field `tx_fifo_full_1` reader - 1: TX FIFO_1 is full"]
pub type TxFifoFull1R = crate::BitReader;
#[doc = "Field `rx_fifo_full_1` reader - 1: RX FIFO_1 is full"]
pub type RxFifoFull1R = crate::BitReader;
#[doc = "Field `tx_fifo_empty_1` reader - 1: TX FIFO_1 is empty"]
pub type TxFifoEmpty1R = crate::BitReader;
#[doc = "Field `rx_fifo_empty_1` reader - 1: RX FIFO_1 is empty"]
pub type RxFifoEmpty1R = crate::BitReader;
#[doc = "Field `tx_fifo_full_0` reader - 1: TX FIFO_0 is full"]
pub type TxFifoFull0R = crate::BitReader;
#[doc = "Field `rx_fifo_full_0` reader - 1: RX FIFO_0 is full"]
pub type RxFifoFull0R = crate::BitReader;
#[doc = "Field `tx_fifo_empty_0` reader - 1: TX FIFO_0 is empty"]
pub type TxFifoEmpty0R = crate::BitReader;
#[doc = "Field `rx_fifo_empty_0` reader - 1: RX FIFO_0 is empty"]
pub type RxFifoEmpty0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: It is ready to send data out (SPK path)"]
    #[inline(always)]
    pub fn sp_ready_to_tx_0(&self) -> SpReadyToTx0R {
        SpReadyToTx0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: It is ready to receive data (MIC path)"]
    #[inline(always)]
    pub fn sp_ready_to_rx_0(&self) -> SpReadyToRx0R {
        SpReadyToRx0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: TX FIFO_0 is full (SPK path)"]
    #[inline(always)]
    pub fn tx_fifo_full_intr_0(&self) -> TxFifoFullIntr0R {
        TxFifoFullIntr0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: RX FIFO_0 is full (MIC path)"]
    #[inline(always)]
    pub fn rx_fifo_full_intr_0(&self) -> RxFifoFullIntr0R {
        RxFifoFullIntr0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: TX FIFO_0 is empty (SPK path)"]
    #[inline(always)]
    pub fn tx_fifo_empty_intr_0(&self) -> TxFifoEmptyIntr0R {
        TxFifoEmptyIntr0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: RX FIFO_0 is empty (MIC path)"]
    #[inline(always)]
    pub fn rx_fifo_empty_intr_0(&self) -> RxFifoEmptyIntr0R {
        RxFifoEmptyIntr0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: TX is working but FIFO_0 is empty."]
    #[inline(always)]
    pub fn tx_i2s_idle_0(&self) -> TxI2sIdle0R {
        TxI2sIdle0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: It is ready to send data out (SPK path)"]
    #[inline(always)]
    pub fn sp_ready_to_tx_1(&self) -> SpReadyToTx1R {
        SpReadyToTx1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: It is ready to receive data (MIC path)"]
    #[inline(always)]
    pub fn sp_ready_to_rx_1(&self) -> SpReadyToRx1R {
        SpReadyToRx1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: TX FIFO_1 is full (SPK path)"]
    #[inline(always)]
    pub fn tx_fifo_full_intr_1(&self) -> TxFifoFullIntr1R {
        TxFifoFullIntr1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: RX FIFO_1 is full (MIC path)"]
    #[inline(always)]
    pub fn rx_fifo_full_intr_1(&self) -> RxFifoFullIntr1R {
        RxFifoFullIntr1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: TX FIFO_1 is empty (SPK path)"]
    #[inline(always)]
    pub fn tx_fifo_empty_intr_1(&self) -> TxFifoEmptyIntr1R {
        TxFifoEmptyIntr1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: RX FIFO_1 is empty (MIC path)"]
    #[inline(always)]
    pub fn rx_fifo_empty_intr_1(&self) -> RxFifoEmptyIntr1R {
        RxFifoEmptyIntr1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: TX is working but FIFO_1 is empty."]
    #[inline(always)]
    pub fn tx_i2s_idle_1(&self) -> TxI2sIdle1R {
        TxI2sIdle1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: TX FIFO_1 is full"]
    #[inline(always)]
    pub fn tx_fifo_full_1(&self) -> TxFifoFull1R {
        TxFifoFull1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: RX FIFO_1 is full"]
    #[inline(always)]
    pub fn rx_fifo_full_1(&self) -> RxFifoFull1R {
        RxFifoFull1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: TX FIFO_1 is empty"]
    #[inline(always)]
    pub fn tx_fifo_empty_1(&self) -> TxFifoEmpty1R {
        TxFifoEmpty1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: RX FIFO_1 is empty"]
    #[inline(always)]
    pub fn rx_fifo_empty_1(&self) -> RxFifoEmpty1R {
        RxFifoEmpty1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: TX FIFO_0 is full"]
    #[inline(always)]
    pub fn tx_fifo_full_0(&self) -> TxFifoFull0R {
        TxFifoFull0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: RX FIFO_0 is full"]
    #[inline(always)]
    pub fn rx_fifo_full_0(&self) -> RxFifoFull0R {
        RxFifoFull0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: TX FIFO_0 is empty"]
    #[inline(always)]
    pub fn tx_fifo_empty_0(&self) -> TxFifoEmpty0R {
        TxFifoEmpty0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: RX FIFO_0 is empty"]
    #[inline(always)]
    pub fn rx_fifo_empty_0(&self) -> RxFifoEmpty0R {
        RxFifoEmpty0R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_fifo_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpFifoCtrlSpec;
impl crate::RegisterSpec for SpFifoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for SpFifoCtrlSpec {}
#[doc = "`reset()` method sets SP_FIFO_CTRL to value 0xcc00_0000"]
impl crate::Resettable for SpFifoCtrlSpec {
    const RESET_VALUE: u32 = 0xcc00_0000;
}
