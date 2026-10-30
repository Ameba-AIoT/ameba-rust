#[doc = "Register `REG_AX_PWR_UL_CTRL6` reader"]
pub type R = crate::R<RegAxPwrUlCtrl6Spec>;
#[doc = "Register `REG_AX_PWR_UL_CTRL6` writer"]
pub type W = crate::W<RegAxPwrUlCtrl6Spec>;
#[doc = "Field `r_txpwr_force_rate` reader - (RTL_only) No use"]
pub type RTxpwrForceRateR = crate::FieldReader<u16>;
#[doc = "Field `r_txpwr_force_rate` writer - (RTL_only) No use"]
pub type RTxpwrForceRateW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_txpwr_force_rate_en` reader - (RTL_only) No use"]
pub type RTxpwrForceRateEnR = crate::BitReader;
#[doc = "Field `r_txpwr_force_rate_en` writer - (RTL_only) No use"]
pub type RTxpwrForceRateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_STA_pwr_ctrl_AP_TXPWR_lim_max` reader - (RTL_only) S(7.0 ) MAX=+40 Limit AP TXPWR"]
pub type RStaPwrCtrlApTxpwrLimMaxR = crate::FieldReader;
#[doc = "Field `r_STA_pwr_ctrl_AP_TXPWR_lim_max` writer - (RTL_only) S(7.0 ) MAX=+40 Limit AP TXPWR"]
pub type RStaPwrCtrlApTxpwrLimMaxW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_STA_pwr_ctrl_AP_TXPWR_lim_min` reader - (RTL_only) S(7.0 ) MIN=-20 Limit AP TXPWR"]
pub type RStaPwrCtrlApTxpwrLimMinR = crate::FieldReader;
#[doc = "Field `r_STA_pwr_ctrl_AP_TXPWR_lim_min` writer - (RTL_only) S(7.0 ) MIN=-20 Limit AP TXPWR"]
pub type RStaPwrCtrlApTxpwrLimMinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:11 - (RTL_only) No use"]
    #[inline(always)]
    pub fn r_txpwr_force_rate(&self) -> RTxpwrForceRateR {
        RTxpwrForceRateR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - (RTL_only) No use"]
    #[inline(always)]
    pub fn r_txpwr_force_rate_en(&self) -> RTxpwrForceRateEnR {
        RTxpwrForceRateEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - (RTL_only) S(7.0 ) MAX=+40 Limit AP TXPWR"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_ap_txpwr_lim_max(&self) -> RStaPwrCtrlApTxpwrLimMaxR {
        RStaPwrCtrlApTxpwrLimMaxR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - (RTL_only) S(7.0 ) MIN=-20 Limit AP TXPWR"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_ap_txpwr_lim_min(&self) -> RStaPwrCtrlApTxpwrLimMinR {
        RStaPwrCtrlApTxpwrLimMinR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - (RTL_only) No use"]
    #[inline(always)]
    pub fn r_txpwr_force_rate(&mut self) -> RTxpwrForceRateW<'_, RegAxPwrUlCtrl6Spec> {
        RTxpwrForceRateW::new(self, 0)
    }
    #[doc = "Bit 12 - (RTL_only) No use"]
    #[inline(always)]
    pub fn r_txpwr_force_rate_en(&mut self) -> RTxpwrForceRateEnW<'_, RegAxPwrUlCtrl6Spec> {
        RTxpwrForceRateEnW::new(self, 12)
    }
    #[doc = "Bits 16:22 - (RTL_only) S(7.0 ) MAX=+40 Limit AP TXPWR"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_ap_txpwr_lim_max(
        &mut self,
    ) -> RStaPwrCtrlApTxpwrLimMaxW<'_, RegAxPwrUlCtrl6Spec> {
        RStaPwrCtrlApTxpwrLimMaxW::new(self, 16)
    }
    #[doc = "Bits 24:30 - (RTL_only) S(7.0 ) MIN=-20 Limit AP TXPWR"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_ap_txpwr_lim_min(
        &mut self,
    ) -> RStaPwrCtrlApTxpwrLimMinW<'_, RegAxPwrUlCtrl6Spec> {
        RStaPwrCtrlApTxpwrLimMinW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ul_ctrl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ul_ctrl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrUlCtrl6Spec;
impl crate::RegisterSpec for RegAxPwrUlCtrl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ul_ctrl6::R`](R) reader structure"]
impl crate::Readable for RegAxPwrUlCtrl6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ul_ctrl6::W`](W) writer structure"]
impl crate::Writable for RegAxPwrUlCtrl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_UL_CTRL6 to value 0x6c28_0000"]
impl crate::Resettable for RegAxPwrUlCtrl6Spec {
    const RESET_VALUE: u32 = 0x6c28_0000;
}
