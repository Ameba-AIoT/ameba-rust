#[doc = "Register `LCDC_IMG_SLIDE` reader"]
pub type R = crate::R<LcdcImgSlideSpec>;
#[doc = "Register `LCDC_IMG_SLIDE` writer"]
pub type W = crate::W<LcdcImgSlideSpec>;
#[doc = "Field `img_hs_ofst` reader - pixel offset for image horizontal slide"]
pub type ImgHsOfstR = crate::FieldReader<u16>;
#[doc = "Field `img_hs_ofst` writer - pixel offset for image horizontal slide"]
pub type ImgHsOfstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `img_vs_ofst` reader - pixel offset for image vertical slide"]
pub type ImgVsOfstR = crate::FieldReader<u16>;
#[doc = "Field `img_vs_ofst` writer - pixel offset for image vertical slide"]
pub type ImgVsOfstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - pixel offset for image horizontal slide"]
    #[inline(always)]
    pub fn img_hs_ofst(&self) -> ImgHsOfstR {
        ImgHsOfstR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - pixel offset for image vertical slide"]
    #[inline(always)]
    pub fn img_vs_ofst(&self) -> ImgVsOfstR {
        ImgVsOfstR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - pixel offset for image horizontal slide"]
    #[inline(always)]
    pub fn img_hs_ofst(&mut self) -> ImgHsOfstW<'_, LcdcImgSlideSpec> {
        ImgHsOfstW::new(self, 0)
    }
    #[doc = "Bits 16:27 - pixel offset for image vertical slide"]
    #[inline(always)]
    pub fn img_vs_ofst(&mut self) -> ImgVsOfstW<'_, LcdcImgSlideSpec> {
        ImgVsOfstW::new(self, 16)
    }
}
#[doc = "LCDC image slide\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_img_slide::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_img_slide::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcImgSlideSpec;
impl crate::RegisterSpec for LcdcImgSlideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_img_slide::R`](R) reader structure"]
impl crate::Readable for LcdcImgSlideSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_img_slide::W`](W) writer structure"]
impl crate::Writable for LcdcImgSlideSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_IMG_SLIDE to value 0"]
impl crate::Resettable for LcdcImgSlideSpec {}
