#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR17` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr17Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR17` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr17Spec>;
#[doc = "Field `r_force_pwrul_rpl_up_en_val` reader - "]
pub type RForcePwrulRplUpEnValR = crate::BitReader;
#[doc = "Field `r_force_pwrul_rpl_up_en_val` writer - "]
pub type RForcePwrulRplUpEnValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_pwrul_rpl_up_on` reader - "]
pub type RForcePwrulRplUpOnR = crate::BitReader;
#[doc = "Field `r_force_pwrul_rpl_up_on` writer - "]
pub type RForcePwrulRplUpOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_pwrul_tar_rssi_on` reader - "]
pub type RForcePwrulTarRssiOnR = crate::BitReader;
#[doc = "Field `r_force_pwrul_tar_rssi_on` writer - "]
pub type RForcePwrulTarRssiOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_heru_ch20_with_data_from_f2p_dis` reader - "]
pub type RHeruCh20WithDataFromF2pDisR = crate::BitReader;
#[doc = "Field `r_heru_ch20_with_data_from_f2p_dis` writer - "]
pub type RHeruCh20WithDataFromF2pDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_heru_mask_dis` reader - "]
pub type RHeruMaskDisR = crate::BitReader;
#[doc = "Field `r_heru_mask_dis` writer - "]
pub type RHeruMaskDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_mumimo_apply_pwrlimbf_en` reader - "]
pub type RMumimoApplyPwrlimbfEnR = crate::BitReader;
#[doc = "Field `r_mumimo_apply_pwrlimbf_en` writer - "]
pub type RMumimoApplyPwrlimbfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pwrru_ch20_with_data_hang_proof_en` reader - "]
pub type RPwrruCh20WithDataHangProofEnR = crate::BitReader;
#[doc = "Field `r_pwrru_ch20_with_data_hang_proof_en` writer - "]
pub type RPwrruCh20WithDataHangProofEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_sr_mcs_mask_dis` reader - "]
pub type RSrMcsMaskDisR = crate::BitReader;
#[doc = "Field `r_sr_mcs_mask_dis` writer - "]
pub type RSrMcsMaskDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_tssi_ctrl_mac_pwrcom_ch20_with_data_bit_inv` reader - "]
pub type RTssiCtrlMacPwrcomCh20WithDataBitInvR = crate::BitReader;
#[doc = "Field `r_tssi_ctrl_mac_pwrcom_ch20_with_data_bit_inv` writer - "]
pub type RTssiCtrlMacPwrcomCh20WithDataBitInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_tssi_ctrl_mac_pwrcom_ch20_with_data_bit_rev` reader - "]
pub type RTssiCtrlMacPwrcomCh20WithDataBitRevR = crate::BitReader;
#[doc = "Field `r_tssi_ctrl_mac_pwrcom_ch20_with_data_bit_rev` writer - "]
pub type RTssiCtrlMacPwrcomCh20WithDataBitRevW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn r_force_pwrul_rpl_up_en_val(&self) -> RForcePwrulRplUpEnValR {
        RForcePwrulRplUpEnValR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn r_force_pwrul_rpl_up_on(&self) -> RForcePwrulRplUpOnR {
        RForcePwrulRplUpOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn r_force_pwrul_tar_rssi_on(&self) -> RForcePwrulTarRssiOnR {
        RForcePwrulTarRssiOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_heru_ch20_with_data_from_f2p_dis(&self) -> RHeruCh20WithDataFromF2pDisR {
        RHeruCh20WithDataFromF2pDisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn r_heru_mask_dis(&self) -> RHeruMaskDisR {
        RHeruMaskDisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn r_mumimo_apply_pwrlimbf_en(&self) -> RMumimoApplyPwrlimbfEnR {
        RMumimoApplyPwrlimbfEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn r_pwrru_ch20_with_data_hang_proof_en(&self) -> RPwrruCh20WithDataHangProofEnR {
        RPwrruCh20WithDataHangProofEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn r_sr_mcs_mask_dis(&self) -> RSrMcsMaskDisR {
        RSrMcsMaskDisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_pwrcom_ch20_with_data_bit_inv(
        &self,
    ) -> RTssiCtrlMacPwrcomCh20WithDataBitInvR {
        RTssiCtrlMacPwrcomCh20WithDataBitInvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_pwrcom_ch20_with_data_bit_rev(
        &self,
    ) -> RTssiCtrlMacPwrcomCh20WithDataBitRevR {
        RTssiCtrlMacPwrcomCh20WithDataBitRevR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn r_force_pwrul_rpl_up_en_val(
        &mut self,
    ) -> RForcePwrulRplUpEnValW<'_, RegAxPwrTcmWrapperCr17Spec> {
        RForcePwrulRplUpEnValW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn r_force_pwrul_rpl_up_on(
        &mut self,
    ) -> RForcePwrulRplUpOnW<'_, RegAxPwrTcmWrapperCr17Spec> {
        RForcePwrulRplUpOnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn r_force_pwrul_tar_rssi_on(
        &mut self,
    ) -> RForcePwrulTarRssiOnW<'_, RegAxPwrTcmWrapperCr17Spec> {
        RForcePwrulTarRssiOnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_heru_ch20_with_data_from_f2p_dis(
        &mut self,
    ) -> RHeruCh20WithDataFromF2pDisW<'_, RegAxPwrTcmWrapperCr17Spec> {
        RHeruCh20WithDataFromF2pDisW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn r_heru_mask_dis(&mut self) -> RHeruMaskDisW<'_, RegAxPwrTcmWrapperCr17Spec> {
        RHeruMaskDisW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn r_mumimo_apply_pwrlimbf_en(
        &mut self,
    ) -> RMumimoApplyPwrlimbfEnW<'_, RegAxPwrTcmWrapperCr17Spec> {
        RMumimoApplyPwrlimbfEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn r_pwrru_ch20_with_data_hang_proof_en(
        &mut self,
    ) -> RPwrruCh20WithDataHangProofEnW<'_, RegAxPwrTcmWrapperCr17Spec> {
        RPwrruCh20WithDataHangProofEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn r_sr_mcs_mask_dis(&mut self) -> RSrMcsMaskDisW<'_, RegAxPwrTcmWrapperCr17Spec> {
        RSrMcsMaskDisW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_pwrcom_ch20_with_data_bit_inv(
        &mut self,
    ) -> RTssiCtrlMacPwrcomCh20WithDataBitInvW<'_, RegAxPwrTcmWrapperCr17Spec> {
        RTssiCtrlMacPwrcomCh20WithDataBitInvW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_pwrcom_ch20_with_data_bit_rev(
        &mut self,
    ) -> RTssiCtrlMacPwrcomCh20WithDataBitRevW<'_, RegAxPwrTcmWrapperCr17Spec> {
        RTssiCtrlMacPwrcomCh20WithDataBitRevW::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr17Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr17::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr17Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr17::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR17 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr17Spec {}
