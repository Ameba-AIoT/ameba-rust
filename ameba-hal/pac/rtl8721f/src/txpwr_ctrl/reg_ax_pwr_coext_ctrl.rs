#[doc = "Register `REG_AX_PWR_COEXT_CTRL` reader"]
pub type R = crate::R<RegAxPwrCoextCtrlSpec>;
#[doc = "Register `REG_AX_PWR_COEXT_CTRL` writer"]
pub type W = crate::W<RegAxPwrCoextCtrlSpec>;
#[doc = "Field `r_txagc_BT_en` reader - Coexist BT_en"]
pub type RTxagcBtEnR = crate::BitReader;
#[doc = "Field `r_txagc_BT_en` writer - Coexist BT_en"]
pub type RTxagcBtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txagc_LTE_en` reader - Coexist LTE_en"]
pub type RTxagcLteEnR = crate::BitReader;
#[doc = "Field `r_txagc_LTE_en` writer - Coexist LTE_en"]
pub type RTxagcLteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txagc_BT` reader - S(9.2) Coexist BT pwr dbm"]
pub type RTxagcBtR = crate::FieldReader<u16>;
#[doc = "Field `r_txagc_BT` writer - S(9.2) Coexist BT pwr dbm"]
pub type RTxagcBtW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_txpwr_mac_max_bnd` reader - (RTL_only) u(6.0) txpwr mac max bnd dB"]
pub type RTxpwrMacMaxBndR = crate::FieldReader;
#[doc = "Field `r_txpwr_mac_max_bnd` writer - (RTL_only) u(6.0) txpwr mac max bnd dB"]
pub type RTxpwrMacMaxBndW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_cck_norm_term` reader - (RTL_only) u(6.4) Map txpwr_bb to txpwr_mac 0x7A=20log(20/11)=2.625"]
pub type RCckNormTermR = crate::FieldReader;
#[doc = "Field `r_cck_norm_term` writer - (RTL_only) u(6.4) Map txpwr_bb to txpwr_mac 0x7A=20log(20/11)=2.625"]
pub type RCckNormTermW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 1 - Coexist BT_en"]
    #[inline(always)]
    pub fn r_txagc_bt_en(&self) -> RTxagcBtEnR {
        RTxagcBtEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Coexist LTE_en"]
    #[inline(always)]
    pub fn r_txagc_lte_en(&self) -> RTxagcLteEnR {
        RTxagcLteEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - S(9.2) Coexist BT pwr dbm"]
    #[inline(always)]
    pub fn r_txagc_bt(&self) -> RTxagcBtR {
        RTxagcBtR::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:17 - (RTL_only) u(6.0) txpwr mac max bnd dB"]
    #[inline(always)]
    pub fn r_txpwr_mac_max_bnd(&self) -> RTxpwrMacMaxBndR {
        RTxpwrMacMaxBndR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:24 - (RTL_only) u(6.4) Map txpwr_bb to txpwr_mac 0x7A=20log(20/11)=2.625"]
    #[inline(always)]
    pub fn r_cck_norm_term(&self) -> RCckNormTermR {
        RCckNormTermR::new(((self.bits >> 18) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Coexist BT_en"]
    #[inline(always)]
    pub fn r_txagc_bt_en(&mut self) -> RTxagcBtEnW<'_, RegAxPwrCoextCtrlSpec> {
        RTxagcBtEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Coexist LTE_en"]
    #[inline(always)]
    pub fn r_txagc_lte_en(&mut self) -> RTxagcLteEnW<'_, RegAxPwrCoextCtrlSpec> {
        RTxagcLteEnW::new(self, 2)
    }
    #[doc = "Bits 3:11 - S(9.2) Coexist BT pwr dbm"]
    #[inline(always)]
    pub fn r_txagc_bt(&mut self) -> RTxagcBtW<'_, RegAxPwrCoextCtrlSpec> {
        RTxagcBtW::new(self, 3)
    }
    #[doc = "Bits 12:17 - (RTL_only) u(6.0) txpwr mac max bnd dB"]
    #[inline(always)]
    pub fn r_txpwr_mac_max_bnd(&mut self) -> RTxpwrMacMaxBndW<'_, RegAxPwrCoextCtrlSpec> {
        RTxpwrMacMaxBndW::new(self, 12)
    }
    #[doc = "Bits 18:24 - (RTL_only) u(6.4) Map txpwr_bb to txpwr_mac 0x7A=20log(20/11)=2.625"]
    #[inline(always)]
    pub fn r_cck_norm_term(&mut self) -> RCckNormTermW<'_, RegAxPwrCoextCtrlSpec> {
        RCckNormTermW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_coext_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_coext_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrCoextCtrlSpec;
impl crate::RegisterSpec for RegAxPwrCoextCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_coext_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxPwrCoextCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_coext_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxPwrCoextCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_COEXT_CTRL to value 0x01eb_f006"]
impl crate::Resettable for RegAxPwrCoextCtrlSpec {
    const RESET_VALUE: u32 = 0x01eb_f006;
}
