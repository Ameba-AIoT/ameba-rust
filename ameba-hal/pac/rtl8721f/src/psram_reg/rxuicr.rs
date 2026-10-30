#[doc = "Register `RXUICR` reader"]
pub type R = crate::R<RxuicrSpec>;
#[doc = "Register `RXUICR` writer"]
pub type W = crate::W<RxuicrSpec>;
#[doc = "Field `RXUICR` reader - When reading/writing the register, spi_rxuir_r would be cleared. Note: Only response value 0 when read."]
pub type RxuicrR = crate::BitReader;
#[doc = "Field `RXUICR` writer - When reading/writing the register, spi_rxuir_r would be cleared. Note: Only response value 0 when read."]
pub type RxuicrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When reading/writing the register, spi_rxuir_r would be cleared. Note: Only response value 0 when read."]
    #[inline(always)]
    pub fn rxuicr(&self) -> RxuicrR {
        RxuicrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When reading/writing the register, spi_rxuir_r would be cleared. Note: Only response value 0 when read."]
    #[inline(always)]
    pub fn rxuicr(&mut self) -> RxuicrW<'_, RxuicrSpec> {
        RxuicrW::new(self, 0)
    }
}
#[doc = "Receive FIFO Underflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxuicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxuicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxuicrSpec;
impl crate::RegisterSpec for RxuicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxuicr::R`](R) reader structure"]
impl crate::Readable for RxuicrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxuicr::W`](W) writer structure"]
impl crate::Writable for RxuicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXUICR to value 0"]
impl crate::Resettable for RxuicrSpec {}
