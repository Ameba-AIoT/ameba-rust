#[doc = "Register `ana_read` reader"]
pub type R = crate::R<AnaReadSpec>;
#[doc = "Field `micbias_oc` reader - The status flag of MICBIAS over-current protection"]
pub type MicbiasOcR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status flag of MICBIAS over-current protection"]
    #[inline(always)]
    pub fn micbias_oc(&self) -> MicbiasOcR {
        MicbiasOcR::new((self.bits & 1) != 0)
    }
}
#[doc = "ANA Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_read::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaReadSpec;
impl crate::RegisterSpec for AnaReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_read::R`](R) reader structure"]
impl crate::Readable for AnaReadSpec {}
#[doc = "`reset()` method sets ana_read to value 0"]
impl crate::Resettable for AnaReadSpec {}
