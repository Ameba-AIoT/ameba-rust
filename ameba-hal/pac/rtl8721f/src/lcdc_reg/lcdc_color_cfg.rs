#[doc = "Register `LCDC_COLOR_CFG` reader"]
pub type R = crate::R<LcdcColorCfgSpec>;
#[doc = "Register `LCDC_COLOR_CFG` writer"]
pub type W = crate::W<LcdcColorCfgSpec>;
#[doc = "Field `color_format` reader - 0000: RGB888 0001: RGB565 0010: RGB666 1000: BGR888 1001: BGR565 1010: BGR666"]
pub type ColorFormatR = crate::FieldReader;
#[doc = "Field `color_format` writer - 0000: RGB888 0001: RGB565 0010: RGB666 1000: BGR888 1001: BGR565 1010: BGR666"]
pub type ColorFormatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 2:5 - 0000: RGB888 0001: RGB565 0010: RGB666 1000: BGR888 1001: BGR565 1010: BGR666"]
    #[inline(always)]
    pub fn color_format(&self) -> ColorFormatR {
        ColorFormatR::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:5 - 0000: RGB888 0001: RGB565 0010: RGB666 1000: BGR888 1001: BGR565 1010: BGR666"]
    #[inline(always)]
    pub fn color_format(&mut self) -> ColorFormatW<'_, LcdcColorCfgSpec> {
        ColorFormatW::new(self, 2)
    }
}
#[doc = "color config\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_color_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_color_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcColorCfgSpec;
impl crate::RegisterSpec for LcdcColorCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_color_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcColorCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_color_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcColorCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_COLOR_CFG to value 0"]
impl crate::Resettable for LcdcColorCfgSpec {}
