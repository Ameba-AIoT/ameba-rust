#[doc = "Register `REG_RXBCN_TSFT_STATISTIC` reader"]
pub type R = crate::R<RegRxbcnTsftStatisticSpec>;
#[doc = "Register `REG_RXBCN_TSFT_STATISTIC` writer"]
pub type W = crate::W<RegRxbcnTsftStatisticSpec>;
#[doc = "Field `BIT_RXBCN_GT_THRES_CNT` reader - valid when BIT_EN_RXBCN_GT_THRES_CNT = 1. counter the number of beacon with the |TSFT - rxbcn.timestamp| exceed the threshold"]
pub type BitRxbcnGtThresCntR = crate::FieldReader;
#[doc = "Field `BIT_CLR_RXBCN_GT_THRES_CNT` writer - clear the BIT_RXBCN_GT_THRES_CNT to 0"]
pub type BitClrRxbcnGtThresCntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_RXBCN_GT_THRES_CNT` reader - enable the counter of BIT_RXBCN_GT_THRES_CNT"]
pub type BitEnRxbcnGtThresCntR = crate::BitReader;
#[doc = "Field `BIT_EN_RXBCN_GT_THRES_CNT` writer - enable the counter of BIT_RXBCN_GT_THRES_CNT"]
pub type BitEnRxbcnGtThresCntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXBCN_TSFT_THRES` reader - used for LPS MAC, if |TSFT - rxbcn.timestamp| > BIT_RXBCN_TSFT_THRES, need to wait fcsok. Else, update TSFT at wmac_rxtsf_rdy (before fcsok) unit is 1us,max is 8ms"]
pub type BitRxbcnTsftThresR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXBCN_TSFT_THRES` writer - used for LPS MAC, if |TSFT - rxbcn.timestamp| > BIT_RXBCN_TSFT_THRES, need to wait fcsok. Else, update TSFT at wmac_rxtsf_rdy (before fcsok) unit is 1us,max is 8ms"]
pub type BitRxbcnTsftThresW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_UPD_TSFT_EARLY` reader - 0: don't update TSFT when the following scenarios occur: tim_no_hit, |TSFT - rxbcn.timestamp| <= BIT_RXBCN_TSFT_THRES, and r_wait_rxbcn_finish = 1 1: update TSFT"]
pub type BitUpdTsftEarlyR = crate::BitReader;
#[doc = "Field `BIT_UPD_TSFT_EARLY` writer - 0: don't update TSFT when the following scenarios occur: tim_no_hit, |TSFT - rxbcn.timestamp| <= BIT_RXBCN_TSFT_THRES, and r_wait_rxbcn_finish = 1 1: update TSFT"]
pub type BitUpdTsftEarlyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - valid when BIT_EN_RXBCN_GT_THRES_CNT = 1. counter the number of beacon with the |TSFT - rxbcn.timestamp| exceed the threshold"]
    #[inline(always)]
    pub fn bit_rxbcn_gt_thres_cnt(&self) -> BitRxbcnGtThresCntR {
        BitRxbcnGtThresCntR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - enable the counter of BIT_RXBCN_GT_THRES_CNT"]
    #[inline(always)]
    pub fn bit_en_rxbcn_gt_thres_cnt(&self) -> BitEnRxbcnGtThresCntR {
        BitEnRxbcnGtThresCntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:28 - used for LPS MAC, if |TSFT - rxbcn.timestamp| > BIT_RXBCN_TSFT_THRES, need to wait fcsok. Else, update TSFT at wmac_rxtsf_rdy (before fcsok) unit is 1us,max is 8ms"]
    #[inline(always)]
    pub fn bit_rxbcn_tsft_thres(&self) -> BitRxbcnTsftThresR {
        BitRxbcnTsftThresR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - 0: don't update TSFT when the following scenarios occur: tim_no_hit, |TSFT - rxbcn.timestamp| <= BIT_RXBCN_TSFT_THRES, and r_wait_rxbcn_finish = 1 1: update TSFT"]
    #[inline(always)]
    pub fn bit_upd_tsft_early(&self) -> BitUpdTsftEarlyR {
        BitUpdTsftEarlyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - clear the BIT_RXBCN_GT_THRES_CNT to 0"]
    #[inline(always)]
    pub fn bit_clr_rxbcn_gt_thres_cnt(
        &mut self,
    ) -> BitClrRxbcnGtThresCntW<'_, RegRxbcnTsftStatisticSpec> {
        BitClrRxbcnGtThresCntW::new(self, 6)
    }
    #[doc = "Bit 7 - enable the counter of BIT_RXBCN_GT_THRES_CNT"]
    #[inline(always)]
    pub fn bit_en_rxbcn_gt_thres_cnt(
        &mut self,
    ) -> BitEnRxbcnGtThresCntW<'_, RegRxbcnTsftStatisticSpec> {
        BitEnRxbcnGtThresCntW::new(self, 7)
    }
    #[doc = "Bits 16:28 - used for LPS MAC, if |TSFT - rxbcn.timestamp| > BIT_RXBCN_TSFT_THRES, need to wait fcsok. Else, update TSFT at wmac_rxtsf_rdy (before fcsok) unit is 1us,max is 8ms"]
    #[inline(always)]
    pub fn bit_rxbcn_tsft_thres(&mut self) -> BitRxbcnTsftThresW<'_, RegRxbcnTsftStatisticSpec> {
        BitRxbcnTsftThresW::new(self, 16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegRxbcnTsftStatisticSpec> {
        DummyW::new(self, 29)
    }
    #[doc = "Bit 31 - 0: don't update TSFT when the following scenarios occur: tim_no_hit, |TSFT - rxbcn.timestamp| <= BIT_RXBCN_TSFT_THRES, and r_wait_rxbcn_finish = 1 1: update TSFT"]
    #[inline(always)]
    pub fn bit_upd_tsft_early(&mut self) -> BitUpdTsftEarlyW<'_, RegRxbcnTsftStatisticSpec> {
        BitUpdTsftEarlyW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxbcn_tsft_statistic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxbcn_tsft_statistic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxbcnTsftStatisticSpec;
impl crate::RegisterSpec for RegRxbcnTsftStatisticSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxbcn_tsft_statistic::R`](R) reader structure"]
impl crate::Readable for RegRxbcnTsftStatisticSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxbcn_tsft_statistic::W`](W) writer structure"]
impl crate::Writable for RegRxbcnTsftStatisticSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXBCN_TSFT_STATISTIC to value 0x8000_0000"]
impl crate::Resettable for RegRxbcnTsftStatisticSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
