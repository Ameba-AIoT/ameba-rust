#[doc = "Register `DMACFGREG_L` reader"]
pub type R = crate::R<DmacfgregLSpec>;
#[doc = "Register `DMACFGREG_L` writer"]
pub type W = crate::W<DmacfgregLSpec>;
#[doc = "Field `DMACFGREG_L_DMA_EN` reader - RTK_DMAC Enable bit. 0 = RTK_DMAC Disabled 1 = RTK_DMAC Enabled"]
pub type DmacfgregLDmaEnR = crate::BitReader;
#[doc = "Field `DMACFGREG_L_DMA_EN` writer - RTK_DMAC Enable bit. 0 = RTK_DMAC Disabled 1 = RTK_DMAC Enabled"]
pub type DmacfgregLDmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTK_DMAC Enable bit. 0 = RTK_DMAC Disabled 1 = RTK_DMAC Enabled"]
    #[inline(always)]
    pub fn dmacfgreg_l_dma_en(&self) -> DmacfgregLDmaEnR {
        DmacfgregLDmaEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTK_DMAC Enable bit. 0 = RTK_DMAC Disabled 1 = RTK_DMAC Enabled"]
    #[inline(always)]
    pub fn dmacfgreg_l_dma_en(&mut self) -> DmacfgregLDmaEnW<'_, DmacfgregLSpec> {
        DmacfgregLDmaEnW::new(self, 0)
    }
}
#[doc = "DMA low-32 bits Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfgreg_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfgreg_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacfgregLSpec;
impl crate::RegisterSpec for DmacfgregLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfgreg_l::R`](R) reader structure"]
impl crate::Readable for DmacfgregLSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacfgreg_l::W`](W) writer structure"]
impl crate::Writable for DmacfgregLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACFGREG_L to value 0x01"]
impl crate::Resettable for DmacfgregLSpec {
    const RESET_VALUE: u32 = 0x01;
}
