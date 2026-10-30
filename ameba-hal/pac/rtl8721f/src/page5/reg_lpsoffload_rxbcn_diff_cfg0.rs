#[doc = "Register `REG_LPSOFFLOAD_RXBCN_DIFF_CFG0` reader"]
pub type R = crate::R<RegLpsoffloadRxbcnDiffCfg0Spec>;
#[doc = "Register `REG_LPSOFFLOAD_RXBCN_DIFF_CFG0` writer"]
pub type W = crate::W<RegLpsoffloadRxbcnDiffCfg0Spec>;
#[doc = "Field `BIT_TSFT_AVG_THRES` reader - The threshold of deltaTSFT(latch hioe_int_chn_on and cca_up), unit us"]
pub type BitTsftAvgThresR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TSFT_AVG_THRES` writer - The threshold of deltaTSFT(latch hioe_int_chn_on and cca_up), unit us"]
pub type BitTsftAvgThresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_TSFT_AVG_GT_THRES_CNT_MAX` reader - The max times of the BIT_TSFT_AVG_VALUE greater than BIT_TSFT_AVG_THRES, if the BIT_TSFT_AVG_VALUE exceeds the BIT_TSFT_AVG_THRES several times in a row, and the number exceeds the BIT_TSFT_AVG_GT_THRES_CNT_MAX, generate an interrupt"]
pub type BitTsftAvgGtThresCntMaxR = crate::FieldReader;
#[doc = "Field `BIT_TSFT_AVG_GT_THRES_CNT_MAX` writer - The max times of the BIT_TSFT_AVG_VALUE greater than BIT_TSFT_AVG_THRES, if the BIT_TSFT_AVG_VALUE exceeds the BIT_TSFT_AVG_THRES several times in a row, and the number exceeds the BIT_TSFT_AVG_GT_THRES_CNT_MAX, generate an interrupt"]
pub type BitTsftAvgGtThresCntMaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_TSFT_AVG_CAL_PRD` reader - 2^(BIT_TSFT_AVG_CAL_PERIOD), default is 128 times"]
pub type BitTsftAvgCalPrdR = crate::FieldReader;
#[doc = "Field `BIT_TSFT_AVG_CAL_PRD` writer - 2^(BIT_TSFT_AVG_CAL_PERIOD), default is 128 times"]
pub type BitTsftAvgCalPrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_EN_BCNELY_LATCH_TSFT` reader - 1: latch TSFT to BIT_TSFT_AT_HIOE_RFON (0x153C) when P0's bcn_early int asserted"]
pub type BitEnBcnelyLatchTsftR = crate::BitReader;
#[doc = "Field `BIT_EN_BCNELY_LATCH_TSFT` writer - 1: latch TSFT to BIT_TSFT_AT_HIOE_RFON (0x153C) when P0's bcn_early int asserted"]
pub type BitEnBcnelyLatchTsftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TSFT_AVG_CAL` reader - 1: calculate the average value of deltaTSFT(latch hioe_int_chn_on and cca_up) in specified period"]
pub type BitEnTsftAvgCalR = crate::BitReader;
#[doc = "Field `BIT_EN_TSFT_AVG_CAL` writer - 1: calculate the average value of deltaTSFT(latch hioe_int_chn_on and cca_up) in specified period"]
pub type BitEnTsftAvgCalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - The threshold of deltaTSFT(latch hioe_int_chn_on and cca_up), unit us"]
    #[inline(always)]
    pub fn bit_tsft_avg_thres(&self) -> BitTsftAvgThresR {
        BitTsftAvgThresR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - The max times of the BIT_TSFT_AVG_VALUE greater than BIT_TSFT_AVG_THRES, if the BIT_TSFT_AVG_VALUE exceeds the BIT_TSFT_AVG_THRES several times in a row, and the number exceeds the BIT_TSFT_AVG_GT_THRES_CNT_MAX, generate an interrupt"]
    #[inline(always)]
    pub fn bit_tsft_avg_gt_thres_cnt_max(&self) -> BitTsftAvgGtThresCntMaxR {
        BitTsftAvgGtThresCntMaxR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 2^(BIT_TSFT_AVG_CAL_PERIOD), default is 128 times"]
    #[inline(always)]
    pub fn bit_tsft_avg_cal_prd(&self) -> BitTsftAvgCalPrdR {
        BitTsftAvgCalPrdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - 1: latch TSFT to BIT_TSFT_AT_HIOE_RFON (0x153C) when P0's bcn_early int asserted"]
    #[inline(always)]
    pub fn bit_en_bcnely_latch_tsft(&self) -> BitEnBcnelyLatchTsftR {
        BitEnBcnelyLatchTsftR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: calculate the average value of deltaTSFT(latch hioe_int_chn_on and cca_up) in specified period"]
    #[inline(always)]
    pub fn bit_en_tsft_avg_cal(&self) -> BitEnTsftAvgCalR {
        BitEnTsftAvgCalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - The threshold of deltaTSFT(latch hioe_int_chn_on and cca_up), unit us"]
    #[inline(always)]
    pub fn bit_tsft_avg_thres(&mut self) -> BitTsftAvgThresW<'_, RegLpsoffloadRxbcnDiffCfg0Spec> {
        BitTsftAvgThresW::new(self, 0)
    }
    #[doc = "Bits 16:19 - The max times of the BIT_TSFT_AVG_VALUE greater than BIT_TSFT_AVG_THRES, if the BIT_TSFT_AVG_VALUE exceeds the BIT_TSFT_AVG_THRES several times in a row, and the number exceeds the BIT_TSFT_AVG_GT_THRES_CNT_MAX, generate an interrupt"]
    #[inline(always)]
    pub fn bit_tsft_avg_gt_thres_cnt_max(
        &mut self,
    ) -> BitTsftAvgGtThresCntMaxW<'_, RegLpsoffloadRxbcnDiffCfg0Spec> {
        BitTsftAvgGtThresCntMaxW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 2^(BIT_TSFT_AVG_CAL_PERIOD), default is 128 times"]
    #[inline(always)]
    pub fn bit_tsft_avg_cal_prd(
        &mut self,
    ) -> BitTsftAvgCalPrdW<'_, RegLpsoffloadRxbcnDiffCfg0Spec> {
        BitTsftAvgCalPrdW::new(self, 24)
    }
    #[doc = "Bit 30 - 1: latch TSFT to BIT_TSFT_AT_HIOE_RFON (0x153C) when P0's bcn_early int asserted"]
    #[inline(always)]
    pub fn bit_en_bcnely_latch_tsft(
        &mut self,
    ) -> BitEnBcnelyLatchTsftW<'_, RegLpsoffloadRxbcnDiffCfg0Spec> {
        BitEnBcnelyLatchTsftW::new(self, 30)
    }
    #[doc = "Bit 31 - 1: calculate the average value of deltaTSFT(latch hioe_int_chn_on and cca_up) in specified period"]
    #[inline(always)]
    pub fn bit_en_tsft_avg_cal(&mut self) -> BitEnTsftAvgCalW<'_, RegLpsoffloadRxbcnDiffCfg0Spec> {
        BitEnTsftAvgCalW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lpsoffload_rxbcn_diff_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lpsoffload_rxbcn_diff_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLpsoffloadRxbcnDiffCfg0Spec;
impl crate::RegisterSpec for RegLpsoffloadRxbcnDiffCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lpsoffload_rxbcn_diff_cfg0::R`](R) reader structure"]
impl crate::Readable for RegLpsoffloadRxbcnDiffCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_lpsoffload_rxbcn_diff_cfg0::W`](W) writer structure"]
impl crate::Writable for RegLpsoffloadRxbcnDiffCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LPSOFFLOAD_RXBCN_DIFF_CFG0 to value 0x0701_0800"]
impl crate::Resettable for RegLpsoffloadRxbcnDiffCfg0Spec {
    const RESET_VALUE: u32 = 0x0701_0800;
}
