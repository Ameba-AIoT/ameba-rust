#[doc = "Register `FLUSH_DONE` reader"]
pub type R = crate::R<FlushDoneSpec>;
#[doc = "Field `FLUSH_DONE` reader - Indicate flush is complete"]
pub type FlushDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicate flush is complete"]
    #[inline(always)]
    pub fn flush_done(&self) -> FlushDoneR {
        FlushDoneR::new((self.bits & 1) != 0)
    }
}
#[doc = "Indicate flush is complete\n\nYou can [`read`](crate::Reg::read) this register and get [`flush_done::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlushDoneSpec;
impl crate::RegisterSpec for FlushDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flush_done::R`](R) reader structure"]
impl crate::Readable for FlushDoneSpec {}
#[doc = "`reset()` method sets FLUSH_DONE to value 0"]
impl crate::Resettable for FlushDoneSpec {}
