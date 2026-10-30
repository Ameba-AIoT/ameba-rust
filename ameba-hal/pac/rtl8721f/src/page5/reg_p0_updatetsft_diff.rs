#[doc = "Register `REG_P0_UPDATETSFT_DIFF` reader"]
pub type R = crate::R<RegP0UpdatetsftDiffSpec>;
#[doc = "Field `BIT_TSFT_DIFF_NS` reader - unit:1ns update_tsft_ns - TSFT. \\[10\\] is the sign bit"]
pub type BitTsftDiffNsR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TSFT_DIFF_NS_DIR` reader - 1: update_tsft_ns > TSFT_ns 0: update_tsft_ns <= TSFT_ns"]
pub type BitTsftDiffNsDirR = crate::BitReader;
#[doc = "Field `BIT_TSFT_DIFF_US` reader - unit:1us update_tsft_us - TSFT. \\[20\\] is the sign bit if diff >=255, this value would be 8'hff"]
pub type BitTsftDiffUsR = crate::FieldReader;
#[doc = "Field `BIT_TSFT_DIFF_US_DIR` reader - 1: update_tsft_us > TSFT_us 0: update_tsft_us <= TSFT_us"]
pub type BitTsftDiffUsDirR = crate::BitReader;
#[doc = "Field `BIT_TSFT_DIFF_VLD` reader - 1: indicate \\[20:0\\] is valid. 0: wmac_rx_tsf <= TSFT"]
pub type BitTsftDiffVldR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - unit:1ns update_tsft_ns - TSFT. \\[10\\] is the sign bit"]
    #[inline(always)]
    pub fn bit_tsft_diff_ns(&self) -> BitTsftDiffNsR {
        BitTsftDiffNsR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - 1: update_tsft_ns > TSFT_ns 0: update_tsft_ns <= TSFT_ns"]
    #[inline(always)]
    pub fn bit_tsft_diff_ns_dir(&self) -> BitTsftDiffNsDirR {
        BitTsftDiffNsDirR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:19 - unit:1us update_tsft_us - TSFT. \\[20\\] is the sign bit if diff >=255, this value would be 8'hff"]
    #[inline(always)]
    pub fn bit_tsft_diff_us(&self) -> BitTsftDiffUsR {
        BitTsftDiffUsR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 20 - 1: update_tsft_us > TSFT_us 0: update_tsft_us <= TSFT_us"]
    #[inline(always)]
    pub fn bit_tsft_diff_us_dir(&self) -> BitTsftDiffUsDirR {
        BitTsftDiffUsDirR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: indicate \\[20:0\\] is valid. 0: wmac_rx_tsf <= TSFT"]
    #[inline(always)]
    pub fn bit_tsft_diff_vld(&self) -> BitTsftDiffVldR {
        BitTsftDiffVldR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p0_updatetsft_diff::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP0UpdatetsftDiffSpec;
impl crate::RegisterSpec for RegP0UpdatetsftDiffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p0_updatetsft_diff::R`](R) reader structure"]
impl crate::Readable for RegP0UpdatetsftDiffSpec {}
#[doc = "`reset()` method sets REG_P0_UPDATETSFT_DIFF to value 0"]
impl crate::Resettable for RegP0UpdatetsftDiffSpec {}
