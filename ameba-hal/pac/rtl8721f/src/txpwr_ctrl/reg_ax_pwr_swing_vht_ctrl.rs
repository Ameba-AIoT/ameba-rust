#[doc = "Register `REG_AX_PWR_SWING_VHT_CTRL` reader"]
pub type R = crate::R<RegAxPwrSwingVhtCtrlSpec>;
#[doc = "Register `REG_AX_PWR_SWING_VHT_CTRL` writer"]
pub type W = crate::W<RegAxPwrSwingVhtCtrlSpec>;
#[doc = "Field `r_txbbswing_txbias_HT_th` reader - 0~8 Bbswing txbias mcs th"]
pub type RTxbbswingTxbiasHtThR = crate::FieldReader;
#[doc = "Field `r_txbbswing_txbias_HT_th` writer - 0~8 Bbswing txbias mcs th"]
pub type RTxbbswingTxbiasHtThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbbswing_HT_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbbswingHtOvThValR = crate::FieldReader;
#[doc = "Field `r_txbbswing_HT_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbbswingHtOvThValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbbswing_HT_below_th_val` reader - Bbswing ,txbias mcs below th sel this"]
pub type RTxbbswingHtBelowThValR = crate::FieldReader;
#[doc = "Field `r_txbbswing_HT_below_th_val` writer - Bbswing ,txbias mcs below th sel this"]
pub type RTxbbswingHtBelowThValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_HT_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasHtOvThValR = crate::FieldReader;
#[doc = "Field `r_txbias_HT_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasHtOvThValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_HT_below_th_val` reader - Bbswing ,txbias mcs below th sel this"]
pub type RTxbiasHtBelowThValR = crate::FieldReader;
#[doc = "Field `r_txbias_HT_below_th_val` writer - Bbswing ,txbias mcs below th sel this"]
pub type RTxbiasHtBelowThValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbbswing_txbias_VHT_th` reader - 0~12 Bbswing txbias mcs th"]
pub type RTxbbswingTxbiasVhtThR = crate::FieldReader;
#[doc = "Field `r_txbbswing_txbias_VHT_th` writer - 0~12 Bbswing txbias mcs th"]
pub type RTxbbswingTxbiasVhtThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbbswing_VHT_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbbswingVhtOvThValR = crate::FieldReader;
#[doc = "Field `r_txbbswing_VHT_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbbswingVhtOvThValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbbswing_VHT_below_th_val` reader - Bbswing ,txbias mcs below th sel this"]
pub type RTxbbswingVhtBelowThValR = crate::FieldReader;
#[doc = "Field `r_txbbswing_VHT_below_th_val` writer - Bbswing ,txbias mcs below th sel this"]
pub type RTxbbswingVhtBelowThValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txbias_VHT_ov_th_val` reader - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasVhtOvThValR = crate::FieldReader;
#[doc = "Field `r_txbias_VHT_ov_th_val` writer - Bbswing ,txbias mcs ov th sel this"]
pub type RTxbiasVhtOvThValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:5 - 0~8 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_ht_th(&self) -> RTxbbswingTxbiasHtThR {
        RTxbbswingTxbiasHtThR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_ht_ov_th_val(&self) -> RTxbbswingHtOvThValR {
        RTxbbswingHtOvThValR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_ht_below_th_val(&self) -> RTxbbswingHtBelowThValR {
        RTxbbswingHtBelowThValR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_ht_ov_th_val(&self) -> RTxbiasHtOvThValR {
        RTxbiasHtOvThValR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbias_ht_below_th_val(&self) -> RTxbiasHtBelowThValR {
        RTxbiasHtBelowThValR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - 0~12 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_vht_th(&self) -> RTxbbswingTxbiasVhtThR {
        RTxbbswingTxbiasVhtThR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_vht_ov_th_val(&self) -> RTxbbswingVhtOvThValR {
        RTxbbswingVhtOvThValR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_vht_below_th_val(&self) -> RTxbbswingVhtBelowThValR {
        RTxbbswingVhtBelowThValR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_vht_ov_th_val(&self) -> RTxbiasVhtOvThValR {
        RTxbiasVhtOvThValR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:5 - 0~8 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_ht_th(
        &mut self,
    ) -> RTxbbswingTxbiasHtThW<'_, RegAxPwrSwingVhtCtrlSpec> {
        RTxbbswingTxbiasHtThW::new(self, 2)
    }
    #[doc = "Bits 6:9 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_ht_ov_th_val(
        &mut self,
    ) -> RTxbbswingHtOvThValW<'_, RegAxPwrSwingVhtCtrlSpec> {
        RTxbbswingHtOvThValW::new(self, 6)
    }
    #[doc = "Bits 10:13 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_ht_below_th_val(
        &mut self,
    ) -> RTxbbswingHtBelowThValW<'_, RegAxPwrSwingVhtCtrlSpec> {
        RTxbbswingHtBelowThValW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_ht_ov_th_val(&mut self) -> RTxbiasHtOvThValW<'_, RegAxPwrSwingVhtCtrlSpec> {
        RTxbiasHtOvThValW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbias_ht_below_th_val(
        &mut self,
    ) -> RTxbiasHtBelowThValW<'_, RegAxPwrSwingVhtCtrlSpec> {
        RTxbiasHtBelowThValW::new(self, 16)
    }
    #[doc = "Bits 18:21 - 0~12 Bbswing txbias mcs th"]
    #[inline(always)]
    pub fn r_txbbswing_txbias_vht_th(
        &mut self,
    ) -> RTxbbswingTxbiasVhtThW<'_, RegAxPwrSwingVhtCtrlSpec> {
        RTxbbswingTxbiasVhtThW::new(self, 18)
    }
    #[doc = "Bits 22:25 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_vht_ov_th_val(
        &mut self,
    ) -> RTxbbswingVhtOvThValW<'_, RegAxPwrSwingVhtCtrlSpec> {
        RTxbbswingVhtOvThValW::new(self, 22)
    }
    #[doc = "Bits 26:29 - Bbswing ,txbias mcs below th sel this"]
    #[inline(always)]
    pub fn r_txbbswing_vht_below_th_val(
        &mut self,
    ) -> RTxbbswingVhtBelowThValW<'_, RegAxPwrSwingVhtCtrlSpec> {
        RTxbbswingVhtBelowThValW::new(self, 26)
    }
    #[doc = "Bits 30:31 - Bbswing ,txbias mcs ov th sel this"]
    #[inline(always)]
    pub fn r_txbias_vht_ov_th_val(&mut self) -> RTxbiasVhtOvThValW<'_, RegAxPwrSwingVhtCtrlSpec> {
        RTxbiasVhtOvThValW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_swing_vht_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_swing_vht_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSwingVhtCtrlSpec;
impl crate::RegisterSpec for RegAxPwrSwingVhtCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_swing_vht_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSwingVhtCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_swing_vht_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSwingVhtCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SWING_VHT_CTRL to value 0"]
impl crate::Resettable for RegAxPwrSwingVhtCtrlSpec {}
