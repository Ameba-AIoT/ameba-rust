#[doc = "Register `RXFTLR` reader"]
pub type R = crate::R<RxftlrSpec>;
#[doc = "Register `RXFTLR` writer"]
pub type W = crate::W<RxftlrSpec>;
#[doc = "Field `RFT` reader - Receive FIFO threshold. To Control FIFO entries of valid data in receiving mode. When a number of FIFO entry is greater than RFT, SPIC triggers a spi_rxfir interrupt. Ex: If RXFTLR = 5'h1, set spi_rxfir when FIFO entries > RFT (1)"]
pub type RftR = crate::FieldReader;
#[doc = "Field `RFT` writer - Receive FIFO threshold. To Control FIFO entries of valid data in receiving mode. When a number of FIFO entry is greater than RFT, SPIC triggers a spi_rxfir interrupt. Ex: If RXFTLR = 5'h1, set spi_rxfir when FIFO entries > RFT (1)"]
pub type RftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Receive FIFO threshold. To Control FIFO entries of valid data in receiving mode. When a number of FIFO entry is greater than RFT, SPIC triggers a spi_rxfir interrupt. Ex: If RXFTLR = 5'h1, set spi_rxfir when FIFO entries > RFT (1)"]
    #[inline(always)]
    pub fn rft(&self) -> RftR {
        RftR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Receive FIFO threshold. To Control FIFO entries of valid data in receiving mode. When a number of FIFO entry is greater than RFT, SPIC triggers a spi_rxfir interrupt. Ex: If RXFTLR = 5'h1, set spi_rxfir when FIFO entries > RFT (1)"]
    #[inline(always)]
    pub fn rft(&mut self) -> RftW<'_, RxftlrSpec> {
        RftW::new(self, 0)
    }
}
#[doc = "Receive FIFO Threshold level\n\nYou can [`read`](crate::Reg::read) this register and get [`rxftlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxftlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets RXFTLR to value 0x1f"]
impl crate::Resettable for RxftlrSpec {
    const RESET_VALUE: u32 = 0x1f;
}
