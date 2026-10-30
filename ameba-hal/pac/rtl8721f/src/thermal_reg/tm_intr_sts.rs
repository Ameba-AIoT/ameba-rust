#[doc = "Register `TM_INTR_STS` reader"]
pub type R = crate::R<TmIntrStsSpec>;
#[doc = "Field `isr_tm_high_wt` reader - Interrupt pending flag for thermal meter detect over temperature warning. * 0: normal * 1: interrupt pending"]
pub type IsrTmHighWtR = crate::BitReader;
#[doc = "Field `isr_tm_low_wt` reader - Interrupt pending flag for thermal meter detect low temperature. * 0: normal * 1: interrupt pending"]
pub type IsrTmLowWtR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt pending flag for thermal meter detect over temperature warning. * 0: normal * 1: interrupt pending"]
    #[inline(always)]
    pub fn isr_tm_high_wt(&self) -> IsrTmHighWtR {
        IsrTmHighWtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt pending flag for thermal meter detect low temperature. * 0: normal * 1: interrupt pending"]
    #[inline(always)]
    pub fn isr_tm_low_wt(&self) -> IsrTmLowWtR {
        IsrTmLowWtR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Thermal Meter Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_intr_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmIntrStsSpec;
impl crate::RegisterSpec for TmIntrStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_intr_sts::R`](R) reader structure"]
impl crate::Readable for TmIntrStsSpec {}
#[doc = "`reset()` method sets TM_INTR_STS to value 0"]
impl crate::Resettable for TmIntrStsSpec {}
