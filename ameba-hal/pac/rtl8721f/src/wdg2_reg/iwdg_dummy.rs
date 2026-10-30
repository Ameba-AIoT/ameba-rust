#[doc = "Register `IWDG_DUMMY` reader"]
pub type R = crate::R<IwdgDummySpec>;
#[doc = "Register `IWDG_DUMMY` writer"]
pub type W = crate::W<IwdgDummySpec>;
#[doc = "Field `dummy` reader - rsvd for hw"]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - rsvd for hw"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - rsvd for hw"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - rsvd for hw"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, IwdgDummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "Wdg Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgDummySpec;
impl crate::RegisterSpec for IwdgDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_dummy::R`](R) reader structure"]
impl crate::Readable for IwdgDummySpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_dummy::W`](W) writer structure"]
impl crate::Writable for IwdgDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_DUMMY to value 0xff"]
impl crate::Resettable for IwdgDummySpec {
    const RESET_VALUE: u32 = 0xff;
}
