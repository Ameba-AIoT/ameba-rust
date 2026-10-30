#[doc = "Register `REG_TSFT_AVG_VALUE` reader"]
pub type R = crate::R<RegTsftAvgValueSpec>;
#[doc = "Field `BIT_TSFT_AVG_VALUE` reader - the average value of deltaTSFT(latch hioe_int_chn_on and cca_up) in specified period"]
pub type BitTsftAvgValueR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TSFT_AVG_GT_THRES_CNT` reader - statistic the times of the BIT_TSFT_AVG_VALUE than BIT_TSFT_AVG_THRES"]
pub type BitTsftAvgGtThresCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - the average value of deltaTSFT(latch hioe_int_chn_on and cca_up) in specified period"]
    #[inline(always)]
    pub fn bit_tsft_avg_value(&self) -> BitTsftAvgValueR {
        BitTsftAvgValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - statistic the times of the BIT_TSFT_AVG_VALUE than BIT_TSFT_AVG_THRES"]
    #[inline(always)]
    pub fn bit_tsft_avg_gt_thres_cnt(&self) -> BitTsftAvgGtThresCntR {
        BitTsftAvgGtThresCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tsft_avg_value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTsftAvgValueSpec;
impl crate::RegisterSpec for RegTsftAvgValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tsft_avg_value::R`](R) reader structure"]
impl crate::Readable for RegTsftAvgValueSpec {}
#[doc = "`reset()` method sets REG_TSFT_AVG_VALUE to value 0"]
impl crate::Resettable for RegTsftAvgValueSpec {}
