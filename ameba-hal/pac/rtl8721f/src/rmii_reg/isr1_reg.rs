#[doc = "Register `ISR1_REG` reader"]
pub type R = crate::R<Isr1RegSpec>;
#[doc = "Register `ISR1_REG` writer"]
pub type W = crate::W<Isr1RegSpec>;
#[doc = "Field `ISR_ROK1` reader - This bit is ISR.rok"]
pub type IsrRok1R = crate::BitReader;
#[doc = "Field `ISR_ROK2` reader - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring2. Write 1 to clear."]
pub type IsrRok2R = crate::BitReader;
#[doc = "Field `ISR_ROK2` writer - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring2. Write 1 to clear."]
pub type IsrRok2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_ROK3` reader - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring3. Write 1 to clear."]
pub type IsrRok3R = crate::BitReader;
#[doc = "Field `ISR_ROK3` writer - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring3. Write 1 to clear."]
pub type IsrRok3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_ROK4` reader - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring4. Write 1 to clear."]
pub type IsrRok4R = crate::BitReader;
#[doc = "Field `ISR_ROK4` writer - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring4. Write 1 to clear."]
pub type IsrRok4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_ROK5` reader - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring5. Write 1 to clear."]
pub type IsrRok5R = crate::BitReader;
#[doc = "Field `ISR_ROK5` writer - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring5. Write 1 to clear."]
pub type IsrRok5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_ROK6` reader - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring6. Write 1 to clear."]
pub type IsrRok6R = crate::BitReader;
#[doc = "Field `ISR_ROK6` writer - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring6. Write 1 to clear."]
pub type IsrRok6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_TOK1` reader - This bit is ISR.tok"]
pub type IsrTok1R = crate::BitReader;
#[doc = "Field `ISR_TOK2` reader - Transmit Interrupt: Indicates that the DMA of the last Ring2 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
pub type IsrTok2R = crate::BitReader;
#[doc = "Field `ISR_TOK2` writer - Transmit Interrupt: Indicates that the DMA of the last Ring2 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
pub type IsrTok2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_TOK3` reader - Transmit Interrupt: Indicates that the DMA of the last Ring3 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
pub type IsrTok3R = crate::BitReader;
#[doc = "Field `ISR_TOK3` writer - Transmit Interrupt: Indicates that the DMA of the last Ring3 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
pub type IsrTok3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_TOK4` reader - Transmit Interrupt: Indicates that the DMA of the last Ring4 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
pub type IsrTok4R = crate::BitReader;
#[doc = "Field `ISR_TOK4` writer - Transmit Interrupt: Indicates that the DMA of the last Ring4 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
pub type IsrTok4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_TOK5` reader - Transmit Interrupt: Indicates that the DMA of the last Ring5 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
pub type IsrTok5R = crate::BitReader;
#[doc = "Field `ISR_TOK5` writer - Transmit Interrupt: Indicates that the DMA of the last Ring5 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
pub type IsrTok5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_TDU1` reader - This bit is ISR.tdu"]
pub type IsrTdu1R = crate::BitReader;
#[doc = "Field `ISR_TDU2` reader - Tx Descriptor Ring2 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
pub type IsrTdu2R = crate::BitReader;
#[doc = "Field `ISR_TDU2` writer - Tx Descriptor Ring2 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
pub type IsrTdu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_TDU3` reader - Tx Descriptor Ring3 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
pub type IsrTdu3R = crate::BitReader;
#[doc = "Field `ISR_TDU3` writer - Tx Descriptor Ring3 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
pub type IsrTdu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_TDU4` reader - Tx Descriptor Ring4 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
pub type IsrTdu4R = crate::BitReader;
#[doc = "Field `ISR_TDU4` writer - Tx Descriptor Ring4 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
pub type IsrTdu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_TDU5` reader - Tx Descriptor Ring5 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
pub type IsrTdu5R = crate::BitReader;
#[doc = "Field `ISR_TDU5` writer - Tx Descriptor Ring5 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
pub type IsrTdu5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is ISR.rok"]
    #[inline(always)]
    pub fn isr_rok1(&self) -> IsrRok1R {
        IsrRok1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring2. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok2(&self) -> IsrRok2R {
        IsrRok2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring3. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok3(&self) -> IsrRok3R {
        IsrRok3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring4. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok4(&self) -> IsrRok4R {
        IsrRok4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring5. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok5(&self) -> IsrRok5R {
        IsrRok5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring6. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok6(&self) -> IsrRok6R {
        IsrRok6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit is ISR.tok"]
    #[inline(always)]
    pub fn isr_tok1(&self) -> IsrTok1R {
        IsrTok1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit Interrupt: Indicates that the DMA of the last Ring2 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tok2(&self) -> IsrTok2R {
        IsrTok2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit Interrupt: Indicates that the DMA of the last Ring3 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tok3(&self) -> IsrTok3R {
        IsrTok3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmit Interrupt: Indicates that the DMA of the last Ring4 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tok4(&self) -> IsrTok4R {
        IsrTok4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmit Interrupt: Indicates that the DMA of the last Ring5 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tok5(&self) -> IsrTok5R {
        IsrTok5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is ISR.tdu"]
    #[inline(always)]
    pub fn isr_tdu1(&self) -> IsrTdu1R {
        IsrTdu1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Descriptor Ring2 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tdu2(&self) -> IsrTdu2R {
        IsrTdu2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tx Descriptor Ring3 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tdu3(&self) -> IsrTdu3R {
        IsrTdu3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Tx Descriptor Ring4 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tdu4(&self) -> IsrTdu4R {
        IsrTdu4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Tx Descriptor Ring5 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tdu5(&self) -> IsrTdu5R {
        IsrTdu5R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring2. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok2(&mut self) -> IsrRok2W<'_, Isr1RegSpec> {
        IsrRok2W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring3. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok3(&mut self) -> IsrRok3W<'_, Isr1RegSpec> {
        IsrRok3W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring4. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok4(&mut self) -> IsrRok4W<'_, Isr1RegSpec> {
        IsrRok4W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring5. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok5(&mut self) -> IsrRok5W<'_, Isr1RegSpec> {
        IsrRok5W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires in Ring6. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok6(&mut self) -> IsrRok6W<'_, Isr1RegSpec> {
        IsrRok6W::new(self, 5)
    }
    #[doc = "Bit 17 - Transmit Interrupt: Indicates that the DMA of the last Ring2 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tok2(&mut self) -> IsrTok2W<'_, Isr1RegSpec> {
        IsrTok2W::new(self, 17)
    }
    #[doc = "Bit 18 - Transmit Interrupt: Indicates that the DMA of the last Ring3 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tok3(&mut self) -> IsrTok3W<'_, Isr1RegSpec> {
        IsrTok3W::new(self, 18)
    }
    #[doc = "Bit 19 - Transmit Interrupt: Indicates that the DMA of the last Ring4 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tok4(&mut self) -> IsrTok4W<'_, Isr1RegSpec> {
        IsrTok4W::new(self, 19)
    }
    #[doc = "Bit 20 - Transmit Interrupt: Indicates that the DMA of the last Ring5 descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tok5(&mut self) -> IsrTok5W<'_, Isr1RegSpec> {
        IsrTok5W::new(self, 20)
    }
    #[doc = "Bit 25 - Tx Descriptor Ring2 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tdu2(&mut self) -> IsrTdu2W<'_, Isr1RegSpec> {
        IsrTdu2W::new(self, 25)
    }
    #[doc = "Bit 26 - Tx Descriptor Ring3 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tdu3(&mut self) -> IsrTdu3W<'_, Isr1RegSpec> {
        IsrTdu3W::new(self, 26)
    }
    #[doc = "Bit 27 - Tx Descriptor Ring4 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tdu4(&mut self) -> IsrTdu4W<'_, Isr1RegSpec> {
        IsrTdu4W::new(self, 27)
    }
    #[doc = "Bit 28 - Tx Descriptor Ring5 Unavailable: When set, indicates Tx descriptor Ring5 is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tdu5(&mut self) -> IsrTdu5W<'_, Isr1RegSpec> {
        IsrTdu5W::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`isr1_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr1_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isr1RegSpec;
impl crate::RegisterSpec for Isr1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr1_reg::R`](R) reader structure"]
impl crate::Readable for Isr1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`isr1_reg::W`](W) writer structure"]
impl crate::Writable for Isr1RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR1_REG to value 0"]
impl crate::Resettable for Isr1RegSpec {}
