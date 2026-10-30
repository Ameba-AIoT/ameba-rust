#[doc = "Register `REG_AX_PWR_TSSI_TABLE18` reader"]
pub type R = crate::R<RegAxPwrTssiTable18Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE18` writer"]
pub type W = crate::W<RegAxPwrTssiTable18Spec>;
#[doc = "Field `r_f_txsc` reader - "]
pub type RFTxscR = crate::FieldReader;
#[doc = "Field `r_f_txsc` writer - "]
pub type RFTxscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_f_cbw` reader - "]
pub type RFCbwR = crate::FieldReader;
#[doc = "Field `r_f_cbw` writer - "]
pub type RFCbwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_f_dbw` reader - "]
pub type RFDbwR = crate::FieldReader;
#[doc = "Field `r_f_dbw` writer - "]
pub type RFDbwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_f_precoding_mode_idx` reader - "]
pub type RFPrecodingModeIdxR = crate::FieldReader;
#[doc = "Field `r_f_precoding_mode_idx` writer - "]
pub type RFPrecodingModeIdxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_ch20_with_data_legacy_swap` reader - "]
pub type RCh20WithDataLegacySwapR = crate::BitReader;
#[doc = "Field `r_ch20_with_data_legacy_swap` writer - "]
pub type RCh20WithDataLegacySwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_ch20_with_data_nonlegacy_swap` reader - "]
pub type RCh20WithDataNonlegacySwapR = crate::BitReader;
#[doc = "Field `r_ch20_with_data_nonlegacy_swap` writer - "]
pub type RCh20WithDataNonlegacySwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bandedge_cfg_hetb_en` reader - "]
pub type RFBandedgeCfgHetbEnR = crate::BitReader;
#[doc = "Field `r_f_bandedge_cfg_hetb_en` writer - "]
pub type RFBandedgeCfgHetbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bandedge_cfg_mu_en` reader - "]
pub type RFBandedgeCfgMuEnR = crate::BitReader;
#[doc = "Field `r_f_bandedge_cfg_mu_en` writer - "]
pub type RFBandedgeCfgMuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bandedge_cfg_ndp_en` reader - "]
pub type RFBandedgeCfgNdpEnR = crate::BitReader;
#[doc = "Field `r_f_bandedge_cfg_ndp_en` writer - "]
pub type RFBandedgeCfgNdpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bandedge_cfg_subf_en` reader - "]
pub type RFBandedgeCfgSubfEnR = crate::BitReader;
#[doc = "Field `r_f_bandedge_cfg_subf_en` writer - "]
pub type RFBandedgeCfgSubfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bandedge_cfg_sudcm_en` reader - "]
pub type RFBandedgeCfgSudcmEnR = crate::BitReader;
#[doc = "Field `r_f_bandedge_cfg_sudcm_en` writer - "]
pub type RFBandedgeCfgSudcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bandedge_cfg_sustbc_en` reader - "]
pub type RFBandedgeCfgSustbcEnR = crate::BitReader;
#[doc = "Field `r_f_bandedge_cfg_sustbc_en` writer - "]
pub type RFBandedgeCfgSustbcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bmode_bandedge_ind_en` reader - "]
pub type RFBmodeBandedgeIndEnR = crate::BitReader;
#[doc = "Field `r_f_bmode_bandedge_ind_en` writer - "]
pub type RFBmodeBandedgeIndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bmode_bandedge_ind_val` reader - "]
pub type RFBmodeBandedgeIndValR = crate::BitReader;
#[doc = "Field `r_f_bmode_bandedge_ind_val` writer - "]
pub type RFBmodeBandedgeIndValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bmode_bypass_cbw_fltr_en` reader - "]
pub type RFBmodeBypassCbwFltrEnR = crate::BitReader;
#[doc = "Field `r_f_bmode_bypass_cbw_fltr_en` writer - "]
pub type RFBmodeBypassCbwFltrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bmode_bypass_dbw_fltr_en` reader - "]
pub type RFBmodeBypassDbwFltrEnR = crate::BitReader;
#[doc = "Field `r_f_bmode_bypass_dbw_fltr_en` writer - "]
pub type RFBmodeBypassDbwFltrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bmode_diff_out_en` reader - "]
pub type RFBmodeDiffOutEnR = crate::BitReader;
#[doc = "Field `r_f_bmode_diff_out_en` writer - "]
pub type RFBmodeDiffOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_bmode_ofst_out_en` reader - "]
pub type RFBmodeOfstOutEnR = crate::BitReader;
#[doc = "Field `r_f_bmode_ofst_out_en` writer - "]
pub type RFBmodeOfstOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_cbw_en` reader - "]
pub type RFCbwEnR = crate::BitReader;
#[doc = "Field `r_f_cbw_en` writer - "]
pub type RFCbwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_ch20_with_data_en` reader - "]
pub type RFCh20WithDataEnR = crate::BitReader;
#[doc = "Field `r_f_ch20_with_data_en` writer - "]
pub type RFCh20WithDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_dbw_en` reader - "]
pub type RFDbwEnR = crate::BitReader;
#[doc = "Field `r_f_dbw_en` writer - "]
pub type RFDbwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_fast_mode_comp_init_en` reader - "]
pub type RFFastModeCompInitEnR = crate::BitReader;
#[doc = "Field `r_f_fast_mode_comp_init_en` writer - "]
pub type RFFastModeCompInitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_format_en` reader - "]
pub type RFFormatEnR = crate::BitReader;
#[doc = "Field `r_f_format_en` writer - "]
pub type RFFormatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_heer_bandedge_ind_en` reader - "]
pub type RFHeerBandedgeIndEnR = crate::BitReader;
#[doc = "Field `r_f_heer_bandedge_ind_en` writer - "]
pub type RFHeerBandedgeIndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_heer_bandedge_ind_val` reader - "]
pub type RFHeerBandedgeIndValR = crate::BitReader;
#[doc = "Field `r_f_heer_bandedge_ind_val` writer - "]
pub type RFHeerBandedgeIndValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_f_heer_bypass_cbw_fltr_en` reader - "]
pub type RFHeerBypassCbwFltrEnR = crate::BitReader;
#[doc = "Field `r_f_heer_bypass_cbw_fltr_en` writer - "]
pub type RFHeerBypassCbwFltrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn r_f_txsc(&self) -> RFTxscR {
        RFTxscR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn r_f_cbw(&self) -> RFCbwR {
        RFCbwR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn r_f_dbw(&self) -> RFDbwR {
        RFDbwR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn r_f_precoding_mode_idx(&self) -> RFPrecodingModeIdxR {
        RFPrecodingModeIdxR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn r_ch20_with_data_legacy_swap(&self) -> RCh20WithDataLegacySwapR {
        RCh20WithDataLegacySwapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn r_ch20_with_data_nonlegacy_swap(&self) -> RCh20WithDataNonlegacySwapR {
        RCh20WithDataNonlegacySwapR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_hetb_en(&self) -> RFBandedgeCfgHetbEnR {
        RFBandedgeCfgHetbEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_mu_en(&self) -> RFBandedgeCfgMuEnR {
        RFBandedgeCfgMuEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_ndp_en(&self) -> RFBandedgeCfgNdpEnR {
        RFBandedgeCfgNdpEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_subf_en(&self) -> RFBandedgeCfgSubfEnR {
        RFBandedgeCfgSubfEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_sudcm_en(&self) -> RFBandedgeCfgSudcmEnR {
        RFBandedgeCfgSudcmEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_sustbc_en(&self) -> RFBandedgeCfgSustbcEnR {
        RFBandedgeCfgSustbcEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn r_f_bmode_bandedge_ind_en(&self) -> RFBmodeBandedgeIndEnR {
        RFBmodeBandedgeIndEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn r_f_bmode_bandedge_ind_val(&self) -> RFBmodeBandedgeIndValR {
        RFBmodeBandedgeIndValR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn r_f_bmode_bypass_cbw_fltr_en(&self) -> RFBmodeBypassCbwFltrEnR {
        RFBmodeBypassCbwFltrEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn r_f_bmode_bypass_dbw_fltr_en(&self) -> RFBmodeBypassDbwFltrEnR {
        RFBmodeBypassDbwFltrEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn r_f_bmode_diff_out_en(&self) -> RFBmodeDiffOutEnR {
        RFBmodeDiffOutEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn r_f_bmode_ofst_out_en(&self) -> RFBmodeOfstOutEnR {
        RFBmodeOfstOutEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn r_f_cbw_en(&self) -> RFCbwEnR {
        RFCbwEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn r_f_ch20_with_data_en(&self) -> RFCh20WithDataEnR {
        RFCh20WithDataEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn r_f_dbw_en(&self) -> RFDbwEnR {
        RFDbwEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn r_f_fast_mode_comp_init_en(&self) -> RFFastModeCompInitEnR {
        RFFastModeCompInitEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn r_f_format_en(&self) -> RFFormatEnR {
        RFFormatEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn r_f_heer_bandedge_ind_en(&self) -> RFHeerBandedgeIndEnR {
        RFHeerBandedgeIndEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn r_f_heer_bandedge_ind_val(&self) -> RFHeerBandedgeIndValR {
        RFHeerBandedgeIndValR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn r_f_heer_bypass_cbw_fltr_en(&self) -> RFHeerBypassCbwFltrEnR {
        RFHeerBypassCbwFltrEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn r_f_txsc(&mut self) -> RFTxscW<'_, RegAxPwrTssiTable18Spec> {
        RFTxscW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn r_f_cbw(&mut self) -> RFCbwW<'_, RegAxPwrTssiTable18Spec> {
        RFCbwW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn r_f_dbw(&mut self) -> RFDbwW<'_, RegAxPwrTssiTable18Spec> {
        RFDbwW::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn r_f_precoding_mode_idx(&mut self) -> RFPrecodingModeIdxW<'_, RegAxPwrTssiTable18Spec> {
        RFPrecodingModeIdxW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn r_ch20_with_data_legacy_swap(
        &mut self,
    ) -> RCh20WithDataLegacySwapW<'_, RegAxPwrTssiTable18Spec> {
        RCh20WithDataLegacySwapW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn r_ch20_with_data_nonlegacy_swap(
        &mut self,
    ) -> RCh20WithDataNonlegacySwapW<'_, RegAxPwrTssiTable18Spec> {
        RCh20WithDataNonlegacySwapW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_hetb_en(
        &mut self,
    ) -> RFBandedgeCfgHetbEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBandedgeCfgHetbEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_mu_en(&mut self) -> RFBandedgeCfgMuEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBandedgeCfgMuEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_ndp_en(&mut self) -> RFBandedgeCfgNdpEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBandedgeCfgNdpEnW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_subf_en(
        &mut self,
    ) -> RFBandedgeCfgSubfEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBandedgeCfgSubfEnW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_sudcm_en(
        &mut self,
    ) -> RFBandedgeCfgSudcmEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBandedgeCfgSudcmEnW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn r_f_bandedge_cfg_sustbc_en(
        &mut self,
    ) -> RFBandedgeCfgSustbcEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBandedgeCfgSustbcEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn r_f_bmode_bandedge_ind_en(
        &mut self,
    ) -> RFBmodeBandedgeIndEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBmodeBandedgeIndEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn r_f_bmode_bandedge_ind_val(
        &mut self,
    ) -> RFBmodeBandedgeIndValW<'_, RegAxPwrTssiTable18Spec> {
        RFBmodeBandedgeIndValW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn r_f_bmode_bypass_cbw_fltr_en(
        &mut self,
    ) -> RFBmodeBypassCbwFltrEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBmodeBypassCbwFltrEnW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn r_f_bmode_bypass_dbw_fltr_en(
        &mut self,
    ) -> RFBmodeBypassDbwFltrEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBmodeBypassDbwFltrEnW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn r_f_bmode_diff_out_en(&mut self) -> RFBmodeDiffOutEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBmodeDiffOutEnW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn r_f_bmode_ofst_out_en(&mut self) -> RFBmodeOfstOutEnW<'_, RegAxPwrTssiTable18Spec> {
        RFBmodeOfstOutEnW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn r_f_cbw_en(&mut self) -> RFCbwEnW<'_, RegAxPwrTssiTable18Spec> {
        RFCbwEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn r_f_ch20_with_data_en(&mut self) -> RFCh20WithDataEnW<'_, RegAxPwrTssiTable18Spec> {
        RFCh20WithDataEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn r_f_dbw_en(&mut self) -> RFDbwEnW<'_, RegAxPwrTssiTable18Spec> {
        RFDbwEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn r_f_fast_mode_comp_init_en(
        &mut self,
    ) -> RFFastModeCompInitEnW<'_, RegAxPwrTssiTable18Spec> {
        RFFastModeCompInitEnW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn r_f_format_en(&mut self) -> RFFormatEnW<'_, RegAxPwrTssiTable18Spec> {
        RFFormatEnW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn r_f_heer_bandedge_ind_en(
        &mut self,
    ) -> RFHeerBandedgeIndEnW<'_, RegAxPwrTssiTable18Spec> {
        RFHeerBandedgeIndEnW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn r_f_heer_bandedge_ind_val(
        &mut self,
    ) -> RFHeerBandedgeIndValW<'_, RegAxPwrTssiTable18Spec> {
        RFHeerBandedgeIndValW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn r_f_heer_bypass_cbw_fltr_en(
        &mut self,
    ) -> RFHeerBypassCbwFltrEnW<'_, RegAxPwrTssiTable18Spec> {
        RFHeerBypassCbwFltrEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable18Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table18::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable18Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table18::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE18 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable18Spec {}
