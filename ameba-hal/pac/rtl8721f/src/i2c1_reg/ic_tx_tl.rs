#[doc = "Register `IC_TX_TL` reader"]
pub type R = crate::R<IcTxTlSpec>;
#[doc = "Register `IC_TX_TL` writer"]
pub type W = crate::W<IcTxTlSpec>;
#[doc = "Field `IC_TX_0_TL` reader - Transmit FIFO Threshold Level. Control the level of entries (or below) that triggers the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register)."]
pub type IcTx0TlR = crate::FieldReader;
#[doc = "Field `IC_TX_0_TL` writer - Transmit FIFO Threshold Level. Control the level of entries (or below) that triggers the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register)."]
pub type IcTx0TlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Threshold Level. Control the level of entries (or below) that triggers the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register)."]
    #[inline(always)]
    pub fn ic_tx_0_tl(&self) -> IcTx0TlR {
        IcTx0TlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit FIFO Threshold Level. Control the level of entries (or below) that triggers the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register)."]
    #[inline(always)]
    pub fn ic_tx_0_tl(&mut self) -> IcTx0TlW<'_, IcTxTlSpec> {
        IcTx0TlW::new(self, 0)
    }
}
#[doc = "I2C Transmit FIFO Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_tx_tl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_tx_tl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcTxTlSpec;
impl crate::RegisterSpec for IcTxTlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_tx_tl::R`](R) reader structure"]
impl crate::Readable for IcTxTlSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_tx_tl::W`](W) writer structure"]
impl crate::Writable for IcTxTlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_TX_TL to value 0"]
impl crate::Resettable for IcTxTlSpec {}
