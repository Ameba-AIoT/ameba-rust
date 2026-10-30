#[doc = "Register `SPDIO_RXBD_ADDR` reader"]
pub type R = crate::R<SpdioRxbdAddrSpec>;
#[doc = "Register `SPDIO_RXBD_ADDR` writer"]
pub type W = crate::W<SpdioRxbdAddrSpec>;
#[doc = "Field `SPDIO_RXBD_ADDR` reader - The base address of RX BD. The value of this register should be configured by CPU. It should be 8-Byte alignment"]
pub type SpdioRxbdAddrR = crate::FieldReader<u32>;
#[doc = "Field `SPDIO_RXBD_ADDR` writer - The base address of RX BD. The value of this register should be configured by CPU. It should be 8-Byte alignment"]
pub type SpdioRxbdAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The base address of RX BD. The value of this register should be configured by CPU. It should be 8-Byte alignment"]
    #[inline(always)]
    pub fn spdio_rxbd_addr(&self) -> SpdioRxbdAddrR {
        SpdioRxbdAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The base address of RX BD. The value of this register should be configured by CPU. It should be 8-Byte alignment"]
    #[inline(always)]
    pub fn spdio_rxbd_addr(&mut self) -> SpdioRxbdAddrW<'_, SpdioRxbdAddrSpec> {
        SpdioRxbdAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_rxbd_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_rxbd_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioRxbdAddrSpec;
impl crate::RegisterSpec for SpdioRxbdAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_rxbd_addr::R`](R) reader structure"]
impl crate::Readable for SpdioRxbdAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_rxbd_addr::W`](W) writer structure"]
impl crate::Writable for SpdioRxbdAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_RXBD_ADDR to value 0"]
impl crate::Resettable for SpdioRxbdAddrSpec {}
