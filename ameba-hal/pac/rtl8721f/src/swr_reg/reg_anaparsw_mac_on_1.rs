#[doc = "Register `REG_ANAPARSW_MAC_ON_1` reader"]
pub type R = crate::R<RegAnaparswMacOn1Spec>;
#[doc = "Register `REG_ANAPARSW_MAC_ON_1` writer"]
pub type W = crate::W<RegAnaparswMacOn1Spec>;
#[doc = "Field `Current_Sense_MOS` reader - 1: current sensing MOS for large OC current"]
pub type CurrentSenseMosR = crate::BitReader;
#[doc = "Field `Current_Sense_MOS` writer - 1: current sensing MOS for large OC current"]
pub type CurrentSenseMosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mos_half_L` reader - 1:power mos half"]
pub type RegMosHalfLR = crate::BitReader;
#[doc = "Field `ENFPWMDELAY_H` reader - 1:PFM=>PWM delay PWM enable and wait one clock cycle then PFM off"]
pub type EnfpwmdelayHR = crate::BitReader;
#[doc = "Field `ENFPWMDELAY_H` writer - 1:PFM=>PWM delay PWM enable and wait one clock cycle then PFM off"]
pub type EnfpwmdelayHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPWMDELAY` reader - 1:PFM<=>PWM PFM/PWM change by using DFF(CKMOD)"]
pub type FpwmdelayR = crate::BitReader;
#[doc = "Field `FPWMDELAY` writer - 1:PFM<=>PWM PFM/PWM change by using DFF(CKMOD)"]
pub type FpwmdelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_RSTB` reader - Before SS VC is, 0: pulled low 1: floating"]
pub type VcRstbR = crate::BitReader;
#[doc = "Field `VC_RSTB` writer - Before SS VC is, 0: pulled low 1: floating"]
pub type VcRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFM_IN_SEL` reader - Power MOS into PFM when PFM CTRL, 0: rising 1: falling"]
pub type PfmInSelR = crate::BitReader;
#[doc = "Field `PFM_IN_SEL` writer - Power MOS into PFM when PFM CTRL, 0: rising 1: falling"]
pub type PfmInSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC_PFM_RSTB` reader - When into PFM, VC is, 0: clamped (bit 40 also =1) 1. floating"]
pub type VcPfmRstbR = crate::BitReader;
#[doc = "Field `VC_PFM_RSTB` writer - When into PFM, VC is, 0: clamped (bit 40 also =1) 1. floating"]
pub type VcPfmRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFM_PD_RST` reader - 1: When leave PFM mode, COT logic is reset"]
pub type PfmPdRstR = crate::BitReader;
#[doc = "Field `PFM_PD_RST` writer - 1: When leave PFM mode, COT logic is reset"]
pub type PfmPdRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM_Force` reader - 0: PWM bias enabled only at PWM mode 1: Force PWM bias enable when in PWM/PFM mode"]
pub type PwmForceR = crate::BitReader;
#[doc = "Field `PWM_Force` writer - 0: PWM bias enabled only at PWM mode 1: Force PWM bias enable when in PWM/PFM mode"]
pub type PwmForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ldor_L` reader - 1:ldos res short"]
pub type RegLdorLR = crate::BitReader;
#[doc = "Field `reg_ldor_L` writer - 1:ldos res short"]
pub type RegLdorLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_adjsldo_L` reader - 0000 1.043 0001 1.075 0010 1.108 0011 1.143 0100 1.180 0101 1.220 0110 1.263 0111 1.286 1000 1.309 1001 1.333 1010 1.359 1011 1.385 1100 1.412 1101 1.444 1110 1.469 1111 1.500"]
pub type RegAdjsldoLR = crate::FieldReader;
#[doc = "Field `reg_adjsldo_L` writer - 0000 1.043 0001 1.075 0010 1.108 0011 1.143 0100 1.180 0101 1.220 0110 1.263 0111 1.286 1000 1.309 1001 1.333 1010 1.359 1011 1.385 1100 1.412 1101 1.444 1110 1.469 1111 1.500"]
pub type RegAdjsldoLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_mode_predriver` reader - 1:3 level predriver"]
pub type RegModePredriverR = crate::BitReader;
#[doc = "Field `reg_mode_predriver` writer - 1:3 level predriver"]
pub type RegModePredriverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vo_dischg_pwm_h` reader - `"]
pub type VoDischgPwmHR = crate::BitReader;
#[doc = "Field `vo_dischg_pwm_h` writer - `"]
pub type VoDischgPwmHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP_P_L` reader - PGATE PMOS driving"]
pub type SpPLR = crate::FieldReader;
#[doc = "Field `SP_P_L` writer - PGATE PMOS driving"]
pub type SpPLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SN_P_L` reader - PGATE NMOS driving"]
pub type SnPLR = crate::FieldReader;
#[doc = "Field `SN_P_L` writer - PGATE NMOS driving"]
pub type SnPLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SP_N_L` reader - NGATE PMOS driving"]
pub type SpNLR = crate::FieldReader;
#[doc = "Field `SP_N_L` writer - NGATE PMOS driving"]
pub type SpNLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SN_N_L` reader - NGATE NMOS driving"]
pub type SnNLR = crate::FieldReader;
#[doc = "Field `SN_N_L` writer - NGATE NMOS driving"]
pub type SnNLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 1: current sensing MOS for large OC current"]
    #[inline(always)]
    pub fn current_sense_mos(&self) -> CurrentSenseMosR {
        CurrentSenseMosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:power mos half"]
    #[inline(always)]
    pub fn reg_mos_half_l(&self) -> RegMosHalfLR {
        RegMosHalfLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1:PFM=>PWM delay PWM enable and wait one clock cycle then PFM off"]
    #[inline(always)]
    pub fn enfpwmdelay_h(&self) -> EnfpwmdelayHR {
        EnfpwmdelayHR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1:PFM<=>PWM PFM/PWM change by using DFF(CKMOD)"]
    #[inline(always)]
    pub fn fpwmdelay(&self) -> FpwmdelayR {
        FpwmdelayR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Before SS VC is, 0: pulled low 1: floating"]
    #[inline(always)]
    pub fn vc_rstb(&self) -> VcRstbR {
        VcRstbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power MOS into PFM when PFM CTRL, 0: rising 1: falling"]
    #[inline(always)]
    pub fn pfm_in_sel(&self) -> PfmInSelR {
        PfmInSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When into PFM, VC is, 0: clamped (bit 40 also =1) 1. floating"]
    #[inline(always)]
    pub fn vc_pfm_rstb(&self) -> VcPfmRstbR {
        VcPfmRstbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: When leave PFM mode, COT logic is reset"]
    #[inline(always)]
    pub fn pfm_pd_rst(&self) -> PfmPdRstR {
        PfmPdRstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: PWM bias enabled only at PWM mode 1: Force PWM bias enable when in PWM/PFM mode"]
    #[inline(always)]
    pub fn pwm_force(&self) -> PwmForceR {
        PwmForceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1:ldos res short"]
    #[inline(always)]
    pub fn reg_ldor_l(&self) -> RegLdorLR {
        RegLdorLR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - 0000 1.043 0001 1.075 0010 1.108 0011 1.143 0100 1.180 0101 1.220 0110 1.263 0111 1.286 1000 1.309 1001 1.333 1010 1.359 1011 1.385 1100 1.412 1101 1.444 1110 1.469 1111 1.500"]
    #[inline(always)]
    pub fn reg_adjsldo_l(&self) -> RegAdjsldoLR {
        RegAdjsldoLR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - 1:3 level predriver"]
    #[inline(always)]
    pub fn reg_mode_predriver(&self) -> RegModePredriverR {
        RegModePredriverR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - `"]
    #[inline(always)]
    pub fn vo_dischg_pwm_h(&self) -> VoDischgPwmHR {
        VoDischgPwmHR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - PGATE PMOS driving"]
    #[inline(always)]
    pub fn sp_p_l(&self) -> SpPLR {
        SpPLR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PGATE NMOS driving"]
    #[inline(always)]
    pub fn sn_p_l(&self) -> SnPLR {
        SnPLR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - NGATE PMOS driving"]
    #[inline(always)]
    pub fn sp_n_l(&self) -> SpNLR {
        SpNLR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - NGATE NMOS driving"]
    #[inline(always)]
    pub fn sn_n_l(&self) -> SnNLR {
        SnNLR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1: current sensing MOS for large OC current"]
    #[inline(always)]
    pub fn current_sense_mos(&mut self) -> CurrentSenseMosW<'_, RegAnaparswMacOn1Spec> {
        CurrentSenseMosW::new(self, 0)
    }
    #[doc = "Bit 2 - 1:PFM=>PWM delay PWM enable and wait one clock cycle then PFM off"]
    #[inline(always)]
    pub fn enfpwmdelay_h(&mut self) -> EnfpwmdelayHW<'_, RegAnaparswMacOn1Spec> {
        EnfpwmdelayHW::new(self, 2)
    }
    #[doc = "Bit 3 - 1:PFM<=>PWM PFM/PWM change by using DFF(CKMOD)"]
    #[inline(always)]
    pub fn fpwmdelay(&mut self) -> FpwmdelayW<'_, RegAnaparswMacOn1Spec> {
        FpwmdelayW::new(self, 3)
    }
    #[doc = "Bit 4 - Before SS VC is, 0: pulled low 1: floating"]
    #[inline(always)]
    pub fn vc_rstb(&mut self) -> VcRstbW<'_, RegAnaparswMacOn1Spec> {
        VcRstbW::new(self, 4)
    }
    #[doc = "Bit 5 - Power MOS into PFM when PFM CTRL, 0: rising 1: falling"]
    #[inline(always)]
    pub fn pfm_in_sel(&mut self) -> PfmInSelW<'_, RegAnaparswMacOn1Spec> {
        PfmInSelW::new(self, 5)
    }
    #[doc = "Bit 6 - When into PFM, VC is, 0: clamped (bit 40 also =1) 1. floating"]
    #[inline(always)]
    pub fn vc_pfm_rstb(&mut self) -> VcPfmRstbW<'_, RegAnaparswMacOn1Spec> {
        VcPfmRstbW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: When leave PFM mode, COT logic is reset"]
    #[inline(always)]
    pub fn pfm_pd_rst(&mut self) -> PfmPdRstW<'_, RegAnaparswMacOn1Spec> {
        PfmPdRstW::new(self, 7)
    }
    #[doc = "Bit 8 - 0: PWM bias enabled only at PWM mode 1: Force PWM bias enable when in PWM/PFM mode"]
    #[inline(always)]
    pub fn pwm_force(&mut self) -> PwmForceW<'_, RegAnaparswMacOn1Spec> {
        PwmForceW::new(self, 8)
    }
    #[doc = "Bit 9 - 1:ldos res short"]
    #[inline(always)]
    pub fn reg_ldor_l(&mut self) -> RegLdorLW<'_, RegAnaparswMacOn1Spec> {
        RegLdorLW::new(self, 9)
    }
    #[doc = "Bits 10:13 - 0000 1.043 0001 1.075 0010 1.108 0011 1.143 0100 1.180 0101 1.220 0110 1.263 0111 1.286 1000 1.309 1001 1.333 1010 1.359 1011 1.385 1100 1.412 1101 1.444 1110 1.469 1111 1.500"]
    #[inline(always)]
    pub fn reg_adjsldo_l(&mut self) -> RegAdjsldoLW<'_, RegAnaparswMacOn1Spec> {
        RegAdjsldoLW::new(self, 10)
    }
    #[doc = "Bit 14 - 1:3 level predriver"]
    #[inline(always)]
    pub fn reg_mode_predriver(&mut self) -> RegModePredriverW<'_, RegAnaparswMacOn1Spec> {
        RegModePredriverW::new(self, 14)
    }
    #[doc = "Bit 15 - `"]
    #[inline(always)]
    pub fn vo_dischg_pwm_h(&mut self) -> VoDischgPwmHW<'_, RegAnaparswMacOn1Spec> {
        VoDischgPwmHW::new(self, 15)
    }
    #[doc = "Bits 16:19 - PGATE PMOS driving"]
    #[inline(always)]
    pub fn sp_p_l(&mut self) -> SpPLW<'_, RegAnaparswMacOn1Spec> {
        SpPLW::new(self, 16)
    }
    #[doc = "Bits 20:23 - PGATE NMOS driving"]
    #[inline(always)]
    pub fn sn_p_l(&mut self) -> SnPLW<'_, RegAnaparswMacOn1Spec> {
        SnPLW::new(self, 20)
    }
    #[doc = "Bits 24:27 - NGATE PMOS driving"]
    #[inline(always)]
    pub fn sp_n_l(&mut self) -> SpNLW<'_, RegAnaparswMacOn1Spec> {
        SpNLW::new(self, 24)
    }
    #[doc = "Bits 28:31 - NGATE NMOS driving"]
    #[inline(always)]
    pub fn sn_n_l(&mut self) -> SnNLW<'_, RegAnaparswMacOn1Spec> {
        SnNLW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anaparsw_mac_on_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anaparsw_mac_on_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparswMacOn1Spec;
impl crate::RegisterSpec for RegAnaparswMacOn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anaparsw_mac_on_1::R`](R) reader structure"]
impl crate::Readable for RegAnaparswMacOn1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anaparsw_mac_on_1::W`](W) writer structure"]
impl crate::Writable for RegAnaparswMacOn1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPARSW_MAC_ON_1 to value 0x8888_66a0"]
impl crate::Resettable for RegAnaparswMacOn1Spec {
    const RESET_VALUE: u32 = 0x8888_66a0;
}
