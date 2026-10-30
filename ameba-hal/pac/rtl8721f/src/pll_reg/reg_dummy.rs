#[doc = "Register `REG_DUMMY` reader"]
pub type R = crate::R<RegDummySpec>;
#[doc = "Register `REG_DUMMY` writer"]
pub type W = crate::W<RegDummySpec>;
#[doc = "Field `lpad_dummy` reader - dummy"]
pub type LpadDummyR = crate::FieldReader;
#[doc = "Field `lpad_dummy` writer - dummy"]
pub type LpadDummyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - dummy"]
    #[inline(always)]
    pub fn lpad_dummy(&self) -> LpadDummyR {
        LpadDummyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - dummy"]
    #[inline(always)]
    pub fn lpad_dummy(&mut self) -> LpadDummyW<'_, RegDummySpec> {
        LpadDummyW::new(self, 0)
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
