#[doc = "Register `REG_ALDO_SYS_CTRL` reader"]
pub type R = crate::R<RegAldoSysCtrlSpec>;
#[doc = "Register `REG_ALDO_SYS_CTRL` writer"]
pub type W = crate::W<RegAldoSysCtrlSpec>;
#[doc = "Field `volt_aldo_abn` reader - same as \\[7\\]"]
pub type VoltAldoAbnR = crate::BitReader;
#[doc = "Field `volt_cldo_abn` reader - same as \\[7\\]"]
pub type VoltCldoAbnR = crate::BitReader;
#[doc = "Field `volt_pfm_abn` reader - same as \\[7\\]"]
pub type VoltPfmAbnR = crate::BitReader;
#[doc = "Field `volt_pwm_abn` reader - pwm volt adject abnormal statue . When happen , it is means current voltage is higher than request in a rising commond, same as falling case."]
pub type VoltPwmAbnR = crate::BitReader;
#[doc = "Field `vinc_step` reader - volt decrease step , unit in 100K cycle, acture value is this value + 1 Note: this is common for all LDO and SWR"]
pub type VincStepR = crate::FieldReader;
#[doc = "Field `vinc_step` writer - volt decrease step , unit in 100K cycle, acture value is this value + 1 Note: this is common for all LDO and SWR"]
pub type VincStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vdec_step` reader - volt decrease step , unit in 100K cycle, acture value is this value + 1 Note: this is common for all LDO and SWR"]
pub type VdecStepR = crate::FieldReader;
#[doc = "Field `vdec_step` writer - volt decrease step , unit in 100K cycle, acture value is this value + 1 Note: this is common for all LDO and SWR"]
pub type VdecStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code0_mldo` reader - Mem LDO code0 , which indicate 1.8v"]
pub type VoltCode0MldoR = crate::FieldReader;
#[doc = "Field `volt_code0_mldo` writer - Mem LDO code0 , which indicate 1.8v"]
pub type VoltCode0MldoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code0_aldo` reader - Aon LDO code0 , which indicate 0.7v"]
pub type VoltCode0AldoR = crate::FieldReader;
#[doc = "Field `volt_code0_aldo` writer - Aon LDO code0 , which indicate 0.7v"]
pub type VoltCode0AldoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `volt_code1_aldo` reader - Aon LDO code1 , which indicate 0.8v"]
pub type VoltCode1AldoR = crate::FieldReader;
#[doc = "Field `volt_code1_aldo` writer - Aon LDO code1 , which indicate 0.8v"]
pub type VoltCode1AldoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 4 - same as \\[7\\]"]
    #[inline(always)]
    pub fn volt_aldo_abn(&self) -> VoltAldoAbnR {
        VoltAldoAbnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - same as \\[7\\]"]
    #[inline(always)]
    pub fn volt_cldo_abn(&self) -> VoltCldoAbnR {
        VoltCldoAbnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - same as \\[7\\]"]
    #[inline(always)]
    pub fn volt_pfm_abn(&self) -> VoltPfmAbnR {
        VoltPfmAbnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pwm volt adject abnormal statue . When happen , it is means current voltage is higher than request in a rising commond, same as falling case."]
    #[inline(always)]
    pub fn volt_pwm_abn(&self) -> VoltPwmAbnR {
        VoltPwmAbnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - volt decrease step , unit in 100K cycle, acture value is this value + 1 Note: this is common for all LDO and SWR"]
    #[inline(always)]
    pub fn vinc_step(&self) -> VincStepR {
        VincStepR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - volt decrease step , unit in 100K cycle, acture value is this value + 1 Note: this is common for all LDO and SWR"]
    #[inline(always)]
    pub fn vdec_step(&self) -> VdecStepR {
        VdecStepR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Mem LDO code0 , which indicate 1.8v"]
    #[inline(always)]
    pub fn volt_code0_mldo(&self) -> VoltCode0MldoR {
        VoltCode0MldoR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Aon LDO code0 , which indicate 0.7v"]
    #[inline(always)]
    pub fn volt_code0_aldo(&self) -> VoltCode0AldoR {
        VoltCode0AldoR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Aon LDO code1 , which indicate 0.8v"]
    #[inline(always)]
    pub fn volt_code1_aldo(&self) -> VoltCode1AldoR {
        VoltCode1AldoR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - volt decrease step , unit in 100K cycle, acture value is this value + 1 Note: this is common for all LDO and SWR"]
    #[inline(always)]
    pub fn vinc_step(&mut self) -> VincStepW<'_, RegAldoSysCtrlSpec> {
        VincStepW::new(self, 8)
    }
    #[doc = "Bits 12:15 - volt decrease step , unit in 100K cycle, acture value is this value + 1 Note: this is common for all LDO and SWR"]
    #[inline(always)]
    pub fn vdec_step(&mut self) -> VdecStepW<'_, RegAldoSysCtrlSpec> {
        VdecStepW::new(self, 12)
    }
    #[doc = "Bits 20:23 - Mem LDO code0 , which indicate 1.8v"]
    #[inline(always)]
    pub fn volt_code0_mldo(&mut self) -> VoltCode0MldoW<'_, RegAldoSysCtrlSpec> {
        VoltCode0MldoW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Aon LDO code0 , which indicate 0.7v"]
    #[inline(always)]
    pub fn volt_code0_aldo(&mut self) -> VoltCode0AldoW<'_, RegAldoSysCtrlSpec> {
        VoltCode0AldoW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Aon LDO code1 , which indicate 0.8v"]
    #[inline(always)]
    pub fn volt_code1_aldo(&mut self) -> VoltCode1AldoW<'_, RegAldoSysCtrlSpec> {
        VoltCode1AldoW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aldo_sys_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_aldo_sys_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAldoSysCtrlSpec;
impl crate::RegisterSpec for RegAldoSysCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aldo_sys_ctrl::R`](R) reader structure"]
impl crate::Readable for RegAldoSysCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_aldo_sys_ctrl::W`](W) writer structure"]
impl crate::Writable for RegAldoSysCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ALDO_SYS_CTRL to value 0xa480_2200"]
impl crate::Resettable for RegAldoSysCtrlSpec {
    const RESET_VALUE: u32 = 0xa480_2200;
}
