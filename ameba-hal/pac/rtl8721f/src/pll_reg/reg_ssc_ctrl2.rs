#[doc = "Register `REG_SSC_CTRL2` reader"]
pub type R = crate::R<RegSscCtrl2Spec>;
#[doc = "Register `REG_SSC_CTRL2` writer"]
pub type W = crate::W<RegSscCtrl2Spec>;
#[doc = "Field `flag_initial` reader - Linear overclock control signal 0: do not perform linear overclocking; after the OC_EN 0->1 toggle, the NFcode is loaded directly 1: perform linear overclocking; after the OC_EN 0->1 toggle, ramp to the target NF value per the configured linear overclock mode"]
pub type FlagInitialR = crate::BitReader;
#[doc = "Field `flag_initial` writer - Linear overclock control signal 0: do not perform linear overclocking; after the OC_EN 0->1 toggle, the NFcode is loaded directly 1: perform linear overclocking; after the OC_EN 0->1 toggle, ramp to the target NF value per the configured linear overclock mode"]
pub type FlagInitialW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC_EN` reader - Only an OC_EN 0->1 toggle loads NCODE_T_SCPU\\[7:0\\], FCODE_T_SCPU\\[12:0\\], NCODE_SSC\\[7:0\\], FCODE_SSC\\[12:0\\],GRAN_EST\\[20:0\\]"]
pub type OcEnR = crate::BitReader;
#[doc = "Field `OC_EN` writer - Only an OC_EN 0->1 toggle loads NCODE_T_SCPU\\[7:0\\], FCODE_T_SCPU\\[12:0\\], NCODE_SSC\\[7:0\\], FCODE_SSC\\[12:0\\],GRAN_EST\\[20:0\\]"]
pub type OcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SEL_OC_MODE` reader - Linear overclock mode: 00: ramp linearly to the target value in steps of REG_OC_STEP_SET\\[9:0\\] each time. a) if \"current NF\" - \"target NF\" > REG_SEL_OC_STEP_SET\\[9:0\\], step by REG_SEL_OC_STEP_SET\\[9:0\\] each time b) if \"current NF\" - \"target NF\" <= REG_SEL_OC_STEP_SET\\[9:0\\], overclock directly to the target NF value 01: first overclock directly in one step to \"target NF code\" - delta, then continue overclocking as mode 00. delta magnitude set by REG_HS_OC_STOP_DIFF\\[1:0\\] 10: first overclock {Ncode\\[7:0\\], Fcode\\[12:6\\]} directly in one step to the target value, then continue overclocking as mode 00 11: same as mode 10"]
pub type RegSelOcModeR = crate::FieldReader;
#[doc = "Field `REG_SEL_OC_MODE` writer - Linear overclock mode: 00: ramp linearly to the target value in steps of REG_OC_STEP_SET\\[9:0\\] each time. a) if \"current NF\" - \"target NF\" > REG_SEL_OC_STEP_SET\\[9:0\\], step by REG_SEL_OC_STEP_SET\\[9:0\\] each time b) if \"current NF\" - \"target NF\" <= REG_SEL_OC_STEP_SET\\[9:0\\], overclock directly to the target NF value 01: first overclock directly in one step to \"target NF code\" - delta, then continue overclocking as mode 00. delta magnitude set by REG_HS_OC_STOP_DIFF\\[1:0\\] 10: first overclock {Ncode\\[7:0\\], Fcode\\[12:6\\]} directly in one step to the target value, then continue overclocking as mode 00 11: same as mode 10"]
pub type RegSelOcModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_OC_STEP_SET` reader - Step size per transition when linear overclocking with mode=00"]
pub type RegOcStepSetR = crate::FieldReader<u16>;
#[doc = "Field `REG_OC_STEP_SET` writer - Step size per transition when linear overclocking with mode=00"]
pub type RegOcStepSetW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `REG_HS_OC_STOP_DIFF` reader - In linear overclock mode 01, sets the delta from the target NFcode after the one-step overclock"]
pub type RegHsOcStopDiffR = crate::FieldReader;
#[doc = "Field `REG_HS_OC_STOP_DIFF` writer - In linear overclock mode 01, sets the delta from the target NFcode after the one-step overclock"]
pub type RegHsOcStopDiffW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_OC_DONE_delay` reader - How long after OC_EN 1->0 until the flag signal OC_DONE sent out by SSCD is set to 1 After detecting NFcode==target NF code, OC_DONE is set to 1 after a delay of REG_OC_DONE_delay\\[5:0\\] CKXSTAL cycles"]
pub type RegOcDoneDelayR = crate::FieldReader;
#[doc = "Field `REG_OC_DONE_delay` writer - How long after OC_EN 1->0 until the flag signal OC_DONE sent out by SSCD is set to 1 After detecting NFcode==target NF code, OC_DONE is set to 1 after a delay of REG_OC_DONE_delay\\[5:0\\] CKXSTAL cycles"]
pub type RegOcDoneDelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Linear overclock control signal 0: do not perform linear overclocking; after the OC_EN 0->1 toggle, the NFcode is loaded directly 1: perform linear overclocking; after the OC_EN 0->1 toggle, ramp to the target NF value per the configured linear overclock mode"]
    #[inline(always)]
    pub fn flag_initial(&self) -> FlagInitialR {
        FlagInitialR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - Only an OC_EN 0->1 toggle loads NCODE_T_SCPU\\[7:0\\], FCODE_T_SCPU\\[12:0\\], NCODE_SSC\\[7:0\\], FCODE_SSC\\[12:0\\],GRAN_EST\\[20:0\\]"]
    #[inline(always)]
    pub fn oc_en(&self) -> OcEnR {
        OcEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Linear overclock mode: 00: ramp linearly to the target value in steps of REG_OC_STEP_SET\\[9:0\\] each time. a) if \"current NF\" - \"target NF\" > REG_SEL_OC_STEP_SET\\[9:0\\], step by REG_SEL_OC_STEP_SET\\[9:0\\] each time b) if \"current NF\" - \"target NF\" <= REG_SEL_OC_STEP_SET\\[9:0\\], overclock directly to the target NF value 01: first overclock directly in one step to \"target NF code\" - delta, then continue overclocking as mode 00. delta magnitude set by REG_HS_OC_STOP_DIFF\\[1:0\\] 10: first overclock {Ncode\\[7:0\\], Fcode\\[12:6\\]} directly in one step to the target value, then continue overclocking as mode 00 11: same as mode 10"]
    #[inline(always)]
    pub fn reg_sel_oc_mode(&self) -> RegSelOcModeR {
        RegSelOcModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:23 - Step size per transition when linear overclocking with mode=00"]
    #[inline(always)]
    pub fn reg_oc_step_set(&self) -> RegOcStepSetR {
        RegOcStepSetR::new(((self.bits >> 14) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:25 - In linear overclock mode 01, sets the delta from the target NFcode after the one-step overclock"]
    #[inline(always)]
    pub fn reg_hs_oc_stop_diff(&self) -> RegHsOcStopDiffR {
        RegHsOcStopDiffR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - How long after OC_EN 1->0 until the flag signal OC_DONE sent out by SSCD is set to 1 After detecting NFcode==target NF code, OC_DONE is set to 1 after a delay of REG_OC_DONE_delay\\[5:0\\] CKXSTAL cycles"]
    #[inline(always)]
    pub fn reg_oc_done_delay(&self) -> RegOcDoneDelayR {
        RegOcDoneDelayR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Linear overclock control signal 0: do not perform linear overclocking; after the OC_EN 0->1 toggle, the NFcode is loaded directly 1: perform linear overclocking; after the OC_EN 0->1 toggle, ramp to the target NF value per the configured linear overclock mode"]
    #[inline(always)]
    pub fn flag_initial(&mut self) -> FlagInitialW<'_, RegSscCtrl2Spec> {
        FlagInitialW::new(self, 0)
    }
    #[doc = "Bit 11 - Only an OC_EN 0->1 toggle loads NCODE_T_SCPU\\[7:0\\], FCODE_T_SCPU\\[12:0\\], NCODE_SSC\\[7:0\\], FCODE_SSC\\[12:0\\],GRAN_EST\\[20:0\\]"]
    #[inline(always)]
    pub fn oc_en(&mut self) -> OcEnW<'_, RegSscCtrl2Spec> {
        OcEnW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Linear overclock mode: 00: ramp linearly to the target value in steps of REG_OC_STEP_SET\\[9:0\\] each time. a) if \"current NF\" - \"target NF\" > REG_SEL_OC_STEP_SET\\[9:0\\], step by REG_SEL_OC_STEP_SET\\[9:0\\] each time b) if \"current NF\" - \"target NF\" <= REG_SEL_OC_STEP_SET\\[9:0\\], overclock directly to the target NF value 01: first overclock directly in one step to \"target NF code\" - delta, then continue overclocking as mode 00. delta magnitude set by REG_HS_OC_STOP_DIFF\\[1:0\\] 10: first overclock {Ncode\\[7:0\\], Fcode\\[12:6\\]} directly in one step to the target value, then continue overclocking as mode 00 11: same as mode 10"]
    #[inline(always)]
    pub fn reg_sel_oc_mode(&mut self) -> RegSelOcModeW<'_, RegSscCtrl2Spec> {
        RegSelOcModeW::new(self, 12)
    }
    #[doc = "Bits 14:23 - Step size per transition when linear overclocking with mode=00"]
    #[inline(always)]
    pub fn reg_oc_step_set(&mut self) -> RegOcStepSetW<'_, RegSscCtrl2Spec> {
        RegOcStepSetW::new(self, 14)
    }
    #[doc = "Bits 24:25 - In linear overclock mode 01, sets the delta from the target NFcode after the one-step overclock"]
    #[inline(always)]
    pub fn reg_hs_oc_stop_diff(&mut self) -> RegHsOcStopDiffW<'_, RegSscCtrl2Spec> {
        RegHsOcStopDiffW::new(self, 24)
    }
    #[doc = "Bits 26:31 - How long after OC_EN 1->0 until the flag signal OC_DONE sent out by SSCD is set to 1 After detecting NFcode==target NF code, OC_DONE is set to 1 after a delay of REG_OC_DONE_delay\\[5:0\\] CKXSTAL cycles"]
    #[inline(always)]
    pub fn reg_oc_done_delay(&mut self) -> RegOcDoneDelayW<'_, RegSscCtrl2Spec> {
        RegOcDoneDelayW::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ssc_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ssc_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSscCtrl2Spec;
impl crate::RegisterSpec for RegSscCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ssc_ctrl2::R`](R) reader structure"]
impl crate::Readable for RegSscCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ssc_ctrl2::W`](W) writer structure"]
impl crate::Writable for RegSscCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SSC_CTRL2 to value 0x0080_0000"]
impl crate::Resettable for RegSscCtrl2Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
