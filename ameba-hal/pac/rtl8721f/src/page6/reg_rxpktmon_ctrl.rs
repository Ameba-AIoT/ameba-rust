#[doc = "Register `REG_RXPKTMON_CTRL` reader"]
pub type R = crate::R<RegRxpktmonCtrlSpec>;
#[doc = "Register `REG_RXPKTMON_CTRL` writer"]
pub type W = crate::W<RegRxpktmonCtrlSpec>;
#[doc = "Field `BIT_RXPKT_MON_EN` reader - Enable packet loss monitor function for Rx packets. If this bit is set to 1, then MACRX uses fragment field of sequence number field of rxmpdu as the mpdu index and check if the index is incremental."]
pub type BitRxpktMonEnR = crate::BitReader;
#[doc = "Field `BIT_RXPKT_MON_EN` writer - Enable packet loss monitor function for Rx packets. If this bit is set to 1, then MACRX uses fragment field of sequence number field of rxmpdu as the mpdu index and check if the index is incremental."]
pub type BitRxpktMonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDMA_MON_EN` reader - Enable packet loss monitor function for Rx DMA. If this bit is set to 1, then MACRX append the incremental index to fragment field of sequence number field of rxmpdu restored to rxpkbuffer successfully"]
pub type BitRxdmaMonEnR = crate::BitReader;
#[doc = "Field `BIT_RXDMA_MON_EN` writer - Enable packet loss monitor function for Rx DMA. If this bit is set to 1, then MACRX append the incremental index to fragment field of sequence number field of rxmpdu restored to rxpkbuffer successfully"]
pub type BitRxdmaMonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXVOQPKT_ERR` reader - Indicate monitor find the Rx VOQ packet without incremental index"]
pub type BitRxvoqpktErrR = crate::BitReader;
#[doc = "Field `BIT_RXVIQPKT_ERR` reader - Indicate monitor find the Rx VIQ packet without incremental index"]
pub type BitRxviqpktErrR = crate::BitReader;
#[doc = "Field `BIT_RXBEQPKT_ERR` reader - Indicate monitor find the Rx BEQ packet without incremental index"]
pub type BitRxbeqpktErrR = crate::BitReader;
#[doc = "Field `BIT_RXBKQPKT_ERR` reader - Indicate monitor find the Rx BKQ packet without incremental index"]
pub type BitRxbkqpktErrR = crate::BitReader;
#[doc = "Field `BIT_RXVOQPKT_SEQ` reader - The expected Rx VOQ packet index"]
pub type BitRxvoqpktSeqR = crate::FieldReader;
#[doc = "Field `BIT_RXVIQPKT_SEQ` reader - The expected Rx VIQ packet index"]
pub type BitRxviqpktSeqR = crate::FieldReader;
#[doc = "Field `BIT_RXBEQPKT_SEQ` reader - The expected Rx BEQ packet index"]
pub type BitRxbeqpktSeqR = crate::FieldReader;
#[doc = "Field `BIT_RXBKQPKT_SEQ` reader - The expected Rx BKQ packet index"]
pub type BitRxbkqpktSeqR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enable packet loss monitor function for Rx packets. If this bit is set to 1, then MACRX uses fragment field of sequence number field of rxmpdu as the mpdu index and check if the index is incremental."]
    #[inline(always)]
    pub fn bit_rxpkt_mon_en(&self) -> BitRxpktMonEnR {
        BitRxpktMonEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable packet loss monitor function for Rx DMA. If this bit is set to 1, then MACRX append the incremental index to fragment field of sequence number field of rxmpdu restored to rxpkbuffer successfully"]
    #[inline(always)]
    pub fn bit_rxdma_mon_en(&self) -> BitRxdmaMonEnR {
        BitRxdmaMonEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicate monitor find the Rx VOQ packet without incremental index"]
    #[inline(always)]
    pub fn bit_rxvoqpkt_err(&self) -> BitRxvoqpktErrR {
        BitRxvoqpktErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicate monitor find the Rx VIQ packet without incremental index"]
    #[inline(always)]
    pub fn bit_rxviqpkt_err(&self) -> BitRxviqpktErrR {
        BitRxviqpktErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicate monitor find the Rx BEQ packet without incremental index"]
    #[inline(always)]
    pub fn bit_rxbeqpkt_err(&self) -> BitRxbeqpktErrR {
        BitRxbeqpktErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicate monitor find the Rx BKQ packet without incremental index"]
    #[inline(always)]
    pub fn bit_rxbkqpkt_err(&self) -> BitRxbkqpktErrR {
        BitRxbkqpktErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - The expected Rx VOQ packet index"]
    #[inline(always)]
    pub fn bit_rxvoqpkt_seq(&self) -> BitRxvoqpktSeqR {
        BitRxvoqpktSeqR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - The expected Rx VIQ packet index"]
    #[inline(always)]
    pub fn bit_rxviqpkt_seq(&self) -> BitRxviqpktSeqR {
        BitRxviqpktSeqR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - The expected Rx BEQ packet index"]
    #[inline(always)]
    pub fn bit_rxbeqpkt_seq(&self) -> BitRxbeqpktSeqR {
        BitRxbeqpktSeqR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - The expected Rx BKQ packet index"]
    #[inline(always)]
    pub fn bit_rxbkqpkt_seq(&self) -> BitRxbkqpktSeqR {
        BitRxbkqpktSeqR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable packet loss monitor function for Rx packets. If this bit is set to 1, then MACRX uses fragment field of sequence number field of rxmpdu as the mpdu index and check if the index is incremental."]
    #[inline(always)]
    pub fn bit_rxpkt_mon_en(&mut self) -> BitRxpktMonEnW<'_, RegRxpktmonCtrlSpec> {
        BitRxpktMonEnW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable packet loss monitor function for Rx DMA. If this bit is set to 1, then MACRX append the incremental index to fragment field of sequence number field of rxmpdu restored to rxpkbuffer successfully"]
    #[inline(always)]
    pub fn bit_rxdma_mon_en(&mut self) -> BitRxdmaMonEnW<'_, RegRxpktmonCtrlSpec> {
        BitRxdmaMonEnW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxpktmon_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxpktmon_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxpktmonCtrlSpec;
impl crate::RegisterSpec for RegRxpktmonCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxpktmon_ctrl::R`](R) reader structure"]
impl crate::Readable for RegRxpktmonCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxpktmon_ctrl::W`](W) writer structure"]
impl crate::Writable for RegRxpktmonCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXPKTMON_CTRL to value 0"]
impl crate::Resettable for RegRxpktmonCtrlSpec {}
