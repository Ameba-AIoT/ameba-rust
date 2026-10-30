#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR18` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr18Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR18` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr18Spec>;
#[doc = "Field `r_pwrtssict_pwr_ofst_seg0_db_force_out_val` reader - "]
pub type RPwrtssictPwrOfstSeg0DbForceOutValR = crate::FieldReader<u16>;
#[doc = "Field `r_pwrtssict_pwr_ofst_seg0_db_force_out_val` writer - "]
pub type RPwrtssictPwrOfstSeg0DbForceOutValW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_pwrtssict_tssi_diff_seg0_db_force_out_val` reader - "]
pub type RPwrtssictTssiDiffSeg0DbForceOutValR = crate::FieldReader<u16>;
#[doc = "Field `r_pwrtssict_tssi_diff_seg0_db_force_out_val` writer - "]
pub type RPwrtssictTssiDiffSeg0DbForceOutValW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_pwrtssict_tx_bandedge_cfg_force_out_val` reader - "]
pub type RPwrtssictTxBandedgeCfgForceOutValR = crate::FieldReader;
#[doc = "Field `r_pwrtssict_tx_bandedge_cfg_force_out_val` writer - "]
pub type RPwrtssictTxBandedgeCfgForceOutValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_MAC_def_legacy_rate_idx_begin_at_4_on` reader - "]
pub type RMacDefLegacyRateIdxBeginAt4OnR = crate::BitReader;
#[doc = "Field `r_MAC_def_legacy_rate_idx_begin_at_4_on` writer - "]
pub type RMacDefLegacyRateIdxBeginAt4OnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pwrtssict_pwr_ofst_seg0_db_force_out_on` reader - "]
pub type RPwrtssictPwrOfstSeg0DbForceOutOnR = crate::BitReader;
#[doc = "Field `r_pwrtssict_pwr_ofst_seg0_db_force_out_on` writer - "]
pub type RPwrtssictPwrOfstSeg0DbForceOutOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pwrtssict_tssi_diff_seg0_db_force_out_on` reader - "]
pub type RPwrtssictTssiDiffSeg0DbForceOutOnR = crate::BitReader;
#[doc = "Field `r_pwrtssict_tssi_diff_seg0_db_force_out_on` writer - "]
pub type RPwrtssictTssiDiffSeg0DbForceOutOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pwrtssict_tssi_fast_mode_en_force_out_on` reader - "]
pub type RPwrtssictTssiFastModeEnForceOutOnR = crate::BitReader;
#[doc = "Field `r_pwrtssict_tssi_fast_mode_en_force_out_on` writer - "]
pub type RPwrtssictTssiFastModeEnForceOutOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pwrtssict_tssi_fast_mode_en_force_out_val` reader - "]
pub type RPwrtssictTssiFastModeEnForceOutValR = crate::BitReader;
#[doc = "Field `r_pwrtssict_tssi_fast_mode_en_force_out_val` writer - "]
pub type RPwrtssictTssiFastModeEnForceOutValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pwrtssict_tssi_msr_athestf_force_out_on` reader - "]
pub type RPwrtssictTssiMsrAthestfForceOutOnR = crate::BitReader;
#[doc = "Field `r_pwrtssict_tssi_msr_athestf_force_out_on` writer - "]
pub type RPwrtssictTssiMsrAthestfForceOutOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pwrtssict_tssi_msr_athestf_force_out_val` reader - "]
pub type RPwrtssictTssiMsrAthestfForceOutValR = crate::BitReader;
#[doc = "Field `r_pwrtssict_tssi_msr_athestf_force_out_val` writer - "]
pub type RPwrtssictTssiMsrAthestfForceOutValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pwrtssict_tx_bandedge_cfg_force_out_on` reader - "]
pub type RPwrtssictTxBandedgeCfgForceOutOnR = crate::BitReader;
#[doc = "Field `r_pwrtssict_tx_bandedge_cfg_force_out_on` writer - "]
pub type RPwrtssictTxBandedgeCfgForceOutOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_ru_alloc_dbw160_fix_en` reader - "]
pub type RRuAllocDbw160FixEnR = crate::BitReader;
#[doc = "Field `r_ru_alloc_dbw160_fix_en` writer - "]
pub type RRuAllocDbw160FixEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn r_pwrtssict_pwr_ofst_seg0_db_force_out_val(
        &self,
    ) -> RPwrtssictPwrOfstSeg0DbForceOutValR {
        RPwrtssictPwrOfstSeg0DbForceOutValR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_diff_seg0_db_force_out_val(
        &self,
    ) -> RPwrtssictTssiDiffSeg0DbForceOutValR {
        RPwrtssictTssiDiffSeg0DbForceOutValR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn r_pwrtssict_tx_bandedge_cfg_force_out_val(&self) -> RPwrtssictTxBandedgeCfgForceOutValR {
        RPwrtssictTxBandedgeCfgForceOutValR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn r_mac_def_legacy_rate_idx_begin_at_4_on(&self) -> RMacDefLegacyRateIdxBeginAt4OnR {
        RMacDefLegacyRateIdxBeginAt4OnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn r_pwrtssict_pwr_ofst_seg0_db_force_out_on(&self) -> RPwrtssictPwrOfstSeg0DbForceOutOnR {
        RPwrtssictPwrOfstSeg0DbForceOutOnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_diff_seg0_db_force_out_on(
        &self,
    ) -> RPwrtssictTssiDiffSeg0DbForceOutOnR {
        RPwrtssictTssiDiffSeg0DbForceOutOnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_fast_mode_en_force_out_on(
        &self,
    ) -> RPwrtssictTssiFastModeEnForceOutOnR {
        RPwrtssictTssiFastModeEnForceOutOnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_fast_mode_en_force_out_val(
        &self,
    ) -> RPwrtssictTssiFastModeEnForceOutValR {
        RPwrtssictTssiFastModeEnForceOutValR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_msr_athestf_force_out_on(&self) -> RPwrtssictTssiMsrAthestfForceOutOnR {
        RPwrtssictTssiMsrAthestfForceOutOnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_msr_athestf_force_out_val(
        &self,
    ) -> RPwrtssictTssiMsrAthestfForceOutValR {
        RPwrtssictTssiMsrAthestfForceOutValR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn r_pwrtssict_tx_bandedge_cfg_force_out_on(&self) -> RPwrtssictTxBandedgeCfgForceOutOnR {
        RPwrtssictTxBandedgeCfgForceOutOnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn r_ru_alloc_dbw160_fix_en(&self) -> RRuAllocDbw160FixEnR {
        RRuAllocDbw160FixEnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn r_pwrtssict_pwr_ofst_seg0_db_force_out_val(
        &mut self,
    ) -> RPwrtssictPwrOfstSeg0DbForceOutValW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RPwrtssictPwrOfstSeg0DbForceOutValW::new(self, 0)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_diff_seg0_db_force_out_val(
        &mut self,
    ) -> RPwrtssictTssiDiffSeg0DbForceOutValW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RPwrtssictTssiDiffSeg0DbForceOutValW::new(self, 9)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn r_pwrtssict_tx_bandedge_cfg_force_out_val(
        &mut self,
    ) -> RPwrtssictTxBandedgeCfgForceOutValW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RPwrtssictTxBandedgeCfgForceOutValW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn r_mac_def_legacy_rate_idx_begin_at_4_on(
        &mut self,
    ) -> RMacDefLegacyRateIdxBeginAt4OnW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RMacDefLegacyRateIdxBeginAt4OnW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn r_pwrtssict_pwr_ofst_seg0_db_force_out_on(
        &mut self,
    ) -> RPwrtssictPwrOfstSeg0DbForceOutOnW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RPwrtssictPwrOfstSeg0DbForceOutOnW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_diff_seg0_db_force_out_on(
        &mut self,
    ) -> RPwrtssictTssiDiffSeg0DbForceOutOnW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RPwrtssictTssiDiffSeg0DbForceOutOnW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_fast_mode_en_force_out_on(
        &mut self,
    ) -> RPwrtssictTssiFastModeEnForceOutOnW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RPwrtssictTssiFastModeEnForceOutOnW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_fast_mode_en_force_out_val(
        &mut self,
    ) -> RPwrtssictTssiFastModeEnForceOutValW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RPwrtssictTssiFastModeEnForceOutValW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_msr_athestf_force_out_on(
        &mut self,
    ) -> RPwrtssictTssiMsrAthestfForceOutOnW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RPwrtssictTssiMsrAthestfForceOutOnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn r_pwrtssict_tssi_msr_athestf_force_out_val(
        &mut self,
    ) -> RPwrtssictTssiMsrAthestfForceOutValW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RPwrtssictTssiMsrAthestfForceOutValW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn r_pwrtssict_tx_bandedge_cfg_force_out_on(
        &mut self,
    ) -> RPwrtssictTxBandedgeCfgForceOutOnW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RPwrtssictTxBandedgeCfgForceOutOnW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn r_ru_alloc_dbw160_fix_en(
        &mut self,
    ) -> RRuAllocDbw160FixEnW<'_, RegAxPwrTcmWrapperCr18Spec> {
        RRuAllocDbw160FixEnW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr18Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr18::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr18Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr18::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR18 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr18Spec {}
