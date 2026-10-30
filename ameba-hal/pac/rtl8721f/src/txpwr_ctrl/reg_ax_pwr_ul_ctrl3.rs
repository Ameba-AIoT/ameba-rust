#[doc = "Register `REG_AX_PWR_UL_CTRL3` reader"]
pub type R = crate::R<RegAxPwrUlCtrl3Spec>;
#[doc = "Register `REG_AX_PWR_UL_CTRL3` writer"]
pub type W = crate::W<RegAxPwrUlCtrl3Spec>;
#[doc = "Field `r_STA_pwr_ctrl_PWRmax_lim_min` reader - (RTL_only) S(9.2) Limit pwr from norm resp"]
pub type RStaPwrCtrlPwrmaxLimMinR = crate::FieldReader<u16>;
#[doc = "Field `r_STA_pwr_ctrl_PWRmax_lim_min` writer - (RTL_only) S(9.2) Limit pwr from norm resp"]
pub type RStaPwrCtrlPwrmaxLimMinW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_STA_pwr_ctrl_PWRmax_lim_max` reader - (RTL_only) S(9.2) Limit pwr from norm resp"]
pub type RStaPwrCtrlPwrmaxLimMaxR = crate::FieldReader<u16>;
#[doc = "Field `r_STA_pwr_ctrl_PWRmax_lim_max` writer - (RTL_only) S(9.2) Limit pwr from norm resp"]
pub type RStaPwrCtrlPwrmaxLimMaxW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_N_tx_divide2_off` reader - (RTL_only) 1:force off N_tx/2"]
pub type RForceNTxDivide2OffR = crate::BitReader;
#[doc = "Field `r_force_N_tx_divide2_off` writer - (RTL_only) 1:force off N_tx/2"]
pub type RForceNTxDivide2OffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_PL_upper_en_equl_N_tx_divide2` reader - (RTL_only) 1:PL_upper_en = is N_tx/2"]
pub type RForcePlUpperEnEqulNTxDivide2R = crate::BitReader;
#[doc = "Field `r_force_PL_upper_en_equl_N_tx_divide2` writer - (RTL_only) 1:PL_upper_en = is N_tx/2"]
pub type RForcePlUpperEnEqulNTxDivide2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_TF_rdy_TxBF_force_off` reader - (RTL_only) Jorden Sta_UL doesnt have TxBF"]
pub type RTfRdyTxBfForceOffR = crate::BitReader;
#[doc = "Field `r_TF_rdy_TxBF_force_off` writer - (RTL_only) Jorden Sta_UL doesnt have TxBF"]
pub type RTfRdyTxBfForceOffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - (RTL_only) S(9.2) Limit pwr from norm resp"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_pwrmax_lim_min(&self) -> RStaPwrCtrlPwrmaxLimMinR {
        RStaPwrCtrlPwrmaxLimMinR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 11:19 - (RTL_only) S(9.2) Limit pwr from norm resp"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_pwrmax_lim_max(&self) -> RStaPwrCtrlPwrmaxLimMaxR {
        RStaPwrCtrlPwrmaxLimMaxR::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bit 20 - (RTL_only) 1:force off N_tx/2"]
    #[inline(always)]
    pub fn r_force_n_tx_divide2_off(&self) -> RForceNTxDivide2OffR {
        RForceNTxDivide2OffR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - (RTL_only) 1:PL_upper_en = is N_tx/2"]
    #[inline(always)]
    pub fn r_force_pl_upper_en_equl_n_tx_divide2(&self) -> RForcePlUpperEnEqulNTxDivide2R {
        RForcePlUpperEnEqulNTxDivide2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - (RTL_only) Jorden Sta_UL doesnt have TxBF"]
    #[inline(always)]
    pub fn r_tf_rdy_tx_bf_force_off(&self) -> RTfRdyTxBfForceOffR {
        RTfRdyTxBfForceOffR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - (RTL_only) S(9.2) Limit pwr from norm resp"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_pwrmax_lim_min(
        &mut self,
    ) -> RStaPwrCtrlPwrmaxLimMinW<'_, RegAxPwrUlCtrl3Spec> {
        RStaPwrCtrlPwrmaxLimMinW::new(self, 0)
    }
    #[doc = "Bits 11:19 - (RTL_only) S(9.2) Limit pwr from norm resp"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_pwrmax_lim_max(
        &mut self,
    ) -> RStaPwrCtrlPwrmaxLimMaxW<'_, RegAxPwrUlCtrl3Spec> {
        RStaPwrCtrlPwrmaxLimMaxW::new(self, 11)
    }
    #[doc = "Bit 20 - (RTL_only) 1:force off N_tx/2"]
    #[inline(always)]
    pub fn r_force_n_tx_divide2_off(&mut self) -> RForceNTxDivide2OffW<'_, RegAxPwrUlCtrl3Spec> {
        RForceNTxDivide2OffW::new(self, 20)
    }
    #[doc = "Bit 21 - (RTL_only) 1:PL_upper_en = is N_tx/2"]
    #[inline(always)]
    pub fn r_force_pl_upper_en_equl_n_tx_divide2(
        &mut self,
    ) -> RForcePlUpperEnEqulNTxDivide2W<'_, RegAxPwrUlCtrl3Spec> {
        RForcePlUpperEnEqulNTxDivide2W::new(self, 21)
    }
    #[doc = "Bit 22 - (RTL_only) Jorden Sta_UL doesnt have TxBF"]
    #[inline(always)]
    pub fn r_tf_rdy_tx_bf_force_off(&mut self) -> RTfRdyTxBfForceOffW<'_, RegAxPwrUlCtrl3Spec> {
        RTfRdyTxBfForceOffW::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ul_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ul_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrUlCtrl3Spec;
impl crate::RegisterSpec for RegAxPwrUlCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ul_ctrl3::R`](R) reader structure"]
impl crate::Readable for RegAxPwrUlCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ul_ctrl3::W`](W) writer structure"]
impl crate::Writable for RegAxPwrUlCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_UL_CTRL3 to value 0x0047_f900"]
impl crate::Resettable for RegAxPwrUlCtrl3Spec {
    const RESET_VALUE: u32 = 0x0047_f900;
}
