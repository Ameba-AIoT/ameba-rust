#[doc = "Register `IC_RX_TL` reader"]
pub type R = crate::R<IcRxTlSpec>;
#[doc = "Register `IC_RX_TL` writer"]
pub type W = crate::W<IcRxTlSpec>;
#[doc = "Field `IC_RX_0_TL` reader - Receive FIFO Threshold Level. Control the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register)."]
pub type IcRx0TlR = crate::FieldReader;
#[doc = "Field `IC_RX_0_TL` writer - Receive FIFO Threshold Level. Control the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register)."]
pub type IcRx0TlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO Threshold Level. Control the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register)."]
    #[inline(always)]
    pub fn ic_rx_0_tl(&self) -> IcRx0TlR {
        IcRx0TlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive FIFO Threshold Level. Control the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register)."]
    #[inline(always)]
    pub fn ic_rx_0_tl(&mut self) -> IcRx0TlW<'_, IcRxTlSpec> {
        IcRx0TlW::new(self, 0)
    }
}
#[doc = "I2C Receive FIFO Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_rx_tl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_rx_tl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcRxTlSpec;
impl crate::RegisterSpec for IcRxTlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_rx_tl::R`](R) reader structure"]
impl crate::Readable for IcRxTlSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_rx_tl::W`](W) writer structure"]
impl crate::Writable for IcRxTlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_RX_TL to value 0"]
impl crate::Resettable for IcRxTlSpec {}
