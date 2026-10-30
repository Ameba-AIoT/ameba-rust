#[doc = "Register `SPDIO_RXBD_C2H_RPTR` reader"]
pub type R = crate::R<SpdioRxbdC2hRptrSpec>;
#[doc = "Field `SPDIO_RXBD_C2H_RPTR` reader - When the packet has been moved to host, this index should be updated by SDIO IP."]
pub type SpdioRxbdC2hRptrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - When the packet has been moved to host, this index should be updated by SDIO IP."]
    #[inline(always)]
    pub fn spdio_rxbd_c2h_rptr(&self) -> SpdioRxbdC2hRptrR {
        SpdioRxbdC2hRptrR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_rxbd_c2h_rptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioRxbdC2hRptrSpec;
impl crate::RegisterSpec for SpdioRxbdC2hRptrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spdio_rxbd_c2h_rptr::R`](R) reader structure"]
impl crate::Readable for SpdioRxbdC2hRptrSpec {}
#[doc = "`reset()` method sets SPDIO_RXBD_C2H_RPTR to value 0"]
impl crate::Resettable for SpdioRxbdC2hRptrSpec {}
