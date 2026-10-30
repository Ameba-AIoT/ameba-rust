#[doc = "Register `ISR_and_IMR` reader"]
pub type R = crate::R<IsrAndImrSpec>;
#[doc = "Register `ISR_and_IMR` writer"]
pub type W = crate::W<IsrAndImrSpec>;
#[doc = "Field `ISR_ROK` reader - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires. Write 1 to clear."]
pub type IsrRokR = crate::BitReader;
#[doc = "Field `ISR_CNT_WRAP` reader - Tx/rx counter wrap interrupt."]
pub type IsrCntWrapR = crate::BitReader;
#[doc = "Field `ISR_RER_RUNT` reader - Rx error caused by runt error characterized by the frame length in bytes being less than 64 bytes. Write 1 to clear."]
pub type IsrRerRuntR = crate::BitReader;
#[doc = "Field `ISR_RER_OVF` reader - Rx FIFO Overflow, caused by RBO/RDU, poor system bus (Lexra bus) performance, or overloaded Lexra bus traffic. Write 1 to clear."]
pub type IsrRerOvfR = crate::BitReader;
#[doc = "Field `ISR_RDU1` reader - Rx Descriptor Unavailable for RING1: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
pub type IsrRdu1R = crate::BitReader;
#[doc = "Field `ISR_TOK_TI` reader - Transmit Interrupt: Indicates that the DMA of the last descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
pub type IsrTokTiR = crate::BitReader;
#[doc = "Field `ISR_TER` reader - Transmit (Tx) Error: Indicates that a packet transmission was aborted, due to excessive collisions, according to the TXRR's setting. Write 1 to clear."]
pub type IsrTerR = crate::BitReader;
#[doc = "Field `ISR_LinkChg` reader - Link Change: Write 1 to clear."]
pub type IsrLinkChgR = crate::BitReader;
#[doc = "Field `ISR_TDU` reader - Tx Descriptor Unavailable: When set, indicates Tx descriptor is unavailable. Write 1 to clear."]
pub type IsrTduR = crate::BitReader;
#[doc = "Field `ISR_SWInt` reader - Software Interrupt pending: When set to 1 indicates a software interrupt was forced. Write 1 to clear."]
pub type IsrSwintR = crate::BitReader;
#[doc = "Field `ISR_RDU2` reader - Rx Descriptor Unavailable for RING2: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
pub type IsrRdu2R = crate::BitReader;
#[doc = "Field `ISR_RDU3` reader - Rx Descriptor Unavailable for RING3: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
pub type IsrRdu3R = crate::BitReader;
#[doc = "Field `ISR_RDU4` reader - Rx Descriptor Unavailable for RING4: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
pub type IsrRdu4R = crate::BitReader;
#[doc = "Field `ISR_RDU5` reader - Rx Descriptor Unavailable for RING5: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
pub type IsrRdu5R = crate::BitReader;
#[doc = "Field `ISR_RDU6` reader - Rx Descriptor Unavailable for RING6: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
pub type IsrRdu6R = crate::BitReader;
#[doc = "Field `IMR_ROK` reader - "]
pub type ImrRokR = crate::BitReader;
#[doc = "Field `IMR_ROK` writer - "]
pub type ImrRokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_CNT_WRAP` reader - "]
pub type ImrCntWrapR = crate::BitReader;
#[doc = "Field `IMR_CNT_WRAP` writer - "]
pub type ImrCntWrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_RER_RUNT` reader - "]
pub type ImrRerRuntR = crate::BitReader;
#[doc = "Field `IMR_RER_RUNT` writer - "]
pub type ImrRerRuntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_RER_OVF` reader - "]
pub type ImrRerOvfR = crate::BitReader;
#[doc = "Field `IMR_RER_OVF` writer - "]
pub type ImrRerOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_RDU1` reader - "]
pub type ImrRdu1R = crate::BitReader;
#[doc = "Field `IMR_RDU1` writer - "]
pub type ImrRdu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_TOK_TI` reader - "]
pub type ImrTokTiR = crate::BitReader;
#[doc = "Field `IMR_TOK_TI` writer - "]
pub type ImrTokTiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_TER` reader - "]
pub type ImrTerR = crate::BitReader;
#[doc = "Field `IMR_TER` writer - "]
pub type ImrTerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_LinkChg` reader - "]
pub type ImrLinkChgR = crate::BitReader;
#[doc = "Field `IMR_LinkChg` writer - "]
pub type ImrLinkChgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_TDU` reader - "]
pub type ImrTduR = crate::BitReader;
#[doc = "Field `IMR_TDU` writer - "]
pub type ImrTduW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_SWInt` reader - "]
pub type ImrSwintR = crate::BitReader;
#[doc = "Field `IMR_SWInt` writer - "]
pub type ImrSwintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_RDU2` reader - "]
pub type ImrRdu2R = crate::BitReader;
#[doc = "Field `IMR_RDU2` writer - "]
pub type ImrRdu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_RDU3` reader - "]
pub type ImrRdu3R = crate::BitReader;
#[doc = "Field `IMR_RDU3` writer - "]
pub type ImrRdu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_RDU4` reader - "]
pub type ImrRdu4R = crate::BitReader;
#[doc = "Field `IMR_RDU4` writer - "]
pub type ImrRdu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_RDU5` reader - "]
pub type ImrRdu5R = crate::BitReader;
#[doc = "Field `IMR_RDU5` writer - "]
pub type ImrRdu5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMR_RDU6` reader - "]
pub type ImrRdu6R = crate::BitReader;
#[doc = "Field `IMR_RDU6` writer - "]
pub type ImrRdu6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive (Rx) OK: This interrupt is set either when RxIntMitigation packet is met or RxPktTimer expires. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rok(&self) -> IsrRokR {
        IsrRokR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx/rx counter wrap interrupt."]
    #[inline(always)]
    pub fn isr_cnt_wrap(&self) -> IsrCntWrapR {
        IsrCntWrapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx error caused by runt error characterized by the frame length in bytes being less than 64 bytes. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rer_runt(&self) -> IsrRerRuntR {
        IsrRerRuntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO Overflow, caused by RBO/RDU, poor system bus (Lexra bus) performance, or overloaded Lexra bus traffic. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rer_ovf(&self) -> IsrRerOvfR {
        IsrRerOvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx Descriptor Unavailable for RING1: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rdu1(&self) -> IsrRdu1R {
        IsrRdu1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Interrupt: Indicates that the DMA of the last descriptor of TxIntMitigation number of Tx packet has completed and the last descriptor has been closed. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tok_ti(&self) -> IsrTokTiR {
        IsrTokTiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit (Tx) Error: Indicates that a packet transmission was aborted, due to excessive collisions, according to the TXRR's setting. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_ter(&self) -> IsrTerR {
        IsrTerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Link Change: Write 1 to clear."]
    #[inline(always)]
    pub fn isr_link_chg(&self) -> IsrLinkChgR {
        IsrLinkChgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx Descriptor Unavailable: When set, indicates Tx descriptor is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_tdu(&self) -> IsrTduR {
        IsrTduR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Interrupt pending: When set to 1 indicates a software interrupt was forced. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_swint(&self) -> IsrSwintR {
        IsrSwintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx Descriptor Unavailable for RING2: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rdu2(&self) -> IsrRdu2R {
        IsrRdu2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rx Descriptor Unavailable for RING3: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rdu3(&self) -> IsrRdu3R {
        IsrRdu3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rx Descriptor Unavailable for RING4: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rdu4(&self) -> IsrRdu4R {
        IsrRdu4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rx Descriptor Unavailable for RING5: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rdu5(&self) -> IsrRdu5R {
        IsrRdu5R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx Descriptor Unavailable for RING6: When set, indicates Rx descriptor is unavailable. Write 1 to clear."]
    #[inline(always)]
    pub fn isr_rdu6(&self) -> IsrRdu6R {
        IsrRdu6R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn imr_rok(&self) -> ImrRokR {
        ImrRokR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn imr_cnt_wrap(&self) -> ImrCntWrapR {
        ImrCntWrapR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn imr_rer_runt(&self) -> ImrRerRuntR {
        ImrRerRuntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn imr_rer_ovf(&self) -> ImrRerOvfR {
        ImrRerOvfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn imr_rdu1(&self) -> ImrRdu1R {
        ImrRdu1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn imr_tok_ti(&self) -> ImrTokTiR {
        ImrTokTiR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn imr_ter(&self) -> ImrTerR {
        ImrTerR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn imr_link_chg(&self) -> ImrLinkChgR {
        ImrLinkChgR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn imr_tdu(&self) -> ImrTduR {
        ImrTduR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn imr_swint(&self) -> ImrSwintR {
        ImrSwintR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn imr_rdu2(&self) -> ImrRdu2R {
        ImrRdu2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn imr_rdu3(&self) -> ImrRdu3R {
        ImrRdu3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn imr_rdu4(&self) -> ImrRdu4R {
        ImrRdu4R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn imr_rdu5(&self) -> ImrRdu5R {
        ImrRdu5R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn imr_rdu6(&self) -> ImrRdu6R {
        ImrRdu6R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn imr_rok(&mut self) -> ImrRokW<'_, IsrAndImrSpec> {
        ImrRokW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn imr_cnt_wrap(&mut self) -> ImrCntWrapW<'_, IsrAndImrSpec> {
        ImrCntWrapW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn imr_rer_runt(&mut self) -> ImrRerRuntW<'_, IsrAndImrSpec> {
        ImrRerRuntW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn imr_rer_ovf(&mut self) -> ImrRerOvfW<'_, IsrAndImrSpec> {
        ImrRerOvfW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn imr_rdu1(&mut self) -> ImrRdu1W<'_, IsrAndImrSpec> {
        ImrRdu1W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn imr_tok_ti(&mut self) -> ImrTokTiW<'_, IsrAndImrSpec> {
        ImrTokTiW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn imr_ter(&mut self) -> ImrTerW<'_, IsrAndImrSpec> {
        ImrTerW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn imr_link_chg(&mut self) -> ImrLinkChgW<'_, IsrAndImrSpec> {
        ImrLinkChgW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn imr_tdu(&mut self) -> ImrTduW<'_, IsrAndImrSpec> {
        ImrTduW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn imr_swint(&mut self) -> ImrSwintW<'_, IsrAndImrSpec> {
        ImrSwintW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn imr_rdu2(&mut self) -> ImrRdu2W<'_, IsrAndImrSpec> {
        ImrRdu2W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn imr_rdu3(&mut self) -> ImrRdu3W<'_, IsrAndImrSpec> {
        ImrRdu3W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn imr_rdu4(&mut self) -> ImrRdu4W<'_, IsrAndImrSpec> {
        ImrRdu4W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn imr_rdu5(&mut self) -> ImrRdu5W<'_, IsrAndImrSpec> {
        ImrRdu5W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn imr_rdu6(&mut self) -> ImrRdu6W<'_, IsrAndImrSpec> {
        ImrRdu6W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`isr_and_imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr_and_imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrAndImrSpec;
impl crate::RegisterSpec for IsrAndImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr_and_imr::R`](R) reader structure"]
impl crate::Readable for IsrAndImrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr_and_imr::W`](W) writer structure"]
impl crate::Writable for IsrAndImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR_and_IMR to value 0"]
impl crate::Resettable for IsrAndImrSpec {}
