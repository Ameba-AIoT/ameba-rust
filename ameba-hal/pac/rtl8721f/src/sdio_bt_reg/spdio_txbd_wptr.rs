#[doc = "Register `SPDIO_TXBD_WPTR` reader"]
pub type R = crate::R<SpdioTxbdWptrSpec>;
#[doc = "Field `SPDIO_TXBD_H2C_WPTR` reader - When the packet has been sent from Host to TX Buffer, this index should be updated by SDIO IP."]
pub type SpdioTxbdH2cWptrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - When the packet has been sent from Host to TX Buffer, this index should be updated by SDIO IP."]
    #[inline(always)]
    pub fn spdio_txbd_h2c_wptr(&self) -> SpdioTxbdH2cWptrR {
        SpdioTxbdH2cWptrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_txbd_wptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioTxbdWptrSpec;
impl crate::RegisterSpec for SpdioTxbdWptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_txbd_wptr::R`](R) reader structure"]
impl crate::Readable for SpdioTxbdWptrSpec {}
#[doc = "`reset()` method sets SPDIO_TXBD_WPTR to value 0"]
impl crate::Resettable for SpdioTxbdWptrSpec {}
