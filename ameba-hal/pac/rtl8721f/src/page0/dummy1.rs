#[doc = "Register `DUMMY1` reader"]
pub type R = crate::R<Dummy1Spec>;
#[doc = "Register `DUMMY1` writer"]
pub type W = crate::W<Dummy1Spec>;
#[doc = "Field `dummy_2` reader - dummy"]
pub type Dummy2R = crate::FieldReader;
#[doc = "Field `dummy_2` writer - dummy"]
pub type Dummy2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:3 - dummy"]
    #[inline(always)]
    pub fn dummy_2(&self) -> Dummy2R {
        Dummy2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - dummy"]
    #[inline(always)]
    pub fn dummy_2(&mut self) -> Dummy2W<'_, Dummy1Spec> {
        Dummy2W::new(self, 0)
    }
    #[doc = "Bits 5:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, Dummy1Spec> {
        DummyW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dummy1Spec;
impl crate::RegisterSpec for Dummy1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dummy1::R`](R) reader structure"]
impl crate::Readable for Dummy1Spec {}
#[doc = "`write(|w| ..)` method takes [`dummy1::W`](W) writer structure"]
impl crate::Writable for Dummy1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUMMY1 to value 0"]
impl crate::Resettable for Dummy1Spec {}
