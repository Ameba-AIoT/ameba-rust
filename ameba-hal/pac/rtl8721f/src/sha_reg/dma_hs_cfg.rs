#[doc = "Register `dma_hs_cfg` reader"]
pub type R = crate::R<DmaHsCfgSpec>;
#[doc = "Register `dma_hs_cfg` writer"]
pub type W = crate::W<DmaHsCfgSpec>;
#[doc = "Field `dma_hs_tr_width` reader - The expected data width of each write transaction from DMA. Since the engine uses the APB protocol, please set the field to 3'b010."]
pub type DmaHsTrWidthR = crate::FieldReader;
#[doc = "Field `dma_hs_tr_width` writer - The expected data width of each write transaction from DMA. Since the engine uses the APB protocol, please set the field to 3'b010."]
pub type DmaHsTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dma_hs_msize` reader - The expected number of burst write transaction length. Please set this field to 3'b000 to avoid data loss."]
pub type DmaHsMsizeR = crate::FieldReader;
#[doc = "Field `dma_hs_msize` writer - The expected number of burst write transaction length. Please set this field to 3'b000 to avoid data loss."]
pub type DmaHsMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - The expected data width of each write transaction from DMA. Since the engine uses the APB protocol, please set the field to 3'b010."]
    #[inline(always)]
    pub fn dma_hs_tr_width(&self) -> DmaHsTrWidthR {
        DmaHsTrWidthR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The expected number of burst write transaction length. Please set this field to 3'b000 to avoid data loss."]
    #[inline(always)]
    pub fn dma_hs_msize(&self) -> DmaHsMsizeR {
        DmaHsMsizeR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - The expected data width of each write transaction from DMA. Since the engine uses the APB protocol, please set the field to 3'b010."]
    #[inline(always)]
    pub fn dma_hs_tr_width(&mut self) -> DmaHsTrWidthW<'_, DmaHsCfgSpec> {
        DmaHsTrWidthW::new(self, 0)
    }
    #[doc = "Bits 3:5 - The expected number of burst write transaction length. Please set this field to 3'b000 to avoid data loss."]
    #[inline(always)]
    pub fn dma_hs_msize(&mut self) -> DmaHsMsizeW<'_, DmaHsCfgSpec> {
        DmaHsMsizeW::new(self, 3)
    }
}
#[doc = "DMA hardware handshake configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_hs_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_hs_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaHsCfgSpec;
impl crate::RegisterSpec for DmaHsCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_hs_cfg::R`](R) reader structure"]
impl crate::Readable for DmaHsCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_hs_cfg::W`](W) writer structure"]
impl crate::Writable for DmaHsCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dma_hs_cfg to value 0"]
impl crate::Resettable for DmaHsCfgSpec {}
