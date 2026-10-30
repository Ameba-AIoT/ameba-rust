#[doc = "Register `REG_PWM_VOLT_CTRL` reader"]
pub type R = crate::R<RegPwmVoltCtrlSpec>;
#[doc = "Register `REG_PWM_VOLT_CTRL` writer"]
pub type W = crate::W<RegPwmVoltCtrlSpec>;
#[doc = "Field `volt_code0_pwm` reader - SWR pwm volt code0 , which indicate 0.8v (used for snooze mode , at this time , CLDO is in PC mode) Note:vref must change , pwm_vref_sel must set to 00"]
pub type VoltCode0PwmR = crate::FieldReader;
#[doc = "Field `volt_code0_pwm` writer - SWR pwm volt code0 , which indicate 0.8v (used for snooze mode , at this time , CLDO is in PC mode) Note:vref must change , pwm_vref_sel must set to 00"]
pub type VoltCode0PwmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code1_pwm` reader - NO use"]
pub type VoltCode1PwmR = crate::FieldReader;
#[doc = "Field `volt_code1_pwm` writer - NO use"]
pub type VoltCode1PwmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code2_pwm` reader - SWR pwm volt code2 , which indicate 1.25v (for system in 0.9v, snooze also use) Note:code2/code3 , pwm_vref_sel must set 11(default)"]
pub type VoltCode2PwmR = crate::FieldReader;
#[doc = "Field `volt_code2_pwm` writer - SWR pwm volt code2 , which indicate 1.25v (for system in 0.9v, snooze also use) Note:code2/code3 , pwm_vref_sel must set 11(default)"]
pub type VoltCode2PwmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code3_pwm` reader - SWR pwm volt code3 , which indicate 1.35v (for system in 1.0v)"]
pub type VoltCode3PwmR = crate::FieldReader;
#[doc = "Field `volt_code3_pwm` writer - SWR pwm volt code3 , which indicate 1.35v (for system in 1.0v)"]
pub type VoltCode3PwmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SWR pwm volt code0 , which indicate 0.8v (used for snooze mode , at this time , CLDO is in PC mode) Note:vref must change , pwm_vref_sel must set to 00"]
    #[inline(always)]
    pub fn volt_code0_pwm(&self) -> VoltCode0PwmR {
        VoltCode0PwmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - NO use"]
    #[inline(always)]
    pub fn volt_code1_pwm(&self) -> VoltCode1PwmR {
        VoltCode1PwmR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SWR pwm volt code2 , which indicate 1.25v (for system in 0.9v, snooze also use) Note:code2/code3 , pwm_vref_sel must set 11(default)"]
    #[inline(always)]
    pub fn volt_code2_pwm(&self) -> VoltCode2PwmR {
        VoltCode2PwmR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SWR pwm volt code3 , which indicate 1.35v (for system in 1.0v)"]
    #[inline(always)]
    pub fn volt_code3_pwm(&self) -> VoltCode3PwmR {
        VoltCode3PwmR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SWR pwm volt code0 , which indicate 0.8v (used for snooze mode , at this time , CLDO is in PC mode) Note:vref must change , pwm_vref_sel must set to 00"]
    #[inline(always)]
    pub fn volt_code0_pwm(&mut self) -> VoltCode0PwmW<'_, RegPwmVoltCtrlSpec> {
        VoltCode0PwmW::new(self, 0)
    }
    #[doc = "Bits 4:7 - NO use"]
    #[inline(always)]
    pub fn volt_code1_pwm(&mut self) -> VoltCode1PwmW<'_, RegPwmVoltCtrlSpec> {
        VoltCode1PwmW::new(self, 4)
    }
    #[doc = "Bits 8:11 - SWR pwm volt code2 , which indicate 1.25v (for system in 0.9v, snooze also use) Note:code2/code3 , pwm_vref_sel must set 11(default)"]
    #[inline(always)]
    pub fn volt_code2_pwm(&mut self) -> VoltCode2PwmW<'_, RegPwmVoltCtrlSpec> {
        VoltCode2PwmW::new(self, 8)
    }
    #[doc = "Bits 12:15 - SWR pwm volt code3 , which indicate 1.35v (for system in 1.0v)"]
    #[inline(always)]
    pub fn volt_code3_pwm(&mut self) -> VoltCode3PwmW<'_, RegPwmVoltCtrlSpec> {
        VoltCode3PwmW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pwm_volt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pwm_volt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPwmVoltCtrlSpec;
impl crate::RegisterSpec for RegPwmVoltCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pwm_volt_ctrl::R`](R) reader structure"]
impl crate::Readable for RegPwmVoltCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pwm_volt_ctrl::W`](W) writer structure"]
impl crate::Writable for RegPwmVoltCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PWM_VOLT_CTRL to value 0xb555"]
impl crate::Resettable for RegPwmVoltCtrlSpec {
    const RESET_VALUE: u32 = 0xb555;
}
