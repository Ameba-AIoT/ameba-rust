#[doc = "Register `REG_PARAM_PFM` reader"]
pub type R = crate::R<RegParamPfmSpec>;
#[doc = "Register `REG_PARAM_PFM` writer"]
pub type W = crate::W<RegParamPfmSpec>;
#[doc = "Field `reg_nmos_off_L_pfm` reader - 0:normal 1:NMOS always off"]
pub type RegNmosOffLPfmR = crate::BitReader;
#[doc = "Field `reg_nmos_off_L_pfm` writer - 0:normal 1:NMOS always off"]
pub type RegNmosOffLPfmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_prss_H_pfm` reader - 0:default 1:res+cap improve psrr"]
pub type RegPrssHPfmR = crate::BitReader;
#[doc = "Field `reg_prss_H_pfm` writer - 0:default 1:res+cap improve psrr"]
pub type RegPrssHPfmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZCD_Ioffset_b_pfm` reader - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
pub type ZcdIoffsetBPfmR = crate::FieldReader;
#[doc = "Field `ZCD_Ioffset_b_pfm` writer - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
pub type ZcdIoffsetBPfmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_SWR_ZCD_CTRL_pfm` reader - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
pub type RegSwrZcdCtrlPfmR = crate::FieldReader;
#[doc = "Field `REG_SWR_ZCD_CTRL_pfm` writer - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
pub type RegSwrZcdCtrlPfmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_mos_half_L_pfm` reader - 1:power mos half"]
pub type RegMosHalfLPfmR = crate::BitReader;
#[doc = "Field `reg_mos_half_L_pfm` writer - 1:power mos half"]
pub type RegMosHalfLPfmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAW_FREQ_L_pfm` reader - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
pub type SawFreqLPfmR = crate::FieldReader;
#[doc = "Field `SAW_FREQ_L_pfm` writer - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
pub type SawFreqLPfmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_zcdc_H_pfm` reader - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
pub type RegZcdcHPfmR = crate::FieldReader;
#[doc = "Field `reg_zcdc_H_pfm` writer - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
pub type RegZcdcHPfmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:normal 1:NMOS always off"]
    #[inline(always)]
    pub fn reg_nmos_off_l_pfm(&self) -> RegNmosOffLPfmR {
        RegNmosOffLPfmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0:default 1:res+cap improve psrr"]
    #[inline(always)]
    pub fn reg_prss_h_pfm(&self) -> RegPrssHPfmR {
        RegPrssHPfmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
    #[inline(always)]
    pub fn zcd_ioffset_b_pfm(&self) -> ZcdIoffsetBPfmR {
        ZcdIoffsetBPfmR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
    #[inline(always)]
    pub fn reg_swr_zcd_ctrl_pfm(&self) -> RegSwrZcdCtrlPfmR {
        RegSwrZcdCtrlPfmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 1:power mos half"]
    #[inline(always)]
    pub fn reg_mos_half_l_pfm(&self) -> RegMosHalfLPfmR {
        RegMosHalfLPfmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
    #[inline(always)]
    pub fn saw_freq_l_pfm(&self) -> SawFreqLPfmR {
        SawFreqLPfmR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
    #[inline(always)]
    pub fn reg_zcdc_h_pfm(&self) -> RegZcdcHPfmR {
        RegZcdcHPfmR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:normal 1:NMOS always off"]
    #[inline(always)]
    pub fn reg_nmos_off_l_pfm(&mut self) -> RegNmosOffLPfmW<'_, RegParamPfmSpec> {
        RegNmosOffLPfmW::new(self, 0)
    }
    #[doc = "Bit 1 - 0:default 1:res+cap improve psrr"]
    #[inline(always)]
    pub fn reg_prss_h_pfm(&mut self) -> RegPrssHPfmW<'_, RegParamPfmSpec> {
        RegPrssHPfmW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
    #[inline(always)]
    pub fn zcd_ioffset_b_pfm(&mut self) -> ZcdIoffsetBPfmW<'_, RegParamPfmSpec> {
        ZcdIoffsetBPfmW::new(self, 2)
    }
    #[doc = "Bits 4:5 - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
    #[inline(always)]
    pub fn reg_swr_zcd_ctrl_pfm(&mut self) -> RegSwrZcdCtrlPfmW<'_, RegParamPfmSpec> {
        RegSwrZcdCtrlPfmW::new(self, 4)
    }
    #[doc = "Bit 6 - 1:power mos half"]
    #[inline(always)]
    pub fn reg_mos_half_l_pfm(&mut self) -> RegMosHalfLPfmW<'_, RegParamPfmSpec> {
        RegMosHalfLPfmW::new(self, 6)
    }
    #[doc = "Bits 8:11 - 0000 458.0k 0001 680.2k 0010 898.9k 0011 1.113M 0100 1.321M 0101 1.522M 0110 1.716M 0111 1.902M 1000 2.087M 1001 2.258M 1010 2.426M 1011 2.582M 1100 2.743M 1101 2.888M 1110 3.035M 1111 3.165M"]
    #[inline(always)]
    pub fn saw_freq_l_pfm(&mut self) -> SawFreqLPfmW<'_, RegParamPfmSpec> {
        SawFreqLPfmW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 00:0.1u 01:0.2u(PFM) 10:0.7u(PWM) 11:0.8u"]
    #[inline(always)]
    pub fn reg_zcdc_h_pfm(&mut self) -> RegZcdcHPfmW<'_, RegParamPfmSpec> {
        RegZcdcHPfmW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_param_pfm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_param_pfm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegParamPfmSpec;
impl crate::RegisterSpec for RegParamPfmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_param_pfm::R`](R) reader structure"]
impl crate::Readable for RegParamPfmSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_param_pfm::W`](W) writer structure"]
impl crate::Writable for RegParamPfmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PARAM_PFM to value 0x2808"]
impl crate::Resettable for RegParamPfmSpec {
    const RESET_VALUE: u32 = 0x2808;
}
