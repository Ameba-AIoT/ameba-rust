#[doc = "Register `LOGUART_DUMMY` reader"]
pub type R = crate::R<LoguartDummySpec>;
#[doc = "Field `dummy` reader - rsvd for hw"]
pub type DummyR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - rsvd for hw"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Loguart Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`loguart_dummy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoguartDummySpec;
impl crate::RegisterSpec for LoguartDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loguart_dummy::R`](R) reader structure"]
impl crate::Readable for LoguartDummySpec {}
#[doc = "`reset()` method sets LOGUART_DUMMY to value 0"]
impl crate::Resettable for LoguartDummySpec {}
