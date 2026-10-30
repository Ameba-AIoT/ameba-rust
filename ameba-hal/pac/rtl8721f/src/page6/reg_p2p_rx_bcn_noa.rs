#[doc = "Register `REG_P2P_RX_BCN_NOA` reader"]
pub type R = crate::R<RegP2pRxBcnNoaSpec>;
#[doc = "Register `REG_P2P_RX_BCN_NOA` writer"]
pub type W = crate::W<RegP2pRxBcnNoaSpec>;
#[doc = "Field `BIT_P2P_OUI_TYPE` reader - NOA OUI Type"]
pub type BitP2pOuiTypeR = crate::FieldReader;
#[doc = "Field `BIT_P2P_OUI_TYPE` writer - NOA OUI Type"]
pub type BitP2pOuiTypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_P2P_BSSID_SEL_V1` reader - When this valueis 0/1/2/3/4, then beacon with matched BSSID0/1/2/3/4 is parsed."]
pub type BitP2pBssidSelV1R = crate::FieldReader;
#[doc = "Field `BIT_P2P_BSSID_SEL_V1` writer - When this valueis 0/1/2/3/4, then beacon with matched BSSID0/1/2/3/4 is parsed."]
pub type BitP2pBssidSelV1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_P2P_NOA_PARSER_EN` reader - NoA Parser enabled."]
pub type BitP2pNoaParserEnR = crate::BitReader;
#[doc = "Field `BIT_P2P_NOA_PARSER_EN` writer - NoA Parser enabled."]
pub type BitP2pNoaParserEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - NOA OUI Type"]
    #[inline(always)]
    pub fn bit_p2p_oui_type(&self) -> BitP2pOuiTypeR {
        BitP2pOuiTypeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:14 - When this valueis 0/1/2/3/4, then beacon with matched BSSID0/1/2/3/4 is parsed."]
    #[inline(always)]
    pub fn bit_p2p_bssid_sel_v1(&self) -> BitP2pBssidSelV1R {
        BitP2pBssidSelV1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - NoA Parser enabled."]
    #[inline(always)]
    pub fn bit_p2p_noa_parser_en(&self) -> BitP2pNoaParserEnR {
        BitP2pNoaParserEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - NOA OUI Type"]
    #[inline(always)]
    pub fn bit_p2p_oui_type(&mut self) -> BitP2pOuiTypeW<'_, RegP2pRxBcnNoaSpec> {
        BitP2pOuiTypeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - When this valueis 0/1/2/3/4, then beacon with matched BSSID0/1/2/3/4 is parsed."]
    #[inline(always)]
    pub fn bit_p2p_bssid_sel_v1(&mut self) -> BitP2pBssidSelV1W<'_, RegP2pRxBcnNoaSpec> {
        BitP2pBssidSelV1W::new(self, 12)
    }
    #[doc = "Bit 15 - NoA Parser enabled."]
    #[inline(always)]
    pub fn bit_p2p_noa_parser_en(&mut self) -> BitP2pNoaParserEnW<'_, RegP2pRxBcnNoaSpec> {
        BitP2pNoaParserEnW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p2p_rx_bcn_noa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_p2p_rx_bcn_noa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP2pRxBcnNoaSpec;
impl crate::RegisterSpec for RegP2pRxBcnNoaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p2p_rx_bcn_noa::R`](R) reader structure"]
impl crate::Readable for RegP2pRxBcnNoaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_p2p_rx_bcn_noa::W`](W) writer structure"]
impl crate::Writable for RegP2pRxBcnNoaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_P2P_RX_BCN_NOA to value 0"]
impl crate::Resettable for RegP2pRxBcnNoaSpec {}
