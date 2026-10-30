#[doc = "Register `REG_AX_PWR_SWING_OTHER_CTRL1` reader"]
pub type R = crate::R<RegAxPwrSwingOtherCtrl1Spec>;
#[doc = "Register `REG_AX_PWR_SWING_OTHER_CTRL1` writer"]
pub type W = crate::W<RegAxPwrSwingOtherCtrl1Spec>;
#[doc = "Field `r_cfir_by_rate_off_HT_th` reader - 0~8 dpd by rate off mcs ov th"]
pub type RCfirByRateOffHtThR = crate::FieldReader;
#[doc = "Field `r_cfir_by_rate_off_HT_th` writer - 0~8 dpd by rate off mcs ov th"]
pub type RCfirByRateOffHtThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cfir_by_rate_off_HT_ov_th_val` reader - dpd by rate off mcs ov th sel this"]
pub type RCfirByRateOffHtOvThValR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_HT_ov_th_val` writer - dpd by rate off mcs ov th sel this"]
pub type RCfirByRateOffHtOvThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cfir_by_rate_off_HT_below_th_val` reader - dpd by rate off mcs below th sel this"]
pub type RCfirByRateOffHtBelowThValR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_HT_below_th_val` writer - dpd by rate off mcs below th sel this"]
pub type RCfirByRateOffHtBelowThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cfir_by_rate_off_VHT_th` reader - 0~12 dpd by rate off mcs ov th"]
pub type RCfirByRateOffVhtThR = crate::FieldReader;
#[doc = "Field `r_cfir_by_rate_off_VHT_th` writer - 0~12 dpd by rate off mcs ov th"]
pub type RCfirByRateOffVhtThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cfir_by_rate_off_VHT_ov_th_val` reader - dpd by rate off mcs ov th sel this"]
pub type RCfirByRateOffVhtOvThValR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_VHT_ov_th_val` writer - dpd by rate off mcs ov th sel this"]
pub type RCfirByRateOffVhtOvThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cfir_by_rate_off_VHT_below_th_val` reader - dpd by rate off mcs below th sel this"]
pub type RCfirByRateOffVhtBelowThValR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_VHT_below_th_val` writer - dpd by rate off mcs below th sel this"]
pub type RCfirByRateOffVhtBelowThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cfir_by_rate_off_HE_th` reader - 0~12 dpd by rate off mcs ov th"]
pub type RCfirByRateOffHeThR = crate::FieldReader;
#[doc = "Field `r_cfir_by_rate_off_HE_th` writer - 0~12 dpd by rate off mcs ov th"]
pub type RCfirByRateOffHeThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cfir_by_rate_off_HE_ov_th_val` reader - dpd by rate off mcs ov th sel this"]
pub type RCfirByRateOffHeOvThValR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_HE_ov_th_val` writer - dpd by rate off mcs ov th sel this"]
pub type RCfirByRateOffHeOvThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cfir_by_rate_off_HE_below_th_val` reader - dpd by rate off mcs below th sel this"]
pub type RCfirByRateOffHeBelowThValR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_HE_below_th_val` writer - dpd by rate off mcs below th sel this"]
pub type RCfirByRateOffHeBelowThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txbbswing_txbias_SR_dont_apply` reader - (RTL_only) 1:Disable at SR"]
pub type RTxbbswingTxbiasSrDontApplyR = crate::BitReader;
#[doc = "Field `r_txbbswing_txbias_SR_dont_apply` writer - (RTL_only) 1:Disable at SR"]
pub type RTxbbswingTxbiasSrDontApplyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dpd_by_rate_off_SR_dont_apply` reader - (RTL_only) 1:Disable at SR"]
pub type RDpdByRateOffSrDontApplyR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_SR_dont_apply` writer - (RTL_only) 1:Disable at SR"]
pub type RDpdByRateOffSrDontApplyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cfir_by_rate_off_SR_dont_apply` reader - (RTL_only) 1:Disable at SR"]
pub type RCfirByRateOffSrDontApplyR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_SR_dont_apply` writer - (RTL_only) 1:Disable at SR"]
pub type RCfirByRateOffSrDontApplyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 0~8 dpd by rate off mcs ov th"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_ht_th(&self) -> RCfirByRateOffHtThR {
        RCfirByRateOffHtThR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - dpd by rate off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_ht_ov_th_val(&self) -> RCfirByRateOffHtOvThValR {
        RCfirByRateOffHtOvThValR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - dpd by rate off mcs below th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_ht_below_th_val(&self) -> RCfirByRateOffHtBelowThValR {
        RCfirByRateOffHtBelowThValR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - 0~12 dpd by rate off mcs ov th"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_vht_th(&self) -> RCfirByRateOffVhtThR {
        RCfirByRateOffVhtThR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - dpd by rate off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_vht_ov_th_val(&self) -> RCfirByRateOffVhtOvThValR {
        RCfirByRateOffVhtOvThValR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - dpd by rate off mcs below th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_vht_below_th_val(&self) -> RCfirByRateOffVhtBelowThValR {
        RCfirByRateOffVhtBelowThValR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 0~12 dpd by rate off mcs ov th"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_he_th(&self) -> RCfirByRateOffHeThR {
        RCfirByRateOffHeThR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - dpd by rate off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_he_ov_th_val(&self) -> RCfirByRateOffHeOvThValR {
        RCfirByRateOffHeOvThValR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - dpd by rate off mcs below th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_he_below_th_val(&self) -> RCfirByRateOffHeBelowThValR {
        RCfirByRateOffHeBelowThValR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 28 - (RTL_only) 1:Disable at SR"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_sr_dont_apply(&self) -> RTxbbswingTxbiasSrDontApplyR {
        RTxbbswingTxbiasSrDontApplyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - (RTL_only) 1:Disable at SR"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_sr_dont_apply(&self) -> RDpdByRateOffSrDontApplyR {
        RDpdByRateOffSrDontApplyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - (RTL_only) 1:Disable at SR"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_sr_dont_apply(&self) -> RCfirByRateOffSrDontApplyR {
        RCfirByRateOffSrDontApplyR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0~8 dpd by rate off mcs ov th"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_ht_th(
        &mut self,
    ) -> RCfirByRateOffHtThW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RCfirByRateOffHtThW::new(self, 0)
    }
    #[doc = "Bit 4 - dpd by rate off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_ht_ov_th_val(
        &mut self,
    ) -> RCfirByRateOffHtOvThValW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RCfirByRateOffHtOvThValW::new(self, 4)
    }
    #[doc = "Bit 5 - dpd by rate off mcs below th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_ht_below_th_val(
        &mut self,
    ) -> RCfirByRateOffHtBelowThValW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RCfirByRateOffHtBelowThValW::new(self, 5)
    }
    #[doc = "Bits 6:9 - 0~12 dpd by rate off mcs ov th"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_vht_th(
        &mut self,
    ) -> RCfirByRateOffVhtThW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RCfirByRateOffVhtThW::new(self, 6)
    }
    #[doc = "Bit 10 - dpd by rate off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_vht_ov_th_val(
        &mut self,
    ) -> RCfirByRateOffVhtOvThValW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RCfirByRateOffVhtOvThValW::new(self, 10)
    }
    #[doc = "Bit 11 - dpd by rate off mcs below th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_vht_below_th_val(
        &mut self,
    ) -> RCfirByRateOffVhtBelowThValW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RCfirByRateOffVhtBelowThValW::new(self, 11)
    }
    #[doc = "Bits 12:15 - 0~12 dpd by rate off mcs ov th"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_he_th(
        &mut self,
    ) -> RCfirByRateOffHeThW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RCfirByRateOffHeThW::new(self, 12)
    }
    #[doc = "Bit 16 - dpd by rate off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_he_ov_th_val(
        &mut self,
    ) -> RCfirByRateOffHeOvThValW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RCfirByRateOffHeOvThValW::new(self, 16)
    }
    #[doc = "Bit 17 - dpd by rate off mcs below th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_he_below_th_val(
        &mut self,
    ) -> RCfirByRateOffHeBelowThValW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RCfirByRateOffHeBelowThValW::new(self, 17)
    }
    #[doc = "Bit 28 - (RTL_only) 1:Disable at SR"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_sr_dont_apply(
        &mut self,
    ) -> RTxbbswingTxbiasSrDontApplyW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RTxbbswingTxbiasSrDontApplyW::new(self, 28)
    }
    #[doc = "Bit 29 - (RTL_only) 1:Disable at SR"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_sr_dont_apply(
        &mut self,
    ) -> RDpdByRateOffSrDontApplyW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RDpdByRateOffSrDontApplyW::new(self, 29)
    }
    #[doc = "Bit 30 - (RTL_only) 1:Disable at SR"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_sr_dont_apply(
        &mut self,
    ) -> RCfirByRateOffSrDontApplyW<'_, RegAxPwrSwingOtherCtrl1Spec> {
        RCfirByRateOffSrDontApplyW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_swing_other_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_swing_other_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSwingOtherCtrl1Spec;
impl crate::RegisterSpec for RegAxPwrSwingOtherCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_swing_other_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSwingOtherCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_swing_other_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSwingOtherCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SWING_OTHER_CTRL1 to value 0"]
impl crate::Resettable for RegAxPwrSwingOtherCtrl1Spec {}
