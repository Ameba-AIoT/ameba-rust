#[doc = "Register `LCDC_MCU_VSYNC_CFG` reader"]
pub type R = crate::R<LcdcMcuVsyncCfgSpec>;
#[doc = "Register `LCDC_MCU_VSYNC_CFG` writer"]
pub type W = crate::W<LcdcMcuVsyncCfgSpec>;
#[doc = "Field `mcuvsw` reader - VSYNC signal width - 1. Unit: WR pulse width. (only for MCU VSYNC mode)"]
pub type McuvswR = crate::FieldReader;
#[doc = "Field `mcuvsw` writer - VSYNC signal width - 1. Unit: WR pulse width. (only for MCU VSYNC mode)"]
pub type McuvswW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `mcuvspd` reader - * MCUIFMODE = 1: VSYNC idle period. This value equals to VSYNC period - IMAGEWIDTH * IMAGEHEIGHT * WR_pulse_per_pixel-5. Unit: WR pulse width. * MCUIFMODE = 0: The buffer time between frames. This value equals to the time of a frame's end to the next start -5. Unit: WR pulse width. * MCUIFMODE = 2: Invalid usage. The maximum value is 1048571."]
pub type McuvspdR = crate::FieldReader<u32>;
#[doc = "Field `mcuvspd` writer - * MCUIFMODE = 1: VSYNC idle period. This value equals to VSYNC period - IMAGEWIDTH * IMAGEHEIGHT * WR_pulse_per_pixel-5. Unit: WR pulse width. * MCUIFMODE = 0: The buffer time between frames. This value equals to the time of a frame's end to the next start -5. Unit: WR pulse width. * MCUIFMODE = 2: Invalid usage. The maximum value is 1048571."]
pub type McuvspdW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:7 - VSYNC signal width - 1. Unit: WR pulse width. (only for MCU VSYNC mode)"]
    #[inline(always)]
    pub fn mcuvsw(&self) -> McuvswR {
        McuvswR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:31 - * MCUIFMODE = 1: VSYNC idle period. This value equals to VSYNC period - IMAGEWIDTH * IMAGEHEIGHT * WR_pulse_per_pixel-5. Unit: WR pulse width. * MCUIFMODE = 0: The buffer time between frames. This value equals to the time of a frame's end to the next start -5. Unit: WR pulse width. * MCUIFMODE = 2: Invalid usage. The maximum value is 1048571."]
    #[inline(always)]
    pub fn mcuvspd(&self) -> McuvspdR {
        McuvspdR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - VSYNC signal width - 1. Unit: WR pulse width. (only for MCU VSYNC mode)"]
    #[inline(always)]
    pub fn mcuvsw(&mut self) -> McuvswW<'_, LcdcMcuVsyncCfgSpec> {
        McuvswW::new(self, 0)
    }
    #[doc = "Bits 12:31 - * MCUIFMODE = 1: VSYNC idle period. This value equals to VSYNC period - IMAGEWIDTH * IMAGEHEIGHT * WR_pulse_per_pixel-5. Unit: WR pulse width. * MCUIFMODE = 0: The buffer time between frames. This value equals to the time of a frame's end to the next start -5. Unit: WR pulse width. * MCUIFMODE = 2: Invalid usage. The maximum value is 1048571."]
    #[inline(always)]
    pub fn mcuvspd(&mut self) -> McuvspdW<'_, LcdcMcuVsyncCfgSpec> {
        McuvspdW::new(self, 12)
    }
}
#[doc = "MCU Vsync config\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_mcu_vsync_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_mcu_vsync_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcMcuVsyncCfgSpec;
impl crate::RegisterSpec for LcdcMcuVsyncCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_mcu_vsync_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcMcuVsyncCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_mcu_vsync_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcMcuVsyncCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_MCU_VSYNC_CFG to value 0"]
impl crate::Resettable for LcdcMcuVsyncCfgSpec {}
