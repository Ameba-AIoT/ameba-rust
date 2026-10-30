#[doc = "Register `MBIST_DONE_STA0` reader"]
pub type R = crate::R<MbistDoneSta0Spec>;
#[doc = "Field `mbist_done_31_0` reader - "]
pub type MbistDone31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mbist_done_31_0(&self) -> MbistDone31_0R {
        MbistDone31_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_done_sta0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistDoneSta0Spec;
impl crate::RegisterSpec for MbistDoneSta0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_done_sta0::R`](R) reader structure"]
impl crate::Readable for MbistDoneSta0Spec {}
#[doc = "`reset()` method sets MBIST_DONE_STA0 to value 0"]
impl crate::Resettable for MbistDoneSta0Spec {}
