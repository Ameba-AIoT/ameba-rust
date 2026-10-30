#[doc = "Register `RAWERR_L` reader"]
pub type R = crate::R<RawerrLSpec>;
#[doc = "Field `RAWERR_L_Raw_Err` reader - Raw interrupt status for chn0-chn15. (Protocol error)"]
pub type RawerrLRawErrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Raw interrupt status for chn0-chn15. (Protocol error)"]
    #[inline(always)]
    pub fn rawerr_l_raw_err(&self) -> RawerrLRawErrR {
        RawerrLRawErrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Raw Status for IntErr Interrupt(protocol error)\n\nYou can [`read`](crate::Reg::read) this register and get [`rawerr_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawerrLSpec;
impl crate::RegisterSpec for RawerrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawerr_l::R`](R) reader structure"]
impl crate::Readable for RawerrLSpec {}
#[doc = "`reset()` method sets RAWERR_L to value 0"]
impl crate::Resettable for RawerrLSpec {}
