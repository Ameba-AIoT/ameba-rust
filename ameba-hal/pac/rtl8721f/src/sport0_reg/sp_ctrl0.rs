#[doc = "Register `SP_CTRL0` reader"]
pub type R = crate::R<SpCtrl0Spec>;
#[doc = "Register `SP_CTRL0` writer"]
pub type W = crate::W<SpCtrl0Spec>;
#[doc = "Field `sp_reset` reader - 1'b1: reset SPORT1 module, and remember to write \"1\" to reset and then write \"0\" to release from reset."]
pub type SpResetR = crate::BitReader;
#[doc = "Field `sp_reset` writer - 1'b1: reset SPORT1 module, and remember to write \"1\" to reset and then write \"0\" to release from reset."]
pub type SpResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_inv_i2s_sclk` reader - 1'b1: sclk to TX path (DAC path) is inverted"]
pub type TxInvI2sSclkR = crate::BitReader;
#[doc = "Field `tx_inv_i2s_sclk` writer - 1'b1: sclk to TX path (DAC path) is inverted"]
pub type TxInvI2sSclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_inv_i2s_sclk` reader - 1'b1: sclk to RX path (ADC path) is inverted"]
pub type RxInvI2sSclkR = crate::BitReader;
#[doc = "Field `rx_inv_i2s_sclk` writer - 1'b1: sclk to RX path (ADC path) is inverted"]
pub type RxInvI2sSclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slave_clk_sel` reader - 1'b1: To be an I2S or PCM slave (CLK path)"]
pub type SlaveClkSelR = crate::BitReader;
#[doc = "Field `slave_clk_sel` writer - 1'b1: To be an I2S or PCM slave (CLK path)"]
pub type SlaveClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slave_data_sel` reader - 1'b1: To be an I2S or PCM slave (data path)"]
pub type SlaveDataSelR = crate::BitReader;
#[doc = "Field `slave_data_sel` writer - 1'b1: To be an I2S or PCM slave (data path)"]
pub type SlaveDataSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_wclk_tx_inverse` reader - 1'b1: I2S/PCM word clock is inverted for TX (SPK path)"]
pub type SpWclkTxInverseR = crate::BitReader;
#[doc = "Field `sp_wclk_tx_inverse` writer - 1'b1: I2S/PCM word clock is inverted for TX (SPK path)"]
pub type SpWclkTxInverseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_loopback` reader - 1'b1: self loopback mode"]
pub type SpLoopbackR = crate::BitReader;
#[doc = "Field `sp_loopback` writer - 1'b1: self loopback mode"]
pub type SpLoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dsp_ctl_mode` reader - 1'b1: DSP and SPORT1 handshaking is enabled. 1'b0: GDMA and SPORT1 handshaking is enabled."]
pub type DspCtlModeR = crate::BitReader;
#[doc = "Field `dsp_ctl_mode` writer - 1'b1: DSP and SPORT1 handshaking is enabled. 1'b0: GDMA and SPORT1 handshaking is enabled."]
pub type DspCtlModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_data_format_sel_tx` reader - 2'b00: I2S 2'b01: Left Justified 2'b10: PCM mode A 2'b11: PCM mode B"]
pub type SpDataFormatSelTxR = crate::FieldReader;
#[doc = "Field `sp_data_format_sel_tx` writer - 2'b00: I2S 2'b01: Left Justified 2'b10: PCM mode A 2'b11: PCM mode B"]
pub type SpDataFormatSelTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sp_inv_i2s_sclk` reader - 1'b1: I2S/PCM bit clock is inverted"]
pub type SpInvI2sSclkR = crate::BitReader;
#[doc = "Field `sp_inv_i2s_sclk` writer - 1'b1: I2S/PCM bit clock is inverted"]
pub type SpInvI2sSclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_en_i2s_mono_tx_0` reader - 1'b1: mono 1'b0: stereo"]
pub type SpEnI2sMonoTx0R = crate::BitReader;
#[doc = "Field `sp_en_i2s_mono_tx_0` writer - 1'b1: mono 1'b0: stereo"]
pub type SpEnI2sMonoTx0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_data_len_sel_tx_0` reader - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b100: 32 bits"]
pub type SpDataLenSelTx0R = crate::FieldReader;
#[doc = "Field `sp_data_len_sel_tx_0` writer - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b100: 32 bits"]
pub type SpDataLenSelTx0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sp_i2s_self_lpbk_en` reader - 1'b1: internal loopback mode is enabled"]
pub type SpI2sSelfLpbkEnR = crate::BitReader;
#[doc = "Field `sp_i2s_self_lpbk_en` writer - 1'b1: internal loopback mode is enabled"]
pub type SpI2sSelfLpbkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_tx_disable` reader - 1'b1: SPORT TX is disabled. 1'b0: SPORT TX is enabled."]
pub type SpTxDisableR = crate::BitReader;
#[doc = "Field `sp_tx_disable` writer - 1'b1: SPORT TX is disabled. 1'b0: SPORT TX is enabled."]
pub type SpTxDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_start_tx` reader - 1'b0: TX is disabled. 1'b1: TX is started."]
pub type SpStartTxR = crate::BitReader;
#[doc = "Field `sp_start_tx` writer - 1'b0: TX is disabled. 1'b1: TX is started."]
pub type SpStartTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_tdm_mode_sel_tx` reader - 2'b00: Without TDM 2'b01: TDM4 2'b10: TDM6 2'b11: TDM8"]
pub type SpTdmModeSelTxR = crate::FieldReader;
#[doc = "Field `sp_tdm_mode_sel_tx` writer - 2'b00: Without TDM 2'b01: TDM4 2'b10: TDM6 2'b11: TDM8"]
pub type SpTdmModeSelTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sp_tdm_mode_sel_rx` reader - 2'b00: Without TDM 2'b01: TDM4 2'b10: TDM6 2'b11: TDM8"]
pub type SpTdmModeSelRxR = crate::FieldReader;
#[doc = "Field `sp_tdm_mode_sel_rx` writer - 2'b00: Without TDM 2'b01: TDM4 2'b10: TDM6 2'b11: TDM8"]
pub type SpTdmModeSelRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tx_lsb_first_0` reader - 1'b0: MSB first when TX 1'b1: LSB first"]
pub type TxLsbFirst0R = crate::BitReader;
#[doc = "Field `tx_lsb_first_0` writer - 1'b0: MSB first when TX 1'b1: LSB first"]
pub type TxLsbFirst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_lsb_first_0` reader - 1'b0: MSB first when RX 1'b1: LSB first"]
pub type RxLsbFirst0R = crate::BitReader;
#[doc = "Field `rx_lsb_first_0` writer - 1'b0: MSB first when RX 1'b1: LSB first"]
pub type RxLsbFirst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_rx_disable` reader - 1'b1: SPORT RX is disabled. 1'b0: SPORT RX is enabled."]
pub type SpRxDisableR = crate::BitReader;
#[doc = "Field `sp_rx_disable` writer - 1'b1: SPORT RX is disabled. 1'b0: SPORT RX is enabled."]
pub type SpRxDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_start_rx` reader - 1'b0: RX is disabled 1'b1: RX is started"]
pub type SpStartRxR = crate::BitReader;
#[doc = "Field `sp_start_rx` writer - 1'b0: RX is disabled 1'b1: RX is started"]
pub type SpStartRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_sel_i2s_tx_ch` reader - 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R @ DAC path"]
pub type SpSelI2sTxChR = crate::FieldReader;
#[doc = "Field `sp_sel_i2s_tx_ch` writer - 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R @ DAC path"]
pub type SpSelI2sTxChW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sp_sel_i2s_rx_ch` reader - 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R @ ADC path"]
pub type SpSelI2sRxChR = crate::FieldReader;
#[doc = "Field `sp_sel_i2s_rx_ch` writer - 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R @ ADC path"]
pub type SpSelI2sRxChW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mclk_sel` reader - not used, please refer to 0x64 2'b00: MCLK output=dsp_src_clk/4 2'b01: MCLK output=dsp_src_clk/2 2'b10/2'b11: MCLK output=dsp_src_clk"]
pub type MclkSelR = crate::FieldReader;
#[doc = "Field `mclk_sel` writer - not used, please refer to 0x64 2'b00: MCLK output=dsp_src_clk/4 2'b01: MCLK output=dsp_src_clk/2 2'b10/2'b11: MCLK output=dsp_src_clk"]
pub type MclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 1'b1: reset SPORT1 module, and remember to write \"1\" to reset and then write \"0\" to release from reset."]
    #[inline(always)]
    pub fn sp_reset(&self) -> SpResetR {
        SpResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1'b1: sclk to TX path (DAC path) is inverted"]
    #[inline(always)]
    pub fn tx_inv_i2s_sclk(&self) -> TxInvI2sSclkR {
        TxInvI2sSclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1'b1: sclk to RX path (ADC path) is inverted"]
    #[inline(always)]
    pub fn rx_inv_i2s_sclk(&self) -> RxInvI2sSclkR {
        RxInvI2sSclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1'b1: To be an I2S or PCM slave (CLK path)"]
    #[inline(always)]
    pub fn slave_clk_sel(&self) -> SlaveClkSelR {
        SlaveClkSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1'b1: To be an I2S or PCM slave (data path)"]
    #[inline(always)]
    pub fn slave_data_sel(&self) -> SlaveDataSelR {
        SlaveDataSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1'b1: I2S/PCM word clock is inverted for TX (SPK path)"]
    #[inline(always)]
    pub fn sp_wclk_tx_inverse(&self) -> SpWclkTxInverseR {
        SpWclkTxInverseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1'b1: self loopback mode"]
    #[inline(always)]
    pub fn sp_loopback(&self) -> SpLoopbackR {
        SpLoopbackR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1'b1: DSP and SPORT1 handshaking is enabled. 1'b0: GDMA and SPORT1 handshaking is enabled."]
    #[inline(always)]
    pub fn dsp_ctl_mode(&self) -> DspCtlModeR {
        DspCtlModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 2'b00: I2S 2'b01: Left Justified 2'b10: PCM mode A 2'b11: PCM mode B"]
    #[inline(always)]
    pub fn sp_data_format_sel_tx(&self) -> SpDataFormatSelTxR {
        SpDataFormatSelTxR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 1'b1: I2S/PCM bit clock is inverted"]
    #[inline(always)]
    pub fn sp_inv_i2s_sclk(&self) -> SpInvI2sSclkR {
        SpInvI2sSclkR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1'b1: mono 1'b0: stereo"]
    #[inline(always)]
    pub fn sp_en_i2s_mono_tx_0(&self) -> SpEnI2sMonoTx0R {
        SpEnI2sMonoTx0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_data_len_sel_tx_0(&self) -> SpDataLenSelTx0R {
        SpDataLenSelTx0R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 1'b1: internal loopback mode is enabled"]
    #[inline(always)]
    pub fn sp_i2s_self_lpbk_en(&self) -> SpI2sSelfLpbkEnR {
        SpI2sSelfLpbkEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1'b1: SPORT TX is disabled. 1'b0: SPORT TX is enabled."]
    #[inline(always)]
    pub fn sp_tx_disable(&self) -> SpTxDisableR {
        SpTxDisableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1'b0: TX is disabled. 1'b1: TX is started."]
    #[inline(always)]
    pub fn sp_start_tx(&self) -> SpStartTxR {
        SpStartTxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - 2'b00: Without TDM 2'b01: TDM4 2'b10: TDM6 2'b11: TDM8"]
    #[inline(always)]
    pub fn sp_tdm_mode_sel_tx(&self) -> SpTdmModeSelTxR {
        SpTdmModeSelTxR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 2'b00: Without TDM 2'b01: TDM4 2'b10: TDM6 2'b11: TDM8"]
    #[inline(always)]
    pub fn sp_tdm_mode_sel_rx(&self) -> SpTdmModeSelRxR {
        SpTdmModeSelRxR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - 1'b0: MSB first when TX 1'b1: LSB first"]
    #[inline(always)]
    pub fn tx_lsb_first_0(&self) -> TxLsbFirst0R {
        TxLsbFirst0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1'b0: MSB first when RX 1'b1: LSB first"]
    #[inline(always)]
    pub fn rx_lsb_first_0(&self) -> RxLsbFirst0R {
        RxLsbFirst0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1'b1: SPORT RX is disabled. 1'b0: SPORT RX is enabled."]
    #[inline(always)]
    pub fn sp_rx_disable(&self) -> SpRxDisableR {
        SpRxDisableR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1'b0: RX is disabled 1'b1: RX is started"]
    #[inline(always)]
    pub fn sp_start_rx(&self) -> SpStartRxR {
        SpStartRxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R @ DAC path"]
    #[inline(always)]
    pub fn sp_sel_i2s_tx_ch(&self) -> SpSelI2sTxChR {
        SpSelI2sTxChR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R @ ADC path"]
    #[inline(always)]
    pub fn sp_sel_i2s_rx_ch(&self) -> SpSelI2sRxChR {
        SpSelI2sRxChR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - not used, please refer to 0x64 2'b00: MCLK output=dsp_src_clk/4 2'b01: MCLK output=dsp_src_clk/2 2'b10/2'b11: MCLK output=dsp_src_clk"]
    #[inline(always)]
    pub fn mclk_sel(&self) -> MclkSelR {
        MclkSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1: reset SPORT1 module, and remember to write \"1\" to reset and then write \"0\" to release from reset."]
    #[inline(always)]
    pub fn sp_reset(&mut self) -> SpResetW<'_, SpCtrl0Spec> {
        SpResetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1'b1: sclk to TX path (DAC path) is inverted"]
    #[inline(always)]
    pub fn tx_inv_i2s_sclk(&mut self) -> TxInvI2sSclkW<'_, SpCtrl0Spec> {
        TxInvI2sSclkW::new(self, 1)
    }
    #[doc = "Bit 2 - 1'b1: sclk to RX path (ADC path) is inverted"]
    #[inline(always)]
    pub fn rx_inv_i2s_sclk(&mut self) -> RxInvI2sSclkW<'_, SpCtrl0Spec> {
        RxInvI2sSclkW::new(self, 2)
    }
    #[doc = "Bit 3 - 1'b1: To be an I2S or PCM slave (CLK path)"]
    #[inline(always)]
    pub fn slave_clk_sel(&mut self) -> SlaveClkSelW<'_, SpCtrl0Spec> {
        SlaveClkSelW::new(self, 3)
    }
    #[doc = "Bit 4 - 1'b1: To be an I2S or PCM slave (data path)"]
    #[inline(always)]
    pub fn slave_data_sel(&mut self) -> SlaveDataSelW<'_, SpCtrl0Spec> {
        SlaveDataSelW::new(self, 4)
    }
    #[doc = "Bit 5 - 1'b1: I2S/PCM word clock is inverted for TX (SPK path)"]
    #[inline(always)]
    pub fn sp_wclk_tx_inverse(&mut self) -> SpWclkTxInverseW<'_, SpCtrl0Spec> {
        SpWclkTxInverseW::new(self, 5)
    }
    #[doc = "Bit 6 - 1'b1: self loopback mode"]
    #[inline(always)]
    pub fn sp_loopback(&mut self) -> SpLoopbackW<'_, SpCtrl0Spec> {
        SpLoopbackW::new(self, 6)
    }
    #[doc = "Bit 7 - 1'b1: DSP and SPORT1 handshaking is enabled. 1'b0: GDMA and SPORT1 handshaking is enabled."]
    #[inline(always)]
    pub fn dsp_ctl_mode(&mut self) -> DspCtlModeW<'_, SpCtrl0Spec> {
        DspCtlModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - 2'b00: I2S 2'b01: Left Justified 2'b10: PCM mode A 2'b11: PCM mode B"]
    #[inline(always)]
    pub fn sp_data_format_sel_tx(&mut self) -> SpDataFormatSelTxW<'_, SpCtrl0Spec> {
        SpDataFormatSelTxW::new(self, 8)
    }
    #[doc = "Bit 10 - 1'b1: I2S/PCM bit clock is inverted"]
    #[inline(always)]
    pub fn sp_inv_i2s_sclk(&mut self) -> SpInvI2sSclkW<'_, SpCtrl0Spec> {
        SpInvI2sSclkW::new(self, 10)
    }
    #[doc = "Bit 11 - 1'b1: mono 1'b0: stereo"]
    #[inline(always)]
    pub fn sp_en_i2s_mono_tx_0(&mut self) -> SpEnI2sMonoTx0W<'_, SpCtrl0Spec> {
        SpEnI2sMonoTx0W::new(self, 11)
    }
    #[doc = "Bits 12:14 - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_data_len_sel_tx_0(&mut self) -> SpDataLenSelTx0W<'_, SpCtrl0Spec> {
        SpDataLenSelTx0W::new(self, 12)
    }
    #[doc = "Bit 15 - 1'b1: internal loopback mode is enabled"]
    #[inline(always)]
    pub fn sp_i2s_self_lpbk_en(&mut self) -> SpI2sSelfLpbkEnW<'_, SpCtrl0Spec> {
        SpI2sSelfLpbkEnW::new(self, 15)
    }
    #[doc = "Bit 16 - 1'b1: SPORT TX is disabled. 1'b0: SPORT TX is enabled."]
    #[inline(always)]
    pub fn sp_tx_disable(&mut self) -> SpTxDisableW<'_, SpCtrl0Spec> {
        SpTxDisableW::new(self, 16)
    }
    #[doc = "Bit 17 - 1'b0: TX is disabled. 1'b1: TX is started."]
    #[inline(always)]
    pub fn sp_start_tx(&mut self) -> SpStartTxW<'_, SpCtrl0Spec> {
        SpStartTxW::new(self, 17)
    }
    #[doc = "Bits 18:19 - 2'b00: Without TDM 2'b01: TDM4 2'b10: TDM6 2'b11: TDM8"]
    #[inline(always)]
    pub fn sp_tdm_mode_sel_tx(&mut self) -> SpTdmModeSelTxW<'_, SpCtrl0Spec> {
        SpTdmModeSelTxW::new(self, 18)
    }
    #[doc = "Bits 20:21 - 2'b00: Without TDM 2'b01: TDM4 2'b10: TDM6 2'b11: TDM8"]
    #[inline(always)]
    pub fn sp_tdm_mode_sel_rx(&mut self) -> SpTdmModeSelRxW<'_, SpCtrl0Spec> {
        SpTdmModeSelRxW::new(self, 20)
    }
    #[doc = "Bit 22 - 1'b0: MSB first when TX 1'b1: LSB first"]
    #[inline(always)]
    pub fn tx_lsb_first_0(&mut self) -> TxLsbFirst0W<'_, SpCtrl0Spec> {
        TxLsbFirst0W::new(self, 22)
    }
    #[doc = "Bit 23 - 1'b0: MSB first when RX 1'b1: LSB first"]
    #[inline(always)]
    pub fn rx_lsb_first_0(&mut self) -> RxLsbFirst0W<'_, SpCtrl0Spec> {
        RxLsbFirst0W::new(self, 23)
    }
    #[doc = "Bit 24 - 1'b1: SPORT RX is disabled. 1'b0: SPORT RX is enabled."]
    #[inline(always)]
    pub fn sp_rx_disable(&mut self) -> SpRxDisableW<'_, SpCtrl0Spec> {
        SpRxDisableW::new(self, 24)
    }
    #[doc = "Bit 25 - 1'b0: RX is disabled 1'b1: RX is started"]
    #[inline(always)]
    pub fn sp_start_rx(&mut self) -> SpStartRxW<'_, SpCtrl0Spec> {
        SpStartRxW::new(self, 25)
    }
    #[doc = "Bits 26:27 - 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R @ DAC path"]
    #[inline(always)]
    pub fn sp_sel_i2s_tx_ch(&mut self) -> SpSelI2sTxChW<'_, SpCtrl0Spec> {
        SpSelI2sTxChW::new(self, 26)
    }
    #[doc = "Bits 28:29 - 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R @ ADC path"]
    #[inline(always)]
    pub fn sp_sel_i2s_rx_ch(&mut self) -> SpSelI2sRxChW<'_, SpCtrl0Spec> {
        SpSelI2sRxChW::new(self, 28)
    }
    #[doc = "Bits 30:31 - not used, please refer to 0x64 2'b00: MCLK output=dsp_src_clk/4 2'b01: MCLK output=dsp_src_clk/2 2'b10/2'b11: MCLK output=dsp_src_clk"]
    #[inline(always)]
    pub fn mclk_sel(&mut self) -> MclkSelW<'_, SpCtrl0Spec> {
        MclkSelW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpCtrl0Spec;
impl crate::RegisterSpec for SpCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_ctrl0::R`](R) reader structure"]
impl crate::Readable for SpCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sp_ctrl0::W`](W) writer structure"]
impl crate::Writable for SpCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_CTRL0 to value 0x0101_0000"]
impl crate::Resettable for SpCtrl0Spec {
    const RESET_VALUE: u32 = 0x0101_0000;
}
