#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR15` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr15Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR15` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr15Spec>;
#[doc = "Field `r_force_pwrru_grp_ntx_val` reader - "]
pub type RForcePwrruGrpNtxValR = crate::FieldReader;
#[doc = "Field `r_force_pwrru_grp_ntx_val` writer - "]
pub type RForcePwrruGrpNtxValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_force_pwrcom_cbw_idx_val` reader - "]
pub type RForcePwrcomCbwIdxValR = crate::FieldReader;
#[doc = "Field `r_force_pwrcom_cbw_idx_val` writer - "]
pub type RForcePwrcomCbwIdxValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_force_pwrcom_dbw_idx_val` reader - "]
pub type RForcePwrcomDbwIdxValR = crate::FieldReader;
#[doc = "Field `r_force_pwrcom_dbw_idx_val` writer - "]
pub type RForcePwrcomDbwIdxValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_force_pwrcom_precoding_mode_idx_val` reader - "]
pub type RForcePwrcomPrecodingModeIdxValR = crate::FieldReader;
#[doc = "Field `r_force_pwrcom_precoding_mode_idx_val` writer - "]
pub type RForcePwrcomPrecodingModeIdxValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_ant_sel_dbg_en` reader - "]
pub type RAntSelDbgEnR = crate::BitReader;
#[doc = "Field `r_ant_sel_dbg_en` writer - "]
pub type RAntSelDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_clr_ant_set` reader - "]
pub type RClrAntSetR = crate::BitReader;
#[doc = "Field `r_clr_ant_set` writer - "]
pub type RClrAntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_cca_th_allow_on` reader - "]
pub type RForceCcaThAllowOnR = crate::BitReader;
#[doc = "Field `r_force_cca_th_allow_on` writer - "]
pub type RForceCcaThAllowOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_cca_th_allow_val` reader - "]
pub type RForceCcaThAllowValR = crate::BitReader;
#[doc = "Field `r_force_cca_th_allow_val` writer - "]
pub type RForceCcaThAllowValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_cca_th_allow_on` reader - "]
pub type RForceMacidCcaThAllowOnR = crate::BitReader;
#[doc = "Field `r_force_macid_cca_th_allow_on` writer - "]
pub type RForceMacidCcaThAllowOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_cca_th_allow_val` reader - "]
pub type RForceMacidCcaThAllowValR = crate::BitReader;
#[doc = "Field `r_force_macid_cca_th_allow_val` writer - "]
pub type RForceMacidCcaThAllowValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_mumimo_en_on` reader - "]
pub type RForceMacidMumimoEnOnR = crate::BitReader;
#[doc = "Field `r_force_macid_mumimo_en_on` writer - "]
pub type RForceMacidMumimoEnOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_mumimo_en_val` reader - "]
pub type RForceMacidMumimoEnValR = crate::BitReader;
#[doc = "Field `r_force_macid_mumimo_en_val` writer - "]
pub type RForceMacidMumimoEnValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_ndp_en_on` reader - "]
pub type RForceMacidNdpEnOnR = crate::BitReader;
#[doc = "Field `r_force_macid_ndp_en_on` writer - "]
pub type RForceMacidNdpEnOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_ndp_en_val` reader - "]
pub type RForceMacidNdpEnValR = crate::BitReader;
#[doc = "Field `r_force_macid_ndp_en_val` writer - "]
pub type RForceMacidNdpEnValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_norm_en_on` reader - "]
pub type RForceMacidNormEnOnR = crate::BitReader;
#[doc = "Field `r_force_macid_norm_en_on` writer - "]
pub type RForceMacidNormEnOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_norm_en_val` reader - "]
pub type RForceMacidNormEnValR = crate::BitReader;
#[doc = "Field `r_force_macid_norm_en_val` writer - "]
pub type RForceMacidNormEnValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_pwr_mode_on` reader - "]
pub type RForceMacidPwrModeOnR = crate::BitReader;
#[doc = "Field `r_force_macid_pwr_mode_on` writer - "]
pub type RForceMacidPwrModeOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_resp_en_on` reader - "]
pub type RForceMacidRespEnOnR = crate::BitReader;
#[doc = "Field `r_force_macid_resp_en_on` writer - "]
pub type RForceMacidRespEnOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_resp_en_val` reader - "]
pub type RForceMacidRespEnValR = crate::BitReader;
#[doc = "Field `r_force_macid_resp_en_val` writer - "]
pub type RForceMacidRespEnValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_txbf_en_on` reader - "]
pub type RForceMacidTxbfEnOnR = crate::BitReader;
#[doc = "Field `r_force_macid_txbf_en_on` writer - "]
pub type RForceMacidTxbfEnOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_txbf_en_val` reader - "]
pub type RForceMacidTxbfEnValR = crate::BitReader;
#[doc = "Field `r_force_macid_txbf_en_val` writer - "]
pub type RForceMacidTxbfEnValW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn r_force_pwrru_grp_ntx_val(&self) -> RForcePwrruGrpNtxValR {
        RForcePwrruGrpNtxValR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn r_force_pwrcom_cbw_idx_val(&self) -> RForcePwrcomCbwIdxValR {
        RForcePwrcomCbwIdxValR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn r_force_pwrcom_dbw_idx_val(&self) -> RForcePwrcomDbwIdxValR {
        RForcePwrcomDbwIdxValR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn r_force_pwrcom_precoding_mode_idx_val(&self) -> RForcePwrcomPrecodingModeIdxValR {
        RForcePwrcomPrecodingModeIdxValR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn r_ant_sel_dbg_en(&self) -> RAntSelDbgEnR {
        RAntSelDbgEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn r_clr_ant_set(&self) -> RClrAntSetR {
        RClrAntSetR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn r_force_cca_th_allow_on(&self) -> RForceCcaThAllowOnR {
        RForceCcaThAllowOnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn r_force_cca_th_allow_val(&self) -> RForceCcaThAllowValR {
        RForceCcaThAllowValR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn r_force_macid_cca_th_allow_on(&self) -> RForceMacidCcaThAllowOnR {
        RForceMacidCcaThAllowOnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn r_force_macid_cca_th_allow_val(&self) -> RForceMacidCcaThAllowValR {
        RForceMacidCcaThAllowValR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn r_force_macid_mumimo_en_on(&self) -> RForceMacidMumimoEnOnR {
        RForceMacidMumimoEnOnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn r_force_macid_mumimo_en_val(&self) -> RForceMacidMumimoEnValR {
        RForceMacidMumimoEnValR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn r_force_macid_ndp_en_on(&self) -> RForceMacidNdpEnOnR {
        RForceMacidNdpEnOnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn r_force_macid_ndp_en_val(&self) -> RForceMacidNdpEnValR {
        RForceMacidNdpEnValR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn r_force_macid_norm_en_on(&self) -> RForceMacidNormEnOnR {
        RForceMacidNormEnOnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn r_force_macid_norm_en_val(&self) -> RForceMacidNormEnValR {
        RForceMacidNormEnValR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn r_force_macid_pwr_mode_on(&self) -> RForceMacidPwrModeOnR {
        RForceMacidPwrModeOnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn r_force_macid_resp_en_on(&self) -> RForceMacidRespEnOnR {
        RForceMacidRespEnOnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn r_force_macid_resp_en_val(&self) -> RForceMacidRespEnValR {
        RForceMacidRespEnValR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn r_force_macid_txbf_en_on(&self) -> RForceMacidTxbfEnOnR {
        RForceMacidTxbfEnOnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn r_force_macid_txbf_en_val(&self) -> RForceMacidTxbfEnValR {
        RForceMacidTxbfEnValR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn r_force_pwrru_grp_ntx_val(
        &mut self,
    ) -> RForcePwrruGrpNtxValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForcePwrruGrpNtxValW::new(self, 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn r_force_pwrcom_cbw_idx_val(
        &mut self,
    ) -> RForcePwrcomCbwIdxValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForcePwrcomCbwIdxValW::new(self, 3)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn r_force_pwrcom_dbw_idx_val(
        &mut self,
    ) -> RForcePwrcomDbwIdxValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForcePwrcomDbwIdxValW::new(self, 5)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn r_force_pwrcom_precoding_mode_idx_val(
        &mut self,
    ) -> RForcePwrcomPrecodingModeIdxValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForcePwrcomPrecodingModeIdxValW::new(self, 7)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn r_ant_sel_dbg_en(&mut self) -> RAntSelDbgEnW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RAntSelDbgEnW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn r_clr_ant_set(&mut self) -> RClrAntSetW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RClrAntSetW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn r_force_cca_th_allow_on(
        &mut self,
    ) -> RForceCcaThAllowOnW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceCcaThAllowOnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn r_force_cca_th_allow_val(
        &mut self,
    ) -> RForceCcaThAllowValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceCcaThAllowValW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn r_force_macid_cca_th_allow_on(
        &mut self,
    ) -> RForceMacidCcaThAllowOnW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidCcaThAllowOnW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn r_force_macid_cca_th_allow_val(
        &mut self,
    ) -> RForceMacidCcaThAllowValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidCcaThAllowValW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn r_force_macid_mumimo_en_on(
        &mut self,
    ) -> RForceMacidMumimoEnOnW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidMumimoEnOnW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn r_force_macid_mumimo_en_val(
        &mut self,
    ) -> RForceMacidMumimoEnValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidMumimoEnValW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn r_force_macid_ndp_en_on(
        &mut self,
    ) -> RForceMacidNdpEnOnW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidNdpEnOnW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn r_force_macid_ndp_en_val(
        &mut self,
    ) -> RForceMacidNdpEnValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidNdpEnValW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn r_force_macid_norm_en_on(
        &mut self,
    ) -> RForceMacidNormEnOnW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidNormEnOnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn r_force_macid_norm_en_val(
        &mut self,
    ) -> RForceMacidNormEnValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidNormEnValW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn r_force_macid_pwr_mode_on(
        &mut self,
    ) -> RForceMacidPwrModeOnW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidPwrModeOnW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn r_force_macid_resp_en_on(
        &mut self,
    ) -> RForceMacidRespEnOnW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidRespEnOnW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn r_force_macid_resp_en_val(
        &mut self,
    ) -> RForceMacidRespEnValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidRespEnValW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn r_force_macid_txbf_en_on(
        &mut self,
    ) -> RForceMacidTxbfEnOnW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidTxbfEnOnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn r_force_macid_txbf_en_val(
        &mut self,
    ) -> RForceMacidTxbfEnValW<'_, RegAxPwrTcmWrapperCr15Spec> {
        RForceMacidTxbfEnValW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr15Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr15::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr15Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr15::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR15 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr15Spec {}
