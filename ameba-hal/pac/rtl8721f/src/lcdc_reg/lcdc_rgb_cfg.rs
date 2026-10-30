#[doc = "Register `LCDC_RGB_CFG` reader"]
pub type R = crate::R<LcdcRgbCfgSpec>;
#[doc = "Register `LCDC_RGB_CFG` writer"]
pub type W = crate::W<LcdcRgbCfgSpec>;
#[doc = "Field `rgb_tedelay` reader - The delay interval -5. This interval is from detected TE signal to starting frame transfer. Unit: WR pulse width. The maximum value is 65535. The real delay interval is TEDELAY + 4 or TEDELAY + 5."]
pub type RgbTedelayR = crate::FieldReader<u16>;
#[doc = "Field `rgb_tedelay` writer - The delay interval -5. This interval is from detected TE signal to starting frame transfer. Unit: WR pulse width. The maximum value is 65535. The real delay interval is TEDELAY + 4 or TEDELAY + 5."]
pub type RgbTedelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `dclkpl` reader - The polarity of the DCLK active edge. * 0: Data fetched at DCLK rising edge * 1: Data fetched at DCLK falling edge"]
pub type DclkplR = crate::BitReader;
#[doc = "Field `dclkpl` writer - The polarity of the DCLK active edge. * 0: Data fetched at DCLK rising edge * 1: Data fetched at DCLK falling edge"]
pub type DclkplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vspl` reader - The VSYNC pulse polarity. * 0: Low level synchronous clock * 1: High level synchronous clock"]
pub type VsplR = crate::BitReader;
#[doc = "Field `vspl` writer - The VSYNC pulse polarity. * 0: Low level synchronous clock * 1: High level synchronous clock"]
pub type VsplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hspl` reader - The HSYNC pulse polarity. * 0: Low level synchronous clock * 1: High level synchronous clock"]
pub type HsplR = crate::BitReader;
#[doc = "Field `hspl` writer - The HSYNC pulse polarity. * 0: Low level synchronous clock * 1: High level synchronous clock"]
pub type HsplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enpl` reader - The ENABLE pulse polarity. * 0: Low level for active data * 1: High level for active data"]
pub type EnplR = crate::BitReader;
#[doc = "Field `enpl` writer - The ENABLE pulse polarity. * 0: Low level for active data * 1: High level for active data"]
pub type EnplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rgb_datpl` reader - The Data pulse polarity. * 0: Normal * 1: Inverted"]
pub type RgbDatplR = crate::BitReader;
#[doc = "Field `rgb_datpl` writer - The Data pulse polarity. * 0: Normal * 1: Inverted"]
pub type RgbDatplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rgbtepl` reader - The TE pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
pub type RgbteplR = crate::BitReader;
#[doc = "Field `rgbtepl` writer - The TE pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
pub type RgbteplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rgbif_update` reader - Force Hardware updates RGB I/F parameters after current LCD refresh frame done. CPU writes 1 to force Hardware updating parameters. After updating, this bit is cleared. When the LCDC is running, if the following values related with RGB I/F mode are modified dynamically, only writing 1 to this bit can the newer value be used by hardware after the current frame refresh done. * The CLKDIV field in the LCDC_CLK_DIV register * The VFP, VBP, VSW fields in the LCDC_RGB_VSYNC_CFG register * The HBP, HFP, HSW fields in the LCDC_RGB_HSYNC_CFG register"]
pub type RgbifUpdateR = crate::BitReader;
#[doc = "Field `rgbif_update` writer - Force Hardware updates RGB I/F parameters after current LCD refresh frame done. CPU writes 1 to force Hardware updating parameters. After updating, this bit is cleared. When the LCDC is running, if the following values related with RGB I/F mode are modified dynamically, only writing 1 to this bit can the newer value be used by hardware after the current frame refresh done. * The CLKDIV field in the LCDC_CLK_DIV register * The VFP, VBP, VSW fields in the LCDC_RGB_VSYNC_CFG register * The HBP, HFP, HSW fields in the LCDC_RGB_HSYNC_CFG register"]
pub type RgbifUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rgb_sync_mode` reader - * 00: DE mode, frame synchronized with ENABLE signal * 01: HV mode, frame synchronized with synchronous signal * 10: TE mode * Others: Reserved"]
pub type RgbSyncModeR = crate::FieldReader;
#[doc = "Field `rgb_sync_mode` writer - * 00: DE mode, frame synchronized with ENABLE signal * 01: HV mode, frame synchronized with synchronous signal * 10: TE mode * Others: Reserved"]
pub type RgbSyncModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - The delay interval -5. This interval is from detected TE signal to starting frame transfer. Unit: WR pulse width. The maximum value is 65535. The real delay interval is TEDELAY + 4 or TEDELAY + 5."]
    #[inline(always)]
    pub fn rgb_tedelay(&self) -> RgbTedelayR {
        RgbTedelayR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - The polarity of the DCLK active edge. * 0: Data fetched at DCLK rising edge * 1: Data fetched at DCLK falling edge"]
    #[inline(always)]
    pub fn dclkpl(&self) -> DclkplR {
        DclkplR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The VSYNC pulse polarity. * 0: Low level synchronous clock * 1: High level synchronous clock"]
    #[inline(always)]
    pub fn vspl(&self) -> VsplR {
        VsplR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The HSYNC pulse polarity. * 0: Low level synchronous clock * 1: High level synchronous clock"]
    #[inline(always)]
    pub fn hspl(&self) -> HsplR {
        HsplR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The ENABLE pulse polarity. * 0: Low level for active data * 1: High level for active data"]
    #[inline(always)]
    pub fn enpl(&self) -> EnplR {
        EnplR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The Data pulse polarity. * 0: Normal * 1: Inverted"]
    #[inline(always)]
    pub fn rgb_datpl(&self) -> RgbDatplR {
        RgbDatplR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The TE pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
    #[inline(always)]
    pub fn rgbtepl(&self) -> RgbteplR {
        RgbteplR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Force Hardware updates RGB I/F parameters after current LCD refresh frame done. CPU writes 1 to force Hardware updating parameters. After updating, this bit is cleared. When the LCDC is running, if the following values related with RGB I/F mode are modified dynamically, only writing 1 to this bit can the newer value be used by hardware after the current frame refresh done. * The CLKDIV field in the LCDC_CLK_DIV register * The VFP, VBP, VSW fields in the LCDC_RGB_VSYNC_CFG register * The HBP, HFP, HSW fields in the LCDC_RGB_HSYNC_CFG register"]
    #[inline(always)]
    pub fn rgbif_update(&self) -> RgbifUpdateR {
        RgbifUpdateR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - * 00: DE mode, frame synchronized with ENABLE signal * 01: HV mode, frame synchronized with synchronous signal * 10: TE mode * Others: Reserved"]
    #[inline(always)]
    pub fn rgb_sync_mode(&self) -> RgbSyncModeR {
        RgbSyncModeR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - The delay interval -5. This interval is from detected TE signal to starting frame transfer. Unit: WR pulse width. The maximum value is 65535. The real delay interval is TEDELAY + 4 or TEDELAY + 5."]
    #[inline(always)]
    pub fn rgb_tedelay(&mut self) -> RgbTedelayW<'_, LcdcRgbCfgSpec> {
        RgbTedelayW::new(self, 0)
    }
    #[doc = "Bit 16 - The polarity of the DCLK active edge. * 0: Data fetched at DCLK rising edge * 1: Data fetched at DCLK falling edge"]
    #[inline(always)]
    pub fn dclkpl(&mut self) -> DclkplW<'_, LcdcRgbCfgSpec> {
        DclkplW::new(self, 16)
    }
    #[doc = "Bit 17 - The VSYNC pulse polarity. * 0: Low level synchronous clock * 1: High level synchronous clock"]
    #[inline(always)]
    pub fn vspl(&mut self) -> VsplW<'_, LcdcRgbCfgSpec> {
        VsplW::new(self, 17)
    }
    #[doc = "Bit 18 - The HSYNC pulse polarity. * 0: Low level synchronous clock * 1: High level synchronous clock"]
    #[inline(always)]
    pub fn hspl(&mut self) -> HsplW<'_, LcdcRgbCfgSpec> {
        HsplW::new(self, 18)
    }
    #[doc = "Bit 19 - The ENABLE pulse polarity. * 0: Low level for active data * 1: High level for active data"]
    #[inline(always)]
    pub fn enpl(&mut self) -> EnplW<'_, LcdcRgbCfgSpec> {
        EnplW::new(self, 19)
    }
    #[doc = "Bit 20 - The Data pulse polarity. * 0: Normal * 1: Inverted"]
    #[inline(always)]
    pub fn rgb_datpl(&mut self) -> RgbDatplW<'_, LcdcRgbCfgSpec> {
        RgbDatplW::new(self, 20)
    }
    #[doc = "Bit 21 - The TE pulse polarity. * 0: Low level for active pulse * 1: High level for active pulse"]
    #[inline(always)]
    pub fn rgbtepl(&mut self) -> RgbteplW<'_, LcdcRgbCfgSpec> {
        RgbteplW::new(self, 21)
    }
    #[doc = "Bit 24 - Force Hardware updates RGB I/F parameters after current LCD refresh frame done. CPU writes 1 to force Hardware updating parameters. After updating, this bit is cleared. When the LCDC is running, if the following values related with RGB I/F mode are modified dynamically, only writing 1 to this bit can the newer value be used by hardware after the current frame refresh done. * The CLKDIV field in the LCDC_CLK_DIV register * The VFP, VBP, VSW fields in the LCDC_RGB_VSYNC_CFG register * The HBP, HFP, HSW fields in the LCDC_RGB_HSYNC_CFG register"]
    #[inline(always)]
    pub fn rgbif_update(&mut self) -> RgbifUpdateW<'_, LcdcRgbCfgSpec> {
        RgbifUpdateW::new(self, 24)
    }
    #[doc = "Bits 25:26 - * 00: DE mode, frame synchronized with ENABLE signal * 01: HV mode, frame synchronized with synchronous signal * 10: TE mode * Others: Reserved"]
    #[inline(always)]
    pub fn rgb_sync_mode(&mut self) -> RgbSyncModeW<'_, LcdcRgbCfgSpec> {
        RgbSyncModeW::new(self, 25)
    }
}
#[doc = "LCDC RGB mode config\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_rgb_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_rgb_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcRgbCfgSpec;
impl crate::RegisterSpec for LcdcRgbCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_rgb_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcRgbCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_rgb_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcRgbCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_RGB_CFG to value 0x0028_0000"]
impl crate::Resettable for LcdcRgbCfgSpec {
    const RESET_VALUE: u32 = 0x0028_0000;
}
