#[doc = "Register `LCDC_DUMMY` reader"]
pub type R = crate::R<LcdcDummySpec>;
#[doc = "Register `LCDC_DUMMY` writer"]
pub type W = crate::W<LcdcDummySpec>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, LcdcDummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcDummySpec;
impl crate::RegisterSpec for LcdcDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_dummy::R`](R) reader structure"]
impl crate::Readable for LcdcDummySpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_dummy::W`](W) writer structure"]
impl crate::Writable for LcdcDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_DUMMY to value 0"]
impl crate::Resettable for LcdcDummySpec {}
