#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR14` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr14Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR14` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr14Spec>;
#[doc = "Field `r_force_pwrul_ap_tx_pwr_val` reader - "]
pub type RForcePwrulApTxPwrValR = crate::FieldReader;
#[doc = "Field `r_force_pwrul_ap_tx_pwr_val` writer - "]
pub type RForcePwrulApTxPwrValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_force_pwrcom_mcs_val` reader - "]
pub type RForcePwrcomMcsValR = crate::FieldReader;
#[doc = "Field `r_force_pwrcom_mcs_val` writer - "]
pub type RForcePwrcomMcsValW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `r_force_out_pwrcom_max_mcs_val` reader - "]
pub type RForceOutPwrcomMaxMcsValR = crate::FieldReader;
#[doc = "Field `r_force_out_pwrcom_max_mcs_val` writer - "]
pub type RForceOutPwrcomMaxMcsValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_force_pwrcom_ppdu_type_val` reader - "]
pub type RForcePwrcomPpduTypeValR = crate::FieldReader;
#[doc = "Field `r_force_pwrcom_ppdu_type_val` writer - "]
pub type RForcePwrcomPpduTypeValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_force_pwrcom_txsc_val` reader - "]
pub type RForcePwrcomTxscValR = crate::FieldReader;
#[doc = "Field `r_force_pwrcom_txsc_val` writer - "]
pub type RForcePwrcomTxscValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_force_macid_pwr_mode_val` reader - "]
pub type RForceMacidPwrModeValR = crate::FieldReader;
#[doc = "Field `r_force_macid_pwr_mode_val` writer - "]
pub type RForceMacidPwrModeValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_force_pwrant_nsts_val` reader - "]
pub type RForcePwrantNstsValR = crate::FieldReader;
#[doc = "Field `r_force_pwrant_nsts_val` writer - "]
pub type RForcePwrantNstsValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_force_pwrcom_ctrl_val` reader - "]
pub type RForcePwrcomCtrlValR = crate::FieldReader;
#[doc = "Field `r_force_pwrcom_ctrl_val` writer - "]
pub type RForcePwrcomCtrlValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn r_force_pwrul_ap_tx_pwr_val(&self) -> RForcePwrulApTxPwrValR {
        RForcePwrulApTxPwrValR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn r_force_pwrcom_mcs_val(&self) -> RForcePwrcomMcsValR {
        RForcePwrcomMcsValR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn r_force_out_pwrcom_max_mcs_val(&self) -> RForceOutPwrcomMaxMcsValR {
        RForceOutPwrcomMaxMcsValR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn r_force_pwrcom_ppdu_type_val(&self) -> RForcePwrcomPpduTypeValR {
        RForcePwrcomPpduTypeValR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn r_force_pwrcom_txsc_val(&self) -> RForcePwrcomTxscValR {
        RForcePwrcomTxscValR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn r_force_macid_pwr_mode_val(&self) -> RForceMacidPwrModeValR {
        RForceMacidPwrModeValR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn r_force_pwrant_nsts_val(&self) -> RForcePwrantNstsValR {
        RForcePwrantNstsValR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn r_force_pwrcom_ctrl_val(&self) -> RForcePwrcomCtrlValR {
        RForcePwrcomCtrlValR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn r_force_pwrul_ap_tx_pwr_val(
        &mut self,
    ) -> RForcePwrulApTxPwrValW<'_, RegAxPwrTcmWrapperCr14Spec> {
        RForcePwrulApTxPwrValW::new(self, 0)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn r_force_pwrcom_mcs_val(
        &mut self,
    ) -> RForcePwrcomMcsValW<'_, RegAxPwrTcmWrapperCr14Spec> {
        RForcePwrcomMcsValW::new(self, 6)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn r_force_out_pwrcom_max_mcs_val(
        &mut self,
    ) -> RForceOutPwrcomMaxMcsValW<'_, RegAxPwrTcmWrapperCr14Spec> {
        RForceOutPwrcomMaxMcsValW::new(self, 11)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn r_force_pwrcom_ppdu_type_val(
        &mut self,
    ) -> RForcePwrcomPpduTypeValW<'_, RegAxPwrTcmWrapperCr14Spec> {
        RForcePwrcomPpduTypeValW::new(self, 15)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn r_force_pwrcom_txsc_val(
        &mut self,
    ) -> RForcePwrcomTxscValW<'_, RegAxPwrTcmWrapperCr14Spec> {
        RForcePwrcomTxscValW::new(self, 19)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn r_force_macid_pwr_mode_val(
        &mut self,
    ) -> RForceMacidPwrModeValW<'_, RegAxPwrTcmWrapperCr14Spec> {
        RForceMacidPwrModeValW::new(self, 23)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn r_force_pwrant_nsts_val(
        &mut self,
    ) -> RForcePwrantNstsValW<'_, RegAxPwrTcmWrapperCr14Spec> {
        RForcePwrantNstsValW::new(self, 26)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn r_force_pwrcom_ctrl_val(
        &mut self,
    ) -> RForcePwrcomCtrlValW<'_, RegAxPwrTcmWrapperCr14Spec> {
        RForcePwrcomCtrlValW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr14Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr14::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr14Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr14::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR14 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr14Spec {}
