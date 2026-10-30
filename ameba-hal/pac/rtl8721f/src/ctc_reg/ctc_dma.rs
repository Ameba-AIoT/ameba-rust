#[doc = "Register `CTC_DMA` reader"]
pub type R = crate::R<CtcDmaSpec>;
#[doc = "Register `CTC_DMA` writer"]
pub type W = crate::W<CtcDmaSpec>;
#[doc = "Field `rxdma_en` reader - 1:enable ctc rxdma Note: ctc dma interface must used GDMA as flow control , IP flow control is not support ."]
pub type RxdmaEnR = crate::BitReader;
#[doc = "Field `rxdma_en` writer - 1:enable ctc rxdma Note: ctc dma interface must used GDMA as flow control , IP flow control is not support ."]
pub type RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxdma_msize` reader - ctc rxdma burst size. When fifo reach certain entry depth , controller will req dma operation. 0: 1 entry 1: 4 2: 8 N: 2**(N+1) Note: this setting must be same with GDMA burst len , or fifo will underflow or overflow."]
pub type RxdmaMsizeR = crate::FieldReader;
#[doc = "Field `rxdma_msize` writer - ctc rxdma burst size. When fifo reach certain entry depth , controller will req dma operation. 0: 1 entry 1: 4 2: 8 N: 2**(N+1) Note: this setting must be same with GDMA burst len , or fifo will underflow or overflow."]
pub type RxdmaMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 1:enable ctc rxdma Note: ctc dma interface must used GDMA as flow control , IP flow control is not support ."]
    #[inline(always)]
    pub fn rxdma_en(&self) -> RxdmaEnR {
        RxdmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - ctc rxdma burst size. When fifo reach certain entry depth , controller will req dma operation. 0: 1 entry 1: 4 2: 8 N: 2**(N+1) Note: this setting must be same with GDMA burst len , or fifo will underflow or overflow."]
    #[inline(always)]
    pub fn rxdma_msize(&self) -> RxdmaMsizeR {
        RxdmaMsizeR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1:enable ctc rxdma Note: ctc dma interface must used GDMA as flow control , IP flow control is not support ."]
    #[inline(always)]
    pub fn rxdma_en(&mut self) -> RxdmaEnW<'_, CtcDmaSpec> {
        RxdmaEnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - ctc rxdma burst size. When fifo reach certain entry depth , controller will req dma operation. 0: 1 entry 1: 4 2: 8 N: 2**(N+1) Note: this setting must be same with GDMA burst len , or fifo will underflow or overflow."]
    #[inline(always)]
    pub fn rxdma_msize(&mut self) -> RxdmaMsizeW<'_, CtcDmaSpec> {
        RxdmaMsizeW::new(self, 4)
    }
}
#[doc = "Captouch DMA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcDmaSpec;
impl crate::RegisterSpec for CtcDmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_dma::R`](R) reader structure"]
impl crate::Readable for CtcDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctc_dma::W`](W) writer structure"]
impl crate::Writable for CtcDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_DMA to value 0"]
impl crate::Resettable for CtcDmaSpec {}
