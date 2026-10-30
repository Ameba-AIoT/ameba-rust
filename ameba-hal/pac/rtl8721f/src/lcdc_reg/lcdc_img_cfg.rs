#[doc = "Register `LCDC_IMG_CFG` reader"]
pub type R = crate::R<LcdcImgCfgSpec>;
#[doc = "Register `LCDC_IMG_CFG` writer"]
pub type W = crate::W<LcdcImgCfgSpec>;
#[doc = "Field `img_format` reader - Image layer data format, it supports the following values: *0000: ARGB8888 *0001: RGB888 *0010: RGB565 *0011: ARGB1555 *0100: ARGB4444 *0101: RGB666 *0110: ARGB8666 *1000: ABGR8888 *1001: BGR888 *1010: BGR565 *Others: Reserved"]
pub type ImgFormatR = crate::FieldReader;
#[doc = "Field `img_format` writer - Image layer data format, it supports the following values: *0000: ARGB8888 *0001: RGB888 *0010: RGB565 *0011: ARGB1555 *0100: ARGB4444 *0101: RGB666 *0110: ARGB8666 *1000: ABGR8888 *1001: BGR888 *1010: BGR565 *Others: Reserved"]
pub type ImgFormatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:7 - Image layer data format, it supports the following values: *0000: ARGB8888 *0001: RGB888 *0010: RGB565 *0011: ARGB1555 *0100: ARGB4444 *0101: RGB666 *0110: ARGB8666 *1000: ABGR8888 *1001: BGR888 *1010: BGR565 *Others: Reserved"]
    #[inline(always)]
    pub fn img_format(&self) -> ImgFormatR {
        ImgFormatR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Image layer data format, it supports the following values: *0000: ARGB8888 *0001: RGB888 *0010: RGB565 *0011: ARGB1555 *0100: ARGB4444 *0101: RGB666 *0110: ARGB8666 *1000: ABGR8888 *1001: BGR888 *1010: BGR565 *Others: Reserved"]
    #[inline(always)]
    pub fn img_format(&mut self) -> ImgFormatW<'_, LcdcImgCfgSpec> {
        ImgFormatW::new(self, 4)
    }
}
#[doc = "source image config\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_img_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_img_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcImgCfgSpec;
impl crate::RegisterSpec for LcdcImgCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_img_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcImgCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_img_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcImgCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_IMG_CFG to value 0"]
impl crate::Resettable for LcdcImgCfgSpec {}
