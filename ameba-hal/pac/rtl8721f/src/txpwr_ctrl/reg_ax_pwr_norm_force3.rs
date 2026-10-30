#[doc = "Register `REG_AX_PWR_NORM_FORCE3` reader"]
pub type R = crate::R<RegAxPwrNormForce3Spec>;
#[doc = "Register `REG_AX_PWR_NORM_FORCE3` writer"]
pub type W = crate::W<RegAxPwrNormForce3Spec>;
#[doc = "Field `r_force_out_txpwr_mac` reader - (RTL_only) Force output to MAC"]
pub type RForceOutTxpwrMacR = crate::FieldReader;
#[doc = "Field `r_force_out_txpwr_mac` writer - (RTL_only) Force output to MAC"]
pub type RForceOutTxpwrMacW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_force_out_txpwr_mac_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutTxpwrMacOnR = crate::BitReader;
#[doc = "Field `r_force_out_txpwr_mac_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutTxpwrMacOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_txpwr_bb` reader - (RTL_only) Force output to MAC"]
pub type RForceOutTxpwrBbR = crate::FieldReader<u16>;
#[doc = "Field `r_force_out_txpwr_bb` writer - (RTL_only) Force output to MAC"]
pub type RForceOutTxpwrBbW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_out_txpwr_bb_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutTxpwrBbOnR = crate::BitReader;
#[doc = "Field `r_force_out_txpwr_bb_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutTxpwrBbOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_txpwr_bfon_boost_db_seg0` reader - (RTL_only) Force output to MAC"]
pub type RForceOutTxpwrBfonBoostDbSeg0R = crate::FieldReader;
#[doc = "Field `r_force_out_txpwr_bfon_boost_db_seg0` writer - (RTL_only) Force output to MAC"]
pub type RForceOutTxpwrBfonBoostDbSeg0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `r_force_out_txpwr_bfon_boost_db_seg0_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutTxpwrBfonBoostDbSeg0OnR = crate::BitReader;
#[doc = "Field `r_force_out_txpwr_bfon_boost_db_seg0_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutTxpwrBfonBoostDbSeg0OnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_txbias` reader - (RTL_only) Force output to MAC"]
pub type RForceOutTxbiasR = crate::FieldReader;
#[doc = "Field `r_force_out_txbias` writer - (RTL_only) Force output to MAC"]
pub type RForceOutTxbiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_force_out_txbias_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutTxbiasOnR = crate::BitReader;
#[doc = "Field `r_force_out_txbias_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutTxbiasOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_txagc_BBswing` reader - (RTL_only) Force output to MAC"]
pub type RForceOutTxagcBbswingR = crate::FieldReader;
#[doc = "Field `r_force_out_txagc_BBswing` writer - (RTL_only) Force output to MAC"]
pub type RForceOutTxagcBbswingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_force_out_txagc_bbswing_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutTxagcBbswingOnR = crate::BitReader;
#[doc = "Field `r_force_out_txagc_bbswing_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutTxagcBbswingOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_txpwr_mac(&self) -> RForceOutTxpwrMacR {
        RForceOutTxpwrMacR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_txpwr_mac_on(&self) -> RForceOutTxpwrMacOnR {
        RForceOutTxpwrMacOnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:16 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_txpwr_bb(&self) -> RForceOutTxpwrBbR {
        RForceOutTxpwrBbR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bit 17 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_txpwr_bb_on(&self) -> RForceOutTxpwrBbOnR {
        RForceOutTxpwrBbOnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_txpwr_bfon_boost_db_seg0(&self) -> RForceOutTxpwrBfonBoostDbSeg0R {
        RForceOutTxpwrBfonBoostDbSeg0R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_txpwr_bfon_boost_db_seg0_on(&self) -> RForceOutTxpwrBfonBoostDbSeg0OnR {
        RForceOutTxpwrBfonBoostDbSeg0OnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_txbias(&self) -> RForceOutTxbiasR {
        RForceOutTxbiasR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_txbias_on(&self) -> RForceOutTxbiasOnR {
        RForceOutTxbiasOnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:30 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_txagc_bbswing(&self) -> RForceOutTxagcBbswingR {
        RForceOutTxagcBbswingR::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_txagc_bbswing_on(&self) -> RForceOutTxagcBbswingOnR {
        RForceOutTxagcBbswingOnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_txpwr_mac(&mut self) -> RForceOutTxpwrMacW<'_, RegAxPwrNormForce3Spec> {
        RForceOutTxpwrMacW::new(self, 0)
    }
    #[doc = "Bit 6 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_txpwr_mac_on(&mut self) -> RForceOutTxpwrMacOnW<'_, RegAxPwrNormForce3Spec> {
        RForceOutTxpwrMacOnW::new(self, 6)
    }
    #[doc = "Bits 8:16 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_txpwr_bb(&mut self) -> RForceOutTxpwrBbW<'_, RegAxPwrNormForce3Spec> {
        RForceOutTxpwrBbW::new(self, 8)
    }
    #[doc = "Bit 17 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_txpwr_bb_on(&mut self) -> RForceOutTxpwrBbOnW<'_, RegAxPwrNormForce3Spec> {
        RForceOutTxpwrBbOnW::new(self, 17)
    }
    #[doc = "Bits 18:22 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_txpwr_bfon_boost_db_seg0(
        &mut self,
    ) -> RForceOutTxpwrBfonBoostDbSeg0W<'_, RegAxPwrNormForce3Spec> {
        RForceOutTxpwrBfonBoostDbSeg0W::new(self, 18)
    }
    #[doc = "Bit 23 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_txpwr_bfon_boost_db_seg0_on(
        &mut self,
    ) -> RForceOutTxpwrBfonBoostDbSeg0OnW<'_, RegAxPwrNormForce3Spec> {
        RForceOutTxpwrBfonBoostDbSeg0OnW::new(self, 23)
    }
    #[doc = "Bits 24:25 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_txbias(&mut self) -> RForceOutTxbiasW<'_, RegAxPwrNormForce3Spec> {
        RForceOutTxbiasW::new(self, 24)
    }
    #[doc = "Bit 26 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_txbias_on(&mut self) -> RForceOutTxbiasOnW<'_, RegAxPwrNormForce3Spec> {
        RForceOutTxbiasOnW::new(self, 26)
    }
    #[doc = "Bits 27:30 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_txagc_bbswing(
        &mut self,
    ) -> RForceOutTxagcBbswingW<'_, RegAxPwrNormForce3Spec> {
        RForceOutTxagcBbswingW::new(self, 27)
    }
    #[doc = "Bit 31 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_txagc_bbswing_on(
        &mut self,
    ) -> RForceOutTxagcBbswingOnW<'_, RegAxPwrNormForce3Spec> {
        RForceOutTxagcBbswingOnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_norm_force3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_norm_force3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrNormForce3Spec;
impl crate::RegisterSpec for RegAxPwrNormForce3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_norm_force3::R`](R) reader structure"]
impl crate::Readable for RegAxPwrNormForce3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_norm_force3::W`](W) writer structure"]
impl crate::Writable for RegAxPwrNormForce3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_NORM_FORCE3 to value 0"]
impl crate::Resettable for RegAxPwrNormForce3Spec {}
