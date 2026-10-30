#[doc = "Register `REG_PARAM_PWML` reader"]
pub type R = crate::R<RegParamPwmlSpec>;
#[doc = "Register `REG_PARAM_PWML` writer"]
pub type W = crate::W<RegParamPwmlSpec>;
#[doc = "Field `reg_nmos_off_L_pwml` reader - 0:normal 1:NMOS always off"]
pub type RegNmosOffLPwmlR = crate::BitReader;
#[doc = "Field `reg_nmos_off_L_pwml` writer - 0:normal 1:NMOS always off"]
pub type RegNmosOffLPwmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_prss_H_pwml` reader - 0:default 1:res+cap improve psrr"]
pub type RegPrssHPwmlR = crate::BitReader;
#[doc = "Field `reg_prss_H_pwml` writer - 0:default 1:res+cap improve psrr"]
pub type RegPrssHPwmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZCD_Ioffset_b_pwml` reader - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
pub type ZcdIoffsetBPwmlR = crate::FieldReader;
#[doc = "Field `ZCD_Ioffset_b_pwml` writer - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
pub type ZcdIoffsetBPwmlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_SWR_ZCD_CTRL_pwml` reader - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
pub type RegSwrZcdCtrlPwmlR = crate::FieldReader;
#[doc = "Field `REG_SWR_ZCD_CTRL_pwml` writer - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
pub type RegSwrZcdCtrlPwmlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_mos_half_L_pwml` reader - 1:power mos half"]
pub type RegMosHalfLPwmlR = crate::BitReader;
#[doc = "Field `reg_mos_half_L_pwml` writer - 1:power mos half"]
pub type RegMosHalfLPwmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAW_FREQ_L_pwml` reader - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
pub type SawFreqLPwmlR = crate::FieldReader;
#[doc = "Field `SAW_FREQ_L_pwml` writer - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
pub type SawFreqLPwmlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_zcdc_H_pwml` reader - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
pub type RegZcdcHPwmlR = crate::FieldReader;
#[doc = "Field `reg_zcdc_H_pwml` writer - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
pub type RegZcdcHPwmlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:normal 1:NMOS always off"]
    #[inline(always)]
    pub fn reg_nmos_off_l_pwml(&self) -> RegNmosOffLPwmlR {
        RegNmosOffLPwmlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0:default 1:res+cap improve psrr"]
    #[inline(always)]
    pub fn reg_prss_h_pwml(&self) -> RegPrssHPwmlR {
        RegPrssHPwmlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
    #[inline(always)]
    pub fn zcd_ioffset_b_pwml(&self) -> ZcdIoffsetBPwmlR {
        ZcdIoffsetBPwmlR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
    #[inline(always)]
    pub fn reg_swr_zcd_ctrl_pwml(&self) -> RegSwrZcdCtrlPwmlR {
        RegSwrZcdCtrlPwmlR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 1:power mos half"]
    #[inline(always)]
    pub fn reg_mos_half_l_pwml(&self) -> RegMosHalfLPwmlR {
        RegMosHalfLPwmlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
    #[inline(always)]
    pub fn saw_freq_l_pwml(&self) -> SawFreqLPwmlR {
        SawFreqLPwmlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
    #[inline(always)]
    pub fn reg_zcdc_h_pwml(&self) -> RegZcdcHPwmlR {
        RegZcdcHPwmlR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:normal 1:NMOS always off"]
    #[inline(always)]
    pub fn reg_nmos_off_l_pwml(&mut self) -> RegNmosOffLPwmlW<'_, RegParamPwmlSpec> {
        RegNmosOffLPwmlW::new(self, 0)
    }
    #[doc = "Bit 1 - 0:default 1:res+cap improve psrr"]
    #[inline(always)]
    pub fn reg_prss_h_pwml(&mut self) -> RegPrssHPwmlW<'_, RegParamPwmlSpec> {
        RegPrssHPwmlW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
    #[inline(always)]
    pub fn zcd_ioffset_b_pwml(&mut self) -> ZcdIoffsetBPwmlW<'_, RegParamPwmlSpec> {
        ZcdIoffsetBPwmlW::new(self, 2)
    }
    #[doc = "Bits 4:5 - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
    #[inline(always)]
    pub fn reg_swr_zcd_ctrl_pwml(&mut self) -> RegSwrZcdCtrlPwmlW<'_, RegParamPwmlSpec> {
        RegSwrZcdCtrlPwmlW::new(self, 4)
    }
    #[doc = "Bit 6 - 1:power mos half"]
    #[inline(always)]
    pub fn reg_mos_half_l_pwml(&mut self) -> RegMosHalfLPwmlW<'_, RegParamPwmlSpec> {
        RegMosHalfLPwmlW::new(self, 6)
    }
    #[doc = "Bits 8:11 - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
    #[inline(always)]
    pub fn saw_freq_l_pwml(&mut self) -> SawFreqLPwmlW<'_, RegParamPwmlSpec> {
        SawFreqLPwmlW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
    #[inline(always)]
    pub fn reg_zcdc_h_pwml(&mut self) -> RegZcdcHPwmlW<'_, RegParamPwmlSpec> {
        RegZcdcHPwmlW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_param_pwml::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_param_pwml::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegParamPwmlSpec;
impl crate::RegisterSpec for RegParamPwmlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_param_pwml::R`](R) reader structure"]
impl crate::Readable for RegParamPwmlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_param_pwml::W`](W) writer structure"]
impl crate::Writable for RegParamPwmlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PARAM_PWML to value 0x2808"]
impl crate::Resettable for RegParamPwmlSpec {
    const RESET_VALUE: u32 = 0x2808;
}
