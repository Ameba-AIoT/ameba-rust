#[doc = "Register `REG_AX_TB_MUEDCA_CTRL` reader"]
pub type R = crate::R<RegAxTbMuedcaCtrlSpec>;
#[doc = "Register `REG_AX_TB_MUEDCA_CTRL` writer"]
pub type W = crate::W<RegAxTbMuedcaCtrlSpec>;
#[doc = "Field `BIT_muedca_en` reader - 1: enable HW updating MUEDCATimer 0: disable HW updating MUEDCATimer"]
pub type BitMuedcaEnR = crate::BitReader;
#[doc = "Field `BIT_muedca_en` writer - 1: enable HW updating MUEDCATimer 0: disable HW updating MUEDCATimer"]
pub type BitMuedcaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_set_muedcatimer_tf` reader - 1: enable HW updating MUEDCATimer after Tx HE TB PPDU 0: disable HW updating MUEDCATimer after Tx HE TB PPDU"]
pub type BitSetMuedcatimerTfR = crate::BitReader;
#[doc = "Field `BIT_set_muedcatimer_tf` writer - 1: enable HW updating MUEDCATimer after Tx HE TB PPDU 0: disable HW updating MUEDCATimer after Tx HE TB PPDU"]
pub type BitSetMuedcatimerTfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_UPD_BEAM_CHG` reader - 1: txcmd_beam_change TIE 0 0: first PPDU in TXOP will assert txcmd_beam_change"]
pub type BitDisUpdBeamChgR = crate::BitReader;
#[doc = "Field `BIT_DIS_UPD_BEAM_CHG` writer - 1: txcmd_beam_change TIE 0 0: first PPDU in TXOP will assert txcmd_beam_change"]
pub type BitDisUpdBeamChgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `BIT_TB_TXPAUSE` reader - valid if BIT_HE_TB_PPSU_ONLY_EN = 1; select pause edca or not during TWT-SP; default is 1, pause. A TWT requesting STA should not transmit frames to the TWT responding STA outside of negotiated TWT SPs and should not transmit frames that are not contained within HE TB PPDUs to the TWT responding STA within trigger-enabled TWT SPs"]
pub type BitTbTxpauseR = crate::BitReader;
#[doc = "Field `BIT_TB_TXPAUSE` writer - valid if BIT_HE_TB_PPSU_ONLY_EN = 1; select pause edca or not during TWT-SP; default is 1, pause. A TWT requesting STA should not transmit frames to the TWT responding STA outside of negotiated TWT SPs and should not transmit frames that are not contained within HE TB PPDUs to the TWT responding STA within trigger-enabled TWT SPs"]
pub type BitTbTxpauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HE_TB_PPSU_ONLY_EN` reader - asserted if the TWT is trigger-enabled TWT"]
pub type BitHeTbPpsuOnlyEnR = crate::BitReader;
#[doc = "Field `BIT_HE_TB_PPSU_ONLY_EN` writer - asserted if the TWT is trigger-enabled TWT"]
pub type BitHeTbPpsuOnlyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TB_TIMER_CNT_INT` reader - the initial value of th_timer, unit: us"]
pub type BitTbTimerCntIntR = crate::FieldReader;
#[doc = "Field `BIT_TB_TIMER_CNT_INT` writer - the initial value of th_timer, unit: us"]
pub type BitTbTimerCntIntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 1: enable HW updating MUEDCATimer 0: disable HW updating MUEDCATimer"]
    #[inline(always)]
    pub fn bit_muedca_en(&self) -> BitMuedcaEnR {
        BitMuedcaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: enable HW updating MUEDCATimer after Tx HE TB PPDU 0: disable HW updating MUEDCATimer after Tx HE TB PPDU"]
    #[inline(always)]
    pub fn bit_set_muedcatimer_tf(&self) -> BitSetMuedcatimerTfR {
        BitSetMuedcatimerTfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: txcmd_beam_change TIE 0 0: first PPDU in TXOP will assert txcmd_beam_change"]
    #[inline(always)]
    pub fn bit_dis_upd_beam_chg(&self) -> BitDisUpdBeamChgR {
        BitDisUpdBeamChgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:24"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits >> 3) & 0x003f_ffff)
    }
    #[doc = "Bit 25 - valid if BIT_HE_TB_PPSU_ONLY_EN = 1; select pause edca or not during TWT-SP; default is 1, pause. A TWT requesting STA should not transmit frames to the TWT responding STA outside of negotiated TWT SPs and should not transmit frames that are not contained within HE TB PPDUs to the TWT responding STA within trigger-enabled TWT SPs"]
    #[inline(always)]
    pub fn bit_tb_txpause(&self) -> BitTbTxpauseR {
        BitTbTxpauseR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - asserted if the TWT is trigger-enabled TWT"]
    #[inline(always)]
    pub fn bit_he_tb_ppsu_only_en(&self) -> BitHeTbPpsuOnlyEnR {
        BitHeTbPpsuOnlyEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - the initial value of th_timer, unit: us"]
    #[inline(always)]
    pub fn bit_tb_timer_cnt_int(&self) -> BitTbTimerCntIntR {
        BitTbTimerCntIntR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable HW updating MUEDCATimer 0: disable HW updating MUEDCATimer"]
    #[inline(always)]
    pub fn bit_muedca_en(&mut self) -> BitMuedcaEnW<'_, RegAxTbMuedcaCtrlSpec> {
        BitMuedcaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: enable HW updating MUEDCATimer after Tx HE TB PPDU 0: disable HW updating MUEDCATimer after Tx HE TB PPDU"]
    #[inline(always)]
    pub fn bit_set_muedcatimer_tf(&mut self) -> BitSetMuedcatimerTfW<'_, RegAxTbMuedcaCtrlSpec> {
        BitSetMuedcatimerTfW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: txcmd_beam_change TIE 0 0: first PPDU in TXOP will assert txcmd_beam_change"]
    #[inline(always)]
    pub fn bit_dis_upd_beam_chg(&mut self) -> BitDisUpdBeamChgW<'_, RegAxTbMuedcaCtrlSpec> {
        BitDisUpdBeamChgW::new(self, 2)
    }
    #[doc = "Bits 3:24"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegAxTbMuedcaCtrlSpec> {
        DummyW::new(self, 3)
    }
    #[doc = "Bit 25 - valid if BIT_HE_TB_PPSU_ONLY_EN = 1; select pause edca or not during TWT-SP; default is 1, pause. A TWT requesting STA should not transmit frames to the TWT responding STA outside of negotiated TWT SPs and should not transmit frames that are not contained within HE TB PPDUs to the TWT responding STA within trigger-enabled TWT SPs"]
    #[inline(always)]
    pub fn bit_tb_txpause(&mut self) -> BitTbTxpauseW<'_, RegAxTbMuedcaCtrlSpec> {
        BitTbTxpauseW::new(self, 25)
    }
    #[doc = "Bit 26 - asserted if the TWT is trigger-enabled TWT"]
    #[inline(always)]
    pub fn bit_he_tb_ppsu_only_en(&mut self) -> BitHeTbPpsuOnlyEnW<'_, RegAxTbMuedcaCtrlSpec> {
        BitHeTbPpsuOnlyEnW::new(self, 26)
    }
    #[doc = "Bits 27:31 - the initial value of th_timer, unit: us"]
    #[inline(always)]
    pub fn bit_tb_timer_cnt_int(&mut self) -> BitTbTimerCntIntW<'_, RegAxTbMuedcaCtrlSpec> {
        BitTbTimerCntIntW::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_tb_muedca_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_tb_muedca_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxTbMuedcaCtrlSpec;
impl crate::RegisterSpec for RegAxTbMuedcaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_tb_muedca_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxTbMuedcaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_tb_muedca_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxTbMuedcaCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_TB_MUEDCA_CTRL to value 0xa200_0005"]
impl crate::Resettable for RegAxTbMuedcaCtrlSpec {
    const RESET_VALUE: u32 = 0xa200_0005;
}
