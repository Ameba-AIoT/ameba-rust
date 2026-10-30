#[doc = "Register `TXFTLR` reader"]
pub type R = crate::R<TxftlrSpec>;
#[doc = "Register `TXFTLR` writer"]
pub type W = crate::W<TxftlrSpec>;
#[doc = "Field `TFT` reader - Transmit FIFO Threshold. Controls the level of entries (or below) at which the transmit FIFO controller triggers an interrupt. The FIFO depth is 64; this register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than or equal to the depth of the FIFO, this field is not written and retains its current value. When the number of transmit FIFO entries is less than or equal to this value, the transmit FIFO empty interrupt is triggered. For TFT decode, refer to the following description: 000000: ssi_txe_intr is asserted when 0 data entry is present in transmit FIFO. 000001: ssi_txe_intr is asserted when 1 data entry is present in transmit FIFO. 000010: ssi_txe_intr is asserted when 2 data entries are present in transmit FIFO. 000011: ssi_txe_intr is asserted when 3 data entries are present in transmit FIFO. ... 111100: ssi_txe_intr is asserted when 60 data entries are present in transmit FIFO. 111101: ssi_txe_intr is asserted when 61 data entries are present in transmit FIFO. 111110: ssi_txe_intr is asserted when 62 data entries are present in transmit FIFO. 111111: ssi_txe_intr is asserted when 63 data entries are present in transmit FIFO."]
pub type TftR = crate::FieldReader;
#[doc = "Field `TFT` writer - Transmit FIFO Threshold. Controls the level of entries (or below) at which the transmit FIFO controller triggers an interrupt. The FIFO depth is 64; this register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than or equal to the depth of the FIFO, this field is not written and retains its current value. When the number of transmit FIFO entries is less than or equal to this value, the transmit FIFO empty interrupt is triggered. For TFT decode, refer to the following description: 000000: ssi_txe_intr is asserted when 0 data entry is present in transmit FIFO. 000001: ssi_txe_intr is asserted when 1 data entry is present in transmit FIFO. 000010: ssi_txe_intr is asserted when 2 data entries are present in transmit FIFO. 000011: ssi_txe_intr is asserted when 3 data entries are present in transmit FIFO. ... 111100: ssi_txe_intr is asserted when 60 data entries are present in transmit FIFO. 111101: ssi_txe_intr is asserted when 61 data entries are present in transmit FIFO. 111110: ssi_txe_intr is asserted when 62 data entries are present in transmit FIFO. 111111: ssi_txe_intr is asserted when 63 data entries are present in transmit FIFO."]
pub type TftW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Threshold. Controls the level of entries (or below) at which the transmit FIFO controller triggers an interrupt. The FIFO depth is 64; this register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than or equal to the depth of the FIFO, this field is not written and retains its current value. When the number of transmit FIFO entries is less than or equal to this value, the transmit FIFO empty interrupt is triggered. For TFT decode, refer to the following description: 000000: ssi_txe_intr is asserted when 0 data entry is present in transmit FIFO. 000001: ssi_txe_intr is asserted when 1 data entry is present in transmit FIFO. 000010: ssi_txe_intr is asserted when 2 data entries are present in transmit FIFO. 000011: ssi_txe_intr is asserted when 3 data entries are present in transmit FIFO. ... 111100: ssi_txe_intr is asserted when 60 data entries are present in transmit FIFO. 111101: ssi_txe_intr is asserted when 61 data entries are present in transmit FIFO. 111110: ssi_txe_intr is asserted when 62 data entries are present in transmit FIFO. 111111: ssi_txe_intr is asserted when 63 data entries are present in transmit FIFO."]
    #[inline(always)]
    pub fn tft(&self) -> TftR {
        TftR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Transmit FIFO Threshold. Controls the level of entries (or below) at which the transmit FIFO controller triggers an interrupt. The FIFO depth is 64; this register is sized to the number of address bits needed to access the FIFO. If you attempt to set this value greater than or equal to the depth of the FIFO, this field is not written and retains its current value. When the number of transmit FIFO entries is less than or equal to this value, the transmit FIFO empty interrupt is triggered. For TFT decode, refer to the following description: 000000: ssi_txe_intr is asserted when 0 data entry is present in transmit FIFO. 000001: ssi_txe_intr is asserted when 1 data entry is present in transmit FIFO. 000010: ssi_txe_intr is asserted when 2 data entries are present in transmit FIFO. 000011: ssi_txe_intr is asserted when 3 data entries are present in transmit FIFO. ... 111100: ssi_txe_intr is asserted when 60 data entries are present in transmit FIFO. 111101: ssi_txe_intr is asserted when 61 data entries are present in transmit FIFO. 111110: ssi_txe_intr is asserted when 62 data entries are present in transmit FIFO. 111111: ssi_txe_intr is asserted when 63 data entries are present in transmit FIFO."]
    #[inline(always)]
    pub fn tft(&mut self) -> TftW<'_, TxftlrSpec> {
        TftW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Threshold Level\n\nYou can [`read`](crate::Reg::read) this register and get [`txftlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txftlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxftlrSpec;
impl crate::RegisterSpec for TxftlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txftlr::R`](R) reader structure"]
impl crate::Readable for TxftlrSpec {}
#[doc = "`write(|w| ..)` method takes [`txftlr::W`](W) writer structure"]
impl crate::Writable for TxftlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXFTLR to value 0"]
impl crate::Resettable for TxftlrSpec {}
