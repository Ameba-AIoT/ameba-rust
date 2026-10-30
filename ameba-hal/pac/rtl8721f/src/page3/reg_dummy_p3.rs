#[doc = "Register `REG_DUMMY_P3` reader"]
pub type R = crate::R<RegDummyP3Spec>;
#[doc = "Register `REG_DUMMY_P3` writer"]
pub type W = crate::W<RegDummyP3Spec>;
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
    pub fn dummy(&mut self) -> DummyW<'_, RegDummyP3Spec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dummy_p3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dummy_p3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDummyP3Spec;
impl crate::RegisterSpec for RegDummyP3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dummy_p3::R`](R) reader structure"]
impl crate::Readable for RegDummyP3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_dummy_p3::W`](W) writer structure"]
impl crate::Writable for RegDummyP3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DUMMY_P3 to value 0"]
impl crate::Resettable for RegDummyP3Spec {}
