#[doc = "Register `TM_DUMMY` reader"]
pub type R = crate::R<TmDummySpec>;
#[doc = "Register `TM_DUMMY` writer"]
pub type W = crate::W<TmDummySpec>;
#[doc = "Field `dummy` reader - rsvd for hw"]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - rsvd for hw"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - rsvd for hw"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - rsvd for hw"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, TmDummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "Thermal Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmDummySpec;
impl crate::RegisterSpec for TmDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_dummy::R`](R) reader structure"]
impl crate::Readable for TmDummySpec {}
#[doc = "`write(|w| ..)` method takes [`tm_dummy::W`](W) writer structure"]
impl crate::Writable for TmDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TM_DUMMY to value 0"]
impl crate::Resettable for TmDummySpec {}
