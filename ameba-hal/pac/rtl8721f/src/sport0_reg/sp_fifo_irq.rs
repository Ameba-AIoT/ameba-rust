#[doc = "Register `SP_FIFO_IRQ` reader"]
pub type R = crate::R<SpFifoIrqSpec>;
#[doc = "Register `SP_FIFO_IRQ` writer"]
pub type W = crate::W<SpFifoIrqSpec>;
#[doc = "Field `int_enable_mcu_0` reader - Bit\\[0\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[1\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[2\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[3\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[4\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[5\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[6\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[7\\]: Reserved"]
pub type IntEnableMcu0R = crate::FieldReader;
#[doc = "Field `int_enable_mcu_0` writer - Bit\\[0\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[1\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[2\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[3\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[4\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[5\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[6\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[7\\]: Reserved"]
pub type IntEnableMcu0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `int_enable_mcu_1` reader - Bit\\[8\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[9\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[10\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[11\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[12\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[13\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[14\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[15\\]: Reserved"]
pub type IntEnableMcu1R = crate::FieldReader;
#[doc = "Field `int_enable_mcu_1` writer - Bit\\[8\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[9\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[10\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[11\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[12\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[13\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[14\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[15\\]: Reserved"]
pub type IntEnableMcu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DUMMY7` reader - -"]
pub type Dummy7R = crate::FieldReader<u16>;
#[doc = "Field `DUMMY7` writer - -"]
pub type Dummy7W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tx_src_byte_swap_1` reader - 1'b1: swap H/L bytes read from the source memory of TX_FIFO_1"]
pub type TxSrcByteSwap1R = crate::BitReader;
#[doc = "Field `tx_src_byte_swap_1` writer - 1'b1: swap H/L bytes read from the source memory of TX_FIFO_1"]
pub type TxSrcByteSwap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_src_lr_swap_1` reader - 1'b1: swap L/R audio samples read from the source memory of TX_FIFO_1"]
pub type TxSrcLrSwap1R = crate::BitReader;
#[doc = "Field `tx_src_lr_swap_1` writer - 1'b1: swap L/R audio samples read from the source memory of TX_FIFO_1"]
pub type TxSrcLrSwap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_snk_byte_swap_1` reader - 1'b1: swap H/L bytes written to the sink memory of RX_FIFO_1"]
pub type RxSnkByteSwap1R = crate::BitReader;
#[doc = "Field `rx_snk_byte_swap_1` writer - 1'b1: swap H/L bytes written to the sink memory of RX_FIFO_1"]
pub type RxSnkByteSwap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_snk_lr_swap_1` reader - 1'b1: swap L/R audio samples written to the sink memory of RX_FIFO_1"]
pub type RxSnkLrSwap1R = crate::BitReader;
#[doc = "Field `rx_snk_lr_swap_1` writer - 1'b1: swap L/R audio samples written to the sink memory of RX_FIFO_1"]
pub type RxSnkLrSwap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_lsb_first_1` reader - 1'b0: MSB first when TX 1'b1: LSB first"]
pub type TxLsbFirst1R = crate::BitReader;
#[doc = "Field `tx_lsb_first_1` writer - 1'b0: MSB first when TX 1'b1: LSB first"]
pub type TxLsbFirst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_lsb_first_1` reader - 1'b0: MSB first when TX 1'b1: LSB first"]
pub type RxLsbFirst1R = crate::BitReader;
#[doc = "Field `rx_lsb_first_1` writer - 1'b0: MSB first when TX 1'b1: LSB first"]
pub type RxLsbFirst1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bit\\[0\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[1\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[2\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[3\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[4\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[5\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[6\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[7\\]: Reserved"]
    #[inline(always)]
    pub fn int_enable_mcu_0(&self) -> IntEnableMcu0R {
        IntEnableMcu0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Bit\\[8\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[9\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[10\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[11\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[12\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[13\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[14\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[15\\]: Reserved"]
    #[inline(always)]
    pub fn int_enable_mcu_1(&self) -> IntEnableMcu1R {
        IntEnableMcu1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - -"]
    #[inline(always)]
    pub fn dummy7(&self) -> Dummy7R {
        Dummy7R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - 1'b1: swap H/L bytes read from the source memory of TX_FIFO_1"]
    #[inline(always)]
    pub fn tx_src_byte_swap_1(&self) -> TxSrcByteSwap1R {
        TxSrcByteSwap1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1'b1: swap L/R audio samples read from the source memory of TX_FIFO_1"]
    #[inline(always)]
    pub fn tx_src_lr_swap_1(&self) -> TxSrcLrSwap1R {
        TxSrcLrSwap1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1'b1: swap H/L bytes written to the sink memory of RX_FIFO_1"]
    #[inline(always)]
    pub fn rx_snk_byte_swap_1(&self) -> RxSnkByteSwap1R {
        RxSnkByteSwap1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1'b1: swap L/R audio samples written to the sink memory of RX_FIFO_1"]
    #[inline(always)]
    pub fn rx_snk_lr_swap_1(&self) -> RxSnkLrSwap1R {
        RxSnkLrSwap1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1'b0: MSB first when TX 1'b1: LSB first"]
    #[inline(always)]
    pub fn tx_lsb_first_1(&self) -> TxLsbFirst1R {
        TxLsbFirst1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1'b0: MSB first when TX 1'b1: LSB first"]
    #[inline(always)]
    pub fn rx_lsb_first_1(&self) -> RxLsbFirst1R {
        RxLsbFirst1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit\\[0\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[1\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[2\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[3\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[4\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[5\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[6\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[7\\]: Reserved"]
    #[inline(always)]
    pub fn int_enable_mcu_0(&mut self) -> IntEnableMcu0W<'_, SpFifoIrqSpec> {
        IntEnableMcu0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Bit\\[8\\]: for the interrupt of \"sp_ready_to_tx\" Bit\\[9\\]: for the interrupt of \"sp_ready_to_rx\" Bit\\[10\\]: for the interrupt of \"tx_fifo_full_intr\" Bit\\[11\\]: for the interrupt of \"rx_fifo_full_intr\" Bit\\[12\\]: for the interrupt of \"tx_fifo_empty_intr\" Bit\\[13\\]: for the interrupt of \"rx_fifo_empty_intr\" Bit\\[14\\]: for the interrupt of \"tx_i2s_idle\" Bit\\[15\\]: Reserved"]
    #[inline(always)]
    pub fn int_enable_mcu_1(&mut self) -> IntEnableMcu1W<'_, SpFifoIrqSpec> {
        IntEnableMcu1W::new(self, 8)
    }
    #[doc = "Bits 16:25 - -"]
    #[inline(always)]
    pub fn dummy7(&mut self) -> Dummy7W<'_, SpFifoIrqSpec> {
        Dummy7W::new(self, 16)
    }
    #[doc = "Bit 26 - 1'b1: swap H/L bytes read from the source memory of TX_FIFO_1"]
    #[inline(always)]
    pub fn tx_src_byte_swap_1(&mut self) -> TxSrcByteSwap1W<'_, SpFifoIrqSpec> {
        TxSrcByteSwap1W::new(self, 26)
    }
    #[doc = "Bit 27 - 1'b1: swap L/R audio samples read from the source memory of TX_FIFO_1"]
    #[inline(always)]
    pub fn tx_src_lr_swap_1(&mut self) -> TxSrcLrSwap1W<'_, SpFifoIrqSpec> {
        TxSrcLrSwap1W::new(self, 27)
    }
    #[doc = "Bit 28 - 1'b1: swap H/L bytes written to the sink memory of RX_FIFO_1"]
    #[inline(always)]
    pub fn rx_snk_byte_swap_1(&mut self) -> RxSnkByteSwap1W<'_, SpFifoIrqSpec> {
        RxSnkByteSwap1W::new(self, 28)
    }
    #[doc = "Bit 29 - 1'b1: swap L/R audio samples written to the sink memory of RX_FIFO_1"]
    #[inline(always)]
    pub fn rx_snk_lr_swap_1(&mut self) -> RxSnkLrSwap1W<'_, SpFifoIrqSpec> {
        RxSnkLrSwap1W::new(self, 29)
    }
    #[doc = "Bit 30 - 1'b0: MSB first when TX 1'b1: LSB first"]
    #[inline(always)]
    pub fn tx_lsb_first_1(&mut self) -> TxLsbFirst1W<'_, SpFifoIrqSpec> {
        TxLsbFirst1W::new(self, 30)
    }
    #[doc = "Bit 31 - 1'b0: MSB first when TX 1'b1: LSB first"]
    #[inline(always)]
    pub fn rx_lsb_first_1(&mut self) -> RxLsbFirst1W<'_, SpFifoIrqSpec> {
        RxLsbFirst1W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_fifo_irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_fifo_irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpFifoIrqSpec;
impl crate::RegisterSpec for SpFifoIrqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_fifo_irq::R`](R) reader structure"]
impl crate::Readable for SpFifoIrqSpec {}
#[doc = "`write(|w| ..)` method takes [`sp_fifo_irq::W`](W) writer structure"]
impl crate::Writable for SpFifoIrqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_FIFO_IRQ to value 0"]
impl crate::Resettable for SpFifoIrqSpec {}
