#[doc = "Register `RAWERR_H` reader"]
pub type R = crate::R<RawerrHSpec>;
#[doc = "Field `RAWERR_H_Raw_Err` reader - Raw interrupt status. (Non-secure access secure register error) if DMAC_SECURE_MODE has been defined; else it is reserved."]
pub type RawerrHRawErrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Raw interrupt status. (Non-secure access secure register error) if DMAC_SECURE_MODE has been defined; else it is reserved."]
    #[inline(always)]
    pub fn rawerr_h_raw_err(&self) -> RawerrHRawErrR {
        RawerrHRawErrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Raw Status for non-secure access secure register error if defined\n\nYou can [`read`](crate::Reg::read) this register and get [`rawerr_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawerrHSpec;
impl crate::RegisterSpec for RawerrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawerr_h::R`](R) reader structure"]
impl crate::Readable for RawerrHSpec {}
#[doc = "`reset()` method sets RAWERR_H to value 0"]
impl crate::Resettable for RawerrHSpec {}
