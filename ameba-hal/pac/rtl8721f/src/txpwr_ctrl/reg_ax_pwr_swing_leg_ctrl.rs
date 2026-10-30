#[doc = "Register `REG_AX_PWR_SWING_LEG_CTRL` reader"]
pub type R = crate::R<RegAxPwrSwingLegCtrlSpec>;
#[doc = "Register `REG_AX_PWR_SWING_LEG_CTRL` writer"]
pub type W = crate::W<RegAxPwrSwingLegCtrlSpec>;
#[doc = "Field `r_txbbswing_txbias_cck_th` reader - 0~4 Bbswing txbias mcs th"]
pub type RTxbbswingTxbiasCckThR = crate::FieldReader;
#[doc = "Field `r_txbbswing_txbias_cck_th` writer - 0~4 Bbswing txbias mcs th"]
pub type RTxbbswingTxbiasCckThW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_txbbswing_cck_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbbswingCckOvThValR = crate::FieldReader;
#[doc = "Field `r_txbbswing_cck_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbbswingCckOvThValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbbswing_cck_below_th_val` reader - Bbswing ,txbias mcs below th sel this"]
pub type RTxbbswingCckBelowThValR = crate::FieldReader;
#[doc = "Field `r_txbbswing_cck_below_th_val` writer - Bbswing ,txbias mcs below th sel this"]
pub type RTxbbswingCckBelowThValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_cck_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasCckOvThValR = crate::FieldReader;
#[doc = "Field `r_txbias_cck_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasCckOvThValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_cck_below_th_val` reader - Bbswing ,txbias mcs below th sel this"]
pub type RTxbiasCckBelowThValR = crate::FieldReader;
#[doc = "Field `r_txbias_cck_below_th_val` writer - Bbswing ,txbias mcs below th sel this"]
pub type RTxbiasCckBelowThValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbbswing_txbias_legacy_th` reader - 0~4 Bbswing txbias mcs th"]
pub type RTxbbswingTxbiasLegacyThR = crate::FieldReader;
#[doc = "Field `r_txbbswing_txbias_legacy_th` writer - 0~4 Bbswing txbias mcs th"]
pub type RTxbbswingTxbiasLegacyThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbbswing_legacy_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbbswingLegacyOvThValR = crate::FieldReader;
#[doc = "Field `r_txbbswing_legacy_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbbswingLegacyOvThValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbbswing_legacy_below_th_val` reader - Bbswing ,txbias mcs below th sel this"]
pub type RTxbbswingLegacyBelowThValR = crate::FieldReader;
#[doc = "Field `r_txbbswing_legacy_below_th_val` writer - Bbswing ,txbias mcs below th sel this"]
pub type RTxbbswingLegacyBelowThValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_legacy_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasLegacyOvThValR = crate::FieldReader;
#[doc = "Field `r_txbias_legacy_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasLegacyOvThValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_legacy_below_th_val` reader - Bbswing , txbias mcs below th sel this"]
pub type RTxbiasLegacyBelowThValR = crate::FieldReader;
#[doc = "Field `r_txbias_legacy_below_th_val` writer - Bbswing , txbias mcs below th sel this"]
pub type RTxbiasLegacyBelowThValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - 0~4 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_cck_th(&self) -> RTxbbswingTxbiasCckThR {
        RTxbbswingTxbiasCckThR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_cck_ov_th_val(&self) -> RTxbbswingCckOvThValR {
        RTxbbswingCckOvThValR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_cck_below_th_val(&self) -> RTxbbswingCckBelowThValR {
        RTxbbswingCckBelowThValR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:12 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_cck_ov_th_val(&self) -> RTxbiasCckOvThValR {
        RTxbiasCckOvThValR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbias_cck_below_th_val(&self) -> RTxbiasCckBelowThValR {
        RTxbiasCckBelowThValR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:18 - 0~4 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_legacy_th(&self) -> RTxbbswingTxbiasLegacyThR {
        RTxbbswingTxbiasLegacyThR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_legacy_ov_th_val(&self) -> RTxbbswingLegacyOvThValR {
        RTxbbswingLegacyOvThValR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_legacy_below_th_val(&self) -> RTxbbswingLegacyBelowThValR {
        RTxbbswingLegacyBelowThValR::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:28 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_legacy_ov_th_val(&self) -> RTxbiasLegacyOvThValR {
        RTxbiasLegacyOvThValR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - Bbswing , txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbias_legacy_below_th_val(&self) -> RTxbiasLegacyBelowThValR {
        RTxbiasLegacyBelowThValR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 0~4 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_cck_th(
        &mut self,
    ) -> RTxbbswingTxbiasCckThW<'_, RegAxPwrSwingLegCtrlSpec> {
        RTxbbswingTxbiasCckThW::new(self, 0)
    }
    #[doc = "Bits 3:6 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_cck_ov_th_val(
        &mut self,
    ) -> RTxbbswingCckOvThValW<'_, RegAxPwrSwingLegCtrlSpec> {
        RTxbbswingCckOvThValW::new(self, 3)
    }
    #[doc = "Bits 7:10 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_cck_below_th_val(
        &mut self,
    ) -> RTxbbswingCckBelowThValW<'_, RegAxPwrSwingLegCtrlSpec> {
        RTxbbswingCckBelowThValW::new(self, 7)
    }
    #[doc = "Bits 11:12 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_cck_ov_th_val(&mut self) -> RTxbiasCckOvThValW<'_, RegAxPwrSwingLegCtrlSpec> {
        RTxbiasCckOvThValW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbias_cck_below_th_val(
        &mut self,
    ) -> RTxbiasCckBelowThValW<'_, RegAxPwrSwingLegCtrlSpec> {
        RTxbiasCckBelowThValW::new(self, 13)
    }
    #[doc = "Bits 15:18 - 0~4 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_legacy_th(
        &mut self,
    ) -> RTxbbswingTxbiasLegacyThW<'_, RegAxPwrSwingLegCtrlSpec> {
        RTxbbswingTxbiasLegacyThW::new(self, 15)
    }
    #[doc = "Bits 19:22 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_legacy_ov_th_val(
        &mut self,
    ) -> RTxbbswingLegacyOvThValW<'_, RegAxPwrSwingLegCtrlSpec> {
        RTxbbswingLegacyOvThValW::new(self, 19)
    }
    #[doc = "Bits 23:26 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_legacy_below_th_val(
        &mut self,
    ) -> RTxbbswingLegacyBelowThValW<'_, RegAxPwrSwingLegCtrlSpec> {
        RTxbbswingLegacyBelowThValW::new(self, 23)
    }
    #[doc = "Bits 27:28 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_legacy_ov_th_val(
        &mut self,
    ) -> RTxbiasLegacyOvThValW<'_, RegAxPwrSwingLegCtrlSpec> {
        RTxbiasLegacyOvThValW::new(self, 27)
    }
    #[doc = "Bits 29:30 - Bbswing , txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbias_legacy_below_th_val(
        &mut self,
    ) -> RTxbiasLegacyBelowThValW<'_, RegAxPwrSwingLegCtrlSpec> {
        RTxbiasLegacyBelowThValW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_swing_leg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_swing_leg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSwingLegCtrlSpec;
impl crate::RegisterSpec for RegAxPwrSwingLegCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_swing_leg_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSwingLegCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_swing_leg_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSwingLegCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SWING_LEG_CTRL to value 0x0002_0000"]
impl crate::Resettable for RegAxPwrSwingLegCtrlSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
