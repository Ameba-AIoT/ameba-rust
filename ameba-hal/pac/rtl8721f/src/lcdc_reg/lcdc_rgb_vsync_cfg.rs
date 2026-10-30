#[doc = "Register `LCDC_RGB_VSYNC_CFG` reader"]
pub type R = crate::R<LcdcRgbVsyncCfgSpec>;
#[doc = "Register `LCDC_RGB_VSYNC_CFG` writer"]
pub type W = crate::W<LcdcRgbVsyncCfgSpec>;
#[doc = "Field `vsw` reader - Vertical synchronization signal width -1. Unit: inactive lines"]
pub type VswR = crate::FieldReader;
#[doc = "Field `vsw` writer - Vertical synchronization signal width -1. Unit: inactive lines"]
pub type VswW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vbp` reader - Back porch line number-1. The number of inactive lines at the start of a frame, after vertical synchronization period."]
pub type VbpR = crate::FieldReader;
#[doc = "Field `vbp` writer - Back porch line number-1. The number of inactive lines at the start of a frame, after vertical synchronization period."]
pub type VbpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vfp` reader - Front porch line number-1. The number of inactive lines at the end of a frame, before vertical synchronization period."]
pub type VfpR = crate::FieldReader;
#[doc = "Field `vfp` writer - Front porch line number-1. The number of inactive lines at the end of a frame, before vertical synchronization period."]
pub type VfpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Vertical synchronization signal width -1. Unit: inactive lines"]
    #[inline(always)]
    pub fn vsw(&self) -> VswR {
        VswR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Back porch line number-1. The number of inactive lines at the start of a frame, after vertical synchronization period."]
    #[inline(always)]
    pub fn vbp(&self) -> VbpR {
        VbpR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Front porch line number-1. The number of inactive lines at the end of a frame, before vertical synchronization period."]
    #[inline(always)]
    pub fn vfp(&self) -> VfpR {
        VfpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Vertical synchronization signal width -1. Unit: inactive lines"]
    #[inline(always)]
    pub fn vsw(&mut self) -> VswW<'_, LcdcRgbVsyncCfgSpec> {
        VswW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Back porch line number-1. The number of inactive lines at the start of a frame, after vertical synchronization period."]
    #[inline(always)]
    pub fn vbp(&mut self) -> VbpW<'_, LcdcRgbVsyncCfgSpec> {
        VbpW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Front porch line number-1. The number of inactive lines at the end of a frame, before vertical synchronization period."]
    #[inline(always)]
    pub fn vfp(&mut self) -> VfpW<'_, LcdcRgbVsyncCfgSpec> {
        VfpW::new(self, 16)
    }
}
#[doc = "RGB Vsync config\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_rgb_vsync_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_rgb_vsync_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcRgbVsyncCfgSpec;
impl crate::RegisterSpec for LcdcRgbVsyncCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_rgb_vsync_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcRgbVsyncCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_rgb_vsync_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcRgbVsyncCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_RGB_VSYNC_CFG to value 0x0003_0301"]
impl crate::Resettable for LcdcRgbVsyncCfgSpec {
    const RESET_VALUE: u32 = 0x0003_0301;
}
