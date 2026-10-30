#[doc = "Register `SPDIO_RX_REQ_ADDR` reader"]
pub type R = crate::R<SpdioRxReqAddrSpec>;
#[doc = "Register `SPDIO_RX_REQ_ADDR` writer"]
pub type W = crate::W<SpdioRxReqAddrSpec>;
#[doc = "Field `SPDIO_RX_REQ_ADDR` reader - The address of the requested RX Packet. This is unnecessary for driver."]
pub type SpdioRxReqAddrR = crate::FieldReader<u32>;
#[doc = "Field `SPDIO_RX_REQ_ADDR` writer - The address of the requested RX Packet. This is unnecessary for driver."]
pub type SpdioRxReqAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the requested RX Packet. This is unnecessary for driver."]
    #[inline(always)]
    pub fn spdio_rx_req_addr(&self) -> SpdioRxReqAddrR {
        SpdioRxReqAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The address of the requested RX Packet. This is unnecessary for driver."]
    #[inline(always)]
    pub fn spdio_rx_req_addr(&mut self) -> SpdioRxReqAddrW<'_, SpdioRxReqAddrSpec> {
        SpdioRxReqAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_rx_req_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_rx_req_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioRxReqAddrSpec;
impl crate::RegisterSpec for SpdioRxReqAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_rx_req_addr::R`](R) reader structure"]
impl crate::Readable for SpdioRxReqAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_rx_req_addr::W`](W) writer structure"]
impl crate::Writable for SpdioRxReqAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_RX_REQ_ADDR to value 0"]
impl crate::Resettable for SpdioRxReqAddrSpec {}
