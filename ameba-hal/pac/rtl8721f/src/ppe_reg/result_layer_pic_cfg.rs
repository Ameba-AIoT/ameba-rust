#[doc = "Register `RESULT_LAYER_PIC_CFG` reader"]
pub type R = crate::R<ResultLayerPicCfgSpec>;
#[doc = "Register `RESULT_LAYER_PIC_CFG` writer"]
pub type W = crate::W<ResultLayerPicCfgSpec>;
#[doc = "Field `RESULT_LAYER_FORMAT` reader - Pixel format"]
pub type ResultLayerFormatR = crate::FieldReader;
#[doc = "Field `RESULT_LAYER_FORMAT` writer - Pixel format"]
pub type ResultLayerFormatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESULT_LAYER_BG_SRC` reader - Background source 0x0: Use CONST_BG as background 0x1: Use lyr1 as background"]
pub type ResultLayerBgSrcR = crate::BitReader;
#[doc = "Field `RESULT_LAYER_BG_SRC` writer - Background source 0x0: Use CONST_BG as background 0x1: Use lyr1 as background"]
pub type ResultLayerBgSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESULT_LAYER_LINE_LEN` reader - Indicates the bytes in memory between two vertically adjacent pixels."]
pub type ResultLayerLineLenR = crate::FieldReader<u16>;
#[doc = "Field `RESULT_LAYER_LINE_LEN` writer - Indicates the bytes in memory between two vertically adjacent pixels."]
pub type ResultLayerLineLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Pixel format"]
    #[inline(always)]
    pub fn result_layer_format(&self) -> ResultLayerFormatR {
        ResultLayerFormatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Background source 0x0: Use CONST_BG as background 0x1: Use lyr1 as background"]
    #[inline(always)]
    pub fn result_layer_bg_src(&self) -> ResultLayerBgSrcR {
        ResultLayerBgSrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Indicates the bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn result_layer_line_len(&self) -> ResultLayerLineLenR {
        ResultLayerLineLenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pixel format"]
    #[inline(always)]
    pub fn result_layer_format(&mut self) -> ResultLayerFormatW<'_, ResultLayerPicCfgSpec> {
        ResultLayerFormatW::new(self, 0)
    }
    #[doc = "Bit 8 - Background source 0x0: Use CONST_BG as background 0x1: Use lyr1 as background"]
    #[inline(always)]
    pub fn result_layer_bg_src(&mut self) -> ResultLayerBgSrcW<'_, ResultLayerPicCfgSpec> {
        ResultLayerBgSrcW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Indicates the bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn result_layer_line_len(&mut self) -> ResultLayerLineLenW<'_, ResultLayerPicCfgSpec> {
        ResultLayerLineLenW::new(self, 16)
    }
}
#[doc = "Picture configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`result_layer_pic_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result_layer_pic_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultLayerPicCfgSpec;
impl crate::RegisterSpec for ResultLayerPicCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result_layer_pic_cfg::R`](R) reader structure"]
impl crate::Readable for ResultLayerPicCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`result_layer_pic_cfg::W`](W) writer structure"]
impl crate::Writable for ResultLayerPicCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESULT_LAYER_PIC_CFG to value 0"]
impl crate::Resettable for ResultLayerPicCfgSpec {}
