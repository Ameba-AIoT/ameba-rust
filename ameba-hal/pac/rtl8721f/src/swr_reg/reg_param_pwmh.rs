#[doc = "Register `REG_PARAM_PWMH` reader"]
pub type R = crate::R<RegParamPwmhSpec>;
#[doc = "Register `REG_PARAM_PWMH` writer"]
pub type W = crate::W<RegParamPwmhSpec>;
#[doc = "Field `reg_nmos_off_L_pwmh` reader - 0:normal 1:NMOS always off"]
pub type RegNmosOffLPwmhR = crate::BitReader;
#[doc = "Field `reg_nmos_off_L_pwmh` writer - 0:normal 1:NMOS always off"]
pub type RegNmosOffLPwmhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_prss_H_pwmh` reader - 0:default 1:res+cap improve psrr"]
pub type RegPrssHPwmhR = crate::BitReader;
#[doc = "Field `reg_prss_H_pwmh` writer - 0:default 1:res+cap improve psrr"]
pub type RegPrssHPwmhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZCD_Ioffset_b_pwmh` reader - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
pub type ZcdIoffsetBPwmhR = crate::FieldReader;
#[doc = "Field `ZCD_Ioffset_b_pwmh` writer - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
pub type ZcdIoffsetBPwmhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_SWR_ZCD_CTRL_pwmh` reader - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
pub type RegSwrZcdCtrlPwmhR = crate::FieldReader;
#[doc = "Field `REG_SWR_ZCD_CTRL_pwmh` writer - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
pub type RegSwrZcdCtrlPwmhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_mos_half_L_pwmh` reader - 1:power mos half"]
pub type RegMosHalfLPwmhR = crate::BitReader;
#[doc = "Field `reg_mos_half_L_pwmh` writer - 1:power mos half"]
pub type RegMosHalfLPwmhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAW_FREQ_L_pwmh` reader - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
pub type SawFreqLPwmhR = crate::FieldReader;
#[doc = "Field `SAW_FREQ_L_pwmh` writer - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
pub type SawFreqLPwmhW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_zcdc_H_pwmh` reader - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
pub type RegZcdcHPwmhR = crate::FieldReader;
#[doc = "Field `reg_zcdc_H_pwmh` writer - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
pub type RegZcdcHPwmhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:normal 1:NMOS always off"]
    #[inline(always)]
    pub fn reg_nmos_off_l_pwmh(&self) -> RegNmosOffLPwmhR {
        RegNmosOffLPwmhR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0:default 1:res+cap improve psrr"]
    #[inline(always)]
    pub fn reg_prss_h_pwmh(&self) -> RegPrssHPwmhR {
        RegPrssHPwmhR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
    #[inline(always)]
    pub fn zcd_ioffset_b_pwmh(&self) -> ZcdIoffsetBPwmhR {
        ZcdIoffsetBPwmhR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
    #[inline(always)]
    pub fn reg_swr_zcd_ctrl_pwmh(&self) -> RegSwrZcdCtrlPwmhR {
        RegSwrZcdCtrlPwmhR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 1:power mos half"]
    #[inline(always)]
    pub fn reg_mos_half_l_pwmh(&self) -> RegMosHalfLPwmhR {
        RegMosHalfLPwmhR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
    #[inline(always)]
    pub fn saw_freq_l_pwmh(&self) -> SawFreqLPwmhR {
        SawFreqLPwmhR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
    #[inline(always)]
    pub fn reg_zcdc_h_pwmh(&self) -> RegZcdcHPwmhR {
        RegZcdcHPwmhR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:normal 1:NMOS always off"]
    #[inline(always)]
    pub fn reg_nmos_off_l_pwmh(&mut self) -> RegNmosOffLPwmhW<'_, RegParamPwmhSpec> {
        RegNmosOffLPwmhW::new(self, 0)
    }
    #[doc = "Bit 1 - 0:default 1:res+cap improve psrr"]
    #[inline(always)]
    pub fn reg_prss_h_pwmh(&mut self) -> RegPrssHPwmhW<'_, RegParamPwmhSpec> {
        RegPrssHPwmhW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
    #[inline(always)]
    pub fn zcd_ioffset_b_pwmh(&mut self) -> ZcdIoffsetBPwmhW<'_, RegParamPwmhSpec> {
        ZcdIoffsetBPwmhW::new(self, 2)
    }
    #[doc = "Bits 4:5 - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
    #[inline(always)]
    pub fn reg_swr_zcd_ctrl_pwmh(&mut self) -> RegSwrZcdCtrlPwmhW<'_, RegParamPwmhSpec> {
        RegSwrZcdCtrlPwmhW::new(self, 4)
    }
    #[doc = "Bit 6 - 1:power mos half"]
    #[inline(always)]
    pub fn reg_mos_half_l_pwmh(&mut self) -> RegMosHalfLPwmhW<'_, RegParamPwmhSpec> {
        RegMosHalfLPwmhW::new(self, 6)
    }
    #[doc = "Bits 8:11 - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
    #[inline(always)]
    pub fn saw_freq_l_pwmh(&mut self) -> SawFreqLPwmhW<'_, RegParamPwmhSpec> {
        SawFreqLPwmhW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
    #[inline(always)]
    pub fn reg_zcdc_h_pwmh(&mut self) -> RegZcdcHPwmhW<'_, RegParamPwmhSpec> {
        RegZcdcHPwmhW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_param_pwmh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_param_pwmh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegParamPwmhSpec;
impl crate::RegisterSpec for RegParamPwmhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_param_pwmh::R`](R) reader structure"]
impl crate::Readable for RegParamPwmhSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_param_pwmh::W`](W) writer structure"]
impl crate::Writable for RegParamPwmhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PARAM_PWMH to value 0x2808"]
impl crate::Resettable for RegParamPwmhSpec {
    const RESET_VALUE: u32 = 0x2808;
}
