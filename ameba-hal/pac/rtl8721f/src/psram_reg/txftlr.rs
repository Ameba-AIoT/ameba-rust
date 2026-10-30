#[doc = "Register `TXFTLR` reader"]
pub type R = crate::R<TxftlrSpec>;
#[doc = "Register `TXFTLR` writer"]
pub type W = crate::W<TxftlrSpec>;
#[doc = "Field `TFT` reader - Transmit FIFO threshold. To Control FIFO entries of valid data in normal transfer except in receiving mode. When a number of FIFO entry is equal or smaller than TFT, SPIC triggers a spi_txeir interrupt. Ex: If TXFTLR = 5'h1, set spi_txeir when FIFO entries <= TFT (1)"]
pub type TftR = crate::FieldReader;
#[doc = "Field `TFT` writer - Transmit FIFO threshold. To Control FIFO entries of valid data in normal transfer except in receiving mode. When a number of FIFO entry is equal or smaller than TFT, SPIC triggers a spi_txeir interrupt. Ex: If TXFTLR = 5'h1, set spi_txeir when FIFO entries <= TFT (1)"]
pub type TftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO threshold. To Control FIFO entries of valid data in normal transfer except in receiving mode. When a number of FIFO entry is equal or smaller than TFT, SPIC triggers a spi_txeir interrupt. Ex: If TXFTLR = 5'h1, set spi_txeir when FIFO entries <= TFT (1)"]
    #[inline(always)]
    pub fn tft(&self) -> TftR {
        TftR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO threshold. To Control FIFO entries of valid data in normal transfer except in receiving mode. When a number of FIFO entry is equal or smaller than TFT, SPIC triggers a spi_txeir interrupt. Ex: If TXFTLR = 5'h1, set spi_txeir when FIFO entries <= TFT (1)"]
    #[inline(always)]
    pub fn tft(&mut self) -> TftW<'_, TxftlrSpec> {
        TftW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Threshold level\n\nYou can [`read`](crate::Reg::read) this register and get [`txftlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txftlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
