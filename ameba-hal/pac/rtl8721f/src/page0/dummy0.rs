#[doc = "Register `DUMMY0` reader"]
pub type R = crate::R<Dummy0Spec>;
#[doc = "Register `DUMMY0` writer"]
pub type W = crate::W<Dummy0Spec>;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, Dummy0Spec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dummy0Spec;
impl crate::RegisterSpec for Dummy0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dummy0::R`](R) reader structure"]
impl crate::Readable for Dummy0Spec {}
#[doc = "`write(|w| ..)` method takes [`dummy0::W`](W) writer structure"]
impl crate::Writable for Dummy0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUMMY0 to value 0"]
impl crate::Resettable for Dummy0Spec {}
