#[doc = "Register `SP_CTRL1` reader"]
pub type R = crate::R<SpCtrl1Spec>;
#[doc = "Register `SP_CTRL1` writer"]
pub type W = crate::W<SpCtrl1Spec>;
#[doc = "Field `sp_reset_smooth` reader - 1'b1: reset SPORT1 module with complete LRCK cycle."]
pub type SpResetSmoothR = crate::BitReader;
#[doc = "Field `sp_reset_smooth` writer - 1'b1: reset SPORT1 module with complete LRCK cycle."]
pub type SpResetSmoothW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_fill_zero` writer - X is the burst size of TX_FIFO_0. Y is the burst size of TX_FIFO_1. Fill TX_FIFO_0 with X zero data and fill TX_FIFO_1 with Y zero data. This control bit is \"write 1 clear\" type"]
pub type TxFifoFillZeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `multi_io_en_tx` reader - 1'b1: Enable multi-IO of tx 1'b0: Disable multi-IO of tx"]
pub type MultiIoEnTxR = crate::BitReader;
#[doc = "Field `multi_io_en_tx` writer - 1'b1: Enable multi-IO of tx 1'b0: Disable multi-IO of tx"]
pub type MultiIoEnTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `multi_io_en_rx` reader - 1'b1: Enable multi-IO of rx 1'b0: Disable multi-IO of rx"]
pub type MultiIoEnRxR = crate::BitReader;
#[doc = "Field `multi_io_en_rx` writer - 1'b1: Enable multi-IO of rx 1'b0: Disable multi-IO of rx"]
pub type MultiIoEnRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bclk_pull_zero` reader - Write 1'b1 to pull bclk to 0 smoothly. Write 1'b0 to reopen bclk."]
pub type BclkPullZeroR = crate::BitReader;
#[doc = "Field `bclk_pull_zero` writer - Write 1'b1 to pull bclk to 0 smoothly. Write 1'b0 to reopen bclk."]
pub type BclkPullZeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bclk_reset` reader - 1'b0: Enable bclk 1'b1: Disable and reset bclk"]
pub type BclkResetR = crate::BitReader;
#[doc = "Field `bclk_reset` writer - 1'b0: Enable bclk 1'b1: Disable and reset bclk"]
pub type BclkResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ws_force` reader - 1'b1: Make ws_out_tx and ws_out_rx = \"ws_force_val.\""]
pub type WsForceR = crate::BitReader;
#[doc = "Field `ws_force` writer - 1'b1: Make ws_out_tx and ws_out_rx = \"ws_force_val.\""]
pub type WsForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ws_force_val` reader - When \"ws_force\" = 1, ws_out_tx and ws_out_rx = \"ws_force_val.\""]
pub type WsForceValR = crate::BitReader;
#[doc = "Field `ws_force_val` writer - When \"ws_force\" = 1, ws_out_tx and ws_out_rx = \"ws_force_val.\""]
pub type WsForceValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `debug_bus_sel` reader - 3'b000: debug_bus_a 3'b001: debug_bus_b ... 3'b111: debug_bus_h"]
pub type DebugBusSelR = crate::FieldReader;
#[doc = "Field `debug_bus_sel` writer - 3'b000: debug_bus_a 3'b001: debug_bus_b ... 3'b111: debug_bus_h"]
pub type DebugBusSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `enable_mclk` reader - Enable mclk."]
pub type EnableMclkR = crate::BitReader;
#[doc = "Field `enable_mclk` writer - Enable mclk."]
pub type EnableMclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clear_tx_err_cnt` reader - Write 1'b1 and then write 0 to clear TX error counter"]
pub type ClearTxErrCntR = crate::BitReader;
#[doc = "Field `clear_tx_err_cnt` writer - Write 1'b1 and then write 0 to clear TX error counter"]
pub type ClearTxErrCntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clear_rx_err_cnt` reader - Write 1'b1 and then write 0 to clear RX error counter"]
pub type ClearRxErrCntR = crate::BitReader;
#[doc = "Field `clear_rx_err_cnt` writer - Write 1'b1 and then write 0 to clear RX error counter"]
pub type ClearRxErrCntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sport_clk_sel` reader - 2'b0x00: dsp_src_clk(BCLK*2) 2'b10: dsp_src_clk(BCLK*4)/2 2'b11: dsp_src_clk (BCLK*8)/4"]
pub type SportClkSelR = crate::FieldReader;
#[doc = "Field `sport_clk_sel` writer - 2'b0x00: dsp_src_clk(BCLK*2) 2'b10: dsp_src_clk(BCLK*4)/2 2'b11: dsp_src_clk (BCLK*8)/4"]
pub type SportClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `err_cnt_sat_set` reader - 1'b1: saturation count (65534 --> 65535 --> 65535 ...) 1'b0: wrap count (65534 --> 65535 --> 0 --> 1 --> 2 ...)"]
pub type ErrCntSatSetR = crate::BitReader;
#[doc = "Field `err_cnt_sat_set` writer - 1'b1: saturation count (65534 --> 65535 --> 65535 ...) 1'b0: wrap count (65534 --> 65535 --> 0 --> 1 --> 2 ...)"]
pub type ErrCntSatSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_direct_src_sel` reader - 2'b00: ws and sck are from sport0. 2'b01: ws and sck are from sport1. 2'b10: ws and sck are from sport2. 2'b11: ws and sck are from sport3."]
pub type SpDirectSrcSelR = crate::FieldReader;
#[doc = "Field `sp_direct_src_sel` writer - 2'b00: ws and sck are from sport0. 2'b01: ws and sck are from sport1. 2'b10: ws and sck are from sport2. 2'b11: ws and sck are from sport3."]
pub type SpDirectSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `direct_mode_en` reader - 1'b1: WS(LRCK) and SCK(BCLK) are from other sport"]
pub type DirectModeEnR = crate::BitReader;
#[doc = "Field `direct_mode_en` writer - 1'b1: WS(LRCK) and SCK(BCLK) are from other sport"]
pub type DirectModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_src_byte_swap_0` reader - 1'b1: swap H/L bytes read from the source memory of TX_FIFO_0;"]
pub type TxSrcByteSwap0R = crate::BitReader;
#[doc = "Field `tx_src_byte_swap_0` writer - 1'b1: swap H/L bytes read from the source memory of TX_FIFO_0;"]
pub type TxSrcByteSwap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_src_lr_swap_0` reader - 1'b1: swap L/R audio samples read from the source memory of TX_FIFO_0;"]
pub type TxSrcLrSwap0R = crate::BitReader;
#[doc = "Field `tx_src_lr_swap_0` writer - 1'b1: swap L/R audio samples read from the source memory of TX_FIFO_0;"]
pub type TxSrcLrSwap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_snk_byte_swap_0` reader - 1'b1: swap H/L bytes written to the sink memory of RX_FIFO_0;"]
pub type RxSnkByteSwap0R = crate::BitReader;
#[doc = "Field `rx_snk_byte_swap_0` writer - 1'b1: swap H/L bytes written to the sink memory of RX_FIFO_0;"]
pub type RxSnkByteSwap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_snk_lr_swap_0` reader - 1'b1: swap L/R audio samples written to the sink memory of RX_FIFO_0;"]
pub type RxSnkLrSwap0R = crate::BitReader;
#[doc = "Field `rx_snk_lr_swap_0` writer - 1'b1: swap L/R audio samples written to the sink memory of RX_FIFO_0;"]
pub type RxSnkLrSwap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_0_reg_0_en` reader - 1'b1: Enable first two channel of TX_FIFO_0. Disable 0x0008\\[24\\] ~ Disable 0x0008\\[27\\] at the same time to reset TX FIFO."]
pub type TxFifo0Reg0EnR = crate::BitReader;
#[doc = "Field `tx_fifo_0_reg_0_en` writer - 1'b1: Enable first two channel of TX_FIFO_0. Disable 0x0008\\[24\\] ~ Disable 0x0008\\[27\\] at the same time to reset TX FIFO."]
pub type TxFifo0Reg0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_0_reg_1_en` reader - 1'b1: Enable last two channel of TX_FIFO_0. Only enable when \"tx_fifo_0_reg_0_en\" = 1."]
pub type TxFifo0Reg1EnR = crate::BitReader;
#[doc = "Field `tx_fifo_0_reg_1_en` writer - 1'b1: Enable last two channel of TX_FIFO_0. Only enable when \"tx_fifo_0_reg_0_en\" = 1."]
pub type TxFifo0Reg1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_1_reg_0_en` reader - 1'b1: Enable first two channel of TX_FIFO_1. Only enable when \"tx_fifo_0_reg_0_en\" = 1."]
pub type TxFifo1Reg0EnR = crate::BitReader;
#[doc = "Field `tx_fifo_1_reg_0_en` writer - 1'b1: Enable first two channel of TX_FIFO_1. Only enable when \"tx_fifo_0_reg_0_en\" = 1."]
pub type TxFifo1Reg0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_fifo_1_reg_1_en` reader - 1'b1: Enable last two channel of TX_FIFO_1. Only enable when \"tx_fifo_1_reg_0_en\" = 1."]
pub type TxFifo1Reg1EnR = crate::BitReader;
#[doc = "Field `tx_fifo_1_reg_1_en` writer - 1'b1: Enable last two channel of TX_FIFO_1. Only enable when \"tx_fifo_1_reg_0_en\" = 1."]
pub type TxFifo1Reg1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_fifo_0_reg_0_en` reader - 1'b1: Enable first two channel of RX_FIFO_0. Disable 0x0008\\[28\\] ~ Disable 0x0008\\[31\\] at the same time to reset RX FIFO."]
pub type RxFifo0Reg0EnR = crate::BitReader;
#[doc = "Field `rx_fifo_0_reg_0_en` writer - 1'b1: Enable first two channel of RX_FIFO_0. Disable 0x0008\\[28\\] ~ Disable 0x0008\\[31\\] at the same time to reset RX FIFO."]
pub type RxFifo0Reg0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_fifo_0_reg_1_en` reader - 1'b1: Enable last two channel of RX_FIFO_0. Only enable when \"rx_fifo_0_reg_0_en\" = 1."]
pub type RxFifo0Reg1EnR = crate::BitReader;
#[doc = "Field `rx_fifo_0_reg_1_en` writer - 1'b1: Enable last two channel of RX_FIFO_0. Only enable when \"rx_fifo_0_reg_0_en\" = 1."]
pub type RxFifo0Reg1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_fifo_1_reg_0_en` reader - 1'b1: Enable first two channel of RX_FIFO_1. Only enable when \"rx_fifo_0_reg_0_en\" = 1."]
pub type RxFifo1Reg0EnR = crate::BitReader;
#[doc = "Field `rx_fifo_1_reg_0_en` writer - 1'b1: Enable first two channel of RX_FIFO_1. Only enable when \"rx_fifo_0_reg_0_en\" = 1."]
pub type RxFifo1Reg0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_fifo_1_reg_1_en` reader - 1'b1: Enable last two channel of RX_FIFO_1. Only enable when \"rx_fifo_1_reg_0_en\" = 1."]
pub type RxFifo1Reg1EnR = crate::BitReader;
#[doc = "Field `rx_fifo_1_reg_1_en` writer - 1'b1: Enable last two channel of RX_FIFO_1. Only enable when \"rx_fifo_1_reg_0_en\" = 1."]
pub type RxFifo1Reg1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1'b1: reset SPORT1 module with complete LRCK cycle."]
    #[inline(always)]
    pub fn sp_reset_smooth(&self) -> SpResetSmoothR {
        SpResetSmoothR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1'b1: Enable multi-IO of tx 1'b0: Disable multi-IO of tx"]
    #[inline(always)]
    pub fn multi_io_en_tx(&self) -> MultiIoEnTxR {
        MultiIoEnTxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1'b1: Enable multi-IO of rx 1'b0: Disable multi-IO of rx"]
    #[inline(always)]
    pub fn multi_io_en_rx(&self) -> MultiIoEnRxR {
        MultiIoEnRxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1'b1 to pull bclk to 0 smoothly. Write 1'b0 to reopen bclk."]
    #[inline(always)]
    pub fn bclk_pull_zero(&self) -> BclkPullZeroR {
        BclkPullZeroR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1'b0: Enable bclk 1'b1: Disable and reset bclk"]
    #[inline(always)]
    pub fn bclk_reset(&self) -> BclkResetR {
        BclkResetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1'b1: Make ws_out_tx and ws_out_rx = \"ws_force_val.\""]
    #[inline(always)]
    pub fn ws_force(&self) -> WsForceR {
        WsForceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When \"ws_force\" = 1, ws_out_tx and ws_out_rx = \"ws_force_val.\""]
    #[inline(always)]
    pub fn ws_force_val(&self) -> WsForceValR {
        WsForceValR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 3'b000: debug_bus_a 3'b001: debug_bus_b ... 3'b111: debug_bus_h"]
    #[inline(always)]
    pub fn debug_bus_sel(&self) -> DebugBusSelR {
        DebugBusSelR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Enable mclk."]
    #[inline(always)]
    pub fn enable_mclk(&self) -> EnableMclkR {
        EnableMclkR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write 1'b1 and then write 0 to clear TX error counter"]
    #[inline(always)]
    pub fn clear_tx_err_cnt(&self) -> ClearTxErrCntR {
        ClearTxErrCntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write 1'b1 and then write 0 to clear RX error counter"]
    #[inline(always)]
    pub fn clear_rx_err_cnt(&self) -> ClearRxErrCntR {
        ClearRxErrCntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 2'b0x00: dsp_src_clk(BCLK*2) 2'b10: dsp_src_clk(BCLK*4)/2 2'b11: dsp_src_clk (BCLK*8)/4"]
    #[inline(always)]
    pub fn sport_clk_sel(&self) -> SportClkSelR {
        SportClkSelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 1'b1: saturation count (65534 --> 65535 --> 65535 ...) 1'b0: wrap count (65534 --> 65535 --> 0 --> 1 --> 2 ...)"]
    #[inline(always)]
    pub fn err_cnt_sat_set(&self) -> ErrCntSatSetR {
        ErrCntSatSetR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - 2'b00: ws and sck are from sport0. 2'b01: ws and sck are from sport1. 2'b10: ws and sck are from sport2. 2'b11: ws and sck are from sport3."]
    #[inline(always)]
    pub fn sp_direct_src_sel(&self) -> SpDirectSrcSelR {
        SpDirectSrcSelR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1'b1: WS(LRCK) and SCK(BCLK) are from other sport"]
    #[inline(always)]
    pub fn direct_mode_en(&self) -> DirectModeEnR {
        DirectModeEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1'b1: swap H/L bytes read from the source memory of TX_FIFO_0;"]
    #[inline(always)]
    pub fn tx_src_byte_swap_0(&self) -> TxSrcByteSwap0R {
        TxSrcByteSwap0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1'b1: swap L/R audio samples read from the source memory of TX_FIFO_0;"]
    #[inline(always)]
    pub fn tx_src_lr_swap_0(&self) -> TxSrcLrSwap0R {
        TxSrcLrSwap0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1'b1: swap H/L bytes written to the sink memory of RX_FIFO_0;"]
    #[inline(always)]
    pub fn rx_snk_byte_swap_0(&self) -> RxSnkByteSwap0R {
        RxSnkByteSwap0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1'b1: swap L/R audio samples written to the sink memory of RX_FIFO_0;"]
    #[inline(always)]
    pub fn rx_snk_lr_swap_0(&self) -> RxSnkLrSwap0R {
        RxSnkLrSwap0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1'b1: Enable first two channel of TX_FIFO_0. Disable 0x0008\\[24\\] ~ Disable 0x0008\\[27\\] at the same time to reset TX FIFO."]
    #[inline(always)]
    pub fn tx_fifo_0_reg_0_en(&self) -> TxFifo0Reg0EnR {
        TxFifo0Reg0EnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1'b1: Enable last two channel of TX_FIFO_0. Only enable when \"tx_fifo_0_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn tx_fifo_0_reg_1_en(&self) -> TxFifo0Reg1EnR {
        TxFifo0Reg1EnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1'b1: Enable first two channel of TX_FIFO_1. Only enable when \"tx_fifo_0_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn tx_fifo_1_reg_0_en(&self) -> TxFifo1Reg0EnR {
        TxFifo1Reg0EnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1'b1: Enable last two channel of TX_FIFO_1. Only enable when \"tx_fifo_1_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn tx_fifo_1_reg_1_en(&self) -> TxFifo1Reg1EnR {
        TxFifo1Reg1EnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1'b1: Enable first two channel of RX_FIFO_0. Disable 0x0008\\[28\\] ~ Disable 0x0008\\[31\\] at the same time to reset RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_0_reg_0_en(&self) -> RxFifo0Reg0EnR {
        RxFifo0Reg0EnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1'b1: Enable last two channel of RX_FIFO_0. Only enable when \"rx_fifo_0_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn rx_fifo_0_reg_1_en(&self) -> RxFifo0Reg1EnR {
        RxFifo0Reg1EnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1'b1: Enable first two channel of RX_FIFO_1. Only enable when \"rx_fifo_0_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn rx_fifo_1_reg_0_en(&self) -> RxFifo1Reg0EnR {
        RxFifo1Reg0EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1'b1: Enable last two channel of RX_FIFO_1. Only enable when \"rx_fifo_1_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn rx_fifo_1_reg_1_en(&self) -> RxFifo1Reg1EnR {
        RxFifo1Reg1EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1: reset SPORT1 module with complete LRCK cycle."]
    #[inline(always)]
    pub fn sp_reset_smooth(&mut self) -> SpResetSmoothW<'_, SpCtrl1Spec> {
        SpResetSmoothW::new(self, 0)
    }
    #[doc = "Bit 1 - X is the burst size of TX_FIFO_0. Y is the burst size of TX_FIFO_1. Fill TX_FIFO_0 with X zero data and fill TX_FIFO_1 with Y zero data. This control bit is \"write 1 clear\" type"]
    #[inline(always)]
    pub fn tx_fifo_fill_zero(&mut self) -> TxFifoFillZeroW<'_, SpCtrl1Spec> {
        TxFifoFillZeroW::new(self, 1)
    }
    #[doc = "Bit 2 - 1'b1: Enable multi-IO of tx 1'b0: Disable multi-IO of tx"]
    #[inline(always)]
    pub fn multi_io_en_tx(&mut self) -> MultiIoEnTxW<'_, SpCtrl1Spec> {
        MultiIoEnTxW::new(self, 2)
    }
    #[doc = "Bit 3 - 1'b1: Enable multi-IO of rx 1'b0: Disable multi-IO of rx"]
    #[inline(always)]
    pub fn multi_io_en_rx(&mut self) -> MultiIoEnRxW<'_, SpCtrl1Spec> {
        MultiIoEnRxW::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1'b1 to pull bclk to 0 smoothly. Write 1'b0 to reopen bclk."]
    #[inline(always)]
    pub fn bclk_pull_zero(&mut self) -> BclkPullZeroW<'_, SpCtrl1Spec> {
        BclkPullZeroW::new(self, 4)
    }
    #[doc = "Bit 5 - 1'b0: Enable bclk 1'b1: Disable and reset bclk"]
    #[inline(always)]
    pub fn bclk_reset(&mut self) -> BclkResetW<'_, SpCtrl1Spec> {
        BclkResetW::new(self, 5)
    }
    #[doc = "Bit 6 - 1'b1: Make ws_out_tx and ws_out_rx = \"ws_force_val.\""]
    #[inline(always)]
    pub fn ws_force(&mut self) -> WsForceW<'_, SpCtrl1Spec> {
        WsForceW::new(self, 6)
    }
    #[doc = "Bit 7 - When \"ws_force\" = 1, ws_out_tx and ws_out_rx = \"ws_force_val.\""]
    #[inline(always)]
    pub fn ws_force_val(&mut self) -> WsForceValW<'_, SpCtrl1Spec> {
        WsForceValW::new(self, 7)
    }
    #[doc = "Bits 8:10 - 3'b000: debug_bus_a 3'b001: debug_bus_b ... 3'b111: debug_bus_h"]
    #[inline(always)]
    pub fn debug_bus_sel(&mut self) -> DebugBusSelW<'_, SpCtrl1Spec> {
        DebugBusSelW::new(self, 8)
    }
    #[doc = "Bit 11 - Enable mclk."]
    #[inline(always)]
    pub fn enable_mclk(&mut self) -> EnableMclkW<'_, SpCtrl1Spec> {
        EnableMclkW::new(self, 11)
    }
    #[doc = "Bit 12 - Write 1'b1 and then write 0 to clear TX error counter"]
    #[inline(always)]
    pub fn clear_tx_err_cnt(&mut self) -> ClearTxErrCntW<'_, SpCtrl1Spec> {
        ClearTxErrCntW::new(self, 12)
    }
    #[doc = "Bit 13 - Write 1'b1 and then write 0 to clear RX error counter"]
    #[inline(always)]
    pub fn clear_rx_err_cnt(&mut self) -> ClearRxErrCntW<'_, SpCtrl1Spec> {
        ClearRxErrCntW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 2'b0x00: dsp_src_clk(BCLK*2) 2'b10: dsp_src_clk(BCLK*4)/2 2'b11: dsp_src_clk (BCLK*8)/4"]
    #[inline(always)]
    pub fn sport_clk_sel(&mut self) -> SportClkSelW<'_, SpCtrl1Spec> {
        SportClkSelW::new(self, 14)
    }
    #[doc = "Bit 16 - 1'b1: saturation count (65534 --> 65535 --> 65535 ...) 1'b0: wrap count (65534 --> 65535 --> 0 --> 1 --> 2 ...)"]
    #[inline(always)]
    pub fn err_cnt_sat_set(&mut self) -> ErrCntSatSetW<'_, SpCtrl1Spec> {
        ErrCntSatSetW::new(self, 16)
    }
    #[doc = "Bits 17:18 - 2'b00: ws and sck are from sport0. 2'b01: ws and sck are from sport1. 2'b10: ws and sck are from sport2. 2'b11: ws and sck are from sport3."]
    #[inline(always)]
    pub fn sp_direct_src_sel(&mut self) -> SpDirectSrcSelW<'_, SpCtrl1Spec> {
        SpDirectSrcSelW::new(self, 17)
    }
    #[doc = "Bit 19 - 1'b1: WS(LRCK) and SCK(BCLK) are from other sport"]
    #[inline(always)]
    pub fn direct_mode_en(&mut self) -> DirectModeEnW<'_, SpCtrl1Spec> {
        DirectModeEnW::new(self, 19)
    }
    #[doc = "Bit 20 - 1'b1: swap H/L bytes read from the source memory of TX_FIFO_0;"]
    #[inline(always)]
    pub fn tx_src_byte_swap_0(&mut self) -> TxSrcByteSwap0W<'_, SpCtrl1Spec> {
        TxSrcByteSwap0W::new(self, 20)
    }
    #[doc = "Bit 21 - 1'b1: swap L/R audio samples read from the source memory of TX_FIFO_0;"]
    #[inline(always)]
    pub fn tx_src_lr_swap_0(&mut self) -> TxSrcLrSwap0W<'_, SpCtrl1Spec> {
        TxSrcLrSwap0W::new(self, 21)
    }
    #[doc = "Bit 22 - 1'b1: swap H/L bytes written to the sink memory of RX_FIFO_0;"]
    #[inline(always)]
    pub fn rx_snk_byte_swap_0(&mut self) -> RxSnkByteSwap0W<'_, SpCtrl1Spec> {
        RxSnkByteSwap0W::new(self, 22)
    }
    #[doc = "Bit 23 - 1'b1: swap L/R audio samples written to the sink memory of RX_FIFO_0;"]
    #[inline(always)]
    pub fn rx_snk_lr_swap_0(&mut self) -> RxSnkLrSwap0W<'_, SpCtrl1Spec> {
        RxSnkLrSwap0W::new(self, 23)
    }
    #[doc = "Bit 24 - 1'b1: Enable first two channel of TX_FIFO_0. Disable 0x0008\\[24\\] ~ Disable 0x0008\\[27\\] at the same time to reset TX FIFO."]
    #[inline(always)]
    pub fn tx_fifo_0_reg_0_en(&mut self) -> TxFifo0Reg0EnW<'_, SpCtrl1Spec> {
        TxFifo0Reg0EnW::new(self, 24)
    }
    #[doc = "Bit 25 - 1'b1: Enable last two channel of TX_FIFO_0. Only enable when \"tx_fifo_0_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn tx_fifo_0_reg_1_en(&mut self) -> TxFifo0Reg1EnW<'_, SpCtrl1Spec> {
        TxFifo0Reg1EnW::new(self, 25)
    }
    #[doc = "Bit 26 - 1'b1: Enable first two channel of TX_FIFO_1. Only enable when \"tx_fifo_0_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn tx_fifo_1_reg_0_en(&mut self) -> TxFifo1Reg0EnW<'_, SpCtrl1Spec> {
        TxFifo1Reg0EnW::new(self, 26)
    }
    #[doc = "Bit 27 - 1'b1: Enable last two channel of TX_FIFO_1. Only enable when \"tx_fifo_1_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn tx_fifo_1_reg_1_en(&mut self) -> TxFifo1Reg1EnW<'_, SpCtrl1Spec> {
        TxFifo1Reg1EnW::new(self, 27)
    }
    #[doc = "Bit 28 - 1'b1: Enable first two channel of RX_FIFO_0. Disable 0x0008\\[28\\] ~ Disable 0x0008\\[31\\] at the same time to reset RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_0_reg_0_en(&mut self) -> RxFifo0Reg0EnW<'_, SpCtrl1Spec> {
        RxFifo0Reg0EnW::new(self, 28)
    }
    #[doc = "Bit 29 - 1'b1: Enable last two channel of RX_FIFO_0. Only enable when \"rx_fifo_0_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn rx_fifo_0_reg_1_en(&mut self) -> RxFifo0Reg1EnW<'_, SpCtrl1Spec> {
        RxFifo0Reg1EnW::new(self, 29)
    }
    #[doc = "Bit 30 - 1'b1: Enable first two channel of RX_FIFO_1. Only enable when \"rx_fifo_0_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn rx_fifo_1_reg_0_en(&mut self) -> RxFifo1Reg0EnW<'_, SpCtrl1Spec> {
        RxFifo1Reg0EnW::new(self, 30)
    }
    #[doc = "Bit 31 - 1'b1: Enable last two channel of RX_FIFO_1. Only enable when \"rx_fifo_1_reg_0_en\" = 1."]
    #[inline(always)]
    pub fn rx_fifo_1_reg_1_en(&mut self) -> RxFifo1Reg1EnW<'_, SpCtrl1Spec> {
        RxFifo1Reg1EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpCtrl1Spec;
impl crate::RegisterSpec for SpCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_ctrl1::R`](R) reader structure"]
impl crate::Readable for SpCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sp_ctrl1::W`](W) writer structure"]
impl crate::Writable for SpCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_CTRL1 to value 0x1100_008c"]
impl crate::Resettable for SpCtrl1Spec {
    const RESET_VALUE: u32 = 0x1100_008c;
}
