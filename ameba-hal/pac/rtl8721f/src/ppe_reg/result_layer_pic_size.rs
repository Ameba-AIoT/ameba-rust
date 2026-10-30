#[doc = "Register `RESULT_LAYER_PIC_SIZE` reader"]
pub type R = crate::R<ResultLayerPicSizeSpec>;
#[doc = "Register `RESULT_LAYER_PIC_SIZE` writer"]
pub type W = crate::W<ResultLayerPicSizeSpec>;
#[doc = "Field `RESULT_LAYER_PIC_WIDTH` reader - Canvas width (in pixels) Must be greater than 0."]
pub type ResultLayerPicWidthR = crate::FieldReader<u16>;
#[doc = "Field `RESULT_LAYER_PIC_WIDTH` writer - Canvas width (in pixels) Must be greater than 0."]
pub type ResultLayerPicWidthW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RESULT_LAYER_PIC_HEIGHT` reader - Canvas height (in pixels) Must be greater than 0."]
pub type ResultLayerPicHeightR = crate::FieldReader<u16>;
#[doc = "Field `RESULT_LAYER_PIC_HEIGHT` writer - Canvas height (in pixels) Must be greater than 0."]
pub type ResultLayerPicHeightW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Canvas width (in pixels) Must be greater than 0."]
    #[inline(always)]
    pub fn result_layer_pic_width(&self) -> ResultLayerPicWidthR {
        ResultLayerPicWidthR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Canvas height (in pixels) Must be greater than 0."]
    #[inline(always)]
    pub fn result_layer_pic_height(&self) -> ResultLayerPicHeightR {
        ResultLayerPicHeightR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Canvas width (in pixels) Must be greater than 0."]
    #[inline(always)]
    pub fn result_layer_pic_width(&mut self) -> ResultLayerPicWidthW<'_, ResultLayerPicSizeSpec> {
        ResultLayerPicWidthW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Canvas height (in pixels) Must be greater than 0."]
    #[inline(always)]
    pub fn result_layer_pic_height(&mut self) -> ResultLayerPicHeightW<'_, ResultLayerPicSizeSpec> {
        ResultLayerPicHeightW::new(self, 16)
    }
}
#[doc = "Canvas size register\n\nYou can [`read`](crate::Reg::read) this register and get [`result_layer_pic_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result_layer_pic_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultLayerPicSizeSpec;
impl crate::RegisterSpec for ResultLayerPicSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result_layer_pic_size::R`](R) reader structure"]
impl crate::Readable for ResultLayerPicSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`result_layer_pic_size::W`](W) writer structure"]
impl crate::Writable for ResultLayerPicSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESULT_LAYER_PIC_SIZE to value 0x0001_0001"]
impl crate::Resettable for ResultLayerPicSizeSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
