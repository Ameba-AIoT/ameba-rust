#[doc = "Register `REG_RXAI_CTRL` reader"]
pub type R = crate::R<RegRxaiCtrlSpec>;
#[doc = "Register `REG_RXAI_CTRL` writer"]
pub type W = crate::W<RegRxaiCtrlSpec>;
#[doc = "Field `BIT_RXAI_ADDR_CHKEN` reader - This bit is set to 1 to enable MAC to inform BB not doing reCCA during receiving the address matched PPDU."]
pub type BitRxaiAddrChkenR = crate::BitReader;
#[doc = "Field `BIT_RXAI_ADDR_CHKEN` writer - This bit is set to 1 to enable MAC to inform BB not doing reCCA during receiving the address matched PPDU."]
pub type BitRxaiAddrChkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXAI_AGG_CHKEN` reader - This bit is only workable when BIT_RXAI_ADDR_CHKEN = 1. 1: if and only if the receiving address matched PPDU is A-MPDU and is not VHT single MPDU, then MAC inform BB not doing reCCA."]
pub type BitRxaiAggChkenR = crate::BitReader;
#[doc = "Field `BIT_RXAI_AGG_CHKEN` writer - This bit is only workable when BIT_RXAI_ADDR_CHKEN = 1. 1: if and only if the receiving address matched PPDU is A-MPDU and is not VHT single MPDU, then MAC inform BB not doing reCCA."]
pub type BitRxaiAggChkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXAI_PRTCT_SEL` reader - MAC_reset_protect rises at 0: DATA_ON rises 1: PLCP check OK (OFDM/CCK) or receiving 3rd/2nd (VHT/HT) RDRDY"]
pub type BitRxaiPrtctSelR = crate::BitReader;
#[doc = "Field `BIT_RXAI_PRTCT_SEL` writer - MAC_reset_protect rises at 0: DATA_ON rises 1: PLCP check OK (OFDM/CCK) or receiving 3rd/2nd (VHT/HT) RDRDY"]
pub type BitRxaiPrtctSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXAI_PRTCT_VIO` reader - If MAC gets BB_reset_MAC pulse when MAC_reset_protect = 1, set this bit to 1. Reset by BIT_RXAI_INFO_RST = 1"]
pub type BitRxaiPrtctVioR = crate::BitReader;
#[doc = "Field `BIT_RXAI_PRTCT_REL` reader - If MAC_reset_protect is set to 0 by MAC decision circuit (address/A-MPDU checking), set this bit to 1. Reset by BIT_RXAI_INFO_RST = 1"]
pub type BitRxaiPrtctRelR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit is set to 1 to enable MAC to inform BB not doing reCCA during receiving the address matched PPDU."]
    #[inline(always)]
    pub fn bit_rxai_addr_chken(&self) -> BitRxaiAddrChkenR {
        BitRxaiAddrChkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is only workable when BIT_RXAI_ADDR_CHKEN = 1. 1: if and only if the receiving address matched PPDU is A-MPDU and is not VHT single MPDU, then MAC inform BB not doing reCCA."]
    #[inline(always)]
    pub fn bit_rxai_agg_chken(&self) -> BitRxaiAggChkenR {
        BitRxaiAggChkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MAC_reset_protect rises at 0: DATA_ON rises 1: PLCP check OK (OFDM/CCK) or receiving 3rd/2nd (VHT/HT) RDRDY"]
    #[inline(always)]
    pub fn bit_rxai_prtct_sel(&self) -> BitRxaiPrtctSelR {
        BitRxaiPrtctSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - If MAC gets BB_reset_MAC pulse when MAC_reset_protect = 1, set this bit to 1. Reset by BIT_RXAI_INFO_RST = 1"]
    #[inline(always)]
    pub fn bit_rxai_prtct_vio(&self) -> BitRxaiPrtctVioR {
        BitRxaiPrtctVioR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If MAC_reset_protect is set to 0 by MAC decision circuit (address/A-MPDU checking), set this bit to 1. Reset by BIT_RXAI_INFO_RST = 1"]
    #[inline(always)]
    pub fn bit_rxai_prtct_rel(&self) -> BitRxaiPrtctRelR {
        BitRxaiPrtctRelR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set to 1 to enable MAC to inform BB not doing reCCA during receiving the address matched PPDU."]
    #[inline(always)]
    pub fn bit_rxai_addr_chken(&mut self) -> BitRxaiAddrChkenW<'_, RegRxaiCtrlSpec> {
        BitRxaiAddrChkenW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is only workable when BIT_RXAI_ADDR_CHKEN = 1. 1: if and only if the receiving address matched PPDU is A-MPDU and is not VHT single MPDU, then MAC inform BB not doing reCCA."]
    #[inline(always)]
    pub fn bit_rxai_agg_chken(&mut self) -> BitRxaiAggChkenW<'_, RegRxaiCtrlSpec> {
        BitRxaiAggChkenW::new(self, 1)
    }
    #[doc = "Bit 2 - MAC_reset_protect rises at 0: DATA_ON rises 1: PLCP check OK (OFDM/CCK) or receiving 3rd/2nd (VHT/HT) RDRDY"]
    #[inline(always)]
    pub fn bit_rxai_prtct_sel(&mut self) -> BitRxaiPrtctSelW<'_, RegRxaiCtrlSpec> {
        BitRxaiPrtctSelW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxai_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxai_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxaiCtrlSpec;
impl crate::RegisterSpec for RegRxaiCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxai_ctrl::R`](R) reader structure"]
impl crate::Readable for RegRxaiCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxai_ctrl::W`](W) writer structure"]
impl crate::Writable for RegRxaiCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXAI_CTRL to value 0"]
impl crate::Resettable for RegRxaiCtrlSpec {}
