#[doc = "Register `REG_CTC_DUMMY` reader"]
pub type R = crate::R<RegCtcDummySpec>;
#[doc = "Register `REG_CTC_DUMMY` writer"]
pub type W = crate::W<RegCtcDummySpec>;
#[doc = "Field `dummy` reader - dummy register"]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - dummy register"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - dummy register"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - dummy register"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegCtcDummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "Captouch Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ctc_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ctc_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCtcDummySpec;
impl crate::RegisterSpec for RegCtcDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ctc_dummy::R`](R) reader structure"]
impl crate::Readable for RegCtcDummySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ctc_dummy::W`](W) writer structure"]
impl crate::Writable for RegCtcDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CTC_DUMMY to value 0xf0"]
impl crate::Resettable for RegCtcDummySpec {
    const RESET_VALUE: u32 = 0xf0;
}
