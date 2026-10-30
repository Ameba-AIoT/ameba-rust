#[doc = "Register `REG_AX_PWR_RATE_CTRL` reader"]
pub type R = crate::R<RegAxPwrRateCtrlSpec>;
#[doc = "Register `REG_AX_PWR_RATE_CTRL` writer"]
pub type W = crate::W<RegAxPwrRateCtrlSpec>;
#[doc = "Field `r_force_pwr_by_rate_value` reader - S(9.2) force pwr by rate value (txagc_ref+pwr_by_rate_tbl)"]
pub type RForcePwrByRateValueR = crate::FieldReader<u16>;
#[doc = "Field `r_force_pwr_by_rate_value` writer - S(9.2) force pwr by rate value (txagc_ref+pwr_by_rate_tbl)"]
pub type RForcePwrByRateValueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_pwr_by_rate_en` reader - force pwr by rate value_en"]
pub type RForcePwrByRateEnR = crate::BitReader;
#[doc = "Field `r_force_pwr_by_rate_en` writer - force pwr by rate value_en"]
pub type RForcePwrByRateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txagc_cck_ref` reader - S(9.2) cck ref dbm"]
pub type RTxagcCckRefR = crate::FieldReader<u16>;
#[doc = "Field `r_txagc_cck_ref` writer - S(9.2) cck ref dbm"]
pub type RTxagcCckRefW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_txagc_ofdm_ref` reader - S(9.2) ofdm ref dbm"]
pub type RTxagcOfdmRefR = crate::FieldReader<u16>;
#[doc = "Field `r_txagc_ofdm_ref` writer - S(9.2) ofdm ref dbm"]
pub type RTxagcOfdmRefW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_mode_idx` reader - Force txpwr_bb from specific source 0:FOCRE_NONE (normal) 1:FORCE_PWR_BY_RATE 2:FORCE_MACID 3:FORCE_PWR_LIM 4:FORCE_PWR_RU_LIM 5:FORCE_COEXIST"]
pub type RForceModeIdxR = crate::FieldReader;
#[doc = "Field `r_force_mode_idx` writer - Force txpwr_bb from specific source 0:FOCRE_NONE (normal) 1:FORCE_PWR_BY_RATE 2:FORCE_MACID 3:FORCE_PWR_LIM 4:FORCE_PWR_RU_LIM 5:FORCE_COEXIST"]
pub type RForceModeIdxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_txpwr_ctrl_clr` reader - (RTL_only) 1: clr txpwr, sync rst"]
pub type RTxpwrCtrlClrR = crate::BitReader;
#[doc = "Field `r_txpwr_ctrl_clr` writer - (RTL_only) 1: clr txpwr, sync rst"]
pub type RTxpwrCtrlClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - S(9.2) force pwr by rate value (txagc_ref+pwr_by_rate_tbl)"]
    #[inline(always)]
    pub fn r_force_pwr_by_rate_value(&self) -> RForcePwrByRateValueR {
        RForcePwrByRateValueR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - force pwr by rate value_en"]
    #[inline(always)]
    pub fn r_force_pwr_by_rate_en(&self) -> RForcePwrByRateEnR {
        RForcePwrByRateEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:18 - S(9.2) cck ref dbm"]
    #[inline(always)]
    pub fn r_txagc_cck_ref(&self) -> RTxagcCckRefR {
        RTxagcCckRefR::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 19:27 - S(9.2) ofdm ref dbm"]
    #[inline(always)]
    pub fn r_txagc_ofdm_ref(&self) -> RTxagcOfdmRefR {
        RTxagcOfdmRefR::new(((self.bits >> 19) & 0x01ff) as u16)
    }
    #[doc = "Bits 28:30 - Force txpwr_bb from specific source 0:FOCRE_NONE (normal) 1:FORCE_PWR_BY_RATE 2:FORCE_MACID 3:FORCE_PWR_LIM 4:FORCE_PWR_RU_LIM 5:FORCE_COEXIST"]
    #[inline(always)]
    pub fn r_force_mode_idx(&self) -> RForceModeIdxR {
        RForceModeIdxR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - (RTL_only) 1: clr txpwr, sync rst"]
    #[inline(always)]
    pub fn r_txpwr_ctrl_clr(&self) -> RTxpwrCtrlClrR {
        RTxpwrCtrlClrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - S(9.2) force pwr by rate value (txagc_ref+pwr_by_rate_tbl)"]
    #[inline(always)]
    pub fn r_force_pwr_by_rate_value(&mut self) -> RForcePwrByRateValueW<'_, RegAxPwrRateCtrlSpec> {
        RForcePwrByRateValueW::new(self, 0)
    }
    #[doc = "Bit 9 - force pwr by rate value_en"]
    #[inline(always)]
    pub fn r_force_pwr_by_rate_en(&mut self) -> RForcePwrByRateEnW<'_, RegAxPwrRateCtrlSpec> {
        RForcePwrByRateEnW::new(self, 9)
    }
    #[doc = "Bits 10:18 - S(9.2) cck ref dbm"]
    #[inline(always)]
    pub fn r_txagc_cck_ref(&mut self) -> RTxagcCckRefW<'_, RegAxPwrRateCtrlSpec> {
        RTxagcCckRefW::new(self, 10)
    }
    #[doc = "Bits 19:27 - S(9.2) ofdm ref dbm"]
    #[inline(always)]
    pub fn r_txagc_ofdm_ref(&mut self) -> RTxagcOfdmRefW<'_, RegAxPwrRateCtrlSpec> {
        RTxagcOfdmRefW::new(self, 19)
    }
    #[doc = "Bits 28:30 - Force txpwr_bb from specific source 0:FOCRE_NONE (normal) 1:FORCE_PWR_BY_RATE 2:FORCE_MACID 3:FORCE_PWR_LIM 4:FORCE_PWR_RU_LIM 5:FORCE_COEXIST"]
    #[inline(always)]
    pub fn r_force_mode_idx(&mut self) -> RForceModeIdxW<'_, RegAxPwrRateCtrlSpec> {
        RForceModeIdxW::new(self, 28)
    }
    #[doc = "Bit 31 - (RTL_only) 1: clr txpwr, sync rst"]
    #[inline(always)]
    pub fn r_txpwr_ctrl_clr(&mut self) -> RTxpwrCtrlClrW<'_, RegAxPwrRateCtrlSpec> {
        RTxpwrCtrlClrW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rate_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rate_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRateCtrlSpec;
impl crate::RegisterSpec for RegAxPwrRateCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rate_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRateCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rate_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRateCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RATE_CTRL to value 0x0201_0000"]
impl crate::Resettable for RegAxPwrRateCtrlSpec {
    const RESET_VALUE: u32 = 0x0201_0000;
}
