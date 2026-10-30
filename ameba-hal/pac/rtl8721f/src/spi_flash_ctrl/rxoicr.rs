#[doc = "Register `RXOICR` reader"]
pub type R = crate::R<RxoicrSpec>;
#[doc = "Register `RXOICR` writer"]
pub type W = crate::W<RxoicrSpec>;
#[doc = "Field `RXOICR` reader - When reading/writing the register, spi_rxoir_r would be cleared. Note: Only response value 0 when read."]
pub type RxoicrR = crate::BitReader;
#[doc = "Field `RXOICR` writer - When reading/writing the register, spi_rxoir_r would be cleared. Note: Only response value 0 when read."]
pub type RxoicrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When reading/writing the register, spi_rxoir_r would be cleared. Note: Only response value 0 when read."]
    #[inline(always)]
    pub fn rxoicr(&self) -> RxoicrR {
        RxoicrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When reading/writing the register, spi_rxoir_r would be cleared. Note: Only response value 0 when read."]
    #[inline(always)]
    pub fn rxoicr(&mut self) -> RxoicrW<'_, RxoicrSpec> {
        RxoicrW::new(self, 0)
    }
}
#[doc = "Receive FIFO Overflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxoicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxoicrSpec;
impl crate::RegisterSpec for RxoicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoicr::R`](R) reader structure"]
impl crate::Readable for RxoicrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxoicr::W`](W) writer structure"]
impl crate::Writable for RxoicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXOICR to value 0"]
impl crate::Resettable for RxoicrSpec {}
