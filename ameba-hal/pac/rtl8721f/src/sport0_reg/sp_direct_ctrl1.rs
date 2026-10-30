#[doc = "Register `SP_DIRECT_CTRL1` reader"]
pub type R = crate::R<SpDirectCtrl1Spec>;
#[doc = "Register `SP_DIRECT_CTRL1` writer"]
pub type W = crate::W<SpDirectCtrl1Spec>;
#[doc = "Field `direct_reg_0_sel` reader - (Ibid.) 5'h1c: TDM_RX_CH0"]
pub type DirectReg0SelR = crate::FieldReader;
#[doc = "Field `direct_reg_0_sel` writer - (Ibid.) 5'h1c: TDM_RX_CH0"]
pub type DirectReg0SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `direct_reg_0_en` reader - 1'b1: Enable direct_reg_0."]
pub type DirectReg0EnR = crate::BitReader;
#[doc = "Field `direct_reg_0_en` writer - 1'b1: Enable direct_reg_0."]
pub type DirectReg0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `direct_reg_1_sel` reader - (Ibid.) 5'h1c: TDM_RX_CH1"]
pub type DirectReg1SelR = crate::FieldReader;
#[doc = "Field `direct_reg_1_sel` writer - (Ibid.) 5'h1c: TDM_RX_CH1"]
pub type DirectReg1SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `direct_reg_1_en` reader - 1'b1: Enable direct_reg_1."]
pub type DirectReg1EnR = crate::BitReader;
#[doc = "Field `direct_reg_1_en` writer - 1'b1: Enable direct_reg_1."]
pub type DirectReg1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `direct_reg_2_sel` reader - (Ibid.) 5'h1c: TDM_RX_CH2"]
pub type DirectReg2SelR = crate::FieldReader;
#[doc = "Field `direct_reg_2_sel` writer - (Ibid.) 5'h1c: TDM_RX_CH2"]
pub type DirectReg2SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `direct_reg_2_en` reader - 1'b1: Enable direct_reg_2."]
pub type DirectReg2EnR = crate::BitReader;
#[doc = "Field `direct_reg_2_en` writer - 1'b1: Enable direct_reg_2."]
pub type DirectReg2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `direct_reg_3_sel` reader - 5'h0: spa_direct_in_0 5'h1: spa_direct_in_1 5'h2: spa_direct_in_2 5'h3: spa_direct_in_3 5'h4: spa_direct_in_4 5'h5: spa_direct_in_5 5'h6: spa_direct_in_6 5'h7: spa_direct_in_7 5'h8: spb_direct_in_0 5'h9: spb_direct_in_1 5'ha: spb_direct_in_2 5'hb: spb_direct_in_3 5'hc: spb_direct_in_4 5'hd: spb_direct_in_5 5'he: spb_direct_in_6 5'hf: spb_direct_in_7 5'h10: spc_direct_in_0 5'h11: spc_direct_in_1 5'h12: spc_direct_in_2 5'h13: spc_direct_in_3 5'h14: spc_direct_in_4 5'h15: spc_direct_in_5 5'h16: spc_direct_in_6 5'h17: spc_direct_in_7 5'h18: sp0_direct_in_tx_fifo_0_reg_0_l 5'h19: sp0_direct_in_tx_fifo_0_reg_0_r 5'h1a: sp0_direct_in_tx_fifo_0_reg_1_l 5'h1b: sp0_direct_in_tx_fifo_0_reg_1_r 5'h1c: TDM_RX_CH3 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
pub type DirectReg3SelR = crate::FieldReader;
#[doc = "Field `direct_reg_3_sel` writer - 5'h0: spa_direct_in_0 5'h1: spa_direct_in_1 5'h2: spa_direct_in_2 5'h3: spa_direct_in_3 5'h4: spa_direct_in_4 5'h5: spa_direct_in_5 5'h6: spa_direct_in_6 5'h7: spa_direct_in_7 5'h8: spb_direct_in_0 5'h9: spb_direct_in_1 5'ha: spb_direct_in_2 5'hb: spb_direct_in_3 5'hc: spb_direct_in_4 5'hd: spb_direct_in_5 5'he: spb_direct_in_6 5'hf: spb_direct_in_7 5'h10: spc_direct_in_0 5'h11: spc_direct_in_1 5'h12: spc_direct_in_2 5'h13: spc_direct_in_3 5'h14: spc_direct_in_4 5'h15: spc_direct_in_5 5'h16: spc_direct_in_6 5'h17: spc_direct_in_7 5'h18: sp0_direct_in_tx_fifo_0_reg_0_l 5'h19: sp0_direct_in_tx_fifo_0_reg_0_r 5'h1a: sp0_direct_in_tx_fifo_0_reg_1_l 5'h1b: sp0_direct_in_tx_fifo_0_reg_1_r 5'h1c: TDM_RX_CH3 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
pub type DirectReg3SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `direct_reg_3_en` reader - 1'b1: Enable direct_reg_3."]
pub type DirectReg3EnR = crate::BitReader;
#[doc = "Field `direct_reg_3_en` writer - 1'b1: Enable direct_reg_3."]
pub type DirectReg3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_data_len_sel_tx_1` reader - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
pub type SpDataLenSelTx1R = crate::FieldReader;
#[doc = "Field `sp_data_len_sel_tx_1` writer - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
pub type SpDataLenSelTx1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sp_en_i2s_mono_tx_1` reader - 1'b1: mono 1'b0: stereo"]
pub type SpEnI2sMonoTx1R = crate::BitReader;
#[doc = "Field `sp_en_i2s_mono_tx_1` writer - 1'b1: mono 1'b0: stereo"]
pub type SpEnI2sMonoTx1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_data_len_sel_rx_1` reader - Data length of MIC path and it is valid if \"trx_same_length\" == 1'b0. 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
pub type SpDataLenSelRx1R = crate::FieldReader;
#[doc = "Field `sp_data_len_sel_rx_1` writer - Data length of MIC path and it is valid if \"trx_same_length\" == 1'b0. 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
pub type SpDataLenSelRx1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sp_en_i2s_mono_rx_1` reader - Channel format of MIC path and it is valid if \"trx_same_ch\" == 1'b0. 1'b1: mono 1'b0: stereo"]
pub type SpEnI2sMonoRx1R = crate::BitReader;
#[doc = "Field `sp_en_i2s_mono_rx_1` writer - Channel format of MIC path and it is valid if \"trx_same_ch\" == 1'b0. 1'b1: mono 1'b0: stereo"]
pub type SpEnI2sMonoRx1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - (Ibid.) 5'h1c: TDM_RX_CH0"]
    #[inline(always)]
    pub fn direct_reg_0_sel(&self) -> DirectReg0SelR {
        DirectReg0SelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 1'b1: Enable direct_reg_0."]
    #[inline(always)]
    pub fn direct_reg_0_en(&self) -> DirectReg0EnR {
        DirectReg0EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - (Ibid.) 5'h1c: TDM_RX_CH1"]
    #[inline(always)]
    pub fn direct_reg_1_sel(&self) -> DirectReg1SelR {
        DirectReg1SelR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - 1'b1: Enable direct_reg_1."]
    #[inline(always)]
    pub fn direct_reg_1_en(&self) -> DirectReg1EnR {
        DirectReg1EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - (Ibid.) 5'h1c: TDM_RX_CH2"]
    #[inline(always)]
    pub fn direct_reg_2_sel(&self) -> DirectReg2SelR {
        DirectReg2SelR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - 1'b1: Enable direct_reg_2."]
    #[inline(always)]
    pub fn direct_reg_2_en(&self) -> DirectReg2EnR {
        DirectReg2EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - 5'h0: spa_direct_in_0 5'h1: spa_direct_in_1 5'h2: spa_direct_in_2 5'h3: spa_direct_in_3 5'h4: spa_direct_in_4 5'h5: spa_direct_in_5 5'h6: spa_direct_in_6 5'h7: spa_direct_in_7 5'h8: spb_direct_in_0 5'h9: spb_direct_in_1 5'ha: spb_direct_in_2 5'hb: spb_direct_in_3 5'hc: spb_direct_in_4 5'hd: spb_direct_in_5 5'he: spb_direct_in_6 5'hf: spb_direct_in_7 5'h10: spc_direct_in_0 5'h11: spc_direct_in_1 5'h12: spc_direct_in_2 5'h13: spc_direct_in_3 5'h14: spc_direct_in_4 5'h15: spc_direct_in_5 5'h16: spc_direct_in_6 5'h17: spc_direct_in_7 5'h18: sp0_direct_in_tx_fifo_0_reg_0_l 5'h19: sp0_direct_in_tx_fifo_0_reg_0_r 5'h1a: sp0_direct_in_tx_fifo_0_reg_1_l 5'h1b: sp0_direct_in_tx_fifo_0_reg_1_r 5'h1c: TDM_RX_CH3 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
    #[inline(always)]
    pub fn direct_reg_3_sel(&self) -> DirectReg3SelR {
        DirectReg3SelR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - 1'b1: Enable direct_reg_3."]
    #[inline(always)]
    pub fn direct_reg_3_en(&self) -> DirectReg3EnR {
        DirectReg3EnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_data_len_sel_tx_1(&self) -> SpDataLenSelTx1R {
        SpDataLenSelTx1R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 1'b1: mono 1'b0: stereo"]
    #[inline(always)]
    pub fn sp_en_i2s_mono_tx_1(&self) -> SpEnI2sMonoTx1R {
        SpEnI2sMonoTx1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Data length of MIC path and it is valid if \"trx_same_length\" == 1'b0. 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_data_len_sel_rx_1(&self) -> SpDataLenSelRx1R {
        SpDataLenSelRx1R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Channel format of MIC path and it is valid if \"trx_same_ch\" == 1'b0. 1'b1: mono 1'b0: stereo"]
    #[inline(always)]
    pub fn sp_en_i2s_mono_rx_1(&self) -> SpEnI2sMonoRx1R {
        SpEnI2sMonoRx1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - (Ibid.) 5'h1c: TDM_RX_CH0"]
    #[inline(always)]
    pub fn direct_reg_0_sel(&mut self) -> DirectReg0SelW<'_, SpDirectCtrl1Spec> {
        DirectReg0SelW::new(self, 0)
    }
    #[doc = "Bit 5 - 1'b1: Enable direct_reg_0."]
    #[inline(always)]
    pub fn direct_reg_0_en(&mut self) -> DirectReg0EnW<'_, SpDirectCtrl1Spec> {
        DirectReg0EnW::new(self, 5)
    }
    #[doc = "Bits 6:10 - (Ibid.) 5'h1c: TDM_RX_CH1"]
    #[inline(always)]
    pub fn direct_reg_1_sel(&mut self) -> DirectReg1SelW<'_, SpDirectCtrl1Spec> {
        DirectReg1SelW::new(self, 6)
    }
    #[doc = "Bit 11 - 1'b1: Enable direct_reg_1."]
    #[inline(always)]
    pub fn direct_reg_1_en(&mut self) -> DirectReg1EnW<'_, SpDirectCtrl1Spec> {
        DirectReg1EnW::new(self, 11)
    }
    #[doc = "Bits 12:16 - (Ibid.) 5'h1c: TDM_RX_CH2"]
    #[inline(always)]
    pub fn direct_reg_2_sel(&mut self) -> DirectReg2SelW<'_, SpDirectCtrl1Spec> {
        DirectReg2SelW::new(self, 12)
    }
    #[doc = "Bit 17 - 1'b1: Enable direct_reg_2."]
    #[inline(always)]
    pub fn direct_reg_2_en(&mut self) -> DirectReg2EnW<'_, SpDirectCtrl1Spec> {
        DirectReg2EnW::new(self, 17)
    }
    #[doc = "Bits 18:22 - 5'h0: spa_direct_in_0 5'h1: spa_direct_in_1 5'h2: spa_direct_in_2 5'h3: spa_direct_in_3 5'h4: spa_direct_in_4 5'h5: spa_direct_in_5 5'h6: spa_direct_in_6 5'h7: spa_direct_in_7 5'h8: spb_direct_in_0 5'h9: spb_direct_in_1 5'ha: spb_direct_in_2 5'hb: spb_direct_in_3 5'hc: spb_direct_in_4 5'hd: spb_direct_in_5 5'he: spb_direct_in_6 5'hf: spb_direct_in_7 5'h10: spc_direct_in_0 5'h11: spc_direct_in_1 5'h12: spc_direct_in_2 5'h13: spc_direct_in_3 5'h14: spc_direct_in_4 5'h15: spc_direct_in_5 5'h16: spc_direct_in_6 5'h17: spc_direct_in_7 5'h18: sp0_direct_in_tx_fifo_0_reg_0_l 5'h19: sp0_direct_in_tx_fifo_0_reg_0_r 5'h1a: sp0_direct_in_tx_fifo_0_reg_1_l 5'h1b: sp0_direct_in_tx_fifo_0_reg_1_r 5'h1c: TDM_RX_CH3 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
    #[inline(always)]
    pub fn direct_reg_3_sel(&mut self) -> DirectReg3SelW<'_, SpDirectCtrl1Spec> {
        DirectReg3SelW::new(self, 18)
    }
    #[doc = "Bit 23 - 1'b1: Enable direct_reg_3."]
    #[inline(always)]
    pub fn direct_reg_3_en(&mut self) -> DirectReg3EnW<'_, SpDirectCtrl1Spec> {
        DirectReg3EnW::new(self, 23)
    }
    #[doc = "Bits 24:26 - 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_data_len_sel_tx_1(&mut self) -> SpDataLenSelTx1W<'_, SpDirectCtrl1Spec> {
        SpDataLenSelTx1W::new(self, 24)
    }
    #[doc = "Bit 27 - 1'b1: mono 1'b0: stereo"]
    #[inline(always)]
    pub fn sp_en_i2s_mono_tx_1(&mut self) -> SpEnI2sMonoTx1W<'_, SpDirectCtrl1Spec> {
        SpEnI2sMonoTx1W::new(self, 27)
    }
    #[doc = "Bits 28:30 - Data length of MIC path and it is valid if \"trx_same_length\" == 1'b0. 3'b000: 16 bits 3'b001: 20 bits 3'b010: 24 bits 3'b011: 8 bits 3'b100: 32 bits"]
    #[inline(always)]
    pub fn sp_data_len_sel_rx_1(&mut self) -> SpDataLenSelRx1W<'_, SpDirectCtrl1Spec> {
        SpDataLenSelRx1W::new(self, 28)
    }
    #[doc = "Bit 31 - Channel format of MIC path and it is valid if \"trx_same_ch\" == 1'b0. 1'b1: mono 1'b0: stereo"]
    #[inline(always)]
    pub fn sp_en_i2s_mono_rx_1(&mut self) -> SpEnI2sMonoRx1W<'_, SpDirectCtrl1Spec> {
        SpEnI2sMonoRx1W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_direct_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_direct_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpDirectCtrl1Spec;
impl crate::RegisterSpec for SpDirectCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_direct_ctrl1::R`](R) reader structure"]
impl crate::Readable for SpDirectCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sp_direct_ctrl1::W`](W) writer structure"]
impl crate::Writable for SpDirectCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_DIRECT_CTRL1 to value 0"]
impl crate::Resettable for SpDirectCtrl1Spec {}
