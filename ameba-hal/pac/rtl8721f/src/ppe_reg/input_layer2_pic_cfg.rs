#[doc = "Register `INPUT_LAYER2_PIC_CFG` reader"]
pub type R = crate::R<InputLayer2PicCfgSpec>;
#[doc = "Register `INPUT_LAYER2_PIC_CFG` writer"]
pub type W = crate::W<InputLayer2PicCfgSpec>;
#[doc = "Field `INPUT_LAYER2_FORMAT` reader - Format of pixels from memory"]
pub type InputLayer2FormatR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER2_FORMAT` writer - Format of pixels from memory"]
pub type InputLayer2FormatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INPUT_LAYER2_PIX_SRC` reader - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
pub type InputLayer2PixSrcR = crate::BitReader;
#[doc = "Field `INPUT_LAYER2_PIX_SRC` writer - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
pub type InputLayer2PixSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_LAYER2_INTERP` reader - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
pub type InputLayer2InterpR = crate::BitReader;
#[doc = "Field `INPUT_LAYER2_INTERP` writer - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
pub type InputLayer2InterpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_LAYER2_KEY_MODE` reader - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
pub type InputLayer2KeyModeR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER2_KEY_MODE` writer - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
pub type InputLayer2KeyModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INPUT_LAYER2_LINE_LEN` reader - Indicates the bytes in memory between two vertically adjacent pixels."]
pub type InputLayer2LineLenR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER2_LINE_LEN` writer - Indicates the bytes in memory between two vertically adjacent pixels."]
pub type InputLayer2LineLenW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Format of pixels from memory"]
    #[inline(always)]
    pub fn input_layer2_format(&self) -> InputLayer2FormatR {
        InputLayer2FormatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
    #[inline(always)]
    pub fn input_layer2_pix_src(&self) -> InputLayer2PixSrcR {
        InputLayer2PixSrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
    #[inline(always)]
    pub fn input_layer2_interp(&self) -> InputLayer2InterpR {
        InputLayer2InterpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
    #[inline(always)]
    pub fn input_layer2_key_mode(&self) -> InputLayer2KeyModeR {
        InputLayer2KeyModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:26 - Indicates the bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn input_layer2_line_len(&self) -> InputLayer2LineLenR {
        InputLayer2LineLenR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Format of pixels from memory"]
    #[inline(always)]
    pub fn input_layer2_format(&mut self) -> InputLayer2FormatW<'_, InputLayer2PicCfgSpec> {
        InputLayer2FormatW::new(self, 0)
    }
    #[doc = "Bit 8 - Pixel source: 0x0: CONST_PIX 0x1: DMA (read from memory)"]
    #[inline(always)]
    pub fn input_layer2_pix_src(&mut self) -> InputLayer2PixSrcW<'_, InputLayer2PicCfgSpec> {
        InputLayer2PixSrcW::new(self, 8)
    }
    #[doc = "Bit 10 - Interpolation 0x0: Nearest-neighbor 0x1: Bilinear"]
    #[inline(always)]
    pub fn input_layer2_interp(&mut self) -> InputLayer2InterpW<'_, InputLayer2PicCfgSpec> {
        InputLayer2InterpW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Color keying match mode 0x0, 0x1: Disable 0x2: Inside ((Rmin<=R<=Rmax) & (Gmin<=G<=Gmax) & (Bmin<=B<=Bmax)) 0x3: Outside ((R<Rmin) | (R>Rmax) | (G<Gmin) | (G>Gmax) | (B<Bmin) | (B>Bmax))"]
    #[inline(always)]
    pub fn input_layer2_key_mode(&mut self) -> InputLayer2KeyModeW<'_, InputLayer2PicCfgSpec> {
        InputLayer2KeyModeW::new(self, 12)
    }
    #[doc = "Bits 16:26 - Indicates the bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn input_layer2_line_len(&mut self) -> InputLayer2LineLenW<'_, InputLayer2PicCfgSpec> {
        InputLayer2LineLenW::new(self, 16)
    }
}
#[doc = "Picture configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer2_pic_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer2_pic_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer2PicCfgSpec;
impl crate::RegisterSpec for InputLayer2PicCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer2_pic_cfg::R`](R) reader structure"]
impl crate::Readable for InputLayer2PicCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer2_pic_cfg::W`](W) writer structure"]
impl crate::Writable for InputLayer2PicCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER2_PIC_CFG to value 0"]
impl crate::Resettable for InputLayer2PicCfgSpec {}
