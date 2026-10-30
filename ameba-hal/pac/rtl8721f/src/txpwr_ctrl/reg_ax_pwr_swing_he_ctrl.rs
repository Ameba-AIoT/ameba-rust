#[doc = "Register `REG_AX_PWR_SWING_HE_CTRL` reader"]
pub type R = crate::R<RegAxPwrSwingHeCtrlSpec>;
#[doc = "Register `REG_AX_PWR_SWING_HE_CTRL` writer"]
pub type W = crate::W<RegAxPwrSwingHeCtrlSpec>;
#[doc = "Field `r_txbias_VHT_below_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasVhtBelowThValR = crate::FieldReader;
#[doc = "Field `r_txbias_VHT_below_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasVhtBelowThValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbbswing_txbias_HE_th` reader - 0~12 Bbswing txbias mcs th"]
pub type RTxbbswingTxbiasHeThR = crate::FieldReader;
#[doc = "Field `r_txbbswing_txbias_HE_th` writer - 0~12 Bbswing txbias mcs th"]
pub type RTxbbswingTxbiasHeThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbbswing_HE_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbbswingHeOvThValR = crate::FieldReader;
#[doc = "Field `r_txbbswing_HE_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbbswingHeOvThValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbbswing_HE_below_th_val` reader - Bbswing ,txbias mcs below th sel this"]
pub type RTxbbswingHeBelowThValR = crate::FieldReader;
#[doc = "Field `r_txbbswing_HE_below_th_val` writer - Bbswing ,txbias mcs below th sel this"]
pub type RTxbbswingHeBelowThValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_HE_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasHeOvThValR = crate::FieldReader;
#[doc = "Field `r_txbias_HE_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasHeOvThValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_HE_below_th_val` reader - Bbswing ,txbias mcs below th sel this"]
pub type RTxbiasHeBelowThValR = crate::FieldReader;
#[doc = "Field `r_txbias_HE_below_th_val` writer - Bbswing ,txbias mcs below th sel this"]
pub type RTxbiasHeBelowThValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_dpd_by_rate_off_cck_th` reader - 0~4 Bbswing txbias mcs th"]
pub type RDpdByRateOffCckThR = crate::FieldReader;
#[doc = "Field `r_dpd_by_rate_off_cck_th` writer - 0~4 Bbswing txbias mcs th"]
pub type RDpdByRateOffCckThW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_dpd_by_rate_off_cck_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RDpdByRateOffCckOvThValR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_cck_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RDpdByRateOffCckOvThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dpd_by_rate_off_cck_below_th_val` reader - Bbswing ,txbias mcs below th sel this"]
pub type RDpdByRateOffCckBelowThValR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_cck_below_th_val` writer - Bbswing ,txbias mcs below th sel this"]
pub type RDpdByRateOffCckBelowThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dpd_by_rate_off_legacy_th` reader - 4~8 Bbswing txbias mcs th"]
pub type RDpdByRateOffLegacyThR = crate::FieldReader;
#[doc = "Field `r_dpd_by_rate_off_legacy_th` writer - 4~8 Bbswing txbias mcs th"]
pub type RDpdByRateOffLegacyThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_dpd_by_rate_off_legacy_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RDpdByRateOffLegacyOvThValR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_legacy_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RDpdByRateOffLegacyOvThValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dpd_by_rate_off_legacy_below_th_val` reader - Bbswing ,txbias mcs below th sel this"]
pub type RDpdByRateOffLegacyBelowThValR = crate::BitReader;
#[doc = "Field `r_dpd_by_rate_off_legacy_below_th_val` writer - Bbswing ,txbias mcs below th sel this"]
pub type RDpdByRateOffLegacyBelowThValW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_vht_below_th_val(&self) -> RTxbiasVhtBelowThValR {
        RTxbiasVhtBelowThValR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - 0~12 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_he_th(&self) -> RTxbbswingTxbiasHeThR {
        RTxbbswingTxbiasHeThR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_he_ov_th_val(&self) -> RTxbbswingHeOvThValR {
        RTxbbswingHeOvThValR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_he_below_th_val(&self) -> RTxbbswingHeBelowThValR {
        RTxbbswingHeBelowThValR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_he_ov_th_val(&self) -> RTxbiasHeOvThValR {
        RTxbiasHeOvThValR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbias_he_below_th_val(&self) -> RTxbiasHeBelowThValR {
        RTxbiasHeBelowThValR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - 0~4 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_cck_th(&self) -> RDpdByRateOffCckThR {
        RDpdByRateOffCckThR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_cck_ov_th_val(&self) -> RDpdByRateOffCckOvThValR {
        RDpdByRateOffCckOvThValR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_cck_below_th_val(&self) -> RDpdByRateOffCckBelowThValR {
        RDpdByRateOffCckBelowThValR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - 4~8 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_legacy_th(&self) -> RDpdByRateOffLegacyThR {
        RDpdByRateOffLegacyThR::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_legacy_ov_th_val(&self) -> RDpdByRateOffLegacyOvThValR {
        RDpdByRateOffLegacyOvThValR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_legacy_below_th_val(&self) -> RDpdByRateOffLegacyBelowThValR {
        RDpdByRateOffLegacyBelowThValR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_vht_below_th_val(
        &mut self,
    ) -> RTxbiasVhtBelowThValW<'_, RegAxPwrSwingHeCtrlSpec> {
        RTxbiasVhtBelowThValW::new(self, 0)
    }
    #[doc = "Bits 2:5 - 0~12 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_he_th(
        &mut self,
    ) -> RTxbbswingTxbiasHeThW<'_, RegAxPwrSwingHeCtrlSpec> {
        RTxbbswingTxbiasHeThW::new(self, 2)
    }
    #[doc = "Bits 6:9 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_he_ov_th_val(
        &mut self,
    ) -> RTxbbswingHeOvThValW<'_, RegAxPwrSwingHeCtrlSpec> {
        RTxbbswingHeOvThValW::new(self, 6)
    }
    #[doc = "Bits 10:13 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_he_below_th_val(
        &mut self,
    ) -> RTxbbswingHeBelowThValW<'_, RegAxPwrSwingHeCtrlSpec> {
        RTxbbswingHeBelowThValW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_he_ov_th_val(&mut self) -> RTxbiasHeOvThValW<'_, RegAxPwrSwingHeCtrlSpec> {
        RTxbiasHeOvThValW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbias_he_below_th_val(
        &mut self,
    ) -> RTxbiasHeBelowThValW<'_, RegAxPwrSwingHeCtrlSpec> {
        RTxbiasHeBelowThValW::new(self, 16)
    }
    #[doc = "Bits 18:20 - 0~4 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_cck_th(&mut self) -> RDpdByRateOffCckThW<'_, RegAxPwrSwingHeCtrlSpec> {
        RDpdByRateOffCckThW::new(self, 18)
    }
    #[doc = "Bit 21 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_cck_ov_th_val(
        &mut self,
    ) -> RDpdByRateOffCckOvThValW<'_, RegAxPwrSwingHeCtrlSpec> {
        RDpdByRateOffCckOvThValW::new(self, 21)
    }
    #[doc = "Bit 22 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_cck_below_th_val(
        &mut self,
    ) -> RDpdByRateOffCckBelowThValW<'_, RegAxPwrSwingHeCtrlSpec> {
        RDpdByRateOffCckBelowThValW::new(self, 22)
    }
    #[doc = "Bits 23:26 - 4~8 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_legacy_th(
        &mut self,
    ) -> RDpdByRateOffLegacyThW<'_, RegAxPwrSwingHeCtrlSpec> {
        RDpdByRateOffLegacyThW::new(self, 23)
    }
    #[doc = "Bit 27 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_legacy_ov_th_val(
        &mut self,
    ) -> RDpdByRateOffLegacyOvThValW<'_, RegAxPwrSwingHeCtrlSpec> {
        RDpdByRateOffLegacyOvThValW::new(self, 27)
    }
    #[doc = "Bit 28 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_dpd_by_rate_off_legacy_below_th_val(
        &mut self,
    ) -> RDpdByRateOffLegacyBelowThValW<'_, RegAxPwrSwingHeCtrlSpec> {
        RDpdByRateOffLegacyBelowThValW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_swing_he_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_swing_he_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSwingHeCtrlSpec;
impl crate::RegisterSpec for RegAxPwrSwingHeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_swing_he_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSwingHeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_swing_he_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSwingHeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SWING_HE_CTRL to value 0"]
impl crate::Resettable for RegAxPwrSwingHeCtrlSpec {}
