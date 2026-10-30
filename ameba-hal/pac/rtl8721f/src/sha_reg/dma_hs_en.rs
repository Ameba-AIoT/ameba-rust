#[doc = "Register `dma_hs_en` reader"]
pub type R = crate::R<DmaHsEnSpec>;
#[doc = "Register `dma_hs_en` writer"]
pub type W = crate::W<DmaHsEnSpec>;
#[doc = "Field `dma_hs_en` reader - Once the register is enabled, the engine starts to send dma_req until received wdata reaches trans_data_request."]
pub type DmaHsEnR = crate::BitReader;
#[doc = "Field `dma_hs_en` writer - Once the register is enabled, the engine starts to send dma_req until received wdata reaches trans_data_request."]
pub type DmaHsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Once the register is enabled, the engine starts to send dma_req until received wdata reaches trans_data_request."]
    #[inline(always)]
    pub fn dma_hs_en(&self) -> DmaHsEnR {
        DmaHsEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Once the register is enabled, the engine starts to send dma_req until received wdata reaches trans_data_request."]
    #[inline(always)]
    pub fn dma_hs_en(&mut self) -> DmaHsEnW<'_, DmaHsEnSpec> {
        DmaHsEnW::new(self, 0)
    }
}
#[doc = "DMA hardware handshake enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_hs_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_hs_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaHsEnSpec;
impl crate::RegisterSpec for DmaHsEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_hs_en::R`](R) reader structure"]
impl crate::Readable for DmaHsEnSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_hs_en::W`](W) writer structure"]
impl crate::Writable for DmaHsEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dma_hs_en to value 0"]
impl crate::Resettable for DmaHsEnSpec {}
