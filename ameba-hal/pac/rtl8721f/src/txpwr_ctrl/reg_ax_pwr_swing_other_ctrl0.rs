#[doc = "Register `REG_AX_PWR_SWING_OTHER_CTRL0` reader"]
pub type R = crate::R<RegAxPwrSwingOtherCtrl0Spec>;
#[doc = "Register `REG_AX_PWR_SWING_OTHER_CTRL0` writer"]
pub type W = crate::W<RegAxPwrSwingOtherCtrl0Spec>;
#[doc = "Field `r_dpd_by_rate_off_HT_th` reader - 0~8 cfir by rate_off mcs th"]
pub type RDpdByRateOffHtThR = crate::FieldReader;
#[doc = "Field `r_dpd_by_rate_off_HT_th` writer - 0~8 cfir by rate_off mcs th"]
pub type RDpdByRateOffHtThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_dpd_by_rate_off_HT_ov_th_val` reader - cfir by rate_off mcs ov th sel this"]
pub type RDpdByRateOffHtOvThValR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_HT_ov_th_val` writer - cfir by rate_off mcs ov th sel this"]
pub type RDpdByRateOffHtOvThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dpd_by_rate_off_HT_below_th_val` reader - cfir by rate_off mcs below th sel this"]
pub type RDpdByRateOffHtBelowThValR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_HT_below_th_val` writer - cfir by rate_off mcs below th sel this"]
pub type RDpdByRateOffHtBelowThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dpd_by_rate_off_VHT_th` reader - 0~12 cfir by rate_off mcs th"]
pub type RDpdByRateOffVhtThR = crate::FieldReader;
#[doc = "Field `r_dpd_by_rate_off_VHT_th` writer - 0~12 cfir by rate_off mcs th"]
pub type RDpdByRateOffVhtThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_dpd_by_rate_off_VHT_ov_th_val` reader - cfir by rate_off mcs ov th sel this"]
pub type RDpdByRateOffVhtOvThValR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_VHT_ov_th_val` writer - cfir by rate_off mcs ov th sel this"]
pub type RDpdByRateOffVhtOvThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dpd_by_rate_off_VHT_below_th_val` reader - cfir by rate_off mcs below th sel this"]
pub type RDpdByRateOffVhtBelowThValR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_VHT_below_th_val` writer - cfir by rate_off mcs below th sel this"]
pub type RDpdByRateOffVhtBelowThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dpd_by_rate_off_HE_th` reader - 0~12 cfir by rate_off mcs th"]
pub type RDpdByRateOffHeThR = crate::FieldReader;
#[doc = "Field `r_dpd_by_rate_off_HE_th` writer - 0~12 cfir by rate_off mcs th"]
pub type RDpdByRateOffHeThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_dpd_by_rate_off_HE_ov_th_val` reader - cfir by rate_off mcs ov th sel this"]
pub type RDpdByRateOffHeOvThValR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_HE_ov_th_val` writer - cfir by rate_off mcs ov th sel this"]
pub type RDpdByRateOffHeOvThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dpd_by_rate_off_HE_below_th_val` reader - cfir by rate_off mcs below th sel this"]
pub type RDpdByRateOffHeBelowThValR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_HE_below_th_val` writer - cfir by rate_off mcs below th sel this"]
pub type RDpdByRateOffHeBelowThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cfir_by_rate_off_cck_th` reader - 0~4 dpd by rate off mcs th"]
pub type RCfirByRateOffCckThR = crate::FieldReader;
#[doc = "Field `r_cfir_by_rate_off_cck_th` writer - 0~4 dpd by rate off mcs th"]
pub type RCfirByRateOffCckThW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_cfir_by_rate_off_cck_ov_th_val` reader - dpd by rate off mcs ov th sel this"]
pub type RCfirByRateOffCckOvThValR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_cck_ov_th_val` writer - dpd by rate off mcs ov th sel this"]
pub type RCfirByRateOffCckOvThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cfir_by_rate_off_cck_below_th_val` reader - dpd by rate off mcs below th sel this"]
pub type RCfirByRateOffCckBelowThValR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_cck_below_th_val` writer - dpd by rate off mcs below th sel this"]
pub type RCfirByRateOffCckBelowThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cfir_by_rate_off_legacy_th` reader - 4~8 dpd by rate off mcs ov th"]
pub type RCfirByRateOffLegacyThR = crate::FieldReader;
#[doc = "Field `r_cfir_by_rate_off_legacy_th` writer - 4~8 dpd by rate off mcs ov th"]
pub type RCfirByRateOffLegacyThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cfir_by_rate_off_legacy_ov_th_val` reader - dpd by rate off mcs ov th sel this"]
pub type RCfirByRateOffLegacyOvThValR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_legacy_ov_th_val` writer - dpd by rate off mcs ov th sel this"]
pub type RCfirByRateOffLegacyOvThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_cfir_by_rate_off_legacy_below_th_val` reader - dpd by rate off mcs below th sel this"]
pub type RCfirByRateOffLegacyBelowThValR = crate::BitReader;
#[doc = "Field `r_cfir_by_rate_off_legacy_below_th_val` writer - dpd by rate off mcs below th sel this"]
pub type RCfirByRateOffLegacyBelowThValW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 0~8 cfir by rate_off mcs th"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_ht_th(&self) -> RDpdByRateOffHtThR {
        RDpdByRateOffHtThR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - cfir by rate_off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_ht_ov_th_val(&self) -> RDpdByRateOffHtOvThValR {
        RDpdByRateOffHtOvThValR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - cfir by rate_off mcs below th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_ht_below_th_val(&self) -> RDpdByRateOffHtBelowThValR {
        RDpdByRateOffHtBelowThValR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - 0~12 cfir by rate_off mcs th"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_vht_th(&self) -> RDpdByRateOffVhtThR {
        RDpdByRateOffVhtThR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - cfir by rate_off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_vht_ov_th_val(&self) -> RDpdByRateOffVhtOvThValR {
        RDpdByRateOffVhtOvThValR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - cfir by rate_off mcs below th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_vht_below_th_val(&self) -> RDpdByRateOffVhtBelowThValR {
        RDpdByRateOffVhtBelowThValR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 0~12 cfir by rate_off mcs th"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_he_th(&self) -> RDpdByRateOffHeThR {
        RDpdByRateOffHeThR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - cfir by rate_off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_he_ov_th_val(&self) -> RDpdByRateOffHeOvThValR {
        RDpdByRateOffHeOvThValR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - cfir by rate_off mcs below th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_he_below_th_val(&self) -> RDpdByRateOffHeBelowThValR {
        RDpdByRateOffHeBelowThValR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - 0~4 dpd by rate off mcs th"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_cck_th(&self) -> RCfirByRateOffCckThR {
        RCfirByRateOffCckThR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - dpd by rate off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_cck_ov_th_val(&self) -> RCfirByRateOffCckOvThValR {
        RCfirByRateOffCckOvThValR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - dpd by rate off mcs below th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_cck_below_th_val(&self) -> RCfirByRateOffCckBelowThValR {
        RCfirByRateOffCckBelowThValR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - 4~8 dpd by rate off mcs ov th"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_legacy_th(&self) -> RCfirByRateOffLegacyThR {
        RCfirByRateOffLegacyThR::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - dpd by rate off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_legacy_ov_th_val(&self) -> RCfirByRateOffLegacyOvThValR {
        RCfirByRateOffLegacyOvThValR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - dpd by rate off mcs below th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_legacy_below_th_val(&self) -> RCfirByRateOffLegacyBelowThValR {
        RCfirByRateOffLegacyBelowThValR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0~8 cfir by rate_off mcs th"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_ht_th(
        &mut self,
    ) -> RDpdByRateOffHtThW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RDpdByRateOffHtThW::new(self, 0)
    }
    #[doc = "Bit 4 - cfir by rate_off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_ht_ov_th_val(
        &mut self,
    ) -> RDpdByRateOffHtOvThValW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RDpdByRateOffHtOvThValW::new(self, 4)
    }
    #[doc = "Bit 5 - cfir by rate_off mcs below th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_ht_below_th_val(
        &mut self,
    ) -> RDpdByRateOffHtBelowThValW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RDpdByRateOffHtBelowThValW::new(self, 5)
    }
    #[doc = "Bits 6:9 - 0~12 cfir by rate_off mcs th"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_vht_th(
        &mut self,
    ) -> RDpdByRateOffVhtThW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RDpdByRateOffVhtThW::new(self, 6)
    }
    #[doc = "Bit 10 - cfir by rate_off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_vht_ov_th_val(
        &mut self,
    ) -> RDpdByRateOffVhtOvThValW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RDpdByRateOffVhtOvThValW::new(self, 10)
    }
    #[doc = "Bit 11 - cfir by rate_off mcs below th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_vht_below_th_val(
        &mut self,
    ) -> RDpdByRateOffVhtBelowThValW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RDpdByRateOffVhtBelowThValW::new(self, 11)
    }
    #[doc = "Bits 12:15 - 0~12 cfir by rate_off mcs th"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_he_th(
        &mut self,
    ) -> RDpdByRateOffHeThW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RDpdByRateOffHeThW::new(self, 12)
    }
    #[doc = "Bit 16 - cfir by rate_off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_he_ov_th_val(
        &mut self,
    ) -> RDpdByRateOffHeOvThValW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RDpdByRateOffHeOvThValW::new(self, 16)
    }
    #[doc = "Bit 17 - cfir by rate_off mcs below th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_he_below_th_val(
        &mut self,
    ) -> RDpdByRateOffHeBelowThValW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RDpdByRateOffHeBelowThValW::new(self, 17)
    }
    #[doc = "Bits 18:20 - 0~4 dpd by rate off mcs th"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_cck_th(
        &mut self,
    ) -> RCfirByRateOffCckThW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RCfirByRateOffCckThW::new(self, 18)
    }
    #[doc = "Bit 21 - dpd by rate off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_cck_ov_th_val(
        &mut self,
    ) -> RCfirByRateOffCckOvThValW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RCfirByRateOffCckOvThValW::new(self, 21)
    }
    #[doc = "Bit 22 - dpd by rate off mcs below th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_cck_below_th_val(
        &mut self,
    ) -> RCfirByRateOffCckBelowThValW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RCfirByRateOffCckBelowThValW::new(self, 22)
    }
    #[doc = "Bits 23:26 - 4~8 dpd by rate off mcs ov th"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_legacy_th(
        &mut self,
    ) -> RCfirByRateOffLegacyThW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RCfirByRateOffLegacyThW::new(self, 23)
    }
    #[doc = "Bit 27 - dpd by rate off mcs ov th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_legacy_ov_th_val(
        &mut self,
    ) -> RCfirByRateOffLegacyOvThValW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RCfirByRateOffLegacyOvThValW::new(self, 27)
    }
    #[doc = "Bit 28 - dpd by rate off mcs below th sel this"]
    #[inline(always)]
    pub fn r_cfir_by_rate_off_legacy_below_th_val(
        &mut self,
    ) -> RCfirByRateOffLegacyBelowThValW<'_, RegAxPwrSwingOtherCtrl0Spec> {
        RCfirByRateOffLegacyBelowThValW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_swing_other_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_swing_other_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSwingOtherCtrl0Spec;
impl crate::RegisterSpec for RegAxPwrSwingOtherCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_swing_other_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSwingOtherCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_swing_other_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSwingOtherCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SWING_OTHER_CTRL0 to value 0"]
impl crate::Resettable for RegAxPwrSwingOtherCtrl0Spec {}
