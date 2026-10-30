#[doc = "Register `REG_SL1_PRE_CAL_PAR` reader"]
pub type R = crate::R<RegSl1PreCalParSpec>;
#[doc = "Register `REG_SL1_PRE_CAL_PAR` writer"]
pub type W = crate::W<RegSl1PreCalParSpec>;
#[doc = "Field `SL1_PRE_CAL_PHASE` reader - 8-tap delay line to adjust rwds phase, delay rwds, dq\\[7:0\\] no change min: 0, max: 8 pad_rwds -> delay x-tap -> calibration pad_dq\\[7:0\\] -------------------> calibration"]
pub type Sl1PreCalPhaseR = crate::FieldReader;
#[doc = "Field `SL1_PRE_CAL_PHASE` writer - 8-tap delay line to adjust rwds phase, delay rwds, dq\\[7:0\\] no change min: 0, max: 8 pad_rwds -> delay x-tap -> calibration pad_dq\\[7:0\\] -------------------> calibration"]
pub type Sl1PreCalPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SL1_DQ_DLY_PHASE` reader - 8-tap delay line to adjust dq\\[7:0\\] phase, delay dq\\[7:0\\], rwds no change min: 0, max: 8 pad_rwds -------------------------> calibration pad_dq\\[7:0\\] -> delay x-tap -> calibration"]
pub type Sl1DqDlyPhaseR = crate::FieldReader;
#[doc = "Field `SL1_DQ_DLY_PHASE` writer - 8-tap delay line to adjust dq\\[7:0\\] phase, delay dq\\[7:0\\], rwds no change min: 0, max: 8 pad_rwds -------------------------> calibration pad_dq\\[7:0\\] -> delay x-tap -> calibration"]
pub type Sl1DqDlyPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 8-tap delay line to adjust rwds phase, delay rwds, dq\\[7:0\\] no change min: 0, max: 8 pad_rwds -> delay x-tap -> calibration pad_dq\\[7:0\\] -------------------> calibration"]
    #[inline(always)]
    pub fn sl1_pre_cal_phase(&self) -> Sl1PreCalPhaseR {
        Sl1PreCalPhaseR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 8-tap delay line to adjust dq\\[7:0\\] phase, delay dq\\[7:0\\], rwds no change min: 0, max: 8 pad_rwds -------------------------> calibration pad_dq\\[7:0\\] -> delay x-tap -> calibration"]
    #[inline(always)]
    pub fn sl1_dq_dly_phase(&self) -> Sl1DqDlyPhaseR {
        Sl1DqDlyPhaseR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 8-tap delay line to adjust rwds phase, delay rwds, dq\\[7:0\\] no change min: 0, max: 8 pad_rwds -> delay x-tap -> calibration pad_dq\\[7:0\\] -------------------> calibration"]
    #[inline(always)]
    pub fn sl1_pre_cal_phase(&mut self) -> Sl1PreCalPhaseW<'_, RegSl1PreCalParSpec> {
        Sl1PreCalPhaseW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 8-tap delay line to adjust dq\\[7:0\\] phase, delay dq\\[7:0\\], rwds no change min: 0, max: 8 pad_rwds -------------------------> calibration pad_dq\\[7:0\\] -> delay x-tap -> calibration"]
    #[inline(always)]
    pub fn sl1_dq_dly_phase(&mut self) -> Sl1DqDlyPhaseW<'_, RegSl1PreCalParSpec> {
        Sl1DqDlyPhaseW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sl1_pre_cal_par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sl1_pre_cal_par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSl1PreCalParSpec;
impl crate::RegisterSpec for RegSl1PreCalParSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sl1_pre_cal_par::R`](R) reader structure"]
impl crate::Readable for RegSl1PreCalParSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sl1_pre_cal_par::W`](W) writer structure"]
impl crate::Writable for RegSl1PreCalParSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SL1_PRE_CAL_PAR to value 0"]
impl crate::Resettable for RegSl1PreCalParSpec {}
