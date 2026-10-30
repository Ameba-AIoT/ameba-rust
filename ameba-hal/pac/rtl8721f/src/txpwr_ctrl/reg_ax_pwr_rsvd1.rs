#[doc = "Register `REG_AX_PWR_RSVD1` reader"]
pub type R = crate::R<RegAxPwrRsvd1Spec>;
#[doc = "Register `REG_AX_PWR_RSVD1` writer"]
pub type W = crate::W<RegAxPwrRsvd1Spec>;
#[doc = "Field `r_func_rst_force_on` reader - Don't obey func rst from MAC"]
pub type RFuncRstForceOnR = crate::BitReader;
#[doc = "Field `r_func_rst_force_on` writer - Don't obey func rst from MAC"]
pub type RFuncRstForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_func_rst_force_val` reader - func rst from txpwr_ctrl _CR"]
pub type RFuncRstForceValR = crate::BitReader;
#[doc = "Field `r_func_rst_force_val` writer - func rst from txpwr_ctrl _CR"]
pub type RFuncRstForceValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pwrcom_ch20_with_data_bit_inv` reader - bit inv,txpwr_lim"]
pub type RPwrcomCh20WithDataBitInvR = crate::BitReader;
#[doc = "Field `r_pwrcom_ch20_with_data_bit_inv` writer - bit inv,txpwr_lim"]
pub type RPwrcomCh20WithDataBitInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_pwrcom_ch20_with_data_bit_rev` reader - bit rev,txpwr_lim"]
pub type RPwrcomCh20WithDataBitRevR = crate::BitReader;
#[doc = "Field `r_pwrcom_ch20_with_data_bit_rev` writer - bit rev,txpwr_lim"]
pub type RPwrcomCh20WithDataBitRevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_sta_ul_mu_off_dis` reader - 1:MU do sta_ul"]
pub type RStaUlMuOffDisR = crate::BitReader;
#[doc = "Field `r_sta_ul_mu_off_dis` writer - 1:MU do sta_ul"]
pub type RStaUlMuOffDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_pwrcom_ch20_with_data_on` reader - 1:force out on"]
pub type RForceOutPwrcomCh20WithDataOnR = crate::BitReader;
#[doc = "Field `r_force_out_pwrcom_ch20_with_data_on` writer - 1:force out on"]
pub type RForceOutPwrcomCh20WithDataOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_pwrcom_ch20_with_data_val` reader - 1:force out off"]
pub type RForceOutPwrcomCh20WithDataValR = crate::FieldReader;
#[doc = "Field `r_force_out_pwrcom_ch20_with_data_val` writer - 1:force out off"]
pub type RForceOutPwrcomCh20WithDataValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_mcs2txinfo_en` reader - 1:txinfo_bbswing=MCS"]
pub type RMcs2txinfoEnR = crate::BitReader;
#[doc = "Field `r_mcs2txinfo_en` writer - 1:txinfo_bbswing=MCS"]
pub type RMcs2txinfoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_sram_rst_force_on` reader - 1:rst sram by cr"]
pub type RSramRstForceOnR = crate::BitReader;
#[doc = "Field `r_sram_rst_force_on` writer - 1:rst sram by cr"]
pub type RSramRstForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_sram_rst_force_val` reader - rst sram by cr"]
pub type RSramRstForceValR = crate::BitReader;
#[doc = "Field `r_sram_rst_force_val` writer - rst sram by cr"]
pub type RSramRstForceValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_sta_ul_ini_upd_phase_dis` reader - 1:tst non eco bug"]
pub type RStaUlIniUpdPhaseDisR = crate::BitReader;
#[doc = "Field `r_sta_ul_ini_upd_phase_dis` writer - 1:tst non eco bug"]
pub type RStaUlIniUpdPhaseDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_sta_ul_log_norm_en` reader - 1:sta ul txpwr log norm en 0:log norm=10log(ntx)-10log(dbw)"]
pub type RStaUlLogNormEnR = crate::BitReader;
#[doc = "Field `r_sta_ul_log_norm_en` writer - 1:sta ul txpwr log norm en 0:log norm=10log(ntx)-10log(dbw)"]
pub type RStaUlLogNormEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Don't obey func rst from MAC"]
    #[inline(always)]
    pub fn r_func_rst_force_on(&self) -> RFuncRstForceOnR {
        RFuncRstForceOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - func rst from txpwr_ctrl _CR"]
    #[inline(always)]
    pub fn r_func_rst_force_val(&self) -> RFuncRstForceValR {
        RFuncRstForceValR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - bit inv,txpwr_lim"]
    #[inline(always)]
    pub fn r_pwrcom_ch20_with_data_bit_inv(&self) -> RPwrcomCh20WithDataBitInvR {
        RPwrcomCh20WithDataBitInvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - bit rev,txpwr_lim"]
    #[inline(always)]
    pub fn r_pwrcom_ch20_with_data_bit_rev(&self) -> RPwrcomCh20WithDataBitRevR {
        RPwrcomCh20WithDataBitRevR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:MU do sta_ul"]
    #[inline(always)]
    pub fn r_sta_ul_mu_off_dis(&self) -> RStaUlMuOffDisR {
        RStaUlMuOffDisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1:force out on"]
    #[inline(always)]
    pub fn r_force_out_pwrcom_ch20_with_data_on(&self) -> RForceOutPwrcomCh20WithDataOnR {
        RForceOutPwrcomCh20WithDataOnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - 1:force out off"]
    #[inline(always)]
    pub fn r_force_out_pwrcom_ch20_with_data_val(&self) -> RForceOutPwrcomCh20WithDataValR {
        RForceOutPwrcomCh20WithDataValR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bit 14 - 1:txinfo_bbswing=MCS"]
    #[inline(always)]
    pub fn r_mcs2txinfo_en(&self) -> RMcs2txinfoEnR {
        RMcs2txinfoEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1:rst sram by cr"]
    #[inline(always)]
    pub fn r_sram_rst_force_on(&self) -> RSramRstForceOnR {
        RSramRstForceOnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - rst sram by cr"]
    #[inline(always)]
    pub fn r_sram_rst_force_val(&self) -> RSramRstForceValR {
        RSramRstForceValR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - 1:tst non eco bug"]
    #[inline(always)]
    pub fn r_sta_ul_ini_upd_phase_dis(&self) -> RStaUlIniUpdPhaseDisR {
        RStaUlIniUpdPhaseDisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1:sta ul txpwr log norm en 0:log norm=10log(ntx)-10log(dbw)"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_en(&self) -> RStaUlLogNormEnR {
        RStaUlLogNormEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Don't obey func rst from MAC"]
    #[inline(always)]
    pub fn r_func_rst_force_on(&mut self) -> RFuncRstForceOnW<'_, RegAxPwrRsvd1Spec> {
        RFuncRstForceOnW::new(self, 0)
    }
    #[doc = "Bit 1 - func rst from txpwr_ctrl _CR"]
    #[inline(always)]
    pub fn r_func_rst_force_val(&mut self) -> RFuncRstForceValW<'_, RegAxPwrRsvd1Spec> {
        RFuncRstForceValW::new(self, 1)
    }
    #[doc = "Bit 2 - bit inv,txpwr_lim"]
    #[inline(always)]
    pub fn r_pwrcom_ch20_with_data_bit_inv(
        &mut self,
    ) -> RPwrcomCh20WithDataBitInvW<'_, RegAxPwrRsvd1Spec> {
        RPwrcomCh20WithDataBitInvW::new(self, 2)
    }
    #[doc = "Bit 3 - bit rev,txpwr_lim"]
    #[inline(always)]
    pub fn r_pwrcom_ch20_with_data_bit_rev(
        &mut self,
    ) -> RPwrcomCh20WithDataBitRevW<'_, RegAxPwrRsvd1Spec> {
        RPwrcomCh20WithDataBitRevW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:MU do sta_ul"]
    #[inline(always)]
    pub fn r_sta_ul_mu_off_dis(&mut self) -> RStaUlMuOffDisW<'_, RegAxPwrRsvd1Spec> {
        RStaUlMuOffDisW::new(self, 4)
    }
    #[doc = "Bit 5 - 1:force out on"]
    #[inline(always)]
    pub fn r_force_out_pwrcom_ch20_with_data_on(
        &mut self,
    ) -> RForceOutPwrcomCh20WithDataOnW<'_, RegAxPwrRsvd1Spec> {
        RForceOutPwrcomCh20WithDataOnW::new(self, 5)
    }
    #[doc = "Bits 6:13 - 1:force out off"]
    #[inline(always)]
    pub fn r_force_out_pwrcom_ch20_with_data_val(
        &mut self,
    ) -> RForceOutPwrcomCh20WithDataValW<'_, RegAxPwrRsvd1Spec> {
        RForceOutPwrcomCh20WithDataValW::new(self, 6)
    }
    #[doc = "Bit 14 - 1:txinfo_bbswing=MCS"]
    #[inline(always)]
    pub fn r_mcs2txinfo_en(&mut self) -> RMcs2txinfoEnW<'_, RegAxPwrRsvd1Spec> {
        RMcs2txinfoEnW::new(self, 14)
    }
    #[doc = "Bit 15 - 1:rst sram by cr"]
    #[inline(always)]
    pub fn r_sram_rst_force_on(&mut self) -> RSramRstForceOnW<'_, RegAxPwrRsvd1Spec> {
        RSramRstForceOnW::new(self, 15)
    }
    #[doc = "Bit 16 - rst sram by cr"]
    #[inline(always)]
    pub fn r_sram_rst_force_val(&mut self) -> RSramRstForceValW<'_, RegAxPwrRsvd1Spec> {
        RSramRstForceValW::new(self, 16)
    }
    #[doc = "Bit 30 - 1:tst non eco bug"]
    #[inline(always)]
    pub fn r_sta_ul_ini_upd_phase_dis(&mut self) -> RStaUlIniUpdPhaseDisW<'_, RegAxPwrRsvd1Spec> {
        RStaUlIniUpdPhaseDisW::new(self, 30)
    }
    #[doc = "Bit 31 - 1:sta ul txpwr log norm en 0:log norm=10log(ntx)-10log(dbw)"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_en(&mut self) -> RStaUlLogNormEnW<'_, RegAxPwrRsvd1Spec> {
        RStaUlLogNormEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rsvd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rsvd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRsvd1Spec;
impl crate::RegisterSpec for RegAxPwrRsvd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rsvd1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRsvd1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rsvd1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRsvd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RSVD1 to value 0x02"]
impl crate::Resettable for RegAxPwrRsvd1Spec {
    const RESET_VALUE: u32 = 0x02;
}
