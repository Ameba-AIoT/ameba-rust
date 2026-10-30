#[doc = "Register `LCDC_SHW_RLD_CFG` reader"]
pub type R = crate::R<LcdcShwRldCfgSpec>;
#[doc = "Register `LCDC_SHW_RLD_CFG` writer"]
pub type W = crate::W<LcdcShwRldCfgSpec>;
#[doc = "Field `VBR` reader - vertical blanking reload. This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set). 0: no effect 1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
pub type VbrR = crate::BitReader;
#[doc = "Field `VBR` writer - vertical blanking reload. This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set). 0: no effect 1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
pub type VbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_auto_intvl` reader - interval between two dma transfer in auto-mode"]
pub type DmaAutoIntvlR = crate::FieldReader<u32>;
#[doc = "Field `dma_auto_intvl` writer - interval between two dma transfer in auto-mode"]
pub type DmaAutoIntvlW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `dma_burst_opt` reader - 0: axi 64-byte burst 1: axi 128-byte burst 2: axi 256-byte burst"]
pub type DmaBurstOptR = crate::FieldReader;
#[doc = "Field `dma_burst_opt` writer - 0: axi 64-byte burst 1: axi 128-byte burst 2: axi 256-byte burst"]
pub type DmaBurstOptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - vertical blanking reload. This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set). 0: no effect 1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
    #[inline(always)]
    pub fn vbr(&self) -> VbrR {
        VbrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:27 - interval between two dma transfer in auto-mode"]
    #[inline(always)]
    pub fn dma_auto_intvl(&self) -> DmaAutoIntvlR {
        DmaAutoIntvlR::new((self.bits >> 8) & 0x000f_ffff)
    }
    #[doc = "Bits 30:31 - 0: axi 64-byte burst 1: axi 128-byte burst 2: axi 256-byte burst"]
    #[inline(always)]
    pub fn dma_burst_opt(&self) -> DmaBurstOptR {
        DmaBurstOptR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - vertical blanking reload. This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set). 0: no effect 1: The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
    #[inline(always)]
    pub fn vbr(&mut self) -> VbrW<'_, LcdcShwRldCfgSpec> {
        VbrW::new(self, 0)
    }
    #[doc = "Bits 8:27 - interval between two dma transfer in auto-mode"]
    #[inline(always)]
    pub fn dma_auto_intvl(&mut self) -> DmaAutoIntvlW<'_, LcdcShwRldCfgSpec> {
        DmaAutoIntvlW::new(self, 8)
    }
    #[doc = "Bits 30:31 - 0: axi 64-byte burst 1: axi 128-byte burst 2: axi 256-byte burst"]
    #[inline(always)]
    pub fn dma_burst_opt(&mut self) -> DmaBurstOptW<'_, LcdcShwRldCfgSpec> {
        DmaBurstOptW::new(self, 30)
    }
}
#[doc = "LCDC shadow reload configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_shw_rld_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_shw_rld_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcShwRldCfgSpec;
impl crate::RegisterSpec for LcdcShwRldCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_shw_rld_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcShwRldCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_shw_rld_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcShwRldCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_SHW_RLD_CFG to value 0"]
impl crate::Resettable for LcdcShwRldCfgSpec {}
