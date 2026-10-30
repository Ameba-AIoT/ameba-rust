#[doc = "Register `INPUT_LAYER1_PIC_SIZE` reader"]
pub type R = crate::R<InputLayer1PicSizeSpec>;
#[doc = "Register `INPUT_LAYER1_PIC_SIZE` writer"]
pub type W = crate::W<InputLayer1PicSizeSpec>;
#[doc = "Field `INPUT_LAYER1_PIC_WIDTH` reader - Picture width (in pixels) Must be greater than 0."]
pub type InputLayer1PicWidthR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER1_PIC_WIDTH` writer - Picture width (in pixels) Must be greater than 0."]
pub type InputLayer1PicWidthW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `INPUT_LAYER1_PIC_HEIGHT` reader - Picture height (in pixels) Must be greater than 0."]
pub type InputLayer1PicHeightR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER1_PIC_HEIGHT` writer - Picture height (in pixels) Must be greater than 0."]
pub type InputLayer1PicHeightW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Picture width (in pixels) Must be greater than 0."]
    #[inline(always)]
    pub fn input_layer1_pic_width(&self) -> InputLayer1PicWidthR {
        InputLayer1PicWidthR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Picture height (in pixels) Must be greater than 0."]
    #[inline(always)]
    pub fn input_layer1_pic_height(&self) -> InputLayer1PicHeightR {
        InputLayer1PicHeightR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Picture width (in pixels) Must be greater than 0."]
    #[inline(always)]
    pub fn input_layer1_pic_width(&mut self) -> InputLayer1PicWidthW<'_, InputLayer1PicSizeSpec> {
        InputLayer1PicWidthW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Picture height (in pixels) Must be greater than 0."]
    #[inline(always)]
    pub fn input_layer1_pic_height(&mut self) -> InputLayer1PicHeightW<'_, InputLayer1PicSizeSpec> {
        InputLayer1PicHeightW::new(self, 16)
    }
}
#[doc = "Canvas size register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer1_pic_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer1_pic_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer1PicSizeSpec;
impl crate::RegisterSpec for InputLayer1PicSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer1_pic_size::R`](R) reader structure"]
impl crate::Readable for InputLayer1PicSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer1_pic_size::W`](W) writer structure"]
impl crate::Writable for InputLayer1PicSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER1_PIC_SIZE to value 0x0001_0001"]
impl crate::Resettable for InputLayer1PicSizeSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
