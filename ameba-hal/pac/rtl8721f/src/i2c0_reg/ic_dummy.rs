#[doc = "Register `IC_DUMMY` reader"]
pub type R = crate::R<IcDummySpec>;
#[doc = "Field `dummy` reader - rsvd for hw"]
pub type DummyR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - rsvd for hw"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "IC Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_dummy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcDummySpec;
impl crate::RegisterSpec for IcDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_dummy::R`](R) reader structure"]
impl crate::Readable for IcDummySpec {}
#[doc = "`reset()` method sets IC_DUMMY to value 0"]
impl crate::Resettable for IcDummySpec {}
