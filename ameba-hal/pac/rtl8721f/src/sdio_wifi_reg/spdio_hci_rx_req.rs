#[doc = "Register `SPDIO_HCI_RX_REQ` reader"]
pub type R = crate::R<SpdioHciRxReqSpec>;
#[doc = "Register `SPDIO_HCI_RX_REQ` writer"]
pub type W = crate::W<SpdioHciRxReqSpec>;
#[doc = "Field `HCI_RX_REQ` reader - CPU triggers this bit to enable SDIO IP RX transfer by fetch BD info. SDIO fetch BD to get the RX length and address and transfer RX packet to Host, and clear this bit when all BD transfer done."]
pub type HciRxReqR = crate::BitReader;
#[doc = "Field `HCI_RX_REQ` writer - CPU triggers this bit to enable SDIO IP RX transfer by fetch BD info. SDIO fetch BD to get the RX length and address and transfer RX packet to Host, and clear this bit when all BD transfer done."]
pub type HciRxReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU triggers this bit to enable SDIO IP RX transfer by fetch BD info. SDIO fetch BD to get the RX length and address and transfer RX packet to Host, and clear this bit when all BD transfer done."]
    #[inline(always)]
    pub fn hci_rx_req(&self) -> HciRxReqR {
        HciRxReqR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU triggers this bit to enable SDIO IP RX transfer by fetch BD info. SDIO fetch BD to get the RX length and address and transfer RX packet to Host, and clear this bit when all BD transfer done."]
    #[inline(always)]
    pub fn hci_rx_req(&mut self) -> HciRxReqW<'_, SpdioHciRxReqSpec> {
        HciRxReqW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_hci_rx_req::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_hci_rx_req::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioHciRxReqSpec;
impl crate::RegisterSpec for SpdioHciRxReqSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spdio_hci_rx_req::R`](R) reader structure"]
impl crate::Readable for SpdioHciRxReqSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_hci_rx_req::W`](W) writer structure"]
impl crate::Writable for SpdioHciRxReqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_HCI_RX_REQ to value 0"]
impl crate::Resettable for SpdioHciRxReqSpec {}
