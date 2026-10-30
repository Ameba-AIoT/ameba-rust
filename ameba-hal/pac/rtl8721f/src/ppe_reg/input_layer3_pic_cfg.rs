#[doc = "Register `INPUT_LAYER3_PIC_CFG` reader"]
pub type R = crate::R<InputLayer3PicCfgSpec>;
#[doc = "Register `INPUT_LAYER3_PIC_CFG` writer"]
pub type W = crate::W<InputLayer3PicCfgSpec>;
#[doc = "Field `INPUT_LAYER3_FORMAT` reader - Format of pixels from memory"]
pub type InputLayer3FormatR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER3_FORMAT` writer - Format of pixels from memory"]
pub type InputLayer3FormatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INPUT_LAYER3_PIX_SRC` reader - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
pub type InputLayer3PixSrcR = crate::BitReader;
#[doc = "Field `INPUT_LAYER3_PIX_SRC` writer - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
pub type InputLayer3PixSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_LAYER3_INTERP` reader - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
pub type InputLayer3InterpR = crate::BitReader;
#[doc = "Field `INPUT_LAYER3_INTERP` writer - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
pub type InputLayer3InterpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_LAYER3_KEY_MODE` reader - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
pub type InputLayer3KeyModeR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER3_KEY_MODE` writer - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
pub type InputLayer3KeyModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INPUT_LAYER3_LINE_LEN` reader - Indicates the bytes in memory between two vertically adjacent pixels."]
pub type InputLayer3LineLenR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER3_LINE_LEN` writer - Indicates the bytes in memory between two vertically adjacent pixels."]
pub type InputLayer3LineLenW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Format of pixels from memory"]
    #[inline(always)]
    pub fn input_layer3_format(&self) -> InputLayer3FormatR {
        InputLayer3FormatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
    #[inline(always)]
    pub fn input_layer3_pix_src(&self) -> InputLayer3PixSrcR {
        InputLayer3PixSrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
    #[inline(always)]
    pub fn input_layer3_interp(&self) -> InputLayer3InterpR {
        InputLayer3InterpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
    #[inline(always)]
    pub fn input_layer3_key_mode(&self) -> InputLayer3KeyModeR {
        InputLayer3KeyModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:26 - Indicates the bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn input_layer3_line_len(&self) -> InputLayer3LineLenR {
        InputLayer3LineLenR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Format of pixels from memory"]
    #[inline(always)]
    pub fn input_layer3_format(&mut self) -> InputLayer3FormatW<'_, InputLayer3PicCfgSpec> {
        InputLayer3FormatW::new(self, 0)
    }
    #[doc = "Bit 8 - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
    #[inline(always)]
    pub fn input_layer3_pix_src(&mut self) -> InputLayer3PixSrcW<'_, InputLayer3PicCfgSpec> {
        InputLayer3PixSrcW::new(self, 8)
    }
    #[doc = "Bit 10 - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
    #[inline(always)]
    pub fn input_layer3_interp(&mut self) -> InputLayer3InterpW<'_, InputLayer3PicCfgSpec> {
        InputLayer3InterpW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
    #[inline(always)]
    pub fn input_layer3_key_mode(&mut self) -> InputLayer3KeyModeW<'_, InputLayer3PicCfgSpec> {
        InputLayer3KeyModeW::new(self, 12)
    }
    #[doc = "Bits 16:26 - Indicates the bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn input_layer3_line_len(&mut self) -> InputLayer3LineLenW<'_, InputLayer3PicCfgSpec> {
        InputLayer3LineLenW::new(self, 16)
    }
}
#[doc = "Picture configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer3_pic_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer3_pic_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer3PicCfgSpec;
impl crate::RegisterSpec for InputLayer3PicCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer3_pic_cfg::R`](R) reader structure"]
impl crate::Readable for InputLayer3PicCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer3_pic_cfg::W`](W) writer structure"]
impl crate::Writable for InputLayer3PicCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER3_PIC_CFG to value 0"]
impl crate::Resettable for InputLayer3PicCfgSpec {}
