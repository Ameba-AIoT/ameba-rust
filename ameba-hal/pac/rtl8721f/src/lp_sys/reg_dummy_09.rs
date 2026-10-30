#[doc = "Register `REG_DUMMY_09` reader"]
pub type R = crate::R<RegDummy09Spec>;
#[doc = "Register `REG_DUMMY_09` writer"]
pub type W = crate::W<RegDummy09Spec>;
#[doc = "Field `dummy` reader - HW used only , Used for access boundary check, SYSON is 512B which is 000-1FF."]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - HW used only , Used for access boundary check, SYSON is 512B which is 000-1FF."]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HW used only , Used for access boundary check, SYSON is 512B which is 000-1FF."]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW used only , Used for access boundary check, SYSON is 512B which is 000-1FF."]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegDummy09Spec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dummy_09::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dummy_09::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDummy09Spec;
impl crate::RegisterSpec for RegDummy09Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dummy_09::R`](R) reader structure"]
impl crate::Readable for RegDummy09Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_dummy_09::W`](W) writer structure"]
impl crate::Writable for RegDummy09Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DUMMY_09 to value 0xbdc1_0512"]
impl crate::Resettable for RegDummy09Spec {
    const RESET_VALUE: u32 = 0xbdc1_0512;
}
