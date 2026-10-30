#[doc = "Register `IC_DMA_DATA_LEN` reader"]
pub type R = crate::R<IcDmaDataLenSpec>;
#[doc = "Register `IC_DMA_DATA_LEN` writer"]
pub type W = crate::W<IcDmaDataLenSpec>;
#[doc = "Field `DMA_DAT_LEN` reader - DMA transfer data length"]
pub type DmaDatLenR = crate::FieldReader<u16>;
#[doc = "Field `DMA_DAT_LEN` writer - DMA transfer data length"]
pub type DmaDatLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA transfer data length"]
    #[inline(always)]
    pub fn dma_dat_len(&self) -> DmaDatLenR {
        DmaDatLenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA transfer data length"]
    #[inline(always)]
    pub fn dma_dat_len(&mut self) -> DmaDatLenW<'_, IcDmaDataLenSpec> {
        DmaDatLenW::new(self, 0)
    }
}
#[doc = "I2C DMA Mode Transfer Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_dma_data_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_data_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcDmaDataLenSpec;
impl crate::RegisterSpec for IcDmaDataLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_dma_data_len::R`](R) reader structure"]
impl crate::Readable for IcDmaDataLenSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_dma_data_len::W`](W) writer structure"]
impl crate::Writable for IcDmaDataLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_DMA_DATA_LEN to value 0"]
impl crate::Resettable for IcDmaDataLenSpec {}
