#[doc = "Register `LCDC_RGB_HSYNC_CFG` reader"]
pub type R = crate::R<LcdcRgbHsyncCfgSpec>;
#[doc = "Register `LCDC_RGB_HSYNC_CFG` writer"]
pub type W = crate::W<LcdcRgbHsyncCfgSpec>;
#[doc = "Field `hsw` reader - Horizontal synchronization signal width -1. Unit: DCLK HSW's minimum value is 1"]
pub type HswR = crate::FieldReader;
#[doc = "Field `hsw` writer - Horizontal synchronization signal width -1. Unit: DCLK HSW's minimum value is 1"]
pub type HswW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `hbp` reader - Horizontal back porch -1. The number of DCLK periods between the falling edge of HSYNC and the start of active data. HBP's minimum value is 1"]
pub type HbpR = crate::FieldReader;
#[doc = "Field `hbp` writer - Horizontal back porch -1. The number of DCLK periods between the falling edge of HSYNC and the start of active data. HBP's minimum value is 1"]
pub type HbpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `hfp` reader - Horizontal front porch -1. The number of DCLK periods between the end of active data and the rising edge of HSYNC. HFP's minimum value is 1"]
pub type HfpR = crate::FieldReader;
#[doc = "Field `hfp` writer - Horizontal front porch -1. The number of DCLK periods between the end of active data and the rising edge of HSYNC. HFP's minimum value is 1"]
pub type HfpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Horizontal synchronization signal width -1. Unit: DCLK HSW's minimum value is 1"]
    #[inline(always)]
    pub fn hsw(&self) -> HswR {
        HswR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal back porch -1. The number of DCLK periods between the falling edge of HSYNC and the start of active data. HBP's minimum value is 1"]
    #[inline(always)]
    pub fn hbp(&self) -> HbpR {
        HbpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal front porch -1. The number of DCLK periods between the end of active data and the rising edge of HSYNC. HFP's minimum value is 1"]
    #[inline(always)]
    pub fn hfp(&self) -> HfpR {
        HfpR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Horizontal synchronization signal width -1. Unit: DCLK HSW's minimum value is 1"]
    #[inline(always)]
    pub fn hsw(&mut self) -> HswW<'_, LcdcRgbHsyncCfgSpec> {
        HswW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Horizontal back porch -1. The number of DCLK periods between the falling edge of HSYNC and the start of active data. HBP's minimum value is 1"]
    #[inline(always)]
    pub fn hbp(&mut self) -> HbpW<'_, LcdcRgbHsyncCfgSpec> {
        HbpW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Horizontal front porch -1. The number of DCLK periods between the end of active data and the rising edge of HSYNC. HFP's minimum value is 1"]
    #[inline(always)]
    pub fn hfp(&mut self) -> HfpW<'_, LcdcRgbHsyncCfgSpec> {
        HfpW::new(self, 16)
    }
}
#[doc = "RGB Hsync config\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_rgb_hsync_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_rgb_hsync_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcRgbHsyncCfgSpec;
impl crate::RegisterSpec for LcdcRgbHsyncCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_rgb_hsync_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcRgbHsyncCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_rgb_hsync_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcRgbHsyncCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_RGB_HSYNC_CFG to value 0x001f_1f17"]
impl crate::Resettable for LcdcRgbHsyncCfgSpec {
    const RESET_VALUE: u32 = 0x001f_1f17;
}
