#[doc = "Register `i2s_0_control` reader"]
pub type R = crate::R<I2s0ControlSpec>;
#[doc = "Register `i2s_0_control` writer"]
pub type W = crate::W<I2s0ControlSpec>;
#[doc = "Field `i2s_0_rst_n_reg` reader - 1'b1: rst_n to audio digital IP is de-asserted 1'b0: rst_n to audio digital IP is asserted"]
pub type I2s0RstNRegR = crate::BitReader;
#[doc = "Field `i2s_0_rst_n_reg` writer - 1'b1: rst_n to audio digital IP is de-asserted 1'b0: rst_n to audio digital IP is asserted"]
pub type I2s0RstNRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_inv_sclk` reader - 1'b1: I2S/PCM bit clock is inverted"]
pub type I2s0InvSclkR = crate::BitReader;
#[doc = "Field `i2s_0_inv_sclk` writer - 1'b1: I2S/PCM bit clock is inverted"]
pub type I2s0InvSclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_self_lpbk_en` reader - 1'b1: internal loopback mode is enabled"]
pub type I2s0SelfLpbkEnR = crate::BitReader;
#[doc = "Field `i2s_0_self_lpbk_en` writer - 1'b1: internal loopback mode is enabled"]
pub type I2s0SelfLpbkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_same_lrc_en` reader - 1'b1: LRC_TX share to LRC_RX"]
pub type I2s0SameLrcEnR = crate::BitReader;
#[doc = "Field `i2s_0_same_lrc_en` writer - 1'b1: LRC_TX share to LRC_RX"]
pub type I2s0SameLrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_0_tdm_mode_rx` reader - I2S 0 Rx channel TDM mode 0: 2 channels 1: 4 channels 2/3: Reserved"]
pub type I2s0TdmModeRxR = crate::FieldReader;
#[doc = "Field `i2s_0_tdm_mode_rx` writer - I2S 0 Rx channel TDM mode 0: 2 channels 1: 4 channels 2/3: Reserved"]
pub type I2s0TdmModeRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `i2s_0_data_format_sel_tx` reader - I2S 0 Tx channel data format 2'b00: I2S 2'b01: Left-justified 2'b10: PCM mode A 2'b11: PCM mode B"]
pub type I2s0DataFormatSelTxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_format_sel_tx` writer - I2S 0 Tx channel data format 2'b00: I2S 2'b01: Left-justified 2'b10: PCM mode A 2'b11: PCM mode B"]
pub type I2s0DataFormatSelTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `i2s_0_data_format_sel_rx` reader - I2S 0 Rx channel data format 2'b00: I2S 2'b01: Left-justified 2'b10: PCM mode A 2'b11: PCM mode B"]
pub type I2s0DataFormatSelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_format_sel_rx` writer - I2S 0 Rx channel data format 2'b00: I2S 2'b01: Left-justified 2'b10: PCM mode A 2'b11: PCM mode B"]
pub type I2s0DataFormatSelRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `i2s_0_data_len_sel_tx` reader - I2S 0 Tx channel data length 2'b00: 16 bits 2'b10: 24 bits 2'b11: 8 bits"]
pub type I2s0DataLenSelTxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_len_sel_tx` writer - I2S 0 Tx channel data length 2'b00: 16 bits 2'b10: 24 bits 2'b11: 8 bits"]
pub type I2s0DataLenSelTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `i2s_0_data_len_sel_rx` reader - I2S 0 Rx channel data length 2'b00: 16 bits 2'b10: 24 bits 2'b11: 8 bits"]
pub type I2s0DataLenSelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_len_sel_rx` writer - I2S 0 Rx channel data length 2'b00: 16 bits 2'b10: 24 bits 2'b11: 8 bits"]
pub type I2s0DataLenSelRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `i2s_0_ch_len_sel_tx` reader - I2S 0 Tx channel length 2'b00: 16 bits 2'b01: 32 bits 2'b10: 24 bits 2'b11: 8 bits"]
pub type I2s0ChLenSelTxR = crate::FieldReader;
#[doc = "Field `i2s_0_ch_len_sel_tx` writer - I2S 0 Tx channel length 2'b00: 16 bits 2'b01: 32 bits 2'b10: 24 bits 2'b11: 8 bits"]
pub type I2s0ChLenSelTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `i2s_0_ch_len_sel_rx` reader - I2S 0 Rx channel length 2'b00: 16 bits 2'b01: 32 bits 2'b10: 24 bits 2'b11: 8 bits"]
pub type I2s0ChLenSelRxR = crate::FieldReader;
#[doc = "Field `i2s_0_ch_len_sel_rx` writer - I2S 0 Rx channel length 2'b00: 16 bits 2'b01: 32 bits 2'b10: 24 bits 2'b11: 8 bits"]
pub type I2s0ChLenSelRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `i2s_0_data_ch_sel_tx` reader - I2S 0 Tx channel data 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R"]
pub type I2s0DataChSelTxR = crate::FieldReader;
#[doc = "Field `i2s_0_data_ch_sel_tx` writer - I2S 0 Tx channel data 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R"]
pub type I2s0DataChSelTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `i2s_0_master_sel` reader - I2S 0 master source selection 1'b0: internal SPORT 1'b1: external I2S"]
pub type I2s0MasterSelR = crate::BitReader;
#[doc = "Field `i2s_0_master_sel` writer - I2S 0 master source selection 1'b0: internal SPORT 1'b1: external I2S"]
pub type I2s0MasterSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1'b1: rst_n to audio digital IP is de-asserted 1'b0: rst_n to audio digital IP is asserted"]
    #[inline(always)]
    pub fn i2s_0_rst_n_reg(&self) -> I2s0RstNRegR {
        I2s0RstNRegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1'b1: I2S/PCM bit clock is inverted"]
    #[inline(always)]
    pub fn i2s_0_inv_sclk(&self) -> I2s0InvSclkR {
        I2s0InvSclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1'b1: internal loopback mode is enabled"]
    #[inline(always)]
    pub fn i2s_0_self_lpbk_en(&self) -> I2s0SelfLpbkEnR {
        I2s0SelfLpbkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1'b1: LRC_TX share to LRC_RX"]
    #[inline(always)]
    pub fn i2s_0_same_lrc_en(&self) -> I2s0SameLrcEnR {
        I2s0SameLrcEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S 0 Rx channel TDM mode 0: 2 channels 1: 4 channels 2/3: Reserved"]
    #[inline(always)]
    pub fn i2s_0_tdm_mode_rx(&self) -> I2s0TdmModeRxR {
        I2s0TdmModeRxR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - I2S 0 Tx channel data format 2'b00: I2S 2'b01: Left-justified 2'b10: PCM mode A 2'b11: PCM mode B"]
    #[inline(always)]
    pub fn i2s_0_data_format_sel_tx(&self) -> I2s0DataFormatSelTxR {
        I2s0DataFormatSelTxR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - I2S 0 Rx channel data format 2'b00: I2S 2'b01: Left-justified 2'b10: PCM mode A 2'b11: PCM mode B"]
    #[inline(always)]
    pub fn i2s_0_data_format_sel_rx(&self) -> I2s0DataFormatSelRxR {
        I2s0DataFormatSelRxR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - I2S 0 Tx channel data length 2'b00: 16 bits 2'b10: 24 bits 2'b11: 8 bits"]
    #[inline(always)]
    pub fn i2s_0_data_len_sel_tx(&self) -> I2s0DataLenSelTxR {
        I2s0DataLenSelTxR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2S 0 Rx channel data length 2'b00: 16 bits 2'b10: 24 bits 2'b11: 8 bits"]
    #[inline(always)]
    pub fn i2s_0_data_len_sel_rx(&self) -> I2s0DataLenSelRxR {
        I2s0DataLenSelRxR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2S 0 Tx channel length 2'b00: 16 bits 2'b01: 32 bits 2'b10: 24 bits 2'b11: 8 bits"]
    #[inline(always)]
    pub fn i2s_0_ch_len_sel_tx(&self) -> I2s0ChLenSelTxR {
        I2s0ChLenSelTxR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2S 0 Rx channel length 2'b00: 16 bits 2'b01: 32 bits 2'b10: 24 bits 2'b11: 8 bits"]
    #[inline(always)]
    pub fn i2s_0_ch_len_sel_rx(&self) -> I2s0ChLenSelRxR {
        I2s0ChLenSelRxR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - I2S 0 Tx channel data 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R"]
    #[inline(always)]
    pub fn i2s_0_data_ch_sel_tx(&self) -> I2s0DataChSelTxR {
        I2s0DataChSelTxR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - I2S 0 master source selection 1'b0: internal SPORT 1'b1: external I2S"]
    #[inline(always)]
    pub fn i2s_0_master_sel(&self) -> I2s0MasterSelR {
        I2s0MasterSelR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b1: rst_n to audio digital IP is de-asserted 1'b0: rst_n to audio digital IP is asserted"]
    #[inline(always)]
    pub fn i2s_0_rst_n_reg(&mut self) -> I2s0RstNRegW<'_, I2s0ControlSpec> {
        I2s0RstNRegW::new(self, 0)
    }
    #[doc = "Bit 1 - 1'b1: I2S/PCM bit clock is inverted"]
    #[inline(always)]
    pub fn i2s_0_inv_sclk(&mut self) -> I2s0InvSclkW<'_, I2s0ControlSpec> {
        I2s0InvSclkW::new(self, 1)
    }
    #[doc = "Bit 2 - 1'b1: internal loopback mode is enabled"]
    #[inline(always)]
    pub fn i2s_0_self_lpbk_en(&mut self) -> I2s0SelfLpbkEnW<'_, I2s0ControlSpec> {
        I2s0SelfLpbkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 1'b1: LRC_TX share to LRC_RX"]
    #[inline(always)]
    pub fn i2s_0_same_lrc_en(&mut self) -> I2s0SameLrcEnW<'_, I2s0ControlSpec> {
        I2s0SameLrcEnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2S 0 Rx channel TDM mode 0: 2 channels 1: 4 channels 2/3: Reserved"]
    #[inline(always)]
    pub fn i2s_0_tdm_mode_rx(&mut self) -> I2s0TdmModeRxW<'_, I2s0ControlSpec> {
        I2s0TdmModeRxW::new(self, 4)
    }
    #[doc = "Bits 6:7 - I2S 0 Tx channel data format 2'b00: I2S 2'b01: Left-justified 2'b10: PCM mode A 2'b11: PCM mode B"]
    #[inline(always)]
    pub fn i2s_0_data_format_sel_tx(&mut self) -> I2s0DataFormatSelTxW<'_, I2s0ControlSpec> {
        I2s0DataFormatSelTxW::new(self, 6)
    }
    #[doc = "Bits 8:9 - I2S 0 Rx channel data format 2'b00: I2S 2'b01: Left-justified 2'b10: PCM mode A 2'b11: PCM mode B"]
    #[inline(always)]
    pub fn i2s_0_data_format_sel_rx(&mut self) -> I2s0DataFormatSelRxW<'_, I2s0ControlSpec> {
        I2s0DataFormatSelRxW::new(self, 8)
    }
    #[doc = "Bits 10:11 - I2S 0 Tx channel data length 2'b00: 16 bits 2'b10: 24 bits 2'b11: 8 bits"]
    #[inline(always)]
    pub fn i2s_0_data_len_sel_tx(&mut self) -> I2s0DataLenSelTxW<'_, I2s0ControlSpec> {
        I2s0DataLenSelTxW::new(self, 10)
    }
    #[doc = "Bits 12:13 - I2S 0 Rx channel data length 2'b00: 16 bits 2'b10: 24 bits 2'b11: 8 bits"]
    #[inline(always)]
    pub fn i2s_0_data_len_sel_rx(&mut self) -> I2s0DataLenSelRxW<'_, I2s0ControlSpec> {
        I2s0DataLenSelRxW::new(self, 12)
    }
    #[doc = "Bits 14:15 - I2S 0 Tx channel length 2'b00: 16 bits 2'b01: 32 bits 2'b10: 24 bits 2'b11: 8 bits"]
    #[inline(always)]
    pub fn i2s_0_ch_len_sel_tx(&mut self) -> I2s0ChLenSelTxW<'_, I2s0ControlSpec> {
        I2s0ChLenSelTxW::new(self, 14)
    }
    #[doc = "Bits 16:17 - I2S 0 Rx channel length 2'b00: 16 bits 2'b01: 32 bits 2'b10: 24 bits 2'b11: 8 bits"]
    #[inline(always)]
    pub fn i2s_0_ch_len_sel_rx(&mut self) -> I2s0ChLenSelRxW<'_, I2s0ControlSpec> {
        I2s0ChLenSelRxW::new(self, 16)
    }
    #[doc = "Bits 18:19 - I2S 0 Tx channel data 2'b00: L/R 2'b01: R/L 2'b10: L/L 2'b11: R/R"]
    #[inline(always)]
    pub fn i2s_0_data_ch_sel_tx(&mut self) -> I2s0DataChSelTxW<'_, I2s0ControlSpec> {
        I2s0DataChSelTxW::new(self, 18)
    }
    #[doc = "Bit 20 - I2S 0 master source selection 1'b0: internal SPORT 1'b1: external I2S"]
    #[inline(always)]
    pub fn i2s_0_master_sel(&mut self) -> I2s0MasterSelW<'_, I2s0ControlSpec> {
        I2s0MasterSelW::new(self, 20)
    }
}
#[doc = "I2S 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_0_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_0_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2s0ControlSpec;
impl crate::RegisterSpec for I2s0ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_0_control::R`](R) reader structure"]
impl crate::Readable for I2s0ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2s_0_control::W`](W) writer structure"]
impl crate::Writable for I2s0ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets i2s_0_control to value 0"]
impl crate::Resettable for I2s0ControlSpec {}
