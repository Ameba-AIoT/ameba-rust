#[doc = "Register `REG_AX_PWR_UL_CTRL4` reader"]
pub type R = crate::R<RegAxPwrUlCtrl4Spec>;
#[doc = "Register `REG_AX_PWR_UL_CTRL4` writer"]
pub type W = crate::W<RegAxPwrUlCtrl4Spec>;
#[doc = "Field `r_STA_pwr_ctrl_RSSI_Target_lim_min` reader - (RTL_only) S(8.0) Min=-110 Limit RSSI_Target"]
pub type RStaPwrCtrlRssiTargetLimMinR = crate::FieldReader;
#[doc = "Field `r_STA_pwr_ctrl_RSSI_Target_lim_min` writer - (RTL_only) S(8.0) Min=-110 Limit RSSI_Target"]
pub type RStaPwrCtrlRssiTargetLimMinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_STA_pwr_ctrl_RSSI_Target_lim_max` reader - (RTL_only) S(8.0) Max:0 Limit RSSI_Target"]
pub type RStaPwrCtrlRssiTargetLimMaxR = crate::FieldReader;
#[doc = "Field `r_STA_pwr_ctrl_RSSI_Target_lim_max` writer - (RTL_only) S(8.0) Max:0 Limit RSSI_Target"]
pub type RStaPwrCtrlRssiTargetLimMaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_STA_pwr_ctrl_RPL_lim_min` reader - (RTL_only) S(10.2) Min=-110 (-440) Limit RPL"]
pub type RStaPwrCtrlRplLimMinR = crate::FieldReader<u16>;
#[doc = "Field `r_STA_pwr_ctrl_RPL_lim_min` writer - (RTL_only) S(10.2) Min=-110 (-440) Limit RPL"]
pub type RStaPwrCtrlRplLimMinW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - (RTL_only) S(8.0) Min=-110 Limit RSSI_Target"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_rssi_target_lim_min(&self) -> RStaPwrCtrlRssiTargetLimMinR {
        RStaPwrCtrlRssiTargetLimMinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - (RTL_only) S(8.0) Max:0 Limit RSSI_Target"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_rssi_target_lim_max(&self) -> RStaPwrCtrlRssiTargetLimMaxR {
        RStaPwrCtrlRssiTargetLimMaxR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - (RTL_only) S(10.2) Min=-110 (-440) Limit RPL"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_rpl_lim_min(&self) -> RStaPwrCtrlRplLimMinR {
        RStaPwrCtrlRplLimMinR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - (RTL_only) S(8.0) Min=-110 Limit RSSI_Target"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_rssi_target_lim_min(
        &mut self,
    ) -> RStaPwrCtrlRssiTargetLimMinW<'_, RegAxPwrUlCtrl4Spec> {
        RStaPwrCtrlRssiTargetLimMinW::new(self, 0)
    }
    #[doc = "Bits 8:15 - (RTL_only) S(8.0) Max:0 Limit RSSI_Target"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_rssi_target_lim_max(
        &mut self,
    ) -> RStaPwrCtrlRssiTargetLimMaxW<'_, RegAxPwrUlCtrl4Spec> {
        RStaPwrCtrlRssiTargetLimMaxW::new(self, 8)
    }
    #[doc = "Bits 16:25 - (RTL_only) S(10.2) Min=-110 (-440) Limit RPL"]
    #[inline(always)]
    pub fn r_sta_pwr_ctrl_rpl_lim_min(&mut self) -> RStaPwrCtrlRplLimMinW<'_, RegAxPwrUlCtrl4Spec> {
        RStaPwrCtrlRplLimMinW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ul_ctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ul_ctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrUlCtrl4Spec;
impl crate::RegisterSpec for RegAxPwrUlCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ul_ctrl4::R`](R) reader structure"]
impl crate::Readable for RegAxPwrUlCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ul_ctrl4::W`](W) writer structure"]
impl crate::Writable for RegAxPwrUlCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_UL_CTRL4 to value 0x0248_0092"]
impl crate::Resettable for RegAxPwrUlCtrl4Spec {
    const RESET_VALUE: u32 = 0x0248_0092;
}
