#[doc = "Register `REG_AIP_TRIGGER` reader"]
pub type R = crate::R<RegAipTriggerSpec>;
#[doc = "Register `REG_AIP_TRIGGER` writer"]
pub type W = crate::W<RegAipTriggerSpec>;
#[doc = "Field `trg_sys_regu_sts` writer - write 1 to trigger sys_regu_sts_norm value to take effect"]
pub type TrgSysReguStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trg_sys_xtal` writer - "]
pub type TrgSysXtalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trg_sys_mldo` writer - "]
pub type TrgSysMldoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sys_regu_sts_norm` reader - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9(1.0)/SLP/PFM 101: state5 0.9(1.0)/SLP/PWM 110: state6 0.9(1.0)/NORM/PWM 111: RSVD"]
pub type SysReguStsNormR = crate::FieldReader;
#[doc = "Field `sys_regu_sts_norm` writer - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9(1.0)/SLP/PFM 101: state5 0.9(1.0)/SLP/PWM 110: state6 0.9(1.0)/NORM/PWM 111: RSVD"]
pub type SysReguStsNormW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sys_regu_sts_slp` reader - 000: RSVD 001: state1 010: state2 011: state3 100: state4 101: state5 110: state6 111: RSVD"]
pub type SysReguStsSlpR = crate::FieldReader;
#[doc = "Field `sys_regu_sts_slp` writer - 000: RSVD 001: state1 010: state2 011: state3 100: state4 101: state5 110: state6 111: RSVD"]
pub type SysReguStsSlpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sys_regu_sts_rdy` reader - Ready signal for system regu state. Software write trg register and sts different value will lead this bit set to 0 by hardware. When REGU PMC finish the flow, this bit will be set to 1 by hardware."]
pub type SysReguStsRdyR = crate::BitReader;
#[doc = "Field `sys_xmd_rdy` reader - Ready signal for system xmd state. Software write trg register and sts different value will lead this bit set to 0 by hardware. When HW finish the flow, this bit will be set to 1 by hardware."]
pub type SysXmdRdyR = crate::BitReader;
#[doc = "Field `sys_mldo_rdy` reader - Ready signal for system mldo req. Software write trg register and sts different value will lead this bit set to 0 by hardware. When HW finish the flow, this bit will be set to 1 by hardware."]
pub type SysMldoRdyR = crate::BitReader;
#[doc = "Field `sys_xtal_norm` reader - 00: OFF 001: LPS wo 40M 010: LPS with 40M , 011: NORM 100: HP"]
pub type SysXtalNormR = crate::FieldReader;
#[doc = "Field `sys_xtal_norm` writer - 00: OFF 001: LPS wo 40M 010: LPS with 40M , 011: NORM 100: HP"]
pub type SysXtalNormW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sys_xtal_slp` reader - 00: OFF 001: LPS wo 40M 010: LPS with 40M , 011: NORM 100: HP"]
pub type SysXtalSlpR = crate::FieldReader;
#[doc = "Field `sys_xtal_slp` writer - 00: OFF 001: LPS wo 40M 010: LPS with 40M , 011: NORM 100: HP"]
pub type SysXtalSlpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sys_mldo_norm` reader - mem ldo mode normal option 00:off 01:sleep (wo dummy) 10:normal-0 (wi dummy) 10:normal-1 (wo dummy)"]
pub type SysMldoNormR = crate::FieldReader;
#[doc = "Field `sys_mldo_norm` writer - mem ldo mode normal option 00:off 01:sleep (wo dummy) 10:normal-0 (wi dummy) 10:normal-1 (wo dummy)"]
pub type SysMldoNormW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sys_mldo_slp` reader - mem ldo mode slep option 00:off 01:sleep (wo dummy) 10:normal-0 (wi dummy) 10:normal-1 (wo dummy)"]
pub type SysMldoSlpR = crate::FieldReader;
#[doc = "Field `sys_mldo_slp` writer - mem ldo mode slep option 00:off 01:sleep (wo dummy) 10:normal-0 (wi dummy) 10:normal-1 (wo dummy)"]
pub type SysMldoSlpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sys_pwm_h` reader - Set pwm in heavy loading by system (SWR current > 200mA) 1:Heavy 0:Light"]
pub type SysPwmHR = crate::BitReader;
#[doc = "Field `sys_pwm_h` writer - Set pwm in heavy loading by system (SWR current > 200mA) 1:Heavy 0:Light"]
pub type SysPwmHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:10 - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9(1.0)/SLP/PFM 101: state5 0.9(1.0)/SLP/PWM 110: state6 0.9(1.0)/NORM/PWM 111: RSVD"]
    #[inline(always)]
    pub fn sys_regu_sts_norm(&self) -> SysReguStsNormR {
        SysReguStsNormR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - 000: RSVD 001: state1 010: state2 011: state3 100: state4 101: state5 110: state6 111: RSVD"]
    #[inline(always)]
    pub fn sys_regu_sts_slp(&self) -> SysReguStsSlpR {
        SysReguStsSlpR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Ready signal for system regu state. Software write trg register and sts different value will lead this bit set to 0 by hardware. When REGU PMC finish the flow, this bit will be set to 1 by hardware."]
    #[inline(always)]
    pub fn sys_regu_sts_rdy(&self) -> SysReguStsRdyR {
        SysReguStsRdyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ready signal for system xmd state. Software write trg register and sts different value will lead this bit set to 0 by hardware. When HW finish the flow, this bit will be set to 1 by hardware."]
    #[inline(always)]
    pub fn sys_xmd_rdy(&self) -> SysXmdRdyR {
        SysXmdRdyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ready signal for system mldo req. Software write trg register and sts different value will lead this bit set to 0 by hardware. When HW finish the flow, this bit will be set to 1 by hardware."]
    #[inline(always)]
    pub fn sys_mldo_rdy(&self) -> SysMldoRdyR {
        SysMldoRdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - 00: OFF 001: LPS wo 40M 010: LPS with 40M , 011: NORM 100: HP"]
    #[inline(always)]
    pub fn sys_xtal_norm(&self) -> SysXtalNormR {
        SysXtalNormR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 00: OFF 001: LPS wo 40M 010: LPS with 40M , 011: NORM 100: HP"]
    #[inline(always)]
    pub fn sys_xtal_slp(&self) -> SysXtalSlpR {
        SysXtalSlpR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - mem ldo mode normal option 00:off 01:sleep (wo dummy) 10:normal-0 (wi dummy) 10:normal-1 (wo dummy)"]
    #[inline(always)]
    pub fn sys_mldo_norm(&self) -> SysMldoNormR {
        SysMldoNormR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - mem ldo mode slep option 00:off 01:sleep (wo dummy) 10:normal-0 (wi dummy) 10:normal-1 (wo dummy)"]
    #[inline(always)]
    pub fn sys_mldo_slp(&self) -> SysMldoSlpR {
        SysMldoSlpR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 30 - Set pwm in heavy loading by system (SWR current > 200mA) 1:Heavy 0:Light"]
    #[inline(always)]
    pub fn sys_pwm_h(&self) -> SysPwmHR {
        SysPwmHR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to trigger sys_regu_sts_norm value to take effect"]
    #[inline(always)]
    pub fn trg_sys_regu_sts(&mut self) -> TrgSysReguStsW<'_, RegAipTriggerSpec> {
        TrgSysReguStsW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trg_sys_xtal(&mut self) -> TrgSysXtalW<'_, RegAipTriggerSpec> {
        TrgSysXtalW::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trg_sys_mldo(&mut self) -> TrgSysMldoW<'_, RegAipTriggerSpec> {
        TrgSysMldoW::new(self, 3)
    }
    #[doc = "Bits 8:10 - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9(1.0)/SLP/PFM 101: state5 0.9(1.0)/SLP/PWM 110: state6 0.9(1.0)/NORM/PWM 111: RSVD"]
    #[inline(always)]
    pub fn sys_regu_sts_norm(&mut self) -> SysReguStsNormW<'_, RegAipTriggerSpec> {
        SysReguStsNormW::new(self, 8)
    }
    #[doc = "Bits 11:13 - 000: RSVD 001: state1 010: state2 011: state3 100: state4 101: state5 110: state6 111: RSVD"]
    #[inline(always)]
    pub fn sys_regu_sts_slp(&mut self) -> SysReguStsSlpW<'_, RegAipTriggerSpec> {
        SysReguStsSlpW::new(self, 11)
    }
    #[doc = "Bits 17:19 - 00: OFF 001: LPS wo 40M 010: LPS with 40M , 011: NORM 100: HP"]
    #[inline(always)]
    pub fn sys_xtal_norm(&mut self) -> SysXtalNormW<'_, RegAipTriggerSpec> {
        SysXtalNormW::new(self, 17)
    }
    #[doc = "Bits 20:22 - 00: OFF 001: LPS wo 40M 010: LPS with 40M , 011: NORM 100: HP"]
    #[inline(always)]
    pub fn sys_xtal_slp(&mut self) -> SysXtalSlpW<'_, RegAipTriggerSpec> {
        SysXtalSlpW::new(self, 20)
    }
    #[doc = "Bits 24:25 - mem ldo mode normal option 00:off 01:sleep (wo dummy) 10:normal-0 (wi dummy) 10:normal-1 (wo dummy)"]
    #[inline(always)]
    pub fn sys_mldo_norm(&mut self) -> SysMldoNormW<'_, RegAipTriggerSpec> {
        SysMldoNormW::new(self, 24)
    }
    #[doc = "Bits 26:27 - mem ldo mode slep option 00:off 01:sleep (wo dummy) 10:normal-0 (wi dummy) 10:normal-1 (wo dummy)"]
    #[inline(always)]
    pub fn sys_mldo_slp(&mut self) -> SysMldoSlpW<'_, RegAipTriggerSpec> {
        SysMldoSlpW::new(self, 26)
    }
    #[doc = "Bit 30 - Set pwm in heavy loading by system (SWR current > 200mA) 1:Heavy 0:Light"]
    #[inline(always)]
    pub fn sys_pwm_h(&mut self) -> SysPwmHW<'_, RegAipTriggerSpec> {
        SysPwmHW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aip_trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_aip_trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAipTriggerSpec;
impl crate::RegisterSpec for RegAipTriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aip_trigger::R`](R) reader structure"]
impl crate::Readable for RegAipTriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_aip_trigger::W`](W) writer structure"]
impl crate::Writable for RegAipTriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AIP_TRIGGER to value 0x0216_1600"]
impl crate::Resettable for RegAipTriggerSpec {
    const RESET_VALUE: u32 = 0x0216_1600;
}
