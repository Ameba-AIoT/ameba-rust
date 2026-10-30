#[doc = "Register `REG_AX_PWR_TSSI_TABLE20` reader"]
pub type R = crate::R<RegAxPwrTssiTable20Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE20` writer"]
pub type W = crate::W<RegAxPwrTssiTable20Spec>;
#[doc = "Field `r_f_is_su_stbc_en` reader - "]
pub type RFIsSuStbcEnR = crate::BitReader;
#[doc = "Field `r_f_is_su_stbc_en` writer - "]
pub type RFIsSuStbcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_legacy_bandedge_ind_en` reader - "]
pub type RFLegacyBandedgeIndEnR = crate::BitReader;
#[doc = "Field `r_f_legacy_bandedge_ind_en` writer - "]
pub type RFLegacyBandedgeIndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_legacy_bandedge_ind_val` reader - "]
pub type RFLegacyBandedgeIndValR = crate::BitReader;
#[doc = "Field `r_f_legacy_bandedge_ind_val` writer - "]
pub type RFLegacyBandedgeIndValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_legacy_bypass_cbw_fltr_en` reader - "]
pub type RFLegacyBypassCbwFltrEnR = crate::BitReader;
#[doc = "Field `r_f_legacy_bypass_cbw_fltr_en` writer - "]
pub type RFLegacyBypassCbwFltrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_legacy_bypass_dbw_fltr_en` reader - "]
pub type RFLegacyBypassDbwFltrEnR = crate::BitReader;
#[doc = "Field `r_f_legacy_bypass_dbw_fltr_en` writer - "]
pub type RFLegacyBypassDbwFltrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_legacy_diff_out_en` reader - "]
pub type RFLegacyDiffOutEnR = crate::BitReader;
#[doc = "Field `r_f_legacy_diff_out_en` writer - "]
pub type RFLegacyDiffOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_legacy_ofst_out_en` reader - "]
pub type RFLegacyOfstOutEnR = crate::BitReader;
#[doc = "Field `r_f_legacy_ofst_out_en` writer - "]
pub type RFLegacyOfstOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_precoding_mode_idx_en` reader - "]
pub type RFPrecodingModeIdxEnR = crate::BitReader;
#[doc = "Field `r_f_precoding_mode_idx_en` writer - "]
pub type RFPrecodingModeIdxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_rate_idx_en` reader - "]
pub type RFRateIdxEnR = crate::BitReader;
#[doc = "Field `r_f_rate_idx_en` writer - "]
pub type RFRateIdxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_tssi_ctrl_fast_mode_idx` reader - "]
pub type RFTssiCtrlFastModeIdxR = crate::BitReader;
#[doc = "Field `r_f_tssi_ctrl_fast_mode_idx` writer - "]
pub type RFTssiCtrlFastModeIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_tssi_ctrl_fast_mode_idx_en` reader - "]
pub type RFTssiCtrlFastModeIdxEnR = crate::BitReader;
#[doc = "Field `r_f_tssi_ctrl_fast_mode_idx_en` writer - "]
pub type RFTssiCtrlFastModeIdxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_tssi_ctrl_out_en` reader - "]
pub type RFTssiCtrlOutEnR = crate::BitReader;
#[doc = "Field `r_f_tssi_ctrl_out_en` writer - "]
pub type RFTssiCtrlOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_txsc_en` reader - "]
pub type RFTxscEnR = crate::BitReader;
#[doc = "Field `r_f_txsc_en` writer - "]
pub type RFTxscEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_vht_bandedge_ind_en` reader - "]
pub type RFVhtBandedgeIndEnR = crate::BitReader;
#[doc = "Field `r_f_vht_bandedge_ind_en` writer - "]
pub type RFVhtBandedgeIndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_vht_bandedge_ind_val` reader - "]
pub type RFVhtBandedgeIndValR = crate::BitReader;
#[doc = "Field `r_f_vht_bandedge_ind_val` writer - "]
pub type RFVhtBandedgeIndValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_vht_bypass_cbw_fltr_en` reader - "]
pub type RFVhtBypassCbwFltrEnR = crate::BitReader;
#[doc = "Field `r_f_vht_bypass_cbw_fltr_en` writer - "]
pub type RFVhtBypassCbwFltrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_vht_bypass_dbw_fltr_en` reader - "]
pub type RFVhtBypassDbwFltrEnR = crate::BitReader;
#[doc = "Field `r_f_vht_bypass_dbw_fltr_en` writer - "]
pub type RFVhtBypassDbwFltrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_vht_diff_out_en` reader - "]
pub type RFVhtDiffOutEnR = crate::BitReader;
#[doc = "Field `r_f_vht_diff_out_en` writer - "]
pub type RFVhtDiffOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_vht_ofst_out_en` reader - "]
pub type RFVhtOfstOutEnR = crate::BitReader;
#[doc = "Field `r_f_vht_ofst_out_en` writer - "]
pub type RFVhtOfstOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_hetb_pwr_msr_mode` reader - "]
pub type RHetbPwrMsrModeR = crate::BitReader;
#[doc = "Field `r_hetb_pwr_msr_mode` writer - "]
pub type RHetbPwrMsrModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_tssi_ctrl_mac_clr` reader - "]
pub type RTssiCtrlMacClrR = crate::BitReader;
#[doc = "Field `r_tssi_ctrl_mac_clr` writer - "]
pub type RTssiCtrlMacClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_tssi_ctrl_mac_dbg_dis` reader - "]
pub type RTssiCtrlMacDbgDisR = crate::BitReader;
#[doc = "Field `r_tssi_ctrl_mac_dbg_dis` writer - "]
pub type RTssiCtrlMacDbgDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_tssi_ctrl_mac_use_normal_rate_idx_only` reader - "]
pub type RTssiCtrlMacUseNormalRateIdxOnlyR = crate::BitReader;
#[doc = "Field `r_tssi_ctrl_mac_use_normal_rate_idx_only` writer - "]
pub type RTssiCtrlMacUseNormalRateIdxOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn r_f_is_su_stbc_en(&self) -> RFIsSuStbcEnR {
        RFIsSuStbcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn r_f_legacy_bandedge_ind_en(&self) -> RFLegacyBandedgeIndEnR {
        RFLegacyBandedgeIndEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn r_f_legacy_bandedge_ind_val(&self) -> RFLegacyBandedgeIndValR {
        RFLegacyBandedgeIndValR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_f_legacy_bypass_cbw_fltr_en(&self) -> RFLegacyBypassCbwFltrEnR {
        RFLegacyBypassCbwFltrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn r_f_legacy_bypass_dbw_fltr_en(&self) -> RFLegacyBypassDbwFltrEnR {
        RFLegacyBypassDbwFltrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn r_f_legacy_diff_out_en(&self) -> RFLegacyDiffOutEnR {
        RFLegacyDiffOutEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn r_f_legacy_ofst_out_en(&self) -> RFLegacyOfstOutEnR {
        RFLegacyOfstOutEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn r_f_precoding_mode_idx_en(&self) -> RFPrecodingModeIdxEnR {
        RFPrecodingModeIdxEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn r_f_rate_idx_en(&self) -> RFRateIdxEnR {
        RFRateIdxEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn r_f_tssi_ctrl_fast_mode_idx(&self) -> RFTssiCtrlFastModeIdxR {
        RFTssiCtrlFastModeIdxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn r_f_tssi_ctrl_fast_mode_idx_en(&self) -> RFTssiCtrlFastModeIdxEnR {
        RFTssiCtrlFastModeIdxEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn r_f_tssi_ctrl_out_en(&self) -> RFTssiCtrlOutEnR {
        RFTssiCtrlOutEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn r_f_txsc_en(&self) -> RFTxscEnR {
        RFTxscEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn r_f_vht_bandedge_ind_en(&self) -> RFVhtBandedgeIndEnR {
        RFVhtBandedgeIndEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn r_f_vht_bandedge_ind_val(&self) -> RFVhtBandedgeIndValR {
        RFVhtBandedgeIndValR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn r_f_vht_bypass_cbw_fltr_en(&self) -> RFVhtBypassCbwFltrEnR {
        RFVhtBypassCbwFltrEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn r_f_vht_bypass_dbw_fltr_en(&self) -> RFVhtBypassDbwFltrEnR {
        RFVhtBypassDbwFltrEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn r_f_vht_diff_out_en(&self) -> RFVhtDiffOutEnR {
        RFVhtDiffOutEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn r_f_vht_ofst_out_en(&self) -> RFVhtOfstOutEnR {
        RFVhtOfstOutEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn r_hetb_pwr_msr_mode(&self) -> RHetbPwrMsrModeR {
        RHetbPwrMsrModeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_clr(&self) -> RTssiCtrlMacClrR {
        RTssiCtrlMacClrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_dbg_dis(&self) -> RTssiCtrlMacDbgDisR {
        RTssiCtrlMacDbgDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_use_normal_rate_idx_only(&self) -> RTssiCtrlMacUseNormalRateIdxOnlyR {
        RTssiCtrlMacUseNormalRateIdxOnlyR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn r_f_is_su_stbc_en(&mut self) -> RFIsSuStbcEnW<'_, RegAxPwrTssiTable20Spec> {
        RFIsSuStbcEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn r_f_legacy_bandedge_ind_en(
        &mut self,
    ) -> RFLegacyBandedgeIndEnW<'_, RegAxPwrTssiTable20Spec> {
        RFLegacyBandedgeIndEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn r_f_legacy_bandedge_ind_val(
        &mut self,
    ) -> RFLegacyBandedgeIndValW<'_, RegAxPwrTssiTable20Spec> {
        RFLegacyBandedgeIndValW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_f_legacy_bypass_cbw_fltr_en(
        &mut self,
    ) -> RFLegacyBypassCbwFltrEnW<'_, RegAxPwrTssiTable20Spec> {
        RFLegacyBypassCbwFltrEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn r_f_legacy_bypass_dbw_fltr_en(
        &mut self,
    ) -> RFLegacyBypassDbwFltrEnW<'_, RegAxPwrTssiTable20Spec> {
        RFLegacyBypassDbwFltrEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn r_f_legacy_diff_out_en(&mut self) -> RFLegacyDiffOutEnW<'_, RegAxPwrTssiTable20Spec> {
        RFLegacyDiffOutEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn r_f_legacy_ofst_out_en(&mut self) -> RFLegacyOfstOutEnW<'_, RegAxPwrTssiTable20Spec> {
        RFLegacyOfstOutEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn r_f_precoding_mode_idx_en(
        &mut self,
    ) -> RFPrecodingModeIdxEnW<'_, RegAxPwrTssiTable20Spec> {
        RFPrecodingModeIdxEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn r_f_rate_idx_en(&mut self) -> RFRateIdxEnW<'_, RegAxPwrTssiTable20Spec> {
        RFRateIdxEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn r_f_tssi_ctrl_fast_mode_idx(
        &mut self,
    ) -> RFTssiCtrlFastModeIdxW<'_, RegAxPwrTssiTable20Spec> {
        RFTssiCtrlFastModeIdxW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn r_f_tssi_ctrl_fast_mode_idx_en(
        &mut self,
    ) -> RFTssiCtrlFastModeIdxEnW<'_, RegAxPwrTssiTable20Spec> {
        RFTssiCtrlFastModeIdxEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn r_f_tssi_ctrl_out_en(&mut self) -> RFTssiCtrlOutEnW<'_, RegAxPwrTssiTable20Spec> {
        RFTssiCtrlOutEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn r_f_txsc_en(&mut self) -> RFTxscEnW<'_, RegAxPwrTssiTable20Spec> {
        RFTxscEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn r_f_vht_bandedge_ind_en(&mut self) -> RFVhtBandedgeIndEnW<'_, RegAxPwrTssiTable20Spec> {
        RFVhtBandedgeIndEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn r_f_vht_bandedge_ind_val(
        &mut self,
    ) -> RFVhtBandedgeIndValW<'_, RegAxPwrTssiTable20Spec> {
        RFVhtBandedgeIndValW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn r_f_vht_bypass_cbw_fltr_en(
        &mut self,
    ) -> RFVhtBypassCbwFltrEnW<'_, RegAxPwrTssiTable20Spec> {
        RFVhtBypassCbwFltrEnW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn r_f_vht_bypass_dbw_fltr_en(
        &mut self,
    ) -> RFVhtBypassDbwFltrEnW<'_, RegAxPwrTssiTable20Spec> {
        RFVhtBypassDbwFltrEnW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn r_f_vht_diff_out_en(&mut self) -> RFVhtDiffOutEnW<'_, RegAxPwrTssiTable20Spec> {
        RFVhtDiffOutEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn r_f_vht_ofst_out_en(&mut self) -> RFVhtOfstOutEnW<'_, RegAxPwrTssiTable20Spec> {
        RFVhtOfstOutEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn r_hetb_pwr_msr_mode(&mut self) -> RHetbPwrMsrModeW<'_, RegAxPwrTssiTable20Spec> {
        RHetbPwrMsrModeW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_clr(&mut self) -> RTssiCtrlMacClrW<'_, RegAxPwrTssiTable20Spec> {
        RTssiCtrlMacClrW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_dbg_dis(&mut self) -> RTssiCtrlMacDbgDisW<'_, RegAxPwrTssiTable20Spec> {
        RTssiCtrlMacDbgDisW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_use_normal_rate_idx_only(
        &mut self,
    ) -> RTssiCtrlMacUseNormalRateIdxOnlyW<'_, RegAxPwrTssiTable20Spec> {
        RTssiCtrlMacUseNormalRateIdxOnlyW::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable20Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table20::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable20Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table20::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE20 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable20Spec {}
