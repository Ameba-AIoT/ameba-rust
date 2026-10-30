#[doc = "Register `RXFTLR` reader"]
pub type R = crate::R<RxftlrSpec>;
#[doc = "Register `RXFTLR` writer"]
pub type W = crate::W<RxftlrSpec>;
#[doc = "Field `RFT` reader - Receive FIFO Threshold. Controls the level of entries (or above) at which the receive FIFO controller triggers an interrupt. The FIFO depth is configurable in the range 2~64. This register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than the depth of the FIFO, this field is not written and retains its current value. When the number of receive FIFO entries is greater than or equal to this value + 1, the receive FIFO full interrupt is triggered. For RFT decode, refer to the following description: 000000: ssi_rxf_intr is asserted when 1 or more data entries are present in receive FIFO. 000001: ssi_rxf_intr is asserted when 2 or more data entries are present in receive FIFO. 000010: ssi_rxf_intr is asserted when 3 or more data entries are present in receive FIFO. 000011: ssi_rxf_intr is asserted when 4 or more data entries are present in receive FIFO. ... 111110: ssi_rxf_intr is asserted when 61 or more data entries are present in receive FIFO. 111101: ssi_rxf_intr is asserted when 62 or more data entries are present in receive FIFO. 111110: ssi_rxf_intr is asserted when 63 or more data entries are present in receive FIFO. 111111: ssi_rxf_intr is asserted when 64 or more data entries are present in receive FIFO."]
pub type RftR = crate::FieldReader;
#[doc = "Field `RFT` writer - Receive FIFO Threshold. Controls the level of entries (or above) at which the receive FIFO controller triggers an interrupt. The FIFO depth is configurable in the range 2~64. This register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than the depth of the FIFO, this field is not written and retains its current value. When the number of receive FIFO entries is greater than or equal to this value + 1, the receive FIFO full interrupt is triggered. For RFT decode, refer to the following description: 000000: ssi_rxf_intr is asserted when 1 or more data entries are present in receive FIFO. 000001: ssi_rxf_intr is asserted when 2 or more data entries are present in receive FIFO. 000010: ssi_rxf_intr is asserted when 3 or more data entries are present in receive FIFO. 000011: ssi_rxf_intr is asserted when 4 or more data entries are present in receive FIFO. ... 111110: ssi_rxf_intr is asserted when 61 or more data entries are present in receive FIFO. 111101: ssi_rxf_intr is asserted when 62 or more data entries are present in receive FIFO. 111110: ssi_rxf_intr is asserted when 63 or more data entries are present in receive FIFO. 111111: ssi_rxf_intr is asserted when 64 or more data entries are present in receive FIFO."]
pub type RftW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Receive FIFO Threshold. Controls the level of entries (or above) at which the receive FIFO controller triggers an interrupt. The FIFO depth is configurable in the range 2~64. This register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than the depth of the FIFO, this field is not written and retains its current value. When the number of receive FIFO entries is greater than or equal to this value + 1, the receive FIFO full interrupt is triggered. For RFT decode, refer to the following description: 000000: ssi_rxf_intr is asserted when 1 or more data entries are present in receive FIFO. 000001: ssi_rxf_intr is asserted when 2 or more data entries are present in receive FIFO. 000010: ssi_rxf_intr is asserted when 3 or more data entries are present in receive FIFO. 000011: ssi_rxf_intr is asserted when 4 or more data entries are present in receive FIFO. ... 111110: ssi_rxf_intr is asserted when 61 or more data entries are present in receive FIFO. 111101: ssi_rxf_intr is asserted when 62 or more data entries are present in receive FIFO. 111110: ssi_rxf_intr is asserted when 63 or more data entries are present in receive FIFO. 111111: ssi_rxf_intr is asserted when 64 or more data entries are present in receive FIFO."]
    #[inline(always)]
    pub fn rft(&self) -> RftR {
        RftR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Receive FIFO Threshold. Controls the level of entries (or above) at which the receive FIFO controller triggers an interrupt. The FIFO depth is configurable in the range 2~64. This register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than the depth of the FIFO, this field is not written and retains its current value. When the number of receive FIFO entries is greater than or equal to this value + 1, the receive FIFO full interrupt is triggered. For RFT decode, refer to the following description: 000000: ssi_rxf_intr is asserted when 1 or more data entries are present in receive FIFO. 000001: ssi_rxf_intr is asserted when 2 or more data entries are present in receive FIFO. 000010: ssi_rxf_intr is asserted when 3 or more data entries are present in receive FIFO. 000011: ssi_rxf_intr is asserted when 4 or more data entries are present in receive FIFO. ... 111110: ssi_rxf_intr is asserted when 61 or more data entries are present in receive FIFO. 111101: ssi_rxf_intr is asserted when 62 or more data entries are present in receive FIFO. 111110: ssi_rxf_intr is asserted when 63 or more data entries are present in receive FIFO. 111111: ssi_rxf_intr is asserted when 64 or more data entries are present in receive FIFO."]
    #[inline(always)]
    pub fn rft(&mut self) -> RftW<'_, RxftlrSpec> {
        RftW::new(self, 0)
    }
}
#[doc = "Receive FIFO Threshold Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rxftlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxftlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxftlrSpec;
impl crate::RegisterSpec for RxftlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxftlr::R`](R) reader structure"]
impl crate::Readable for RxftlrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxftlr::W`](W) writer structure"]
impl crate::Writable for RxftlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXFTLR to value 0"]
impl crate::Resettable for RxftlrSpec {}
