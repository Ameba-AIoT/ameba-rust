#[doc = "Register `DUMMY_SYS1` reader"]
pub type R = crate::R<DummySys1Spec>;
#[doc = "Register `DUMMY_SYS1` writer"]
pub type W = crate::W<DummySys1Spec>;
#[doc = "Field `dummy_2` reader - Dummy bit.(Reserved)"]
pub type Dummy2R = crate::BitReader;
#[doc = "Field `dummy` reader - Dummy bit."]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - Dummy bit."]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - Dummy bit.(Reserved)"]
    #[inline(always)]
    pub fn dummy_2(&self) -> Dummy2R {
        Dummy2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Dummy bit."]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:31 - Dummy bit."]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, DummySys1Spec> {
        DummyW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy_sys1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy_sys1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DummySys1Spec;
impl crate::RegisterSpec for DummySys1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dummy_sys1::R`](R) reader structure"]
impl crate::Readable for DummySys1Spec {}
#[doc = "`write(|w| ..)` method takes [`dummy_sys1::W`](W) writer structure"]
impl crate::Writable for DummySys1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DUMMY_SYS1 to value 0"]
impl crate::Resettable for DummySys1Spec {}
