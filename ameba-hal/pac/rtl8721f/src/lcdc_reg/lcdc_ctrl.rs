#[doc = "Register `LCDC_CTRL` reader"]
pub type R = crate::R<LcdcCtrlSpec>;
#[doc = "Register `LCDC_CTRL` writer"]
pub type W = crate::W<LcdcCtrlSpec>;
#[doc = "Field `lcdc_en` reader - Write 1 to enable LCDC."]
pub type LcdcEnR = crate::BitReader;
#[doc = "Field `lcdc_en` writer - Write 1 to enable LCDC."]
pub type LcdcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lcdc_inst_dis` reader - Disable LCDC instantly. Reset LCDC internal states and disable LCDC then clear both this bit and LCDCEN."]
pub type LcdcInstDisR = crate::BitReader;
#[doc = "Field `lcdc_inst_dis` writer - Disable LCDC instantly. Reset LCDC internal states and disable LCDC then clear both this bit and LCDCEN."]
pub type LcdcInstDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lcdc_if_mipi` reader - "]
pub type LcdcIfMipiR = crate::BitReader;
#[doc = "Field `lcdc_if_mipi` writer - "]
pub type LcdcIfMipiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lcdc_if_mode` reader - *0000: 8-bit MCU (RGB565/RGB888 support) *0001: 16-bit MCU (RGB565 support) *0010: 9-bit MCU (RGB666 support) *0011: 18-bit MCU (RGB666 support) *0100: 24-bit MCU (RGB888 support) *1000: 6-bit RGB parallel interface (RGB565/RGB666 support) *1001: 16-bit RGB parallel interface (RGB565 support) *1010: 18-bit RGB parallel interface (RGB666 support) *1011: 8-bit RGB parallel interface (RGB888 support) *1100: 24-bit RGB parallel interface (RGB888 support) *Others: Reserved"]
pub type LcdcIfModeR = crate::FieldReader;
#[doc = "Field `lcdc_if_mode` writer - *0000: 8-bit MCU (RGB565/RGB888 support) *0001: 16-bit MCU (RGB565 support) *0010: 9-bit MCU (RGB666 support) *0011: 18-bit MCU (RGB666 support) *0100: 24-bit MCU (RGB888 support) *1000: 6-bit RGB parallel interface (RGB565/RGB666 support) *1001: 16-bit RGB parallel interface (RGB565 support) *1010: 18-bit RGB parallel interface (RGB666 support) *1011: 8-bit RGB parallel interface (RGB888 support) *1100: 24-bit RGB parallel interface (RGB888 support) *Others: Reserved"]
pub type LcdcIfModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable LCDC."]
    #[inline(always)]
    pub fn lcdc_en(&self) -> LcdcEnR {
        LcdcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Disable LCDC instantly. Reset LCDC internal states and disable LCDC then clear both this bit and LCDCEN."]
    #[inline(always)]
    pub fn lcdc_inst_dis(&self) -> LcdcInstDisR {
        LcdcInstDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lcdc_if_mipi(&self) -> LcdcIfMipiR {
        LcdcIfMipiR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - *0000: 8-bit MCU (RGB565/RGB888 support) *0001: 16-bit MCU (RGB565 support) *0010: 9-bit MCU (RGB666 support) *0011: 18-bit MCU (RGB666 support) *0100: 24-bit MCU (RGB888 support) *1000: 6-bit RGB parallel interface (RGB565/RGB666 support) *1001: 16-bit RGB parallel interface (RGB565 support) *1010: 18-bit RGB parallel interface (RGB666 support) *1011: 8-bit RGB parallel interface (RGB888 support) *1100: 24-bit RGB parallel interface (RGB888 support) *Others: Reserved"]
    #[inline(always)]
    pub fn lcdc_if_mode(&self) -> LcdcIfModeR {
        LcdcIfModeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable LCDC."]
    #[inline(always)]
    pub fn lcdc_en(&mut self) -> LcdcEnW<'_, LcdcCtrlSpec> {
        LcdcEnW::new(self, 0)
    }
    #[doc = "Bit 2 - Disable LCDC instantly. Reset LCDC internal states and disable LCDC then clear both this bit and LCDCEN."]
    #[inline(always)]
    pub fn lcdc_inst_dis(&mut self) -> LcdcInstDisW<'_, LcdcCtrlSpec> {
        LcdcInstDisW::new(self, 2)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lcdc_if_mipi(&mut self) -> LcdcIfMipiW<'_, LcdcCtrlSpec> {
        LcdcIfMipiW::new(self, 15)
    }
    #[doc = "Bits 16:19 - *0000: 8-bit MCU (RGB565/RGB888 support) *0001: 16-bit MCU (RGB565 support) *0010: 9-bit MCU (RGB666 support) *0011: 18-bit MCU (RGB666 support) *0100: 24-bit MCU (RGB888 support) *1000: 6-bit RGB parallel interface (RGB565/RGB666 support) *1001: 16-bit RGB parallel interface (RGB565 support) *1010: 18-bit RGB parallel interface (RGB666 support) *1011: 8-bit RGB parallel interface (RGB888 support) *1100: 24-bit RGB parallel interface (RGB888 support) *Others: Reserved"]
    #[inline(always)]
    pub fn lcdc_if_mode(&mut self) -> LcdcIfModeW<'_, LcdcCtrlSpec> {
        LcdcIfModeW::new(self, 16)
    }
}
#[doc = "LCDC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcCtrlSpec;
impl crate::RegisterSpec for LcdcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_ctrl::R`](R) reader structure"]
impl crate::Readable for LcdcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_ctrl::W`](W) writer structure"]
impl crate::Writable for LcdcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_CTRL to value 0x0009_0000"]
impl crate::Resettable for LcdcCtrlSpec {
    const RESET_VALUE: u32 = 0x0009_0000;
}
