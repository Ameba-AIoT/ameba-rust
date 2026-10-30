#[doc = "Register `audio_ro_dummy1` reader"]
pub type R = crate::R<AudioRoDummy1Spec>;
#[doc = "Field `audio_ro_dummy1` reader - Dummy register"]
pub type AudioRoDummy1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Dummy register"]
    #[inline(always)]
    pub fn audio_ro_dummy1(&self) -> AudioRoDummy1R {
        AudioRoDummy1R::new(self.bits)
    }
}
#[doc = "Audio RO Dummy1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_ro_dummy1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioRoDummy1Spec;
impl crate::RegisterSpec for AudioRoDummy1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_ro_dummy1::R`](R) reader structure"]
impl crate::Readable for AudioRoDummy1Spec {}
#[doc = "`reset()` method sets audio_ro_dummy1 to value 0"]
impl crate::Resettable for AudioRoDummy1Spec {}
