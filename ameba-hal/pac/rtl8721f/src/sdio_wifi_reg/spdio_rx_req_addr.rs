#[doc = "Register `SPDIO_RX_REQ_ADDR` reader"]
pub type R = crate::R<SpdioRxReqAddrSpec>;
#[doc = "Field `SPDIO_RX_REQ_ADDR` reader - The address of the requested RX Packet. This is unnecessary for driver."]
pub type SpdioRxReqAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the requested RX Packet. This is unnecessary for driver."]
    #[inline(always)]
    pub fn spdio_rx_req_addr(&self) -> SpdioRxReqAddrR {
        SpdioRxReqAddrR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_rx_req_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioRxReqAddrSpec;
impl crate::RegisterSpec for SpdioRxReqAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_rx_req_addr::R`](R) reader structure"]
impl crate::Readable for SpdioRxReqAddrSpec {}
#[doc = "`reset()` method sets SPDIO_RX_REQ_ADDR to value 0"]
impl crate::Resettable for SpdioRxReqAddrSpec {}
