#[doc = "Register `REG_AX_PWR_UL_CTRL5` reader"]
pub type R = crate::R<RegAxPwrUlCtrl5Spec>;
#[doc = "Register `REG_AX_PWR_UL_CTRL5` writer"]
pub type W = crate::W<RegAxPwrUlCtrl5Spec>;
#[doc = "Field `r_STA_pwr_ctrl_RPL_lim_max` reader - (RTL_only) S(10.2) Max:0 Limit RPL"]
pub type RStaPwrCtrlRplLimMaxR = crate::FieldReader<u16>;
#[doc = "Field `r_STA_pwr_ctrl_RPL_lim_max` writer - (RTL_only) S(10.2) Max:0 Limit RPL"]
pub type RStaPwrCtrlRplLimMaxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `r_STA_pwr_ctrl_dbg_en` reader - (RTL_only) 0: pwr saving"]
pub type RStaPwrCtrlDbgEnR = crate::BitReader;
#[doc = "Field `r_STA_pwr_ctrl_dbg_en` writer - (RTL_only) 0: pwr saving"]
pub type RStaPwrCtrlDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_TxCtrl_info_rdy_force_on` reader - (RTL_only) 1:force off norm resp trigger"]
pub type RTxCtrlInfoRdyForceOnR = crate::BitReader;
#[doc = "Field `r_TxCtrl_info_rdy_force_on` writer - (RTL_only) 1:force off norm resp trigger"]
pub type RTxCtrlInfoRdyForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_TF_rdy_force_on` reader - (RTL_only) 1:force off TF trigger"]
pub type RTfRdyForceOnR = crate::BitReader;
#[doc = "Field `r_TF_rdy_force_on` writer - (RTL_only) 1:force off TF trigger"]
pub type RTfRdyForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_SR_txpwr_rate_rdy_force_on` reader - (RTL_only) 1:force off SR PD flow trigger"]
pub type RSrTxpwrRateRdyForceOnR = crate::BitReader;
#[doc = "Field `r_SR_txpwr_rate_rdy_force_on` writer - (RTL_only) 1:force off SR PD flow trigger"]
pub type RSrTxpwrRateRdyForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_SR_txpwr_pd_rdy_force_on` reader - (RTL_only) 1:force off SR MCS flow trigger"]
pub type RSrTxpwrPdRdyForceOnR = crate::BitReader;
#[doc = "Field `r_SR_txpwr_pd_rdy_force_on` writer - (RTL_only) 1:force off SR MCS flow trigger"]
pub type RSrTxpwrPdRdyForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_SR_txpwr_resp_rdy_force_on` reader - (RTL_only) 1:force off SR resp trigger"]
pub type RSrTxpwrRespRdyForceOnR = crate::BitReader;
#[doc = "Field `r_SR_txpwr_resp_rdy_force_on` writer - (RTL_only) 1:force off SR resp trigger"]
pub type RSrTxpwrRespRdyForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_PL_upper_en_on` reader - (RTL_only) 1 : PL_upper_en = r_force_PL_upper_en_val"]
pub type RForcePlUpperEnOnR = crate::BitReader;
#[doc = "Field `r_force_PL_upper_en_on` writer - (RTL_only) 1 : PL_upper_en = r_force_PL_upper_en_val"]
pub type RForcePlUpperEnOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_PL_upper_en_val` reader - "]
pub type RForcePlUpperEnValR = crate::BitReader;
#[doc = "Field `r_force_PL_upper_en_val` writer - "]
pub type RForcePlUpperEnValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_TxCtrl_info_rdy` reader - (RTL_only) 1: trig when r_TxCtrl_info_rdy_force_on"]
pub type RTxCtrlInfoRdyR = crate::BitReader;
#[doc = "Field `r_TxCtrl_info_rdy` writer - (RTL_only) 1: trig when r_TxCtrl_info_rdy_force_on"]
pub type RTxCtrlInfoRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_TF_rdy` reader - (RTL_only) 1: trig when r_TF_rdy_force_on"]
pub type RTfRdyR = crate::BitReader;
#[doc = "Field `r_TF_rdy` writer - (RTL_only) 1: trig when r_TF_rdy_force_on"]
pub type RTfRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_SR_txpwr_rate_rdy` reader - (RTL_only) 1: trig when r_SR_txpwr_rate_rdy_force_on"]
pub type RSrTxpwrRateRdyR = crate::BitReader;
#[doc = "Field `r_SR_txpwr_rate_rdy` writer - (RTL_only) 1: trig when r_SR_txpwr_rate_rdy_force_on"]
pub type RSrTxpwrRateRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_SR_txpwr_pd_rdy` reader - (RTL_only) 1: trig when r_SR_txpwr_pd_rdy_force_on"]
pub type RSrTxpwrPdRdyR = crate::BitReader;
#[doc = "Field `r_SR_txpwr_pd_rdy` writer - (RTL_only) 1: trig when r_SR_txpwr_pd_rdy_force_on"]
pub type RSrTxpwrPdRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_SR_txpwr_resp_rdy` reader - (RTL_only) 1: trig when r_SR_txpwr_resp_rdy_force_on"]
pub type RSrTxpwrRespRdyR = crate::BitReader;
#[doc = "Field `r_SR_txpwr_resp_rdy` writer - (RTL_only) 1: trig when r_SR_txpwr_resp_rdy_force_on"]
pub type RSrTxpwrRespRdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - (RTL_only) S(10.2) Max:0 Limit RPL"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_rpl_lim_max(&self) -> RStaPwrCtrlRplLimMaxR {
        RStaPwrCtrlRplLimMaxR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - (RTL_only) 0: pwr saving"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_dbg_en(&self) -> RStaPwrCtrlDbgEnR {
        RStaPwrCtrlDbgEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - (RTL_only) 1:force off norm resp trigger"]
    #[inline(always)]
    pub fn r_tx_ctrl_info_rdy_force_on(&self) -> RTxCtrlInfoRdyForceOnR {
        RTxCtrlInfoRdyForceOnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - (RTL_only) 1:force off TF trigger"]
    #[inline(always)]
    pub fn r_tf_rdy_force_on(&self) -> RTfRdyForceOnR {
        RTfRdyForceOnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - (RTL_only) 1:force off SR PD flow trigger"]
    #[inline(always)]
    pub fn r_sr_txpwr_rate_rdy_force_on(&self) -> RSrTxpwrRateRdyForceOnR {
        RSrTxpwrRateRdyForceOnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - (RTL_only) 1:force off SR MCS flow trigger"]
    #[inline(always)]
    pub fn r_sr_txpwr_pd_rdy_force_on(&self) -> RSrTxpwrPdRdyForceOnR {
        RSrTxpwrPdRdyForceOnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - (RTL_only) 1:force off SR resp trigger"]
    #[inline(always)]
    pub fn r_sr_txpwr_resp_rdy_force_on(&self) -> RSrTxpwrRespRdyForceOnR {
        RSrTxpwrRespRdyForceOnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 22 - (RTL_only) 1 : PL_upper_en = r_force_PL_upper_en_val"]
    #[inline(always)]
    pub fn r_force_pl_upper_en_on(&self) -> RForcePlUpperEnOnR {
        RForcePlUpperEnOnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn r_force_pl_upper_en_val(&self) -> RForcePlUpperEnValR {
        RForcePlUpperEnValR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - (RTL_only) 1: trig when r_TxCtrl_info_rdy_force_on"]
    #[inline(always)]
    pub fn r_tx_ctrl_info_rdy(&self) -> RTxCtrlInfoRdyR {
        RTxCtrlInfoRdyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - (RTL_only) 1: trig when r_TF_rdy_force_on"]
    #[inline(always)]
    pub fn r_tf_rdy(&self) -> RTfRdyR {
        RTfRdyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - (RTL_only) 1: trig when r_SR_txpwr_rate_rdy_force_on"]
    #[inline(always)]
    pub fn r_sr_txpwr_rate_rdy(&self) -> RSrTxpwrRateRdyR {
        RSrTxpwrRateRdyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - (RTL_only) 1: trig when r_SR_txpwr_pd_rdy_force_on"]
    #[inline(always)]
    pub fn r_sr_txpwr_pd_rdy(&self) -> RSrTxpwrPdRdyR {
        RSrTxpwrPdRdyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - (RTL_only) 1: trig when r_SR_txpwr_resp_rdy_force_on"]
    #[inline(always)]
    pub fn r_sr_txpwr_resp_rdy(&self) -> RSrTxpwrRespRdyR {
        RSrTxpwrRespRdyR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - (RTL_only) S(10.2) Max:0 Limit RPL"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_rpl_lim_max(&mut self) -> RStaPwrCtrlRplLimMaxW<'_, RegAxPwrUlCtrl5Spec> {
        RStaPwrCtrlRplLimMaxW::new(self, 0)
    }
    #[doc = "Bit 10 - (RTL_only) 0: pwr saving"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_dbg_en(&mut self) -> RStaPwrCtrlDbgEnW<'_, RegAxPwrUlCtrl5Spec> {
        RStaPwrCtrlDbgEnW::new(self, 10)
    }
    #[doc = "Bit 11 - (RTL_only) 1:force off norm resp trigger"]
    #[inline(always)]
    pub fn r_tx_ctrl_info_rdy_force_on(
        &mut self,
    ) -> RTxCtrlInfoRdyForceOnW<'_, RegAxPwrUlCtrl5Spec> {
        RTxCtrlInfoRdyForceOnW::new(self, 11)
    }
    #[doc = "Bit 12 - (RTL_only) 1:force off TF trigger"]
    #[inline(always)]
    pub fn r_tf_rdy_force_on(&mut self) -> RTfRdyForceOnW<'_, RegAxPwrUlCtrl5Spec> {
        RTfRdyForceOnW::new(self, 12)
    }
    #[doc = "Bit 13 - (RTL_only) 1:force off SR PD flow trigger"]
    #[inline(always)]
    pub fn r_sr_txpwr_rate_rdy_force_on(
        &mut self,
    ) -> RSrTxpwrRateRdyForceOnW<'_, RegAxPwrUlCtrl5Spec> {
        RSrTxpwrRateRdyForceOnW::new(self, 13)
    }
    #[doc = "Bit 14 - (RTL_only) 1:force off SR MCS flow trigger"]
    #[inline(always)]
    pub fn r_sr_txpwr_pd_rdy_force_on(&mut self) -> RSrTxpwrPdRdyForceOnW<'_, RegAxPwrUlCtrl5Spec> {
        RSrTxpwrPdRdyForceOnW::new(self, 14)
    }
    #[doc = "Bit 15 - (RTL_only) 1:force off SR resp trigger"]
    #[inline(always)]
    pub fn r_sr_txpwr_resp_rdy_force_on(
        &mut self,
    ) -> RSrTxpwrRespRdyForceOnW<'_, RegAxPwrUlCtrl5Spec> {
        RSrTxpwrRespRdyForceOnW::new(self, 15)
    }
    #[doc = "Bit 22 - (RTL_only) 1 : PL_upper_en = r_force_PL_upper_en_val"]
    #[inline(always)]
    pub fn r_force_pl_upper_en_on(&mut self) -> RForcePlUpperEnOnW<'_, RegAxPwrUlCtrl5Spec> {
        RForcePlUpperEnOnW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn r_force_pl_upper_en_val(&mut self) -> RForcePlUpperEnValW<'_, RegAxPwrUlCtrl5Spec> {
        RForcePlUpperEnValW::new(self, 23)
    }
    #[doc = "Bit 24 - (RTL_only) 1: trig when r_TxCtrl_info_rdy_force_on"]
    #[inline(always)]
    pub fn r_tx_ctrl_info_rdy(&mut self) -> RTxCtrlInfoRdyW<'_, RegAxPwrUlCtrl5Spec> {
        RTxCtrlInfoRdyW::new(self, 24)
    }
    #[doc = "Bit 25 - (RTL_only) 1: trig when r_TF_rdy_force_on"]
    #[inline(always)]
    pub fn r_tf_rdy(&mut self) -> RTfRdyW<'_, RegAxPwrUlCtrl5Spec> {
        RTfRdyW::new(self, 25)
    }
    #[doc = "Bit 26 - (RTL_only) 1: trig when r_SR_txpwr_rate_rdy_force_on"]
    #[inline(always)]
    pub fn r_sr_txpwr_rate_rdy(&mut self) -> RSrTxpwrRateRdyW<'_, RegAxPwrUlCtrl5Spec> {
        RSrTxpwrRateRdyW::new(self, 26)
    }
    #[doc = "Bit 27 - (RTL_only) 1: trig when r_SR_txpwr_pd_rdy_force_on"]
    #[inline(always)]
    pub fn r_sr_txpwr_pd_rdy(&mut self) -> RSrTxpwrPdRdyW<'_, RegAxPwrUlCtrl5Spec> {
        RSrTxpwrPdRdyW::new(self, 27)
    }
    #[doc = "Bit 28 - (RTL_only) 1: trig when r_SR_txpwr_resp_rdy_force_on"]
    #[inline(always)]
    pub fn r_sr_txpwr_resp_rdy(&mut self) -> RSrTxpwrRespRdyW<'_, RegAxPwrUlCtrl5Spec> {
        RSrTxpwrRespRdyW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ul_ctrl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ul_ctrl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrUlCtrl5Spec;
impl crate::RegisterSpec for RegAxPwrUlCtrl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ul_ctrl5::R`](R) reader structure"]
impl crate::Readable for RegAxPwrUlCtrl5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ul_ctrl5::W`](W) writer structure"]
impl crate::Writable for RegAxPwrUlCtrl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_UL_CTRL5 to value 0"]
impl crate::Resettable for RegAxPwrUlCtrl5Spec {}
