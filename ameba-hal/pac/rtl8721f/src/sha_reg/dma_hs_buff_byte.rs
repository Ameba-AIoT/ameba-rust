#[doc = "Register `dma_hs_buff_byte` reader"]
pub type R = crate::R<DmaHsBuffByteSpec>;
#[doc = "Register `dma_hs_buff_byte` writer"]
pub type W = crate::W<DmaHsBuffByteSpec>;
#[doc = "Field `dma_hs_buff_byte` reader - The register indicates number of data byte which has been processed in engine. This number will not larger than block size of engine."]
pub type DmaHsBuffByteR = crate::FieldReader;
#[doc = "Field `dma_hs_buff_byte` writer - The register indicates number of data byte which has been processed in engine. This number will not larger than block size of engine."]
pub type DmaHsBuffByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - The register indicates number of data byte which has been processed in engine. This number will not larger than block size of engine."]
    #[inline(always)]
    pub fn dma_hs_buff_byte(&self) -> DmaHsBuffByteR {
        DmaHsBuffByteR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - The register indicates number of data byte which has been processed in engine. This number will not larger than block size of engine."]
    #[inline(always)]
    pub fn dma_hs_buff_byte(&mut self) -> DmaHsBuffByteW<'_, DmaHsBuffByteSpec> {
        DmaHsBuffByteW::new(self, 0)
    }
}
#[doc = "DMA hardware handshake buffer data byte of engine register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_hs_buff_byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_hs_buff_byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaHsBuffByteSpec;
impl crate::RegisterSpec for DmaHsBuffByteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_hs_buff_byte::R`](R) reader structure"]
impl crate::Readable for DmaHsBuffByteSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_hs_buff_byte::W`](W) writer structure"]
impl crate::Writable for DmaHsBuffByteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dma_hs_buff_byte to value 0"]
impl crate::Resettable for DmaHsBuffByteSpec {}
