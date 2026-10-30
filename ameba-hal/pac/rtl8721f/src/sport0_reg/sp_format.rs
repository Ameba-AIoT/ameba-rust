#[doc = "Register `SP_FORMAT` reader"]
pub type R = crate::R<SpFormatSpec>;
#[doc = "Register `SP_FORMAT` writer"]
pub type W = crate::W<SpFormatSpec>;
#[doc = "Field `trx_same_fs` reader - 1: TX (SPK path) and RX (MIC path) have the same sampling rate"]
pub type TrxSameFsR = crate::BitReader;
#[doc = "Field `trx_same_fs` writer - 1: TX (SPK path) and RX (MIC path) have the same sampling rate"]
pub type TrxSameFsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trx_same_ch` reader - 1: TX (SPK path) and RX (MIC path) have the same channel setting. Both are either stereo or mono"]
pub type TrxSameChR = crate::BitReader;
#[doc = "Field `trx_same_ch` writer - 1: TX (SPK path) and RX (MIC path) have the same channel setting. Both are either stereo or mono"]
pub type TrxSameChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trx_same_length` reader - 1: TX (SPK path) and RX (MIC path) have the same data length. Both are either 16 or 24 bits"]
pub type TrxSameLengthR = crate::BitReader;
#[doc = "Field `trx_same_length` writer - 1: TX (SPK path) and RX (MIC path) have the same data length. Both are either 16 or 24 bits"]
pub type TrxSameLengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sck_out_inverse` reader - 1: invert the phase of \"sck_out\""]
pub type SckOutInverseR = crate::BitReader;
#[doc = "Field `sck_out_inverse` writer - 1: invert the phase of \"sck_out\""]
pub type SckOutInverseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMMY5` reader - -"]
pub type Dummy5R = crate::BitReader;
#[doc = "Field `DUMMY5` writer - -"]
pub type Dummy5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_wclk_rx_inverse` reader - 1: invert the phase of \"ws_out_rx\" which is also called as \"ADCLRC\""]
pub type SpWclkRxInverseR = crate::BitReader;
#[doc = "Field `sp_wclk_rx_inverse` writer - 1: invert the phase of \"ws_out_rx\" which is also called as \"ADCLRC\""]
pub type SpWclkRxInverseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fixed_bclk_sel` reader - 0: BCLK is fixed at dsp_src_clk/4 1: BCLK is fixed at dsp_src_clk/2"]
pub type FixedBclkSelR = crate::BitReader;
#[doc = "Field `fixed_bclk_sel` writer - 0: BCLK is fixed at dsp_src_clk/4 1: BCLK is fixed at dsp_src_clk/2"]
pub type FixedBclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fixed_bclk` reader - 1: Refer to the description of \"fixed_bclk_sel\" 0: BCLK = dsp_clk/2 when \"mode_40mhz = 0\""]
pub type FixedBclkR = crate::BitReader;
#[doc = "Field `fixed_bclk` writer - 1: Refer to the description of \"fixed_bclk_sel\" 0: BCLK = dsp_clk/2 when \"mode_40mhz = 0\""]
pub type FixedBclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_data_format_sel_rx` reader - Data format of MIC path and it is valid if \"trx_same_fs\" == 1'b0. 2'b00: I2S 2'b01: Left Justified 2'b10: PCM mode A 2'b11: PCM mode B"]
pub type SpDataFormatSelRxR = crate::FieldReader;
#[doc = "Field `sp_data_format_sel_rx` writer - Data format of MIC path and it is valid if \"trx_same_fs\" == 1'b0. 2'b00: I2S 2'b01: Left Justified 2'b10: PCM mode A 2'b11: PCM mode B"]
pub type SpDataFormatSelRxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `trx_same_lrc` reader - 1: \"ws_out_rx\" is as same as \"ws_out_tx\""]
pub type TrxSameLrcR = crate::BitReader;
#[doc = "Field `trx_same_lrc` writer - 1: \"ws_out_rx\" is as same as \"ws_out_tx\""]
pub type TrxSameLrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_en_i2s_mono_rx_0` reader - Channel format of MIC path and it is valid if \"trx_same_ch\" == 1'b0. 1: mono 0: stereo"]
pub type SpEnI2sMonoRx0R = crate::BitReader;
#[doc = "Field `sp_en_i2s_mono_rx_0` writer - Channel format of MIC path and it is valid if \"trx_same_ch\" == 1'b0. 1: mono 0: stereo"]
pub type SpEnI2sMonoRx0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_data_len_sel_rx_0` reader - Data length of MIC path and it is valid if \"trx_same_length\" == 1'b0. 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
pub type SpDataLenSelRx0R = crate::FieldReader;
#[doc = "Field `sp_data_len_sel_rx_0` writer - Data length of MIC path and it is valid if \"trx_same_length\" == 1'b0. 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
pub type SpDataLenSelRx0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DUMMY4` reader - -"]
pub type Dummy4R = crate::BitReader;
#[doc = "Field `DUMMY4` writer - -"]
pub type Dummy4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_ideal_len` reader - sd_out can be sent 1 ~ 8 ( = tx_ideal_len + 1 ) BCLK cycle earlier."]
pub type TxIdealLenR = crate::FieldReader;
#[doc = "Field `tx_ideal_len` writer - sd_out can be sent 1 ~ 8 ( = tx_ideal_len + 1 ) BCLK cycle earlier."]
pub type TxIdealLenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tx_ideal_len_en` reader - Function enable of tx_ideal_len. PCMA SDO will be delayed 1 LRCK."]
pub type TxIdealLenEnR = crate::BitReader;
#[doc = "Field `tx_ideal_len_en` writer - Function enable of tx_ideal_len. PCMA SDO will be delayed 1 LRCK."]
pub type TxIdealLenEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_ideal_len` reader - sd_in can be received 1 ~ 8 ( = rx_ideal_len + 1 ) BCLK cycle latter."]
pub type RxIdealLenR = crate::FieldReader;
#[doc = "Field `rx_ideal_len` writer - sd_in can be received 1 ~ 8 ( = rx_ideal_len + 1 ) BCLK cycle latter."]
pub type RxIdealLenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rx_ideal_len_en` reader - Function enable of rx_ideal_len."]
pub type RxIdealLenEnR = crate::BitReader;
#[doc = "Field `rx_ideal_len_en` writer - Function enable of rx_ideal_len."]
pub type RxIdealLenEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_ch_len_sel_tx` reader - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
pub type SpChLenSelTxR = crate::FieldReader;
#[doc = "Field `sp_ch_len_sel_tx` writer - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
pub type SpChLenSelTxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DUMMY3` reader - -"]
pub type Dummy3R = crate::BitReader;
#[doc = "Field `DUMMY3` writer - -"]
pub type Dummy3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_ch_len_sel_rx` reader - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
pub type SpChLenSelRxR = crate::FieldReader;
#[doc = "Field `sp_ch_len_sel_rx` writer - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
pub type SpChLenSelRxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `trx_same_ch_len` reader - 1: TX (SPK path) and RX (MIC path) have the same channel length."]
pub type TrxSameChLenR = crate::BitReader;
#[doc = "Field `trx_same_ch_len` writer - 1: TX (SPK path) and RX (MIC path) have the same channel length."]
pub type TrxSameChLenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: TX (SPK path) and RX (MIC path) have the same sampling rate"]
    #[inline(always)]
    pub fn trx_same_fs(&self) -> TrxSameFsR {
        TrxSameFsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: TX (SPK path) and RX (MIC path) have the same channel setting. Both are either stereo or mono"]
    #[inline(always)]
    pub fn trx_same_ch(&self) -> TrxSameChR {
        TrxSameChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: TX (SPK path) and RX (MIC path) have the same data length. Both are either 16 or 24 bits"]
    #[inline(always)]
    pub fn trx_same_length(&self) -> TrxSameLengthR {
        TrxSameLengthR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: invert the phase of \"sck_out\""]
    #[inline(always)]
    pub fn sck_out_inverse(&self) -> SckOutInverseR {
        SckOutInverseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - -"]
    #[inline(always)]
    pub fn dummy5(&self) -> Dummy5R {
        Dummy5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: invert the phase of \"ws_out_rx\" which is also called as \"ADCLRC\""]
    #[inline(always)]
    pub fn sp_wclk_rx_inverse(&self) -> SpWclkRxInverseR {
        SpWclkRxInverseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: BCLK is fixed at dsp_src_clk/4 1: BCLK is fixed at dsp_src_clk/2"]
    #[inline(always)]
    pub fn fixed_bclk_sel(&self) -> FixedBclkSelR {
        FixedBclkSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Refer to the description of \"fixed_bclk_sel\" 0: BCLK = dsp_clk/2 when \"mode_40mhz = 0\""]
    #[inline(always)]
    pub fn fixed_bclk(&self) -> FixedBclkR {
        FixedBclkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data format of MIC path and it is valid if \"trx_same_fs\" == 1'b0. 2'b00: I2S 2'b01: Left Justified 2'b10: PCM mode A 2'b11: PCM mode B"]
    #[inline(always)]
    pub fn sp_data_format_sel_rx(&self) -> SpDataFormatSelRxR {
        SpDataFormatSelRxR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 1: \"ws_out_rx\" is as same as \"ws_out_tx\""]
    #[inline(always)]
    pub fn trx_same_lrc(&self) -> TrxSameLrcR {
        TrxSameLrcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel format of MIC path and it is valid if \"trx_same_ch\" == 1'b0. 1: mono 0: stereo"]
    #[inline(always)]
    pub fn sp_en_i2s_mono_rx_0(&self) -> SpEnI2sMonoRx0R {
        SpEnI2sMonoRx0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Data length of MIC path and it is valid if \"trx_same_length\" == 1'b0. 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_data_len_sel_rx_0(&self) -> SpDataLenSelRx0R {
        SpDataLenSelRx0R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - -"]
    #[inline(always)]
    pub fn dummy4(&self) -> Dummy4R {
        Dummy4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - sd_out can be sent 1 ~ 8 ( = tx_ideal_len + 1 ) BCLK cycle earlier."]
    #[inline(always)]
    pub fn tx_ideal_len(&self) -> TxIdealLenR {
        TxIdealLenR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Function enable of tx_ideal_len. PCMA SDO will be delayed 1 LRCK."]
    #[inline(always)]
    pub fn tx_ideal_len_en(&self) -> TxIdealLenEnR {
        TxIdealLenEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - sd_in can be received 1 ~ 8 ( = rx_ideal_len + 1 ) BCLK cycle latter."]
    #[inline(always)]
    pub fn rx_ideal_len(&self) -> RxIdealLenR {
        RxIdealLenR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Function enable of rx_ideal_len."]
    #[inline(always)]
    pub fn rx_ideal_len_en(&self) -> RxIdealLenEnR {
        RxIdealLenEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_ch_len_sel_tx(&self) -> SpChLenSelTxR {
        SpChLenSelTxR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - -"]
    #[inline(always)]
    pub fn dummy3(&self) -> Dummy3R {
        Dummy3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_ch_len_sel_rx(&self) -> SpChLenSelRxR {
        SpChLenSelRxR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 1: TX (SPK path) and RX (MIC path) have the same channel length."]
    #[inline(always)]
    pub fn trx_same_ch_len(&self) -> TrxSameChLenR {
        TrxSameChLenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: TX (SPK path) and RX (MIC path) have the same sampling rate"]
    #[inline(always)]
    pub fn trx_same_fs(&mut self) -> TrxSameFsW<'_, SpFormatSpec> {
        TrxSameFsW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: TX (SPK path) and RX (MIC path) have the same channel setting. Both are either stereo or mono"]
    #[inline(always)]
    pub fn trx_same_ch(&mut self) -> TrxSameChW<'_, SpFormatSpec> {
        TrxSameChW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: TX (SPK path) and RX (MIC path) have the same data length. Both are either 16 or 24 bits"]
    #[inline(always)]
    pub fn trx_same_length(&mut self) -> TrxSameLengthW<'_, SpFormatSpec> {
        TrxSameLengthW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: invert the phase of \"sck_out\""]
    #[inline(always)]
    pub fn sck_out_inverse(&mut self) -> SckOutInverseW<'_, SpFormatSpec> {
        SckOutInverseW::new(self, 3)
    }
    #[doc = "Bit 4 - -"]
    #[inline(always)]
    pub fn dummy5(&mut self) -> Dummy5W<'_, SpFormatSpec> {
        Dummy5W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: invert the phase of \"ws_out_rx\" which is also called as \"ADCLRC\""]
    #[inline(always)]
    pub fn sp_wclk_rx_inverse(&mut self) -> SpWclkRxInverseW<'_, SpFormatSpec> {
        SpWclkRxInverseW::new(self, 5)
    }
    #[doc = "Bit 6 - 0: BCLK is fixed at dsp_src_clk/4 1: BCLK is fixed at dsp_src_clk/2"]
    #[inline(always)]
    pub fn fixed_bclk_sel(&mut self) -> FixedBclkSelW<'_, SpFormatSpec> {
        FixedBclkSelW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Refer to the description of \"fixed_bclk_sel\" 0: BCLK = dsp_clk/2 when \"mode_40mhz = 0\""]
    #[inline(always)]
    pub fn fixed_bclk(&mut self) -> FixedBclkW<'_, SpFormatSpec> {
        FixedBclkW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Data format of MIC path and it is valid if \"trx_same_fs\" == 1'b0. 2'b00: I2S 2'b01: Left Justified 2'b10: PCM mode A 2'b11: PCM mode B"]
    #[inline(always)]
    pub fn sp_data_format_sel_rx(&mut self) -> SpDataFormatSelRxW<'_, SpFormatSpec> {
        SpDataFormatSelRxW::new(self, 8)
    }
    #[doc = "Bit 10 - 1: \"ws_out_rx\" is as same as \"ws_out_tx\""]
    #[inline(always)]
    pub fn trx_same_lrc(&mut self) -> TrxSameLrcW<'_, SpFormatSpec> {
        TrxSameLrcW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel format of MIC path and it is valid if \"trx_same_ch\" == 1'b0. 1: mono 0: stereo"]
    #[inline(always)]
    pub fn sp_en_i2s_mono_rx_0(&mut self) -> SpEnI2sMonoRx0W<'_, SpFormatSpec> {
        SpEnI2sMonoRx0W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Data length of MIC path and it is valid if \"trx_same_length\" == 1'b0. 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_data_len_sel_rx_0(&mut self) -> SpDataLenSelRx0W<'_, SpFormatSpec> {
        SpDataLenSelRx0W::new(self, 12)
    }
    #[doc = "Bit 15 - -"]
    #[inline(always)]
    pub fn dummy4(&mut self) -> Dummy4W<'_, SpFormatSpec> {
        Dummy4W::new(self, 15)
    }
    #[doc = "Bits 16:18 - sd_out can be sent 1 ~ 8 ( = tx_ideal_len + 1 ) BCLK cycle earlier."]
    #[inline(always)]
    pub fn tx_ideal_len(&mut self) -> TxIdealLenW<'_, SpFormatSpec> {
        TxIdealLenW::new(self, 16)
    }
    #[doc = "Bit 19 - Function enable of tx_ideal_len. PCMA SDO will be delayed 1 LRCK."]
    #[inline(always)]
    pub fn tx_ideal_len_en(&mut self) -> TxIdealLenEnW<'_, SpFormatSpec> {
        TxIdealLenEnW::new(self, 19)
    }
    #[doc = "Bits 20:22 - sd_in can be received 1 ~ 8 ( = rx_ideal_len + 1 ) BCLK cycle latter."]
    #[inline(always)]
    pub fn rx_ideal_len(&mut self) -> RxIdealLenW<'_, SpFormatSpec> {
        RxIdealLenW::new(self, 20)
    }
    #[doc = "Bit 23 - Function enable of rx_ideal_len."]
    #[inline(always)]
    pub fn rx_ideal_len_en(&mut self) -> RxIdealLenEnW<'_, SpFormatSpec> {
        RxIdealLenEnW::new(self, 23)
    }
    #[doc = "Bits 24:26 - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_ch_len_sel_tx(&mut self) -> SpChLenSelTxW<'_, SpFormatSpec> {
        SpChLenSelTxW::new(self, 24)
    }
    #[doc = "Bit 27 - -"]
    #[inline(always)]
    pub fn dummy3(&mut self) -> Dummy3W<'_, SpFormatSpec> {
        Dummy3W::new(self, 27)
    }
    #[doc = "Bits 28:30 - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_ch_len_sel_rx(&mut self) -> SpChLenSelRxW<'_, SpFormatSpec> {
        SpChLenSelRxW::new(self, 28)
    }
    #[doc = "Bit 31 - 1: TX (SPK path) and RX (MIC path) have the same channel length."]
    #[inline(always)]
    pub fn trx_same_ch_len(&mut self) -> TrxSameChLenW<'_, SpFormatSpec> {
        TrxSameChLenW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpFormatSpec;
impl crate::RegisterSpec for SpFormatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_format::R`](R) reader structure"]
impl crate::Readable for SpFormatSpec {}
#[doc = "`write(|w| ..)` method takes [`sp_format::W`](W) writer structure"]
impl crate::Writable for SpFormatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_FORMAT to value 0x4400_0007"]
impl crate::Resettable for SpFormatSpec {
    const RESET_VALUE: u32 = 0x4400_0007;
}
