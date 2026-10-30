#[doc = "Register `TXOICR` reader"]
pub type R = crate::R<TxoicrSpec>;
#[doc = "Register `TXOICR` writer"]
pub type W = crate::W<TxoicrSpec>;
#[doc = "Field `TXOICR` reader - When reading/writing the register, spi_rxoir_r would be cleared. Note: Only response value 0 when read."]
pub type TxoicrR = crate::BitReader;
#[doc = "Field `TXOICR` writer - When reading/writing the register, spi_rxoir_r would be cleared. Note: Only response value 0 when read."]
pub type TxoicrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When reading/writing the register, spi_rxoir_r would be cleared. Note: Only response value 0 when read."]
    #[inline(always)]
    pub fn txoicr(&self) -> TxoicrR {
        TxoicrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When reading/writing the register, spi_rxoir_r would be cleared. Note: Only response value 0 when read."]
    #[inline(always)]
    pub fn txoicr(&mut self) -> TxoicrW<'_, TxoicrSpec> {
        TxoicrW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Overflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txoicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txoicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxoicrSpec;
impl crate::RegisterSpec for TxoicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txoicr::R`](R) reader structure"]
impl crate::Readable for TxoicrSpec {}
#[doc = "`write(|w| ..)` method takes [`txoicr::W`](W) writer structure"]
impl crate::Writable for TxoicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXOICR to value 0"]
impl crate::Resettable for TxoicrSpec {}
