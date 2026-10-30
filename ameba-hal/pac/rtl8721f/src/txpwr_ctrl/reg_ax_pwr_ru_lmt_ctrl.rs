#[doc = "Register `REG_AX_PWR_RU_LMT_CTRL` reader"]
pub type R = crate::R<RegAxPwrRuLmtCtrlSpec>;
#[doc = "Register `REG_AX_PWR_RU_LMT_CTRL` writer"]
pub type W = crate::W<RegAxPwrRuLmtCtrlSpec>;
#[doc = "Field `r_force_pwr_by_ru_limit_value` reader - Force RU lim_val enable"]
pub type RForcePwrByRuLimitValueR = crate::FieldReader<u16>;
#[doc = "Field `r_force_pwr_by_ru_limit_value` writer - Force RU lim_val enable"]
pub type RForcePwrByRuLimitValueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_pwr_by_ru_limit_en_value` reader - (ru lim val is valid in HE_TB)"]
pub type RForcePwrByRuLimitEnValueR = crate::BitReader;
#[doc = "Field `r_force_pwr_by_ru_limit_en_value` writer - (ru lim val is valid in HE_TB)"]
pub type RForcePwrByRuLimitEnValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_pwr_by_ru_limit_en` reader - 1: RU_lim = r_force_pwr_by_ru_limit_value"]
pub type RForcePwrByRuLimitEnR = crate::BitReader;
#[doc = "Field `r_force_pwr_by_ru_limit_en` writer - 1: RU_lim = r_force_pwr_by_ru_limit_value"]
pub type RForcePwrByRuLimitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_RU_lim_tbl_ra` reader - (RTL_only) Ru lim indirect read addr"]
pub type RTxpwrRuLimTblRaR = crate::FieldReader;
#[doc = "Field `r_txpwr_RU_lim_tbl_ra` writer - (RTL_only) Ru lim indirect read addr"]
pub type RTxpwrRuLimTblRaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_txpwr_RU_lim_tbl_rd` reader - (RTL_only) 1:Ru lim indirect read"]
pub type RTxpwrRuLimTblRdR = crate::BitReader;
#[doc = "Field `r_txpwr_RU_lim_tbl_rd` writer - (RTL_only) 1:Ru lim indirect read"]
pub type RTxpwrRuLimTblRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_RU_lim_en` reader - (RTL_only) 1:Ru lim indirect read"]
pub type RTxpwrRuLimEnR = crate::BitReader;
#[doc = "Field `r_txpwr_RU_lim_en` writer - (RTL_only) 1:Ru lim indirect read"]
pub type RTxpwrRuLimEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txagc_LTE` reader - S(9.2) Coexist LTE pwr dbm"]
pub type RTxagcLteR = crate::FieldReader<u16>;
#[doc = "Field `r_txagc_LTE` writer - S(9.2) Coexist LTE pwr dbm"]
pub type RTxagcLteW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Force RU lim_val enable"]
    #[inline(always)]
    pub fn r_force_pwr_by_ru_limit_value(&self) -> RForcePwrByRuLimitValueR {
        RForcePwrByRuLimitValueR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - (ru lim val is valid in HE_TB)"]
    #[inline(always)]
    pub fn r_force_pwr_by_ru_limit_en_value(&self) -> RForcePwrByRuLimitEnValueR {
        RForcePwrByRuLimitEnValueR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: RU_lim = r_force_pwr_by_ru_limit_value"]
    #[inline(always)]
    pub fn r_force_pwr_by_ru_limit_en(&self) -> RForcePwrByRuLimitEnR {
        RForcePwrByRuLimitEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:16 - (RTL_only) Ru lim indirect read addr"]
    #[inline(always)]
    pub fn r_txpwr_ru_lim_tbl_ra(&self) -> RTxpwrRuLimTblRaR {
        RTxpwrRuLimTblRaR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - (RTL_only) 1:Ru lim indirect read"]
    #[inline(always)]
    pub fn r_txpwr_ru_lim_tbl_rd(&self) -> RTxpwrRuLimTblRdR {
        RTxpwrRuLimTblRdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - (RTL_only) 1:Ru lim indirect read"]
    #[inline(always)]
    pub fn r_txpwr_ru_lim_en(&self) -> RTxpwrRuLimEnR {
        RTxpwrRuLimEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:27 - S(9.2) Coexist LTE pwr dbm"]
    #[inline(always)]
    pub fn r_txagc_lte(&self) -> RTxagcLteR {
        RTxagcLteR::new(((self.bits >> 19) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Force RU lim_val enable"]
    #[inline(always)]
    pub fn r_force_pwr_by_ru_limit_value(
        &mut self,
    ) -> RForcePwrByRuLimitValueW<'_, RegAxPwrRuLmtCtrlSpec> {
        RForcePwrByRuLimitValueW::new(self, 0)
    }
    #[doc = "Bit 9 - (ru lim val is valid in HE_TB)"]
    #[inline(always)]
    pub fn r_force_pwr_by_ru_limit_en_value(
        &mut self,
    ) -> RForcePwrByRuLimitEnValueW<'_, RegAxPwrRuLmtCtrlSpec> {
        RForcePwrByRuLimitEnValueW::new(self, 9)
    }
    #[doc = "Bit 10 - 1: RU_lim = r_force_pwr_by_ru_limit_value"]
    #[inline(always)]
    pub fn r_force_pwr_by_ru_limit_en(
        &mut self,
    ) -> RForcePwrByRuLimitEnW<'_, RegAxPwrRuLmtCtrlSpec> {
        RForcePwrByRuLimitEnW::new(self, 10)
    }
    #[doc = "Bits 11:16 - (RTL_only) Ru lim indirect read addr"]
    #[inline(always)]
    pub fn r_txpwr_ru_lim_tbl_ra(&mut self) -> RTxpwrRuLimTblRaW<'_, RegAxPwrRuLmtCtrlSpec> {
        RTxpwrRuLimTblRaW::new(self, 11)
    }
    #[doc = "Bit 17 - (RTL_only) 1:Ru lim indirect read"]
    #[inline(always)]
    pub fn r_txpwr_ru_lim_tbl_rd(&mut self) -> RTxpwrRuLimTblRdW<'_, RegAxPwrRuLmtCtrlSpec> {
        RTxpwrRuLimTblRdW::new(self, 17)
    }
    #[doc = "Bit 18 - (RTL_only) 1:Ru lim indirect read"]
    #[inline(always)]
    pub fn r_txpwr_ru_lim_en(&mut self) -> RTxpwrRuLimEnW<'_, RegAxPwrRuLmtCtrlSpec> {
        RTxpwrRuLimEnW::new(self, 18)
    }
    #[doc = "Bits 19:27 - S(9.2) Coexist LTE pwr dbm"]
    #[inline(always)]
    pub fn r_txagc_lte(&mut self) -> RTxagcLteW<'_, RegAxPwrRuLmtCtrlSpec> {
        RTxagcLteW::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ru_lmt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ru_lmt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRuLmtCtrlSpec;
impl crate::RegisterSpec for RegAxPwrRuLmtCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ru_lmt_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRuLmtCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ru_lmt_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRuLmtCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RU_LMT_CTRL to value 0x0004_0200"]
impl crate::Resettable for RegAxPwrRuLmtCtrlSpec {
    const RESET_VALUE: u32 = 0x0004_0200;
}
