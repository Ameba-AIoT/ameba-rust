#[doc = "Register `RESULT_LAYER_BLK_SIZE` reader"]
pub type R = crate::R<ResultLayerBlkSizeSpec>;
#[doc = "Register `RESULT_LAYER_BLK_SIZE` writer"]
pub type W = crate::W<ResultLayerBlkSizeSpec>;
#[doc = "Field `RESULT_LAYER_BLK_WIDTH` reader - Block width (in pixels) Must be greater than 0 and less than or equal to canvas width."]
pub type ResultLayerBlkWidthR = crate::FieldReader<u16>;
#[doc = "Field `RESULT_LAYER_BLK_WIDTH` writer - Block width (in pixels) Must be greater than 0 and less than or equal to canvas width."]
pub type ResultLayerBlkWidthW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RESULT_LAYER_BLK_HEIGHT` reader - Block height (in pixels) Must be greater than 0 and less than or equal to canvas width."]
pub type ResultLayerBlkHeightR = crate::FieldReader<u16>;
#[doc = "Field `RESULT_LAYER_BLK_HEIGHT` writer - Block height (in pixels) Must be greater than 0 and less than or equal to canvas width."]
pub type ResultLayerBlkHeightW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Block width (in pixels) Must be greater than 0 and less than or equal to canvas width."]
    #[inline(always)]
    pub fn result_layer_blk_width(&self) -> ResultLayerBlkWidthR {
        ResultLayerBlkWidthR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Block height (in pixels) Must be greater than 0 and less than or equal to canvas width."]
    #[inline(always)]
    pub fn result_layer_blk_height(&self) -> ResultLayerBlkHeightR {
        ResultLayerBlkHeightR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Block width (in pixels) Must be greater than 0 and less than or equal to canvas width."]
    #[inline(always)]
    pub fn result_layer_blk_width(&mut self) -> ResultLayerBlkWidthW<'_, ResultLayerBlkSizeSpec> {
        ResultLayerBlkWidthW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Block height (in pixels) Must be greater than 0 and less than or equal to canvas width."]
    #[inline(always)]
    pub fn result_layer_blk_height(&mut self) -> ResultLayerBlkHeightW<'_, ResultLayerBlkSizeSpec> {
        ResultLayerBlkHeightW::new(self, 16)
    }
}
#[doc = "Canvas size register\n\nYou can [`read`](crate::Reg::read) this register and get [`result_layer_blk_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result_layer_blk_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultLayerBlkSizeSpec;
impl crate::RegisterSpec for ResultLayerBlkSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result_layer_blk_size::R`](R) reader structure"]
impl crate::Readable for ResultLayerBlkSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`result_layer_blk_size::W`](W) writer structure"]
impl crate::Writable for ResultLayerBlkSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESULT_LAYER_BLK_SIZE to value 0x0001_0001"]
impl crate::Resettable for ResultLayerBlkSizeSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
