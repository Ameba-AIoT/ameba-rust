#[doc = "Register `REG_RX_TSFT_DIFF` reader"]
pub type R = crate::R<RegRxTsftDiffSpec>;
#[doc = "Field `BIT_RXBCN_TIME_DIFF` reader - unit:1us wmac_rx_tsf - TSFT (selected by 0x1520\\[0\\]). \\[30\\] is the sign bit"]
pub type BitRxbcnTimeDiffR = crate::FieldReader<u32>;
#[doc = "Field `BIT_RXBCN_TIME_DIFF_DIR` reader - 1: wmac_rx_tsf > TSFT 0: wmac_rx_tsf <= TSFT"]
pub type BitRxbcnTimeDiffDirR = crate::BitReader;
#[doc = "Field `BIT_RXBCN_TIME_VLD` reader - 1: indicate \\[30:0\\] is valid. Write 1 to clear this bit"]
pub type BitRxbcnTimeVldR = crate::BitReader;
impl R {
    #[doc = "Bits 0:29 - unit:1us wmac_rx_tsf - TSFT (selected by 0x1520\\[0\\]). \\[30\\] is the sign bit"]
    #[inline(always)]
    pub fn bit_rxbcn_time_diff(&self) -> BitRxbcnTimeDiffR {
        BitRxbcnTimeDiffR::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - 1: wmac_rx_tsf > TSFT 0: wmac_rx_tsf <= TSFT"]
    #[inline(always)]
    pub fn bit_rxbcn_time_diff_dir(&self) -> BitRxbcnTimeDiffDirR {
        BitRxbcnTimeDiffDirR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: indicate \\[30:0\\] is valid. Write 1 to clear this bit"]
    #[inline(always)]
    pub fn bit_rxbcn_time_vld(&self) -> BitRxbcnTimeVldR {
        BitRxbcnTimeVldR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_tsft_diff::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxTsftDiffSpec;
impl crate::RegisterSpec for RegRxTsftDiffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_tsft_diff::R`](R) reader structure"]
impl crate::Readable for RegRxTsftDiffSpec {}
#[doc = "`reset()` method sets REG_RX_TSFT_DIFF to value 0"]
impl crate::Resettable for RegRxTsftDiffSpec {}
