#[doc = "Register `STATUSERR_H` reader"]
pub type R = crate::R<StatuserrHSpec>;
#[doc = "Field `STATUSERR_H_Status_Err` reader - Interrupt status. (Non-secure access secure register error, Sec Error) if DMAC_SECURE_MODE has been defined; else it is reserved."]
pub type StatuserrHStatusErrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt status. (Non-secure access secure register error, Sec Error) if DMAC_SECURE_MODE has been defined; else it is reserved."]
    #[inline(always)]
    pub fn statuserr_h_status_err(&self) -> StatuserrHStatusErrR {
        StatuserrHStatusErrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Status for non-secure access secure register error if defined\n\nYou can [`read`](crate::Reg::read) this register and get [`statuserr_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatuserrHSpec;
impl crate::RegisterSpec for StatuserrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statuserr_h::R`](R) reader structure"]
impl crate::Readable for StatuserrHSpec {}
#[doc = "`reset()` method sets STATUSERR_H to value 0"]
impl crate::Resettable for StatuserrHSpec {}
