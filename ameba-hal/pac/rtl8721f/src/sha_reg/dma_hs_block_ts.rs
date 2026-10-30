#[doc = "Register `dma_hs_block_ts` reader"]
pub type R = crate::R<DmaHsBlockTsSpec>;
#[doc = "Register `dma_hs_block_ts` writer"]
pub type W = crate::W<DmaHsBlockTsSpec>;
#[doc = "Field `dma_hs_block_ts` reader - Total number of data byte for requesting DMA"]
pub type DmaHsBlockTsR = crate::FieldReader<u32>;
#[doc = "Field `dma_hs_block_ts` writer - Total number of data byte for requesting DMA"]
pub type DmaHsBlockTsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Total number of data byte for requesting DMA"]
    #[inline(always)]
    pub fn dma_hs_block_ts(&self) -> DmaHsBlockTsR {
        DmaHsBlockTsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total number of data byte for requesting DMA"]
    #[inline(always)]
    pub fn dma_hs_block_ts(&mut self) -> DmaHsBlockTsW<'_, DmaHsBlockTsSpec> {
        DmaHsBlockTsW::new(self, 0)
    }
}
#[doc = "DMA hardware handshake block transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_hs_block_ts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_hs_block_ts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaHsBlockTsSpec;
impl crate::RegisterSpec for DmaHsBlockTsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_hs_block_ts::R`](R) reader structure"]
impl crate::Readable for DmaHsBlockTsSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_hs_block_ts::W`](W) writer structure"]
impl crate::Writable for DmaHsBlockTsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dma_hs_block_ts to value 0"]
impl crate::Resettable for DmaHsBlockTsSpec {}
