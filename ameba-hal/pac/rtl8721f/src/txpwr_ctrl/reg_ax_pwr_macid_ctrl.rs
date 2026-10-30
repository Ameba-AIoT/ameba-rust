#[doc = "Register `REG_AX_PWR_MACID_CTRL` reader"]
pub type R = crate::R<RegAxPwrMacidCtrlSpec>;
#[doc = "Register `REG_AX_PWR_MACID_CTRL` writer"]
pub type W = crate::W<RegAxPwrMacidCtrlSpec>;
#[doc = "Field `r_txpwr_lim_tbl_ra` reader - (RTL_only) pwr lim tbl indirect read addr"]
pub type RTxpwrLimTblRaR = crate::FieldReader;
#[doc = "Field `r_txpwr_lim_tbl_ra` writer - (RTL_only) pwr lim tbl indirect read addr"]
pub type RTxpwrLimTblRaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_txpwr_lim_tbl_rd` reader - (RTL_only) 1: pwr lim tbl indirect read en"]
pub type RTxpwrLimTblRdR = crate::BitReader;
#[doc = "Field `r_txpwr_lim_tbl_rd` writer - (RTL_only) 1: pwr lim tbl indirect read en"]
pub type RTxpwrLimTblRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_pwr_by_macid_value_en` reader - (RTL_only) 1:Force output Pwr_by_macid_value"]
pub type RForcePwrByMacidValueEnR = crate::BitReader;
#[doc = "Field `r_force_pwr_by_macid_value_en` writer - (RTL_only) 1:Force output Pwr_by_macid_value"]
pub type RForcePwrByMacidValueEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_pwr_by_macid_value` reader - S(8.1)Force Pwr_by_macid_value"]
pub type RForcePwrByMacidValueR = crate::FieldReader;
#[doc = "Field `r_force_pwr_by_macid_value` writer - S(8.1)Force Pwr_by_macid_value"]
pub type RForcePwrByMacidValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_force_pwr_by_macid_en` reader - 1:Force output Pwr_by_macid_en"]
pub type RForcePwrByMacidEnR = crate::BitReader;
#[doc = "Field `r_force_pwr_by_macid_en` writer - 1:Force output Pwr_by_macid_en"]
pub type RForcePwrByMacidEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_CCA_PWR_TH_en` reader - (RTL_only) 1:Force output CCA_PWR_TH_en"]
pub type RForceCcaPwrThEnR = crate::BitReader;
#[doc = "Field `r_force_CCA_PWR_TH_en` writer - (RTL_only) 1:Force output CCA_PWR_TH_en"]
pub type RForceCcaPwrThEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_CCA_PWR_TH_value` reader - S(8.1)Force CCA_PWR_TH value"]
pub type RForceCcaPwrThValueR = crate::FieldReader;
#[doc = "Field `r_force_CCA_PWR_TH_value` writer - S(8.1)Force CCA_PWR_TH value"]
pub type RForceCcaPwrThValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_force_CCA_PWR_TH_value_en` reader - 1:Force CCA_PWR_TH value_en"]
pub type RForceCcaPwrThValueEnR = crate::BitReader;
#[doc = "Field `r_force_CCA_PWR_TH_value_en` writer - 1:Force CCA_PWR_TH value_en"]
pub type RForceCcaPwrThValueEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txagc_pwr_by_macid_en` reader - (RTL_only) 1:enable pwr_by_macid"]
pub type RTxagcPwrByMacidEnR = crate::BitReader;
#[doc = "Field `r_txagc_pwr_by_macid_en` writer - (RTL_only) 1:enable pwr_by_macid"]
pub type RTxagcPwrByMacidEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - (RTL_only) pwr lim tbl indirect read addr"]
    #[inline(always)]
    pub fn r_txpwr_lim_tbl_ra(&self) -> RTxpwrLimTblRaR {
        RTxpwrLimTblRaR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - (RTL_only) 1: pwr lim tbl indirect read en"]
    #[inline(always)]
    pub fn r_txpwr_lim_tbl_rd(&self) -> RTxpwrLimTblRdR {
        RTxpwrLimTblRdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - (RTL_only) 1:Force output Pwr_by_macid_value"]
    #[inline(always)]
    pub fn r_force_pwr_by_macid_value_en(&self) -> RForcePwrByMacidValueEnR {
        RForcePwrByMacidValueEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - S(8.1)Force Pwr_by_macid_value"]
    #[inline(always)]
    pub fn r_force_pwr_by_macid_value(&self) -> RForcePwrByMacidValueR {
        RForcePwrByMacidValueR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1:Force output Pwr_by_macid_en"]
    #[inline(always)]
    pub fn r_force_pwr_by_macid_en(&self) -> RForcePwrByMacidEnR {
        RForcePwrByMacidEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - (RTL_only) 1:Force output CCA_PWR_TH_en"]
    #[inline(always)]
    pub fn r_force_cca_pwr_th_en(&self) -> RForceCcaPwrThEnR {
        RForceCcaPwrThEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:27 - S(8.1)Force CCA_PWR_TH value"]
    #[inline(always)]
    pub fn r_force_cca_pwr_th_value(&self) -> RForceCcaPwrThValueR {
        RForceCcaPwrThValueR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - 1:Force CCA_PWR_TH value_en"]
    #[inline(always)]
    pub fn r_force_cca_pwr_th_value_en(&self) -> RForceCcaPwrThValueEnR {
        RForceCcaPwrThValueEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - (RTL_only) 1:enable pwr_by_macid"]
    #[inline(always)]
    pub fn r_txagc_pwr_by_macid_en(&self) -> RTxagcPwrByMacidEnR {
        RTxagcPwrByMacidEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - (RTL_only) pwr lim tbl indirect read addr"]
    #[inline(always)]
    pub fn r_txpwr_lim_tbl_ra(&mut self) -> RTxpwrLimTblRaW<'_, RegAxPwrMacidCtrlSpec> {
        RTxpwrLimTblRaW::new(self, 0)
    }
    #[doc = "Bit 6 - (RTL_only) 1: pwr lim tbl indirect read en"]
    #[inline(always)]
    pub fn r_txpwr_lim_tbl_rd(&mut self) -> RTxpwrLimTblRdW<'_, RegAxPwrMacidCtrlSpec> {
        RTxpwrLimTblRdW::new(self, 6)
    }
    #[doc = "Bit 7 - (RTL_only) 1:Force output Pwr_by_macid_value"]
    #[inline(always)]
    pub fn r_force_pwr_by_macid_value_en(
        &mut self,
    ) -> RForcePwrByMacidValueEnW<'_, RegAxPwrMacidCtrlSpec> {
        RForcePwrByMacidValueEnW::new(self, 7)
    }
    #[doc = "Bits 8:15 - S(8.1)Force Pwr_by_macid_value"]
    #[inline(always)]
    pub fn r_force_pwr_by_macid_value(
        &mut self,
    ) -> RForcePwrByMacidValueW<'_, RegAxPwrMacidCtrlSpec> {
        RForcePwrByMacidValueW::new(self, 8)
    }
    #[doc = "Bit 16 - 1:Force output Pwr_by_macid_en"]
    #[inline(always)]
    pub fn r_force_pwr_by_macid_en(&mut self) -> RForcePwrByMacidEnW<'_, RegAxPwrMacidCtrlSpec> {
        RForcePwrByMacidEnW::new(self, 16)
    }
    #[doc = "Bit 17 - (RTL_only) 1:Force output CCA_PWR_TH_en"]
    #[inline(always)]
    pub fn r_force_cca_pwr_th_en(&mut self) -> RForceCcaPwrThEnW<'_, RegAxPwrMacidCtrlSpec> {
        RForceCcaPwrThEnW::new(self, 17)
    }
    #[doc = "Bits 20:27 - S(8.1)Force CCA_PWR_TH value"]
    #[inline(always)]
    pub fn r_force_cca_pwr_th_value(&mut self) -> RForceCcaPwrThValueW<'_, RegAxPwrMacidCtrlSpec> {
        RForceCcaPwrThValueW::new(self, 20)
    }
    #[doc = "Bit 28 - 1:Force CCA_PWR_TH value_en"]
    #[inline(always)]
    pub fn r_force_cca_pwr_th_value_en(
        &mut self,
    ) -> RForceCcaPwrThValueEnW<'_, RegAxPwrMacidCtrlSpec> {
        RForceCcaPwrThValueEnW::new(self, 28)
    }
    #[doc = "Bit 29 - (RTL_only) 1:enable pwr_by_macid"]
    #[inline(always)]
    pub fn r_txagc_pwr_by_macid_en(&mut self) -> RTxagcPwrByMacidEnW<'_, RegAxPwrMacidCtrlSpec> {
        RTxagcPwrByMacidEnW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_macid_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_macid_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrMacidCtrlSpec;
impl crate::RegisterSpec for RegAxPwrMacidCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_macid_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxPwrMacidCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_macid_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxPwrMacidCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_MACID_CTRL to value 0x2000_0080"]
impl crate::Resettable for RegAxPwrMacidCtrlSpec {
    const RESET_VALUE: u32 = 0x2000_0080;
}
