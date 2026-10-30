#[doc = "Register `REG_AX_PWR_LMT_CTRL` reader"]
pub type R = crate::R<RegAxPwrLmtCtrlSpec>;
#[doc = "Register `REG_AX_PWR_LMT_CTRL` writer"]
pub type W = crate::W<RegAxPwrLmtCtrlSpec>;
#[doc = "Field `r_txagc_BW80_80_BW40_offset` reader - S(4.1) BW80+80 - BW40 pwr offset"]
pub type RTxagcBw80_80Bw40OffsetR = crate::FieldReader;
#[doc = "Field `r_txagc_BW80_80_BW40_offset` writer - S(4.1) BW80+80 - BW40 pwr offset"]
pub type RTxagcBw80_80Bw40OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txagc_BW160_BW40_offset` reader - S(4.1)BW160 - BW40 pwr offset"]
pub type RTxagcBw160Bw40OffsetR = crate::FieldReader;
#[doc = "Field `r_txagc_BW160_BW40_offset` writer - S(4.1)BW160 - BW40 pwr offset"]
pub type RTxagcBw160Bw40OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txagc_BW80_BW40_offset` reader - S(4.1)BW80 - BW40 pwr offset"]
pub type RTxagcBw80Bw40OffsetR = crate::FieldReader;
#[doc = "Field `r_txagc_BW80_BW40_offset` writer - S(4.1)BW80 - BW40 pwr offset"]
pub type RTxagcBw80Bw40OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txagc_rfbw_40M_offset` reader - (RTL_only) S(4.1)BW40 - BW40 pwr offset"]
pub type RTxagcRfbw40mOffsetR = crate::FieldReader;
#[doc = "Field `r_txagc_rfbw_40M_offset` writer - (RTL_only) S(4.1)BW40 - BW40 pwr offset"]
pub type RTxagcRfbw40mOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txagc_BW20_BW40_offset` reader - S(4.1)BW20 - BW40 pwr offset"]
pub type RTxagcBw20Bw40OffsetR = crate::FieldReader;
#[doc = "Field `r_txagc_BW20_BW40_offset` writer - S(4.1)BW20 - BW40 pwr offset"]
pub type RTxagcBw20Bw40OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_txagc_pwr_lim_BF1_en` reader - (RTL_only) BF pwr limit enable"]
pub type RTxagcPwrLimBf1EnR = crate::BitReader;
#[doc = "Field `r_txagc_pwr_lim_BF1_en` writer - (RTL_only) BF pwr limit enable"]
pub type RTxagcPwrLimBf1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txagc_pwr_lim_BF0_en` reader - (RTL_only) None BF pwr limit enable"]
pub type RTxagcPwrLimBf0EnR = crate::BitReader;
#[doc = "Field `r_txagc_pwr_lim_BF0_en` writer - (RTL_only) None BF pwr limit enable"]
pub type RTxagcPwrLimBf0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_pwr_by_limit_BF1_en` reader - (RTL_only) 1: Force pwr_lim_BF1= r_force_pwr_by_limit_bfon_value (0xd210\\[8:0\\])"]
pub type RForcePwrByLimitBf1EnR = crate::BitReader;
#[doc = "Field `r_force_pwr_by_limit_BF1_en` writer - (RTL_only) 1: Force pwr_lim_BF1= r_force_pwr_by_limit_bfon_value (0xd210\\[8:0\\])"]
pub type RForcePwrByLimitBf1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_pwr_by_limit_BF0_en` reader - (RTL_only) 1: Force pwr_lim_BF0= r_force_pwr_by_limit_bfoff_value (0xd210\\[17:9\\])"]
pub type RForcePwrByLimitBf0EnR = crate::BitReader;
#[doc = "Field `r_force_pwr_by_limit_BF0_en` writer - (RTL_only) 1: Force pwr_lim_BF0= r_force_pwr_by_limit_bfoff_value (0xd210\\[17:9\\])"]
pub type RForcePwrByLimitBf0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_Norm_rsp_HE_TB_en_on` reader - (RTL_only) 1:Norm resp always In HE_TB"]
pub type RForceNormRspHeTbEnOnR = crate::BitReader;
#[doc = "Field `r_force_Norm_rsp_HE_TB_en_on` writer - (RTL_only) 1:Norm resp always In HE_TB"]
pub type RForceNormRspHeTbEnOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_TF_dcm_value` reader - (RTL_only) Force input TF dcm value"]
pub type RForceTfDcmValueR = crate::BitReader;
#[doc = "Field `r_force_TF_dcm_value` writer - (RTL_only) Force input TF dcm value"]
pub type RForceTfDcmValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_TF_dcm_en` reader - (RTL_only) 1:Force input TF dcm = R_force_TF_dcm_value"]
pub type RForceTfDcmEnR = crate::BitReader;
#[doc = "Field `r_force_TF_dcm_en` writer - (RTL_only) 1:Force input TF dcm = R_force_TF_dcm_value"]
pub type RForceTfDcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - S(4.1) BW80+80 - BW40 pwr offset"]
    #[inline(always)]
    pub fn r_txagc_bw80_80_bw40_offset(&self) -> RTxagcBw80_80Bw40OffsetR {
        RTxagcBw80_80Bw40OffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - S(4.1)BW160 - BW40 pwr offset"]
    #[inline(always)]
    pub fn r_txagc_bw160_bw40_offset(&self) -> RTxagcBw160Bw40OffsetR {
        RTxagcBw160Bw40OffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - S(4.1)BW80 - BW40 pwr offset"]
    #[inline(always)]
    pub fn r_txagc_bw80_bw40_offset(&self) -> RTxagcBw80Bw40OffsetR {
        RTxagcBw80Bw40OffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - (RTL_only) S(4.1)BW40 - BW40 pwr offset"]
    #[inline(always)]
    pub fn r_txagc_rfbw_40m_offset(&self) -> RTxagcRfbw40mOffsetR {
        RTxagcRfbw40mOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - S(4.1)BW20 - BW40 pwr offset"]
    #[inline(always)]
    pub fn r_txagc_bw20_bw40_offset(&self) -> RTxagcBw20Bw40OffsetR {
        RTxagcBw20Bw40OffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - (RTL_only) BF pwr limit enable"]
    #[inline(always)]
    pub fn r_txagc_pwr_lim_bf1_en(&self) -> RTxagcPwrLimBf1EnR {
        RTxagcPwrLimBf1EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - (RTL_only) None BF pwr limit enable"]
    #[inline(always)]
    pub fn r_txagc_pwr_lim_bf0_en(&self) -> RTxagcPwrLimBf0EnR {
        RTxagcPwrLimBf0EnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - (RTL_only) 1: Force pwr_lim_BF1= r_force_pwr_by_limit_bfon_value (0xd210\\[8:0\\])"]
    #[inline(always)]
    pub fn r_force_pwr_by_limit_bf1_en(&self) -> RForcePwrByLimitBf1EnR {
        RForcePwrByLimitBf1EnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - (RTL_only) 1: Force pwr_lim_BF0= r_force_pwr_by_limit_bfoff_value (0xd210\\[17:9\\])"]
    #[inline(always)]
    pub fn r_force_pwr_by_limit_bf0_en(&self) -> RForcePwrByLimitBf0EnR {
        RForcePwrByLimitBf0EnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - (RTL_only) 1:Norm resp always In HE_TB"]
    #[inline(always)]
    pub fn r_force_norm_rsp_he_tb_en_on(&self) -> RForceNormRspHeTbEnOnR {
        RForceNormRspHeTbEnOnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - (RTL_only) Force input TF dcm value"]
    #[inline(always)]
    pub fn r_force_tf_dcm_value(&self) -> RForceTfDcmValueR {
        RForceTfDcmValueR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - (RTL_only) 1:Force input TF dcm = R_force_TF_dcm_value"]
    #[inline(always)]
    pub fn r_force_tf_dcm_en(&self) -> RForceTfDcmEnR {
        RForceTfDcmEnR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - S(4.1) BW80+80 - BW40 pwr offset"]
    #[inline(always)]
    pub fn r_txagc_bw80_80_bw40_offset(
        &mut self,
    ) -> RTxagcBw80_80Bw40OffsetW<'_, RegAxPwrLmtCtrlSpec> {
        RTxagcBw80_80Bw40OffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - S(4.1)BW160 - BW40 pwr offset"]
    #[inline(always)]
    pub fn r_txagc_bw160_bw40_offset(&mut self) -> RTxagcBw160Bw40OffsetW<'_, RegAxPwrLmtCtrlSpec> {
        RTxagcBw160Bw40OffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - S(4.1)BW80 - BW40 pwr offset"]
    #[inline(always)]
    pub fn r_txagc_bw80_bw40_offset(&mut self) -> RTxagcBw80Bw40OffsetW<'_, RegAxPwrLmtCtrlSpec> {
        RTxagcBw80Bw40OffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - (RTL_only) S(4.1)BW40 - BW40 pwr offset"]
    #[inline(always)]
    pub fn r_txagc_rfbw_40m_offset(&mut self) -> RTxagcRfbw40mOffsetW<'_, RegAxPwrLmtCtrlSpec> {
        RTxagcRfbw40mOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - S(4.1)BW20 - BW40 pwr offset"]
    #[inline(always)]
    pub fn r_txagc_bw20_bw40_offset(&mut self) -> RTxagcBw20Bw40OffsetW<'_, RegAxPwrLmtCtrlSpec> {
        RTxagcBw20Bw40OffsetW::new(self, 16)
    }
    #[doc = "Bit 20 - (RTL_only) BF pwr limit enable"]
    #[inline(always)]
    pub fn r_txagc_pwr_lim_bf1_en(&mut self) -> RTxagcPwrLimBf1EnW<'_, RegAxPwrLmtCtrlSpec> {
        RTxagcPwrLimBf1EnW::new(self, 20)
    }
    #[doc = "Bit 21 - (RTL_only) None BF pwr limit enable"]
    #[inline(always)]
    pub fn r_txagc_pwr_lim_bf0_en(&mut self) -> RTxagcPwrLimBf0EnW<'_, RegAxPwrLmtCtrlSpec> {
        RTxagcPwrLimBf0EnW::new(self, 21)
    }
    #[doc = "Bit 22 - (RTL_only) 1: Force pwr_lim_BF1= r_force_pwr_by_limit_bfon_value (0xd210\\[8:0\\])"]
    #[inline(always)]
    pub fn r_force_pwr_by_limit_bf1_en(
        &mut self,
    ) -> RForcePwrByLimitBf1EnW<'_, RegAxPwrLmtCtrlSpec> {
        RForcePwrByLimitBf1EnW::new(self, 22)
    }
    #[doc = "Bit 23 - (RTL_only) 1: Force pwr_lim_BF0= r_force_pwr_by_limit_bfoff_value (0xd210\\[17:9\\])"]
    #[inline(always)]
    pub fn r_force_pwr_by_limit_bf0_en(
        &mut self,
    ) -> RForcePwrByLimitBf0EnW<'_, RegAxPwrLmtCtrlSpec> {
        RForcePwrByLimitBf0EnW::new(self, 23)
    }
    #[doc = "Bit 24 - (RTL_only) 1:Norm resp always In HE_TB"]
    #[inline(always)]
    pub fn r_force_norm_rsp_he_tb_en_on(
        &mut self,
    ) -> RForceNormRspHeTbEnOnW<'_, RegAxPwrLmtCtrlSpec> {
        RForceNormRspHeTbEnOnW::new(self, 24)
    }
    #[doc = "Bit 25 - (RTL_only) Force input TF dcm value"]
    #[inline(always)]
    pub fn r_force_tf_dcm_value(&mut self) -> RForceTfDcmValueW<'_, RegAxPwrLmtCtrlSpec> {
        RForceTfDcmValueW::new(self, 25)
    }
    #[doc = "Bit 26 - (RTL_only) 1:Force input TF dcm = R_force_TF_dcm_value"]
    #[inline(always)]
    pub fn r_force_tf_dcm_en(&mut self) -> RForceTfDcmEnW<'_, RegAxPwrLmtCtrlSpec> {
        RForceTfDcmEnW::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_lmt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_lmt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrLmtCtrlSpec;
impl crate::RegisterSpec for RegAxPwrLmtCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_lmt_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxPwrLmtCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_lmt_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxPwrLmtCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_LMT_CTRL to value 0x0030_0000"]
impl crate::Resettable for RegAxPwrLmtCtrlSpec {
    const RESET_VALUE: u32 = 0x0030_0000;
}
