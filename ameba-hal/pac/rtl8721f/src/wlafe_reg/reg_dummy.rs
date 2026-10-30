#[doc = "Register `REG_DUMMY` reader"]
pub type R = crate::R<RegDummySpec>;
#[doc = "Register `REG_DUMMY` writer"]
pub type W = crate::W<RegDummySpec>;
#[doc = "Field `dummy` reader - HW used only , these are for reserved."]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - HW used only , these are for reserved."]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HW used only , these are for reserved."]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW used only , these are for reserved."]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegDummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDummySpec;
impl crate::RegisterSpec for RegDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dummy::R`](R) reader structure"]
impl crate::Readable for RegDummySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dummy::W`](W) writer structure"]
impl crate::Writable for RegDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DUMMY to value 0"]
impl crate::Resettable for RegDummySpec {}
