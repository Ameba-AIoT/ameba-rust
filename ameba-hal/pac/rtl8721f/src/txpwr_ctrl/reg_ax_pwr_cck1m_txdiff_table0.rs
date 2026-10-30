#[doc = "Register `REG_AX_PWR_cck1M_TXDIFF_TABLE0` reader"]
pub type R = crate::R<RegAxPwrCck1mTxdiffTable0Spec>;
#[doc = "Register `REG_AX_PWR_cck1M_TXDIFF_TABLE0` writer"]
pub type W = crate::W<RegAxPwrCck1mTxdiffTable0Spec>;
#[doc = "Field `r_cck1M_TxDiff_1dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck1mTxDiff1dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck1M_TxDiff_1dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck1mTxDiff1dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck1M_TxDiff_2dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck1mTxDiff2dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck1M_TxDiff_2dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck1mTxDiff2dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck1M_TxDiff_3dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck1mTxDiff3dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck1M_TxDiff_3dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck1mTxDiff3dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck1M_TxDiff_4dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck1mTxDiff4dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck1M_TxDiff_4dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck1mTxDiff4dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck1M_TxDiff_5dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck1mTxDiff5dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck1M_TxDiff_5dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck1mTxDiff5dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck1m_tx_diff_1d_b_rate_offset(&self) -> RCck1mTxDiff1dBRateOffsetR {
        RCck1mTxDiff1dBRateOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck1m_tx_diff_2d_b_rate_offset(&self) -> RCck1mTxDiff2dBRateOffsetR {
        RCck1mTxDiff2dBRateOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck1m_tx_diff_3d_b_rate_offset(&self) -> RCck1mTxDiff3dBRateOffsetR {
        RCck1mTxDiff3dBRateOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck1m_tx_diff_4d_b_rate_offset(&self) -> RCck1mTxDiff4dBRateOffsetR {
        RCck1mTxDiff4dBRateOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck1m_tx_diff_5d_b_rate_offset(&self) -> RCck1mTxDiff5dBRateOffsetR {
        RCck1mTxDiff5dBRateOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck1m_tx_diff_1d_b_rate_offset(
        &mut self,
    ) -> RCck1mTxDiff1dBRateOffsetW<'_, RegAxPwrCck1mTxdiffTable0Spec> {
        RCck1mTxDiff1dBRateOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck1m_tx_diff_2d_b_rate_offset(
        &mut self,
    ) -> RCck1mTxDiff2dBRateOffsetW<'_, RegAxPwrCck1mTxdiffTable0Spec> {
        RCck1mTxDiff2dBRateOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck1m_tx_diff_3d_b_rate_offset(
        &mut self,
    ) -> RCck1mTxDiff3dBRateOffsetW<'_, RegAxPwrCck1mTxdiffTable0Spec> {
        RCck1mTxDiff3dBRateOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck1m_tx_diff_4d_b_rate_offset(
        &mut self,
    ) -> RCck1mTxDiff4dBRateOffsetW<'_, RegAxPwrCck1mTxdiffTable0Spec> {
        RCck1mTxDiff4dBRateOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck1m_tx_diff_5d_b_rate_offset(
        &mut self,
    ) -> RCck1mTxDiff5dBRateOffsetW<'_, RegAxPwrCck1mTxdiffTable0Spec> {
        RCck1mTxDiff5dBRateOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_cck1m_txdiff_table0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_cck1m_txdiff_table0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrCck1mTxdiffTable0Spec;
impl crate::RegisterSpec for RegAxPwrCck1mTxdiffTable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_cck1m_txdiff_table0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrCck1mTxdiffTable0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_cck1m_txdiff_table0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrCck1mTxdiffTable0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_cck1M_TXDIFF_TABLE0 to value 0"]
impl crate::Resettable for RegAxPwrCck1mTxdiffTable0Spec {}
