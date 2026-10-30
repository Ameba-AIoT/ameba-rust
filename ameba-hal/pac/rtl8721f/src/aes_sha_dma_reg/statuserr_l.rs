#[doc = "Register `STATUSERR_L` reader"]
pub type R = crate::R<StatuserrLSpec>;
#[doc = "Field `STATUSERR_L_Status_Err` reader - Interrupt status for chn0-chn15. (Protocol error)"]
pub type StatuserrLStatusErrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt status for chn0-chn15. (Protocol error)"]
    #[inline(always)]
    pub fn statuserr_l_status_err(&self) -> StatuserrLStatusErrR {
        StatuserrLStatusErrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Status for IntErr Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`statuserr_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatuserrLSpec;
impl crate::RegisterSpec for StatuserrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statuserr_l::R`](R) reader structure"]
impl crate::Readable for StatuserrLSpec {}
#[doc = "`reset()` method sets STATUSERR_L to value 0"]
impl crate::Resettable for StatuserrLSpec {}
