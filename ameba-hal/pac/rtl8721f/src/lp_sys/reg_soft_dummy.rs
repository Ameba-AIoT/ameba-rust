#[doc = "Register `REG_SOFT_DUMMY` reader"]
pub type R = crate::R<RegSoftDummySpec>;
#[doc = "Register `REG_SOFT_DUMMY` writer"]
pub type W = crate::W<RegSoftDummySpec>;
#[doc = "Field `dummy` reader - USED for SW"]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - USED for SW"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USED for SW"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USED for SW"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegSoftDummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_soft_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_soft_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSoftDummySpec;
impl crate::RegisterSpec for RegSoftDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_soft_dummy::R`](R) reader structure"]
impl crate::Readable for RegSoftDummySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_soft_dummy::W`](W) writer structure"]
impl crate::Writable for RegSoftDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SOFT_DUMMY to value 0"]
impl crate::Resettable for RegSoftDummySpec {}
