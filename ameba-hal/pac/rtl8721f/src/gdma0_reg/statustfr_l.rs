#[doc = "Register `STATUSTFR_L` reader"]
pub type R = crate::R<StatustfrLSpec>;
#[doc = "Field `STATUSTFR_L_Status_Tfr` reader - Interrupt status for chn0-chn15"]
pub type StatustfrLStatusTfrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt status for chn0-chn15"]
    #[inline(always)]
    pub fn statustfr_l_status_tfr(&self) -> StatustfrLStatusTfrR {
        StatustfrLStatusTfrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Status for IntTfr Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`statustfr_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatustfrLSpec;
impl crate::RegisterSpec for StatustfrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statustfr_l::R`](R) reader structure"]
impl crate::Readable for StatustfrLSpec {}
#[doc = "`reset()` method sets STATUSTFR_L to value 0"]
impl crate::Resettable for StatustfrLSpec {}
