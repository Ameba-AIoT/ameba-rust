#[doc = "Register `REG_WMAC_TCR_CTL` reader"]
pub type R = crate::R<RegWmacTcrCtlSpec>;
#[doc = "Register `REG_WMAC_TCR_CTL` writer"]
pub type W = crate::W<RegWmacTcrCtlSpec>;
#[doc = "Field `BIT_WMAC_TCR_TSFT_OFS` reader - BIT_WMAC_TCR_TSFT_OFS\\[7:0\\] is for cck stamp time offset; BIT_WMAC_TCR_TSFT_OFS\\[15:8\\] is for ofdm rate time offset;"]
pub type BitWmacTcrTsftOfsR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_TCR_TSFT_OFS` writer - BIT_WMAC_TCR_TSFT_OFS\\[7:0\\] is for cck stamp time offset; BIT_WMAC_TCR_TSFT_OFS\\[15:8\\] is for ofdm rate time offset;"]
pub type BitWmacTcrTsftOfsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_UDF_THSD_VALUE` reader - Underflow threshold"]
pub type BitUdfThsdValueR = crate::FieldReader;
#[doc = "Field `BIT_UDF_THSD_VALUE` writer - Underflow threshold"]
pub type BitUdfThsdValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_UDF_THSD_V1` reader - BIT_UDF_THSD\\[7\\]=1,enable underflow recovery;r_UDF_THSD\\[7\\]=0,disable underflow recovery; BIT_UDF_THSD\\[6:0\\]: when mac_wrptr - PHY_rdptr is less than this value, set underflow;"]
pub type BitUdfThsdV1R = crate::BitReader;
#[doc = "Field `BIT_UDF_THSD_V1` writer - BIT_UDF_THSD\\[7\\]=1,enable underflow recovery;r_UDF_THSD\\[7\\]=0,disable underflow recovery; BIT_UDF_THSD\\[6:0\\]: when mac_wrptr - PHY_rdptr is less than this value, set underflow;"]
pub type BitUdfThsdV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ZLD_NUM` reader - Fetch zero length delimiter number(unit : 4bytes) when mactx underflow"]
pub type BitZldNumR = crate::FieldReader;
#[doc = "Field `BIT_ZLD_NUM` writer - Fetch zero length delimiter number(unit : 4bytes) when mactx underflow"]
pub type BitZldNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - BIT_WMAC_TCR_TSFT_OFS\\[7:0\\] is for cck stamp time offset; BIT_WMAC_TCR_TSFT_OFS\\[15:8\\] is for ofdm rate time offset;"]
    #[inline(always)]
    pub fn bit_wmac_tcr_tsft_ofs(&self) -> BitWmacTcrTsftOfsR {
        BitWmacTcrTsftOfsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Underflow threshold"]
    #[inline(always)]
    pub fn bit_udf_thsd_value(&self) -> BitUdfThsdValueR {
        BitUdfThsdValueR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - BIT_UDF_THSD\\[7\\]=1,enable underflow recovery;r_UDF_THSD\\[7\\]=0,disable underflow recovery; BIT_UDF_THSD\\[6:0\\]: when mac_wrptr - PHY_rdptr is less than this value, set underflow;"]
    #[inline(always)]
    pub fn bit_udf_thsd_v1(&self) -> BitUdfThsdV1R {
        BitUdfThsdV1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Fetch zero length delimiter number(unit : 4bytes) when mactx underflow"]
    #[inline(always)]
    pub fn bit_zld_num(&self) -> BitZldNumR {
        BitZldNumR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - BIT_WMAC_TCR_TSFT_OFS\\[7:0\\] is for cck stamp time offset; BIT_WMAC_TCR_TSFT_OFS\\[15:8\\] is for ofdm rate time offset;"]
    #[inline(always)]
    pub fn bit_wmac_tcr_tsft_ofs(&mut self) -> BitWmacTcrTsftOfsW<'_, RegWmacTcrCtlSpec> {
        BitWmacTcrTsftOfsW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Underflow threshold"]
    #[inline(always)]
    pub fn bit_udf_thsd_value(&mut self) -> BitUdfThsdValueW<'_, RegWmacTcrCtlSpec> {
        BitUdfThsdValueW::new(self, 16)
    }
    #[doc = "Bit 23 - BIT_UDF_THSD\\[7\\]=1,enable underflow recovery;r_UDF_THSD\\[7\\]=0,disable underflow recovery; BIT_UDF_THSD\\[6:0\\]: when mac_wrptr - PHY_rdptr is less than this value, set underflow;"]
    #[inline(always)]
    pub fn bit_udf_thsd_v1(&mut self) -> BitUdfThsdV1W<'_, RegWmacTcrCtlSpec> {
        BitUdfThsdV1W::new(self, 23)
    }
    #[doc = "Bits 24:31 - Fetch zero length delimiter number(unit : 4bytes) when mactx underflow"]
    #[inline(always)]
    pub fn bit_zld_num(&mut self) -> BitZldNumW<'_, RegWmacTcrCtlSpec> {
        BitZldNumW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_tcr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_tcr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacTcrCtlSpec;
impl crate::RegisterSpec for RegWmacTcrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_tcr_ctl::R`](R) reader structure"]
impl crate::Readable for RegWmacTcrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_tcr_ctl::W`](W) writer structure"]
impl crate::Writable for RegWmacTcrCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_TCR_CTL to value 0x0886_0450"]
impl crate::Resettable for RegWmacTcrCtlSpec {
    const RESET_VALUE: u32 = 0x0886_0450;
}
