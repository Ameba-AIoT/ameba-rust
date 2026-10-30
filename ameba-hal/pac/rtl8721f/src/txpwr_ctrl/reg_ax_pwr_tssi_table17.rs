#[doc = "Register `REG_AX_PWR_TSSI_TABLE17` reader"]
pub type R = crate::R<RegAxPwrTssiTable17Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE17` writer"]
pub type W = crate::W<RegAxPwrTssiTable17Spec>;
#[doc = "Field `r_vht_init_ofst_seg0_db` reader - "]
pub type RVhtInitOfstSeg0DbR = crate::FieldReader;
#[doc = "Field `r_vht_init_ofst_seg0_db` writer - "]
pub type RVhtInitOfstSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_tssi_ctrl_mac_dbg_sel` reader - "]
pub type RTssiCtrlMacDbgSelR = crate::FieldReader;
#[doc = "Field `r_tssi_ctrl_mac_dbg_sel` writer - "]
pub type RTssiCtrlMacDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_bandedge_legacy_rateidx_th` reader - "]
pub type RBandedgeLegacyRateidxThR = crate::FieldReader;
#[doc = "Field `r_bandedge_legacy_rateidx_th` writer - "]
pub type RBandedgeLegacyRateidxThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_bandedge_nonlegacy_rateidx_th` reader - "]
pub type RBandedgeNonlegacyRateidxThR = crate::FieldReader;
#[doc = "Field `r_bandedge_nonlegacy_rateidx_th` writer - "]
pub type RBandedgeNonlegacyRateidxThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_f_format` reader - "]
pub type RFFormatR = crate::FieldReader;
#[doc = "Field `r_f_format` writer - "]
pub type RFFormatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_f_rate_idx` reader - "]
pub type RFRateIdxR = crate::FieldReader;
#[doc = "Field `r_f_rate_idx` writer - "]
pub type RFRateIdxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_bandedge_cfg` reader - "]
pub type RBandedgeCfgR = crate::FieldReader;
#[doc = "Field `r_bandedge_cfg` writer - "]
pub type RBandedgeCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_vht_init_ofst_seg0_db(&self) -> RVhtInitOfstSeg0DbR {
        RVhtInitOfstSeg0DbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_dbg_sel(&self) -> RTssiCtrlMacDbgSelR {
        RTssiCtrlMacDbgSelR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn r_bandedge_legacy_rateidx_th(&self) -> RBandedgeLegacyRateidxThR {
        RBandedgeLegacyRateidxThR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn r_bandedge_nonlegacy_rateidx_th(&self) -> RBandedgeNonlegacyRateidxThR {
        RBandedgeNonlegacyRateidxThR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn r_f_format(&self) -> RFFormatR {
        RFFormatR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn r_f_rate_idx(&self) -> RFRateIdxR {
        RFRateIdxR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_bandedge_cfg(&self) -> RBandedgeCfgR {
        RBandedgeCfgR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_vht_init_ofst_seg0_db(&mut self) -> RVhtInitOfstSeg0DbW<'_, RegAxPwrTssiTable17Spec> {
        RVhtInitOfstSeg0DbW::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn r_tssi_ctrl_mac_dbg_sel(&mut self) -> RTssiCtrlMacDbgSelW<'_, RegAxPwrTssiTable17Spec> {
        RTssiCtrlMacDbgSelW::new(self, 8)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn r_bandedge_legacy_rateidx_th(
        &mut self,
    ) -> RBandedgeLegacyRateidxThW<'_, RegAxPwrTssiTable17Spec> {
        RBandedgeLegacyRateidxThW::new(self, 14)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn r_bandedge_nonlegacy_rateidx_th(
        &mut self,
    ) -> RBandedgeNonlegacyRateidxThW<'_, RegAxPwrTssiTable17Spec> {
        RBandedgeNonlegacyRateidxThW::new(self, 18)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn r_f_format(&mut self) -> RFFormatW<'_, RegAxPwrTssiTable17Spec> {
        RFFormatW::new(self, 22)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn r_f_rate_idx(&mut self) -> RFRateIdxW<'_, RegAxPwrTssiTable17Spec> {
        RFRateIdxW::new(self, 26)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_bandedge_cfg(&mut self) -> RBandedgeCfgW<'_, RegAxPwrTssiTable17Spec> {
        RBandedgeCfgW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable17Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table17::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable17Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table17::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE17 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable17Spec {}
