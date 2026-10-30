#[doc = "Register `RAWTFR_L` reader"]
pub type R = crate::R<RawtfrLSpec>;
#[doc = "Field `RAWTFR_L_Raw_Tfr` reader - Raw interrupt status for chn0-chn15"]
pub type RawtfrLRawTfrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Raw interrupt status for chn0-chn15"]
    #[inline(always)]
    pub fn rawtfr_l_raw_tfr(&self) -> RawtfrLRawTfrR {
        RawtfrLRawTfrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Raw Status for IntTfr Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`rawtfr_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawtfrLSpec;
impl crate::RegisterSpec for RawtfrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawtfr_l::R`](R) reader structure"]
impl crate::Readable for RawtfrLSpec {}
#[doc = "`reset()` method sets RAWTFR_L to value 0"]
impl crate::Resettable for RawtfrLSpec {}
