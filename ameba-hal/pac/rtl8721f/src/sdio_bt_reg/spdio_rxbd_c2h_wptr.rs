#[doc = "Register `SPDIO_RXBD_C2H_WPTR` reader"]
pub type R = crate::R<SpdioRxbdC2hWptrSpec>;
#[doc = "Register `SPDIO_RXBD_C2H_WPTR` writer"]
pub type W = crate::W<SpdioRxbdC2hWptrSpec>;
#[doc = "Field `SPDIO_RXBD_C2H_WPTR` reader - When the packet has been processed by CPU and to be moved to host, this index should be updated by CPU"]
pub type SpdioRxbdC2hWptrR = crate::FieldReader<u16>;
#[doc = "Field `SPDIO_RXBD_C2H_WPTR` writer - When the packet has been processed by CPU and to be moved to host, this index should be updated by CPU"]
pub type SpdioRxbdC2hWptrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - When the packet has been processed by CPU and to be moved to host, this index should be updated by CPU"]
    #[inline(always)]
    pub fn spdio_rxbd_c2h_wptr(&self) -> SpdioRxbdC2hWptrR {
        SpdioRxbdC2hWptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - When the packet has been processed by CPU and to be moved to host, this index should be updated by CPU"]
    #[inline(always)]
    pub fn spdio_rxbd_c2h_wptr(&mut self) -> SpdioRxbdC2hWptrW<'_, SpdioRxbdC2hWptrSpec> {
        SpdioRxbdC2hWptrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_rxbd_c2h_wptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_rxbd_c2h_wptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioRxbdC2hWptrSpec;
impl crate::RegisterSpec for SpdioRxbdC2hWptrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spdio_rxbd_c2h_wptr::R`](R) reader structure"]
impl crate::Readable for SpdioRxbdC2hWptrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_rxbd_c2h_wptr::W`](W) writer structure"]
impl crate::Writable for SpdioRxbdC2hWptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_RXBD_C2H_WPTR to value 0"]
impl crate::Resettable for SpdioRxbdC2hWptrSpec {}
