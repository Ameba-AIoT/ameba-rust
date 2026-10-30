#[doc = "Register `SPDIO_TXBD_RPTR` reader"]
pub type R = crate::R<SpdioTxbdRptrSpec>;
#[doc = "Register `SPDIO_TXBD_RPTR` writer"]
pub type W = crate::W<SpdioTxbdRptrSpec>;
#[doc = "Field `SPDIO_TXBD_H2C_RPTR` reader - When the packet has been processed by CPU and moved to MAC TX FIFO, this index should be updated by CPU."]
pub type SpdioTxbdH2cRptrR = crate::FieldReader<u16>;
#[doc = "Field `SPDIO_TXBD_H2C_RPTR` writer - When the packet has been processed by CPU and moved to MAC TX FIFO, this index should be updated by CPU."]
pub type SpdioTxbdH2cRptrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - When the packet has been processed by CPU and moved to MAC TX FIFO, this index should be updated by CPU."]
    #[inline(always)]
    pub fn spdio_txbd_h2c_rptr(&self) -> SpdioTxbdH2cRptrR {
        SpdioTxbdH2cRptrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When the packet has been processed by CPU and moved to MAC TX FIFO, this index should be updated by CPU."]
    #[inline(always)]
    pub fn spdio_txbd_h2c_rptr(&mut self) -> SpdioTxbdH2cRptrW<'_, SpdioTxbdRptrSpec> {
        SpdioTxbdH2cRptrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_txbd_rptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_txbd_rptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioTxbdRptrSpec;
impl crate::RegisterSpec for SpdioTxbdRptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_txbd_rptr::R`](R) reader structure"]
impl crate::Readable for SpdioTxbdRptrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_txbd_rptr::W`](W) writer structure"]
impl crate::Writable for SpdioTxbdRptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_TXBD_RPTR to value 0"]
impl crate::Resettable for SpdioTxbdRptrSpec {}
