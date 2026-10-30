#[doc = "Register `REG_AX_PWR_SR_CTRL0` reader"]
pub type R = crate::R<RegAxPwrSrCtrl0Spec>;
#[doc = "Register `REG_AX_PWR_SR_CTRL0` writer"]
pub type W = crate::W<RegAxPwrSrCtrl0Spec>;
#[doc = "Field `r_txagc_pseudo_pwr` reader - (RTL_only) S(9.2) force norm resp out txpwr_bb"]
pub type RTxagcPseudoPwrR = crate::FieldReader<u16>;
#[doc = "Field `r_txagc_pseudo_pwr` writer - (RTL_only) S(9.2) force norm resp out txpwr_bb"]
pub type RTxagcPseudoPwrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_txagc_pseudo_pwr_en` reader - (RTL_only) 1: force on norm resp out txpwr_bb"]
pub type RTxagcPseudoPwrEnR = crate::BitReader;
#[doc = "Field `r_txagc_pseudo_pwr_en` writer - (RTL_only) 1: force on norm resp out txpwr_bb"]
pub type RTxagcPseudoPwrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_ctrl_norm_resp_dbg_en` reader - (RTL_only) 0:dbg clk off for saving pwr"]
pub type RTxpwrCtrlNormRespDbgEnR = crate::BitReader;
#[doc = "Field `r_txpwr_ctrl_norm_resp_dbg_en` writer - (RTL_only) 0:dbg clk off for saving pwr"]
pub type RTxpwrCtrlNormRespDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_SR_rate_map_tbl_rd_mcs` reader - (RTL_only) indirect read of addr"]
pub type RSrRateMapTblRdMcsR = crate::FieldReader;
#[doc = "Field `r_SR_rate_map_tbl_rd_mcs` writer - (RTL_only) indirect read of addr"]
pub type RSrRateMapTblRdMcsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_SR_rate_map_tbl_rd` reader - (RTL_only) 1:indirect read of addr"]
pub type RSrRateMapTblRdR = crate::BitReader;
#[doc = "Field `r_SR_rate_map_tbl_rd` writer - (RTL_only) 1:indirect read of addr"]
pub type RSrRateMapTblRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_SR_txpwr_pd_with_PD_macid` reader - (RTL_only) 1:txpwr_PD from internal buffer 0: from MAC"]
pub type RSrTxpwrPdWithPdMacidR = crate::BitReader;
#[doc = "Field `r_SR_txpwr_pd_with_PD_macid` writer - (RTL_only) 1:txpwr_PD from internal buffer 0: from MAC"]
pub type RSrTxpwrPdWithPdMacidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_sta_ul_force_off` reader - (RTL_only) 1:force sta mode off"]
pub type RTxpwrStaUlForceOffR = crate::BitReader;
#[doc = "Field `r_txpwr_sta_ul_force_off` writer - (RTL_only) 1:force sta mode off"]
pub type RTxpwrStaUlForceOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_sr_force_off` reader - (RTL_only) 1:force sr mode off"]
pub type RTxpwrSrForceOffR = crate::BitReader;
#[doc = "Field `r_txpwr_sr_force_off` writer - (RTL_only) 1:force sr mode off"]
pub type RTxpwrSrForceOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_SR_rate_map_tbl_rd_mcs_sel` reader - (RTL_only) 1:cckle 0~11 0:ht/vht/he 0~11"]
pub type RSrRateMapTblRdMcsSelR = crate::BitReader;
#[doc = "Field `r_SR_rate_map_tbl_rd_mcs_sel` writer - (RTL_only) 1:cckle 0~11 0:ht/vht/he 0~11"]
pub type RSrRateMapTblRdMcsSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_SR_rate_tbl_rd_mcs_txdiff` reader - (RTL_only) Rd MEM idx 0~47"]
pub type RSrRateTblRdMcsTxdiffR = crate::FieldReader;
#[doc = "Field `r_SR_rate_tbl_rd_mcs_txdiff` writer - (RTL_only) Rd MEM idx 0~47"]
pub type RSrRateTblRdMcsTxdiffW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `r_SR_pwr_ctrl_dbg_en` reader - (RTL_only) 0:saving pwr"]
pub type RSrPwrCtrlDbgEnR = crate::BitReader;
#[doc = "Field `r_SR_pwr_ctrl_dbg_en` writer - (RTL_only) 0:saving pwr"]
pub type RSrPwrCtrlDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - (RTL_only) S(9.2) force norm resp out txpwr_bb"]
    #[inline(always)]
    pub fn r_txagc_pseudo_pwr(&self) -> RTxagcPseudoPwrR {
        RTxagcPseudoPwrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - (RTL_only) 1: force on norm resp out txpwr_bb"]
    #[inline(always)]
    pub fn r_txagc_pseudo_pwr_en(&self) -> RTxagcPseudoPwrEnR {
        RTxagcPseudoPwrEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - (RTL_only) 0:dbg clk off for saving pwr"]
    #[inline(always)]
    pub fn r_txpwr_ctrl_norm_resp_dbg_en(&self) -> RTxpwrCtrlNormRespDbgEnR {
        RTxpwrCtrlNormRespDbgEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - (RTL_only) indirect read of addr"]
    #[inline(always)]
    pub fn r_sr_rate_map_tbl_rd_mcs(&self) -> RSrRateMapTblRdMcsR {
        RSrRateMapTblRdMcsR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - (RTL_only) 1:indirect read of addr"]
    #[inline(always)]
    pub fn r_sr_rate_map_tbl_rd(&self) -> RSrRateMapTblRdR {
        RSrRateMapTblRdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - (RTL_only) 1:txpwr_PD from internal buffer 0: from MAC"]
    #[inline(always)]
    pub fn r_sr_txpwr_pd_with_pd_macid(&self) -> RSrTxpwrPdWithPdMacidR {
        RSrTxpwrPdWithPdMacidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - (RTL_only) 1:force sta mode off"]
    #[inline(always)]
    pub fn r_txpwr_sta_ul_force_off(&self) -> RTxpwrStaUlForceOffR {
        RTxpwrStaUlForceOffR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - (RTL_only) 1:force sr mode off"]
    #[inline(always)]
    pub fn r_txpwr_sr_force_off(&self) -> RTxpwrSrForceOffR {
        RTxpwrSrForceOffR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - (RTL_only) 1:cckle 0~11 0:ht/vht/he 0~11"]
    #[inline(always)]
    pub fn r_sr_rate_map_tbl_rd_mcs_sel(&self) -> RSrRateMapTblRdMcsSelR {
        RSrRateMapTblRdMcsSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:24 - (RTL_only) Rd MEM idx 0~47"]
    #[inline(always)]
    pub fn r_sr_rate_tbl_rd_mcs_txdiff(&self) -> RSrRateTblRdMcsTxdiffR {
        RSrRateTblRdMcsTxdiffR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - (RTL_only) 0:saving pwr"]
    #[inline(always)]
    pub fn r_sr_pwr_ctrl_dbg_en(&self) -> RSrPwrCtrlDbgEnR {
        RSrPwrCtrlDbgEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - (RTL_only) S(9.2) force norm resp out txpwr_bb"]
    #[inline(always)]
    pub fn r_txagc_pseudo_pwr(&mut self) -> RTxagcPseudoPwrW<'_, RegAxPwrSrCtrl0Spec> {
        RTxagcPseudoPwrW::new(self, 0)
    }
    #[doc = "Bit 9 - (RTL_only) 1: force on norm resp out txpwr_bb"]
    #[inline(always)]
    pub fn r_txagc_pseudo_pwr_en(&mut self) -> RTxagcPseudoPwrEnW<'_, RegAxPwrSrCtrl0Spec> {
        RTxagcPseudoPwrEnW::new(self, 9)
    }
    #[doc = "Bit 10 - (RTL_only) 0:dbg clk off for saving pwr"]
    #[inline(always)]
    pub fn r_txpwr_ctrl_norm_resp_dbg_en(
        &mut self,
    ) -> RTxpwrCtrlNormRespDbgEnW<'_, RegAxPwrSrCtrl0Spec> {
        RTxpwrCtrlNormRespDbgEnW::new(self, 10)
    }
    #[doc = "Bits 11:14 - (RTL_only) indirect read of addr"]
    #[inline(always)]
    pub fn r_sr_rate_map_tbl_rd_mcs(&mut self) -> RSrRateMapTblRdMcsW<'_, RegAxPwrSrCtrl0Spec> {
        RSrRateMapTblRdMcsW::new(self, 11)
    }
    #[doc = "Bit 15 - (RTL_only) 1:indirect read of addr"]
    #[inline(always)]
    pub fn r_sr_rate_map_tbl_rd(&mut self) -> RSrRateMapTblRdW<'_, RegAxPwrSrCtrl0Spec> {
        RSrRateMapTblRdW::new(self, 15)
    }
    #[doc = "Bit 16 - (RTL_only) 1:txpwr_PD from internal buffer 0: from MAC"]
    #[inline(always)]
    pub fn r_sr_txpwr_pd_with_pd_macid(
        &mut self,
    ) -> RSrTxpwrPdWithPdMacidW<'_, RegAxPwrSrCtrl0Spec> {
        RSrTxpwrPdWithPdMacidW::new(self, 16)
    }
    #[doc = "Bit 17 - (RTL_only) 1:force sta mode off"]
    #[inline(always)]
    pub fn r_txpwr_sta_ul_force_off(&mut self) -> RTxpwrStaUlForceOffW<'_, RegAxPwrSrCtrl0Spec> {
        RTxpwrStaUlForceOffW::new(self, 17)
    }
    #[doc = "Bit 18 - (RTL_only) 1:force sr mode off"]
    #[inline(always)]
    pub fn r_txpwr_sr_force_off(&mut self) -> RTxpwrSrForceOffW<'_, RegAxPwrSrCtrl0Spec> {
        RTxpwrSrForceOffW::new(self, 18)
    }
    #[doc = "Bit 19 - (RTL_only) 1:cckle 0~11 0:ht/vht/he 0~11"]
    #[inline(always)]
    pub fn r_sr_rate_map_tbl_rd_mcs_sel(
        &mut self,
    ) -> RSrRateMapTblRdMcsSelW<'_, RegAxPwrSrCtrl0Spec> {
        RSrRateMapTblRdMcsSelW::new(self, 19)
    }
    #[doc = "Bits 20:24 - (RTL_only) Rd MEM idx 0~47"]
    #[inline(always)]
    pub fn r_sr_rate_tbl_rd_mcs_txdiff(
        &mut self,
    ) -> RSrRateTblRdMcsTxdiffW<'_, RegAxPwrSrCtrl0Spec> {
        RSrRateTblRdMcsTxdiffW::new(self, 20)
    }
    #[doc = "Bit 31 - (RTL_only) 0:saving pwr"]
    #[inline(always)]
    pub fn r_sr_pwr_ctrl_dbg_en(&mut self) -> RSrPwrCtrlDbgEnW<'_, RegAxPwrSrCtrl0Spec> {
        RSrPwrCtrlDbgEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrCtrl0Spec;
impl crate::RegisterSpec for RegAxPwrSrCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_CTRL0 to value 0x8001_0400"]
impl crate::Resettable for RegAxPwrSrCtrl0Spec {
    const RESET_VALUE: u32 = 0x8001_0400;
}
