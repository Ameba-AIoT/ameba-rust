#[doc = "Register `DUMMY` reader"]
pub type R = crate::R<DummySpec>;
#[doc = "Register `DUMMY` writer"]
pub type W = crate::W<DummySpec>;
#[doc = "Field `dummy` reader - Dummy bit."]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - Dummy bit."]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dummy bit."]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dummy bit."]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, DummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DummySpec;
impl crate::RegisterSpec for DummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dummy::R`](R) reader structure"]
impl crate::Readable for DummySpec {}
#[doc = "`write(|w| ..)` method takes [`dummy::W`](W) writer structure"]
impl crate::Writable for DummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUMMY to value 0"]
impl crate::Resettable for DummySpec {}
