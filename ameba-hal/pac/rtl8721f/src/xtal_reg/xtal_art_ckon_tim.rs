#[doc = "Register `XTAL_ART_CKON_TIM` reader"]
pub type R = crate::R<XtalArtCkonTimSpec>;
#[doc = "Register `XTAL_ART_CKON_TIM` writer"]
pub type W = crate::W<XtalArtCkonTimSpec>;
#[doc = "Field `dummy` reader - HW used only"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - HW used only"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HW used only"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HW used only"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, XtalArtCkonTimSpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_art_ckon_tim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_art_ckon_tim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalArtCkonTimSpec;
impl crate::RegisterSpec for XtalArtCkonTimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_art_ckon_tim::R`](R) reader structure"]
impl crate::Readable for XtalArtCkonTimSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal_art_ckon_tim::W`](W) writer structure"]
impl crate::Writable for XtalArtCkonTimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_ART_CKON_TIM to value 0x0f"]
impl crate::Resettable for XtalArtCkonTimSpec {
    const RESET_VALUE: u32 = 0x0f;
}
