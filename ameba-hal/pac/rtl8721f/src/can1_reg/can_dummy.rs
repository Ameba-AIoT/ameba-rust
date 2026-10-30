#[doc = "Register `CAN_DUMMY` reader"]
pub type R = crate::R<CanDummySpec>;
#[doc = "Register `CAN_DUMMY` writer"]
pub type W = crate::W<CanDummySpec>;
#[doc = "Field `dummy` reader - rsvd for hw"]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - rsvd for hw"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - rsvd for hw"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - rsvd for hw"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, CanDummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "dummy register\n\nYou can [`read`](crate::Reg::read) this register and get [`can_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanDummySpec;
impl crate::RegisterSpec for CanDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_dummy::R`](R) reader structure"]
impl crate::Readable for CanDummySpec {}
#[doc = "`write(|w| ..)` method takes [`can_dummy::W`](W) writer structure"]
impl crate::Writable for CanDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_DUMMY to value 0"]
impl crate::Resettable for CanDummySpec {}
