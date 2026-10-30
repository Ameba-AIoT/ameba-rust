#[doc = "Register `REG_PMC_OPT` reader"]
pub type R = crate::R<RegPmcOptSpec>;
#[doc = "Register `REG_PMC_OPT` writer"]
pub type W = crate::W<RegPmcOptSpec>;
#[doc = "Field `PST_DSLP_LDHM` reader - 1: Enable AON LDO enter into sleep mode when PMC enter deep-sleep mode 0: Disable Note: Default not used , pmc patch may use it"]
pub type PstDslpLdhmR = crate::BitReader;
#[doc = "Field `PST_DSLP_LDHM` writer - 1: Enable AON LDO enter into sleep mode when PMC enter deep-sleep mode 0: Disable Note: Default not used , pmc patch may use it"]
pub type PstDslpLdhmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_DSLP_ERCK` reader - 1: Enable 100K clock for AON register access when PMC enter deep-sleep mode 0: Disable Note: Default not used , pmc patch may use it"]
pub type PstDslpErckR = crate::BitReader;
#[doc = "Field `PST_DSLP_ERCK` writer - 1: Enable 100K clock for AON register access when PMC enter deep-sleep mode 0: Disable Note: Default not used , pmc patch may use it"]
pub type PstDslpErckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_DSLP_ESCK` reader - 1: Enable all sdm132k/rtc32k clock when PMC enter deep-sleep mode 0: Gating part of sdm132k/rtc32k to save power when enter deep sleep"]
pub type PstDslpEsckR = crate::BitReader;
#[doc = "Field `PST_DSLP_ESCK` writer - 1: Enable all sdm132k/rtc32k clock when PMC enter deep-sleep mode 0: Gating part of sdm132k/rtc32k to save power when enter deep sleep"]
pub type PstDslpEsckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_WAIT_XTAL` reader - 1: When POF , polling xtal invalid signal and then run forward 0: not polling xtal invalid"]
pub type PstWaitXtalR = crate::BitReader;
#[doc = "Field `PST_WAIT_XTAL` writer - 1: When POF , polling xtal invalid signal and then run forward 0: not polling xtal invalid"]
pub type PstWaitXtalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdm_xtal_mode` reader - Select xtal mode when xtal used for sdm calibration. 1: Normal 0: LPS"]
pub type SdmXtalModeR = crate::BitReader;
#[doc = "Field `sdm_xtal_mode` writer - Select xtal mode when xtal used for sdm calibration. 1: Normal 0: LPS"]
pub type SdmXtalModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `snooze_pdck` reader - Select xtal auto pdck function for sdm calibration use xtal lps mode 1:Enable 0:Disable"]
pub type SnoozePdckR = crate::BitReader;
#[doc = "Field `snooze_pdck` writer - Select xtal auto pdck function for sdm calibration use xtal lps mode 1:Enable 0:Disable"]
pub type SnoozePdckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_xtal_mode` reader - Select xtal mode when xtal used for adc/ctc/comp. 1: Normal 0: LPS"]
pub type AdcXtalModeR = crate::BitReader;
#[doc = "Field `adc_xtal_mode` writer - Select xtal mode when xtal used for adc/ctc/comp. 1: Normal 0: LPS"]
pub type AdcXtalModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aon_ret_en` reader - Option if aon enter retention mode when DSLP, to save more power 1: when enter to DSLP , aon pmc will shutdown more power 0: as before"]
pub type AonRetEnR = crate::BitReader;
#[doc = "Field `aon_ret_en` writer - Option if aon enter retention mode when DSLP, to save more power 1: when enter to DSLP , aon pmc will shutdown more power 0: as before"]
pub type AonRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `thm_resume_cnt` reader - After thermal reset happen , it will wait a timer then reboot itself . This count unit in 0.65ms"]
pub type ThmResumeCntR = crate::FieldReader;
#[doc = "Field `thm_resume_cnt` writer - After thermal reset happen , it will wait a timer then reboot itself . This count unit in 0.65ms"]
pub type ThmResumeCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 1: Enable AON LDO enter into sleep mode when PMC enter deep-sleep mode 0: Disable Note: Default not used , pmc patch may use it"]
    #[inline(always)]
    pub fn pst_dslp_ldhm(&self) -> PstDslpLdhmR {
        PstDslpLdhmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable 100K clock for AON register access when PMC enter deep-sleep mode 0: Disable Note: Default not used , pmc patch may use it"]
    #[inline(always)]
    pub fn pst_dslp_erck(&self) -> PstDslpErckR {
        PstDslpErckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable all sdm132k/rtc32k clock when PMC enter deep-sleep mode 0: Gating part of sdm132k/rtc32k to save power when enter deep sleep"]
    #[inline(always)]
    pub fn pst_dslp_esck(&self) -> PstDslpEsckR {
        PstDslpEsckR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: When POF , polling xtal invalid signal and then run forward 0: not polling xtal invalid"]
    #[inline(always)]
    pub fn pst_wait_xtal(&self) -> PstWaitXtalR {
        PstWaitXtalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select xtal mode when xtal used for sdm calibration. 1: Normal 0: LPS"]
    #[inline(always)]
    pub fn sdm_xtal_mode(&self) -> SdmXtalModeR {
        SdmXtalModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select xtal auto pdck function for sdm calibration use xtal lps mode 1:Enable 0:Disable"]
    #[inline(always)]
    pub fn snooze_pdck(&self) -> SnoozePdckR {
        SnoozePdckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select xtal mode when xtal used for adc/ctc/comp. 1: Normal 0: LPS"]
    #[inline(always)]
    pub fn adc_xtal_mode(&self) -> AdcXtalModeR {
        AdcXtalModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Option if aon enter retention mode when DSLP, to save more power 1: when enter to DSLP , aon pmc will shutdown more power 0: as before"]
    #[inline(always)]
    pub fn aon_ret_en(&self) -> AonRetEnR {
        AonRetEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 24:31 - After thermal reset happen , it will wait a timer then reboot itself . This count unit in 0.65ms"]
    #[inline(always)]
    pub fn thm_resume_cnt(&self) -> ThmResumeCntR {
        ThmResumeCntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable AON LDO enter into sleep mode when PMC enter deep-sleep mode 0: Disable Note: Default not used , pmc patch may use it"]
    #[inline(always)]
    pub fn pst_dslp_ldhm(&mut self) -> PstDslpLdhmW<'_, RegPmcOptSpec> {
        PstDslpLdhmW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable 100K clock for AON register access when PMC enter deep-sleep mode 0: Disable Note: Default not used , pmc patch may use it"]
    #[inline(always)]
    pub fn pst_dslp_erck(&mut self) -> PstDslpErckW<'_, RegPmcOptSpec> {
        PstDslpErckW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Enable all sdm132k/rtc32k clock when PMC enter deep-sleep mode 0: Gating part of sdm132k/rtc32k to save power when enter deep sleep"]
    #[inline(always)]
    pub fn pst_dslp_esck(&mut self) -> PstDslpEsckW<'_, RegPmcOptSpec> {
        PstDslpEsckW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: When POF , polling xtal invalid signal and then run forward 0: not polling xtal invalid"]
    #[inline(always)]
    pub fn pst_wait_xtal(&mut self) -> PstWaitXtalW<'_, RegPmcOptSpec> {
        PstWaitXtalW::new(self, 3)
    }
    #[doc = "Bit 4 - Select xtal mode when xtal used for sdm calibration. 1: Normal 0: LPS"]
    #[inline(always)]
    pub fn sdm_xtal_mode(&mut self) -> SdmXtalModeW<'_, RegPmcOptSpec> {
        SdmXtalModeW::new(self, 4)
    }
    #[doc = "Bit 5 - Select xtal auto pdck function for sdm calibration use xtal lps mode 1:Enable 0:Disable"]
    #[inline(always)]
    pub fn snooze_pdck(&mut self) -> SnoozePdckW<'_, RegPmcOptSpec> {
        SnoozePdckW::new(self, 5)
    }
    #[doc = "Bit 6 - Select xtal mode when xtal used for adc/ctc/comp. 1: Normal 0: LPS"]
    #[inline(always)]
    pub fn adc_xtal_mode(&mut self) -> AdcXtalModeW<'_, RegPmcOptSpec> {
        AdcXtalModeW::new(self, 6)
    }
    #[doc = "Bit 8 - Option if aon enter retention mode when DSLP, to save more power 1: when enter to DSLP , aon pmc will shutdown more power 0: as before"]
    #[inline(always)]
    pub fn aon_ret_en(&mut self) -> AonRetEnW<'_, RegPmcOptSpec> {
        AonRetEnW::new(self, 8)
    }
    #[doc = "Bits 24:31 - After thermal reset happen , it will wait a timer then reboot itself . This count unit in 0.65ms"]
    #[inline(always)]
    pub fn thm_resume_cnt(&mut self) -> ThmResumeCntW<'_, RegPmcOptSpec> {
        ThmResumeCntW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pmc_opt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pmc_opt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPmcOptSpec;
impl crate::RegisterSpec for RegPmcOptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pmc_opt::R`](R) reader structure"]
impl crate::Readable for RegPmcOptSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pmc_opt::W`](W) writer structure"]
impl crate::Writable for RegPmcOptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PMC_OPT to value 0x0f00_0070"]
impl crate::Resettable for RegPmcOptSpec {
    const RESET_VALUE: u32 = 0x0f00_0070;
}
