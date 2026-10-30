#[doc = "Register `REG_CLDO_VOLT_CTRL` reader"]
pub type R = crate::R<RegCldoVoltCtrlSpec>;
#[doc = "Register `REG_CLDO_VOLT_CTRL` writer"]
pub type W = crate::W<RegCldoVoltCtrlSpec>;
#[doc = "Field `volt_code0_cldo` reader - Core LDO code0 , which indicate 0.7v , used in power cut mode"]
pub type VoltCode0CldoR = crate::FieldReader;
#[doc = "Field `volt_code0_cldo` writer - Core LDO code0 , which indicate 0.7v , used in power cut mode"]
pub type VoltCode0CldoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code1_cldo` reader - Core LDO code1 , which indicate 0.8v"]
pub type VoltCode1CldoR = crate::FieldReader;
#[doc = "Field `volt_code1_cldo` writer - Core LDO code1 , which indicate 0.8v"]
pub type VoltCode1CldoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code2_cldo` reader - Core LDO code2 , which indicate 0.9v"]
pub type VoltCode2CldoR = crate::FieldReader;
#[doc = "Field `volt_code2_cldo` writer - Core LDO code2 , which indicate 0.9v"]
pub type VoltCode2CldoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code3_cldo` reader - Core LDO code3 , which indicate 1.0v"]
pub type VoltCode3CldoR = crate::FieldReader;
#[doc = "Field `volt_code3_cldo` writer - Core LDO code3 , which indicate 1.0v"]
pub type VoltCode3CldoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Core LDO code0 , which indicate 0.7v , used in power cut mode"]
    #[inline(always)]
    pub fn volt_code0_cldo(&self) -> VoltCode0CldoR {
        VoltCode0CldoR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Core LDO code1 , which indicate 0.8v"]
    #[inline(always)]
    pub fn volt_code1_cldo(&self) -> VoltCode1CldoR {
        VoltCode1CldoR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Core LDO code2 , which indicate 0.9v"]
    #[inline(always)]
    pub fn volt_code2_cldo(&self) -> VoltCode2CldoR {
        VoltCode2CldoR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Core LDO code3 , which indicate 1.0v"]
    #[inline(always)]
    pub fn volt_code3_cldo(&self) -> VoltCode3CldoR {
        VoltCode3CldoR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Core LDO code0 , which indicate 0.7v , used in power cut mode"]
    #[inline(always)]
    pub fn volt_code0_cldo(&mut self) -> VoltCode0CldoW<'_, RegCldoVoltCtrlSpec> {
        VoltCode0CldoW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Core LDO code1 , which indicate 0.8v"]
    #[inline(always)]
    pub fn volt_code1_cldo(&mut self) -> VoltCode1CldoW<'_, RegCldoVoltCtrlSpec> {
        VoltCode1CldoW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Core LDO code2 , which indicate 0.9v"]
    #[inline(always)]
    pub fn volt_code2_cldo(&mut self) -> VoltCode2CldoW<'_, RegCldoVoltCtrlSpec> {
        VoltCode2CldoW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Core LDO code3 , which indicate 1.0v"]
    #[inline(always)]
    pub fn volt_code3_cldo(&mut self) -> VoltCode3CldoW<'_, RegCldoVoltCtrlSpec> {
        VoltCode3CldoW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cldo_volt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cldo_volt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCldoVoltCtrlSpec;
impl crate::RegisterSpec for RegCldoVoltCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cldo_volt_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCldoVoltCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cldo_volt_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCldoVoltCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CLDO_VOLT_CTRL to value 0xb730"]
impl crate::Resettable for RegCldoVoltCtrlSpec {
    const RESET_VALUE: u32 = 0xb730;
}
