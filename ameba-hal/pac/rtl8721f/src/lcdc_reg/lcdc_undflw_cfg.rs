#[doc = "Register `LCDC_UNDFLW_CFG` reader"]
pub type R = crate::R<LcdcUndflwCfgSpec>;
#[doc = "Register `LCDC_UNDFLW_CFG` writer"]
pub type W = crate::W<LcdcUndflwCfgSpec>;
#[doc = "Field `errout_data` reader - Output data when DMA FIFO underflow occurred. (directly mapping to output D\\[23:0\\])"]
pub type ErroutDataR = crate::FieldReader<u32>;
#[doc = "Field `errout_data` writer - Output data when DMA FIFO underflow occurred. (directly mapping to output D\\[23:0\\])"]
pub type ErroutDataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `dma_un_mode` reader - *0:Output last data *1:Output erroutdata"]
pub type DmaUnModeR = crate::BitReader;
#[doc = "Field `dma_un_mode` writer - *0:Output last data *1:Output erroutdata"]
pub type DmaUnModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_udf_opt` reader - 0: output error data at current frame when dma underflow occurs 1: pause panel interface(clk and data) until underflow timeout counter reaches threshold value, then entering error mode 2: entering drop mode immediately when dma underflow occurs 3: pause panel interface(clk and data) until underflow timeout counter reaches threshold value, then entering drop mode"]
pub type DmaUdfOptR = crate::FieldReader;
#[doc = "Field `dma_udf_opt` writer - 0: output error data at current frame when dma underflow occurs 1: pause panel interface(clk and data) until underflow timeout counter reaches threshold value, then entering error mode 2: entering drop mode immediately when dma underflow occurs 3: pause panel interface(clk and data) until underflow timeout counter reaches threshold value, then entering drop mode"]
pub type DmaUdfOptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:23 - Output data when DMA FIFO underflow occurred. (directly mapping to output D\\[23:0\\])"]
    #[inline(always)]
    pub fn errout_data(&self) -> ErroutDataR {
        ErroutDataR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - *0:Output last data *1:Output erroutdata"]
    #[inline(always)]
    pub fn dma_un_mode(&self) -> DmaUnModeR {
        DmaUnModeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - 0: output error data at current frame when dma underflow occurs 1: pause panel interface(clk and data) until underflow timeout counter reaches threshold value, then entering error mode 2: entering drop mode immediately when dma underflow occurs 3: pause panel interface(clk and data) until underflow timeout counter reaches threshold value, then entering drop mode"]
    #[inline(always)]
    pub fn dma_udf_opt(&self) -> DmaUdfOptR {
        DmaUdfOptR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Output data when DMA FIFO underflow occurred. (directly mapping to output D\\[23:0\\])"]
    #[inline(always)]
    pub fn errout_data(&mut self) -> ErroutDataW<'_, LcdcUndflwCfgSpec> {
        ErroutDataW::new(self, 0)
    }
    #[doc = "Bit 24 - *0:Output last data *1:Output erroutdata"]
    #[inline(always)]
    pub fn dma_un_mode(&mut self) -> DmaUnModeW<'_, LcdcUndflwCfgSpec> {
        DmaUnModeW::new(self, 24)
    }
    #[doc = "Bits 25:26 - 0: output error data at current frame when dma underflow occurs 1: pause panel interface(clk and data) until underflow timeout counter reaches threshold value, then entering error mode 2: entering drop mode immediately when dma underflow occurs 3: pause panel interface(clk and data) until underflow timeout counter reaches threshold value, then entering drop mode"]
    #[inline(always)]
    pub fn dma_udf_opt(&mut self) -> DmaUdfOptW<'_, LcdcUndflwCfgSpec> {
        DmaUdfOptW::new(self, 25)
    }
}
#[doc = "LCDC underflow configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_undflw_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_undflw_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcUndflwCfgSpec;
impl crate::RegisterSpec for LcdcUndflwCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_undflw_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcUndflwCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_undflw_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcUndflwCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_UNDFLW_CFG to value 0"]
impl crate::Resettable for LcdcUndflwCfgSpec {}
