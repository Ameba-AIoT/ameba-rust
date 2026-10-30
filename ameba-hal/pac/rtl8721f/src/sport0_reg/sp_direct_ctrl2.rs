#[doc = "Register `SP_DIRECT_CTRL2` reader"]
pub type R = crate::R<SpDirectCtrl2Spec>;
#[doc = "Register `SP_DIRECT_CTRL2` writer"]
pub type W = crate::W<SpDirectCtrl2Spec>;
#[doc = "Field `direct_reg_4_sel` reader - (Ibid.) 5'h1c: TDM_RX_CH4"]
pub type DirectReg4SelR = crate::FieldReader;
#[doc = "Field `direct_reg_4_sel` writer - (Ibid.) 5'h1c: TDM_RX_CH4"]
pub type DirectReg4SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `direct_reg_4_en` reader - 1'b1: Enable direct_reg_4."]
pub type DirectReg4EnR = crate::BitReader;
#[doc = "Field `direct_reg_4_en` writer - 1'b1: Enable direct_reg_4."]
pub type DirectReg4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `direct_reg_5_sel` reader - (Ibid.) 5'h1c: TDM_RX_CH5"]
pub type DirectReg5SelR = crate::FieldReader;
#[doc = "Field `direct_reg_5_sel` writer - (Ibid.) 5'h1c: TDM_RX_CH5"]
pub type DirectReg5SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `direct_reg_5_en` reader - 1'b1: Enable direct_reg_5."]
pub type DirectReg5EnR = crate::BitReader;
#[doc = "Field `direct_reg_5_en` writer - 1'b1: Enable direct_reg_5."]
pub type DirectReg5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `direct_reg_6_sel` reader - (Ibid.) 5'h1c: TDM_RX_CH6"]
pub type DirectReg6SelR = crate::FieldReader;
#[doc = "Field `direct_reg_6_sel` writer - (Ibid.) 5'h1c: TDM_RX_CH6"]
pub type DirectReg6SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `direct_reg_6_en` reader - 1'b1: Enable direct_reg_6."]
pub type DirectReg6EnR = crate::BitReader;
#[doc = "Field `direct_reg_6_en` writer - 1'b1: Enable direct_reg_6."]
pub type DirectReg6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `direct_reg_7_sel` reader - 5'h0: spa_direct_in_0 5'h1: spa_direct_in_1 5'h2: spa_direct_in_2 5'h3: spa_direct_in_3 5'h4: spa_direct_in_4 5'h5: spa_direct_in_5 5'h6: spa_direct_in_6 5'h7: spa_direct_in_7 5'h8: spb_direct_in_0 5'h9: spb_direct_in_1 5'ha: spb_direct_in_2 5'hb: spb_direct_in_3 5'hc: spb_direct_in_4 5'hd: spb_direct_in_5 5'he: spb_direct_in_6 5'hf: spb_direct_in_7 5'h10: spc_direct_in_0 5'h11: spc_direct_in_1 5'h12: spc_direct_in_2 5'h13: spc_direct_in_3 5'h14: spc_direct_in_4 5'h15: spc_direct_in_5 5'h16: spc_direct_in_6 5'h17: spc_direct_in_7 5'h18: sp0_direct_in_tx_fifo_0_reg_0_l 5'h19: sp0_direct_in_tx_fifo_0_reg_0_r 5'h1a: sp0_direct_in_tx_fifo_0_reg_1_l 5'h1b: sp0_direct_in_tx_fifo_0_reg_1_r 5'h1c: TDM_RX_CH7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
pub type DirectReg7SelR = crate::FieldReader;
#[doc = "Field `direct_reg_7_sel` writer - 5'h0: spa_direct_in_0 5'h1: spa_direct_in_1 5'h2: spa_direct_in_2 5'h3: spa_direct_in_3 5'h4: spa_direct_in_4 5'h5: spa_direct_in_5 5'h6: spa_direct_in_6 5'h7: spa_direct_in_7 5'h8: spb_direct_in_0 5'h9: spb_direct_in_1 5'ha: spb_direct_in_2 5'hb: spb_direct_in_3 5'hc: spb_direct_in_4 5'hd: spb_direct_in_5 5'he: spb_direct_in_6 5'hf: spb_direct_in_7 5'h10: spc_direct_in_0 5'h11: spc_direct_in_1 5'h12: spc_direct_in_2 5'h13: spc_direct_in_3 5'h14: spc_direct_in_4 5'h15: spc_direct_in_5 5'h16: spc_direct_in_6 5'h17: spc_direct_in_7 5'h18: sp0_direct_in_tx_fifo_0_reg_0_l 5'h19: sp0_direct_in_tx_fifo_0_reg_0_r 5'h1a: sp0_direct_in_tx_fifo_0_reg_1_l 5'h1b: sp0_direct_in_tx_fifo_0_reg_1_r 5'h1c: TDM_RX_CH7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
pub type DirectReg7SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `direct_reg_7_en` reader - 1'b1: Enable direct_reg_7."]
pub type DirectReg7EnR = crate::BitReader;
#[doc = "Field `direct_reg_7_en` writer - 1'b1: Enable direct_reg_7."]
pub type DirectReg7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_direct_out_0_en` reader - Enable sp_direct_out_0."]
pub type SpDirectOut0EnR = crate::BitReader;
#[doc = "Field `sp_direct_out_0_en` writer - Enable sp_direct_out_0."]
pub type SpDirectOut0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_direct_out_1_en` reader - Enable sp_direct_out_1."]
pub type SpDirectOut1EnR = crate::BitReader;
#[doc = "Field `sp_direct_out_1_en` writer - Enable sp_direct_out_1."]
pub type SpDirectOut1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_direct_out_2_en` reader - Enable sp_direct_out_2."]
pub type SpDirectOut2EnR = crate::BitReader;
#[doc = "Field `sp_direct_out_2_en` writer - Enable sp_direct_out_2."]
pub type SpDirectOut2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_direct_out_3_en` reader - Enable sp_direct_out_3."]
pub type SpDirectOut3EnR = crate::BitReader;
#[doc = "Field `sp_direct_out_3_en` writer - Enable sp_direct_out_3."]
pub type SpDirectOut3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_direct_out_4_en` reader - Enable sp_direct_out_4."]
pub type SpDirectOut4EnR = crate::BitReader;
#[doc = "Field `sp_direct_out_4_en` writer - Enable sp_direct_out_4."]
pub type SpDirectOut4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_direct_out_5_en` reader - Enable sp_direct_out_5."]
pub type SpDirectOut5EnR = crate::BitReader;
#[doc = "Field `sp_direct_out_5_en` writer - Enable sp_direct_out_5."]
pub type SpDirectOut5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_direct_out_6_en` reader - Enable sp_direct_out_6."]
pub type SpDirectOut6EnR = crate::BitReader;
#[doc = "Field `sp_direct_out_6_en` writer - Enable sp_direct_out_6."]
pub type SpDirectOut6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_direct_out_7_en` reader - Enable sp_direct_out_7."]
pub type SpDirectOut7EnR = crate::BitReader;
#[doc = "Field `sp_direct_out_7_en` writer - Enable sp_direct_out_7."]
pub type SpDirectOut7EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - (Ibid.) 5'h1c: TDM_RX_CH4"]
    #[inline(always)]
    pub fn direct_reg_4_sel(&self) -> DirectReg4SelR {
        DirectReg4SelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 1'b1: Enable direct_reg_4."]
    #[inline(always)]
    pub fn direct_reg_4_en(&self) -> DirectReg4EnR {
        DirectReg4EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - (Ibid.) 5'h1c: TDM_RX_CH5"]
    #[inline(always)]
    pub fn direct_reg_5_sel(&self) -> DirectReg5SelR {
        DirectReg5SelR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - 1'b1: Enable direct_reg_5."]
    #[inline(always)]
    pub fn direct_reg_5_en(&self) -> DirectReg5EnR {
        DirectReg5EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - (Ibid.) 5'h1c: TDM_RX_CH6"]
    #[inline(always)]
    pub fn direct_reg_6_sel(&self) -> DirectReg6SelR {
        DirectReg6SelR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - 1'b1: Enable direct_reg_6."]
    #[inline(always)]
    pub fn direct_reg_6_en(&self) -> DirectReg6EnR {
        DirectReg6EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - 5'h0: spa_direct_in_0 5'h1: spa_direct_in_1 5'h2: spa_direct_in_2 5'h3: spa_direct_in_3 5'h4: spa_direct_in_4 5'h5: spa_direct_in_5 5'h6: spa_direct_in_6 5'h7: spa_direct_in_7 5'h8: spb_direct_in_0 5'h9: spb_direct_in_1 5'ha: spb_direct_in_2 5'hb: spb_direct_in_3 5'hc: spb_direct_in_4 5'hd: spb_direct_in_5 5'he: spb_direct_in_6 5'hf: spb_direct_in_7 5'h10: spc_direct_in_0 5'h11: spc_direct_in_1 5'h12: spc_direct_in_2 5'h13: spc_direct_in_3 5'h14: spc_direct_in_4 5'h15: spc_direct_in_5 5'h16: spc_direct_in_6 5'h17: spc_direct_in_7 5'h18: sp0_direct_in_tx_fifo_0_reg_0_l 5'h19: sp0_direct_in_tx_fifo_0_reg_0_r 5'h1a: sp0_direct_in_tx_fifo_0_reg_1_l 5'h1b: sp0_direct_in_tx_fifo_0_reg_1_r 5'h1c: TDM_RX_CH7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
    #[inline(always)]
    pub fn direct_reg_7_sel(&self) -> DirectReg7SelR {
        DirectReg7SelR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - 1'b1: Enable direct_reg_7."]
    #[inline(always)]
    pub fn direct_reg_7_en(&self) -> DirectReg7EnR {
        DirectReg7EnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable sp_direct_out_0."]
    #[inline(always)]
    pub fn sp_direct_out_0_en(&self) -> SpDirectOut0EnR {
        SpDirectOut0EnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable sp_direct_out_1."]
    #[inline(always)]
    pub fn sp_direct_out_1_en(&self) -> SpDirectOut1EnR {
        SpDirectOut1EnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable sp_direct_out_2."]
    #[inline(always)]
    pub fn sp_direct_out_2_en(&self) -> SpDirectOut2EnR {
        SpDirectOut2EnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable sp_direct_out_3."]
    #[inline(always)]
    pub fn sp_direct_out_3_en(&self) -> SpDirectOut3EnR {
        SpDirectOut3EnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable sp_direct_out_4."]
    #[inline(always)]
    pub fn sp_direct_out_4_en(&self) -> SpDirectOut4EnR {
        SpDirectOut4EnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable sp_direct_out_5."]
    #[inline(always)]
    pub fn sp_direct_out_5_en(&self) -> SpDirectOut5EnR {
        SpDirectOut5EnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable sp_direct_out_6."]
    #[inline(always)]
    pub fn sp_direct_out_6_en(&self) -> SpDirectOut6EnR {
        SpDirectOut6EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable sp_direct_out_7."]
    #[inline(always)]
    pub fn sp_direct_out_7_en(&self) -> SpDirectOut7EnR {
        SpDirectOut7EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - (Ibid.) 5'h1c: TDM_RX_CH4"]
    #[inline(always)]
    pub fn direct_reg_4_sel(&mut self) -> DirectReg4SelW<'_, SpDirectCtrl2Spec> {
        DirectReg4SelW::new(self, 0)
    }
    #[doc = "Bit 5 - 1'b1: Enable direct_reg_4."]
    #[inline(always)]
    pub fn direct_reg_4_en(&mut self) -> DirectReg4EnW<'_, SpDirectCtrl2Spec> {
        DirectReg4EnW::new(self, 5)
    }
    #[doc = "Bits 6:10 - (Ibid.) 5'h1c: TDM_RX_CH5"]
    #[inline(always)]
    pub fn direct_reg_5_sel(&mut self) -> DirectReg5SelW<'_, SpDirectCtrl2Spec> {
        DirectReg5SelW::new(self, 6)
    }
    #[doc = "Bit 11 - 1'b1: Enable direct_reg_5."]
    #[inline(always)]
    pub fn direct_reg_5_en(&mut self) -> DirectReg5EnW<'_, SpDirectCtrl2Spec> {
        DirectReg5EnW::new(self, 11)
    }
    #[doc = "Bits 12:16 - (Ibid.) 5'h1c: TDM_RX_CH6"]
    #[inline(always)]
    pub fn direct_reg_6_sel(&mut self) -> DirectReg6SelW<'_, SpDirectCtrl2Spec> {
        DirectReg6SelW::new(self, 12)
    }
    #[doc = "Bit 17 - 1'b1: Enable direct_reg_6."]
    #[inline(always)]
    pub fn direct_reg_6_en(&mut self) -> DirectReg6EnW<'_, SpDirectCtrl2Spec> {
        DirectReg6EnW::new(self, 17)
    }
    #[doc = "Bits 18:22 - 5'h0: spa_direct_in_0 5'h1: spa_direct_in_1 5'h2: spa_direct_in_2 5'h3: spa_direct_in_3 5'h4: spa_direct_in_4 5'h5: spa_direct_in_5 5'h6: spa_direct_in_6 5'h7: spa_direct_in_7 5'h8: spb_direct_in_0 5'h9: spb_direct_in_1 5'ha: spb_direct_in_2 5'hb: spb_direct_in_3 5'hc: spb_direct_in_4 5'hd: spb_direct_in_5 5'he: spb_direct_in_6 5'hf: spb_direct_in_7 5'h10: spc_direct_in_0 5'h11: spc_direct_in_1 5'h12: spc_direct_in_2 5'h13: spc_direct_in_3 5'h14: spc_direct_in_4 5'h15: spc_direct_in_5 5'h16: spc_direct_in_6 5'h17: spc_direct_in_7 5'h18: sp0_direct_in_tx_fifo_0_reg_0_l 5'h19: sp0_direct_in_tx_fifo_0_reg_0_r 5'h1a: sp0_direct_in_tx_fifo_0_reg_1_l 5'h1b: sp0_direct_in_tx_fifo_0_reg_1_r 5'h1c: TDM_RX_CH7 SPORT0: a = 1, b = 2, c = 3 SPORT1: a = 0, b = 2, c = 3 SPORT2: a = 0, b = 1, c = 3 SPORT3: a = 0, b = 1, c = 2"]
    #[inline(always)]
    pub fn direct_reg_7_sel(&mut self) -> DirectReg7SelW<'_, SpDirectCtrl2Spec> {
        DirectReg7SelW::new(self, 18)
    }
    #[doc = "Bit 23 - 1'b1: Enable direct_reg_7."]
    #[inline(always)]
    pub fn direct_reg_7_en(&mut self) -> DirectReg7EnW<'_, SpDirectCtrl2Spec> {
        DirectReg7EnW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable sp_direct_out_0."]
    #[inline(always)]
    pub fn sp_direct_out_0_en(&mut self) -> SpDirectOut0EnW<'_, SpDirectCtrl2Spec> {
        SpDirectOut0EnW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable sp_direct_out_1."]
    #[inline(always)]
    pub fn sp_direct_out_1_en(&mut self) -> SpDirectOut1EnW<'_, SpDirectCtrl2Spec> {
        SpDirectOut1EnW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable sp_direct_out_2."]
    #[inline(always)]
    pub fn sp_direct_out_2_en(&mut self) -> SpDirectOut2EnW<'_, SpDirectCtrl2Spec> {
        SpDirectOut2EnW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable sp_direct_out_3."]
    #[inline(always)]
    pub fn sp_direct_out_3_en(&mut self) -> SpDirectOut3EnW<'_, SpDirectCtrl2Spec> {
        SpDirectOut3EnW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable sp_direct_out_4."]
    #[inline(always)]
    pub fn sp_direct_out_4_en(&mut self) -> SpDirectOut4EnW<'_, SpDirectCtrl2Spec> {
        SpDirectOut4EnW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable sp_direct_out_5."]
    #[inline(always)]
    pub fn sp_direct_out_5_en(&mut self) -> SpDirectOut5EnW<'_, SpDirectCtrl2Spec> {
        SpDirectOut5EnW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable sp_direct_out_6."]
    #[inline(always)]
    pub fn sp_direct_out_6_en(&mut self) -> SpDirectOut6EnW<'_, SpDirectCtrl2Spec> {
        SpDirectOut6EnW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable sp_direct_out_7."]
    #[inline(always)]
    pub fn sp_direct_out_7_en(&mut self) -> SpDirectOut7EnW<'_, SpDirectCtrl2Spec> {
        SpDirectOut7EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_direct_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_direct_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpDirectCtrl2Spec;
impl crate::RegisterSpec for SpDirectCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_direct_ctrl2::R`](R) reader structure"]
impl crate::Readable for SpDirectCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sp_direct_ctrl2::W`](W) writer structure"]
impl crate::Writable for SpDirectCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_DIRECT_CTRL2 to value 0"]
impl crate::Resettable for SpDirectCtrl2Spec {}
