#[doc = "Register `MAC_PHY_CR` reader"]
pub type R = crate::R<MacPhyCrSpec>;
#[doc = "Register `MAC_PHY_CR` writer"]
pub type W = crate::W<MacPhyCrSpec>;
#[doc = "Field `r_rxpkt_ok_fail_cnt` reader - rxpkt_ok or rxpkt_fail pulse counter,unit 25ns when mac clock is 40MHz"]
pub type RRxpktOkFailCntR = crate::FieldReader;
#[doc = "Field `r_rxpkt_ok_fail_cnt` writer - rxpkt_ok or rxpkt_fail pulse counter,unit 25ns when mac clock is 40MHz"]
pub type RRxpktOkFailCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_invalid_pkt_cnt` reader - invalid_pkt pulse counter,unit 25ns when mac clock is 40MHz"]
pub type RInvalidPktCntR = crate::FieldReader;
#[doc = "Field `r_invalid_pkt_cnt` writer - invalid_pkt pulse counter,unit 25ns when mac clock is 40MHz"]
pub type RInvalidPktCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_id_match_cnt` reader - rxpkt_ok or rxpkt_fail pulse counter,unit 25ns when mac clock is 40MHz"]
pub type RIdMatchCntR = crate::FieldReader;
#[doc = "Field `r_id_match_cnt` writer - rxpkt_ok or rxpkt_fail pulse counter,unit 25ns when mac clock is 40MHz"]
pub type RIdMatchCntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_id_math_mpdu` reader - when set, per mpdu will trigger rx_id_match, else per ppdu trigger rx_id_match"]
pub type RIdMathMpduR = crate::BitReader;
#[doc = "Field `r_id_math_mpdu` writer - when set, per mpdu will trigger rx_id_match, else per ppdu trigger rx_id_match"]
pub type RIdMathMpduW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:5 - rxpkt_ok or rxpkt_fail pulse counter,unit 25ns when mac clock is 40MHz"]
    #[inline(always)]
    pub fn r_rxpkt_ok_fail_cnt(&self) -> RRxpktOkFailCntR {
        RRxpktOkFailCntR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13 - invalid_pkt pulse counter,unit 25ns when mac clock is 40MHz"]
    #[inline(always)]
    pub fn r_invalid_pkt_cnt(&self) -> RInvalidPktCntR {
        RInvalidPktCntR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:20 - rxpkt_ok or rxpkt_fail pulse counter,unit 25ns when mac clock is 40MHz"]
    #[inline(always)]
    pub fn r_id_match_cnt(&self) -> RIdMatchCntR {
        RIdMatchCntR::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bit 21 - when set, per mpdu will trigger rx_id_match, else per ppdu trigger rx_id_match"]
    #[inline(always)]
    pub fn r_id_math_mpdu(&self) -> RIdMathMpduR {
        RIdMathMpduR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - rxpkt_ok or rxpkt_fail pulse counter,unit 25ns when mac clock is 40MHz"]
    #[inline(always)]
    pub fn r_rxpkt_ok_fail_cnt(&mut self) -> RRxpktOkFailCntW<'_, MacPhyCrSpec> {
        RRxpktOkFailCntW::new(self, 0)
    }
    #[doc = "Bits 6:13 - invalid_pkt pulse counter,unit 25ns when mac clock is 40MHz"]
    #[inline(always)]
    pub fn r_invalid_pkt_cnt(&mut self) -> RInvalidPktCntW<'_, MacPhyCrSpec> {
        RInvalidPktCntW::new(self, 6)
    }
    #[doc = "Bits 14:20 - rxpkt_ok or rxpkt_fail pulse counter,unit 25ns when mac clock is 40MHz"]
    #[inline(always)]
    pub fn r_id_match_cnt(&mut self) -> RIdMatchCntW<'_, MacPhyCrSpec> {
        RIdMatchCntW::new(self, 14)
    }
    #[doc = "Bit 21 - when set, per mpdu will trigger rx_id_match, else per ppdu trigger rx_id_match"]
    #[inline(always)]
    pub fn r_id_math_mpdu(&mut self) -> RIdMathMpduW<'_, MacPhyCrSpec> {
        RIdMathMpduW::new(self, 21)
    }
    #[doc = "Bits 22:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, MacPhyCrSpec> {
        DummyW::new(self, 22)
    }
}
#[doc = "mac phy control reg\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_phy_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_phy_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacPhyCrSpec;
impl crate::RegisterSpec for MacPhyCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_phy_cr::R`](R) reader structure"]
impl crate::Readable for MacPhyCrSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_phy_cr::W`](W) writer structure"]
impl crate::Writable for MacPhyCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_PHY_CR to value 0x0007_c3c9"]
impl crate::Resettable for MacPhyCrSpec {
    const RESET_VALUE: u32 = 0x0007_c3c9;
}
