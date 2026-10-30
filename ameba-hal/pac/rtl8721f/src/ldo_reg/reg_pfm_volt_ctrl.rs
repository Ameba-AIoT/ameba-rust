#[doc = "Register `REG_PFM_VOLT_CTRL` reader"]
pub type R = crate::R<RegPfmVoltCtrlSpec>;
#[doc = "Register `REG_PFM_VOLT_CTRL` writer"]
pub type W = crate::W<RegPfmVoltCtrlSpec>;
#[doc = "Field `volt_code0_pfm` reader - SWR pfm volt code0 , which indicate 0.7v"]
pub type VoltCode0PfmR = crate::FieldReader;
#[doc = "Field `volt_code0_pfm` writer - SWR pfm volt code0 , which indicate 0.7v"]
pub type VoltCode0PfmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code1_pfm` reader - SWR pfm volt code1 , which indicate 0.8v"]
pub type VoltCode1PfmR = crate::FieldReader;
#[doc = "Field `volt_code1_pfm` writer - SWR pfm volt code1 , which indicate 0.8v"]
pub type VoltCode1PfmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code2_pfm` reader - SWR pfm volt code2 , which indicate 0.9v Note:this also used as pre power cut change point"]
pub type VoltCode2PfmR = crate::FieldReader;
#[doc = "Field `volt_code2_pfm` writer - SWR pfm volt code2 , which indicate 0.9v Note:this also used as pre power cut change point"]
pub type VoltCode2PfmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code3_pfm` reader - SWR pfm volt code3 , which indicate 1.25v (for system in 0.9v)"]
pub type VoltCode3PfmR = crate::FieldReader;
#[doc = "Field `volt_code3_pfm` writer - SWR pfm volt code3 , which indicate 1.25v (for system in 0.9v)"]
pub type VoltCode3PfmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code4_pfm` reader - SWR pfm volt code4 , which indicate 1.35v (for system in 1.0v)"]
pub type VoltCode4PfmR = crate::FieldReader;
#[doc = "Field `volt_code4_pfm` writer - SWR pfm volt code4 , which indicate 1.35v (for system in 1.0v)"]
pub type VoltCode4PfmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SWR pfm volt code0 , which indicate 0.7v"]
    #[inline(always)]
    pub fn volt_code0_pfm(&self) -> VoltCode0PfmR {
        VoltCode0PfmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SWR pfm volt code1 , which indicate 0.8v"]
    #[inline(always)]
    pub fn volt_code1_pfm(&self) -> VoltCode1PfmR {
        VoltCode1PfmR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SWR pfm volt code2 , which indicate 0.9v Note:this also used as pre power cut change point"]
    #[inline(always)]
    pub fn volt_code2_pfm(&self) -> VoltCode2PfmR {
        VoltCode2PfmR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SWR pfm volt code3 , which indicate 1.25v (for system in 0.9v)"]
    #[inline(always)]
    pub fn volt_code3_pfm(&self) -> VoltCode3PfmR {
        VoltCode3PfmR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SWR pfm volt code4 , which indicate 1.35v (for system in 1.0v)"]
    #[inline(always)]
    pub fn volt_code4_pfm(&self) -> VoltCode4PfmR {
        VoltCode4PfmR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SWR pfm volt code0 , which indicate 0.7v"]
    #[inline(always)]
    pub fn volt_code0_pfm(&mut self) -> VoltCode0PfmW<'_, RegPfmVoltCtrlSpec> {
        VoltCode0PfmW::new(self, 0)
    }
    #[doc = "Bits 4:7 - SWR pfm volt code1 , which indicate 0.8v"]
    #[inline(always)]
    pub fn volt_code1_pfm(&mut self) -> VoltCode1PfmW<'_, RegPfmVoltCtrlSpec> {
        VoltCode1PfmW::new(self, 4)
    }
    #[doc = "Bits 8:11 - SWR pfm volt code2 , which indicate 0.9v Note:this also used as pre power cut change point"]
    #[inline(always)]
    pub fn volt_code2_pfm(&mut self) -> VoltCode2PfmW<'_, RegPfmVoltCtrlSpec> {
        VoltCode2PfmW::new(self, 8)
    }
    #[doc = "Bits 12:15 - SWR pfm volt code3 , which indicate 1.25v (for system in 0.9v)"]
    #[inline(always)]
    pub fn volt_code3_pfm(&mut self) -> VoltCode3PfmW<'_, RegPfmVoltCtrlSpec> {
        VoltCode3PfmW::new(self, 12)
    }
    #[doc = "Bits 16:19 - SWR pfm volt code4 , which indicate 1.35v (for system in 1.0v)"]
    #[inline(always)]
    pub fn volt_code4_pfm(&mut self) -> VoltCode4PfmW<'_, RegPfmVoltCtrlSpec> {
        VoltCode4PfmW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pfm_volt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pfm_volt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPfmVoltCtrlSpec;
impl crate::RegisterSpec for RegPfmVoltCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pfm_volt_ctrl::R`](R) reader structure"]
impl crate::Readable for RegPfmVoltCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pfm_volt_ctrl::W`](W) writer structure"]
impl crate::Writable for RegPfmVoltCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PFM_VOLT_CTRL to value 0x000d_b642"]
impl crate::Resettable for RegPfmVoltCtrlSpec {
    const RESET_VALUE: u32 = 0x000d_b642;
}
