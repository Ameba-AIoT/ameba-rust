#[doc = "Register `DUMMY2` reader"]
pub type R = crate::R<Dummy2Spec>;
#[doc = "Register `DUMMY2` writer"]
pub type W = crate::W<Dummy2Spec>;
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
    pub fn dummy(&mut self) -> DummyW<'_, Dummy2Spec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dummy2Spec;
impl crate::RegisterSpec for Dummy2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dummy2::R`](R) reader structure"]
impl crate::Readable for Dummy2Spec {}
#[doc = "`write(|w| ..)` method takes [`dummy2::W`](W) writer structure"]
impl crate::Writable for Dummy2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUMMY2 to value 0x02"]
impl crate::Resettable for Dummy2Spec {
    const RESET_VALUE: u32 = 0x02;
}
