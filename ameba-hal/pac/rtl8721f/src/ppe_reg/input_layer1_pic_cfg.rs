#[doc = "Register `INPUT_LAYER1_PIC_CFG` reader"]
pub type R = crate::R<InputLayer1PicCfgSpec>;
#[doc = "Register `INPUT_LAYER1_PIC_CFG` writer"]
pub type W = crate::W<InputLayer1PicCfgSpec>;
#[doc = "Field `INPUT_LAYER1_FORMAT` reader - Format of pixels from memory"]
pub type InputLayer1FormatR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER1_FORMAT` writer - Format of pixels from memory"]
pub type InputLayer1FormatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INPUT_LAYER1_PIX_SRC` reader - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
pub type InputLayer1PixSrcR = crate::BitReader;
#[doc = "Field `INPUT_LAYER1_PIX_SRC` writer - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
pub type InputLayer1PixSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_LAYER1_INTERP` reader - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
pub type InputLayer1InterpR = crate::BitReader;
#[doc = "Field `INPUT_LAYER1_INTERP` writer - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
pub type InputLayer1InterpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_LAYER1_KEY_MODE` reader - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
pub type InputLayer1KeyModeR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER1_KEY_MODE` writer - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
pub type InputLayer1KeyModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INPUT_LAYER1_LINE_LEN` reader - Indicates the bytes in memory between two vertically adjacent pixels."]
pub type InputLayer1LineLenR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER1_LINE_LEN` writer - Indicates the bytes in memory between two vertically adjacent pixels."]
pub type InputLayer1LineLenW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Format of pixels from memory"]
    #[inline(always)]
    pub fn input_layer1_format(&self) -> InputLayer1FormatR {
        InputLayer1FormatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
    #[inline(always)]
    pub fn input_layer1_pix_src(&self) -> InputLayer1PixSrcR {
        InputLayer1PixSrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
    #[inline(always)]
    pub fn input_layer1_interp(&self) -> InputLayer1InterpR {
        InputLayer1InterpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
    #[inline(always)]
    pub fn input_layer1_key_mode(&self) -> InputLayer1KeyModeR {
        InputLayer1KeyModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:26 - Indicates the bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn input_layer1_line_len(&self) -> InputLayer1LineLenR {
        InputLayer1LineLenR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Format of pixels from memory"]
    #[inline(always)]
    pub fn input_layer1_format(&mut self) -> InputLayer1FormatW<'_, InputLayer1PicCfgSpec> {
        InputLayer1FormatW::new(self, 0)
    }
    #[doc = "Bit 8 - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
    #[inline(always)]
    pub fn input_layer1_pix_src(&mut self) -> InputLayer1PixSrcW<'_, InputLayer1PicCfgSpec> {
        InputLayer1PixSrcW::new(self, 8)
    }
    #[doc = "Bit 10 - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
    #[inline(always)]
    pub fn input_layer1_interp(&mut self) -> InputLayer1InterpW<'_, InputLayer1PicCfgSpec> {
        InputLayer1InterpW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
    #[inline(always)]
    pub fn input_layer1_key_mode(&mut self) -> InputLayer1KeyModeW<'_, InputLayer1PicCfgSpec> {
        InputLayer1KeyModeW::new(self, 12)
    }
    #[doc = "Bits 16:26 - Indicates the bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn input_layer1_line_len(&mut self) -> InputLayer1LineLenW<'_, InputLayer1PicCfgSpec> {
        InputLayer1LineLenW::new(self, 16)
    }
}
#[doc = "Picture configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer1_pic_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer1_pic_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer1PicCfgSpec;
impl crate::RegisterSpec for InputLayer1PicCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer1_pic_cfg::R`](R) reader structure"]
impl crate::Readable for InputLayer1PicCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer1_pic_cfg::W`](W) writer structure"]
impl crate::Writable for InputLayer1PicCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER1_PIC_CFG to value 0"]
impl crate::Resettable for InputLayer1PicCfgSpec {}
