#[doc = "Register `REG_AX_PWR_cck11M_TXDIFF_TABLE1` reader"]
pub type R = crate::R<RegAxPwrCck11mTxdiffTable1Spec>;
#[doc = "Register `REG_AX_PWR_cck11M_TXDIFF_TABLE1` writer"]
pub type W = crate::W<RegAxPwrCck11mTxdiffTable1Spec>;
#[doc = "Field `r_cck11M_TxDiff_6dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck11mTxDiff6dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck11M_TxDiff_6dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck11mTxDiff6dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck11M_TxDiff_7dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck11mTxDiff7dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck11M_TxDiff_7dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck11mTxDiff7dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck11M_TxDiff_8dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck11mTxDiff8dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck11M_TxDiff_8dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck11mTxDiff8dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck11M_TxDiff_9dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck11mTxDiff9dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck11M_TxDiff_9dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck11mTxDiff9dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck11M_TxDiff_10dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck11mTxDiff10dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck11M_TxDiff_10dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck11mTxDiff10dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck11m_tx_diff_6d_b_rate_offset(&self) -> RCck11mTxDiff6dBRateOffsetR {
        RCck11mTxDiff6dBRateOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck11m_tx_diff_7d_b_rate_offset(&self) -> RCck11mTxDiff7dBRateOffsetR {
        RCck11mTxDiff7dBRateOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck11m_tx_diff_8d_b_rate_offset(&self) -> RCck11mTxDiff8dBRateOffsetR {
        RCck11mTxDiff8dBRateOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck11m_tx_diff_9d_b_rate_offset(&self) -> RCck11mTxDiff9dBRateOffsetR {
        RCck11mTxDiff9dBRateOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck11m_tx_diff_10d_b_rate_offset(&self) -> RCck11mTxDiff10dBRateOffsetR {
        RCck11mTxDiff10dBRateOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck11m_tx_diff_6d_b_rate_offset(
        &mut self,
    ) -> RCck11mTxDiff6dBRateOffsetW<'_, RegAxPwrCck11mTxdiffTable1Spec> {
        RCck11mTxDiff6dBRateOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck11m_tx_diff_7d_b_rate_offset(
        &mut self,
    ) -> RCck11mTxDiff7dBRateOffsetW<'_, RegAxPwrCck11mTxdiffTable1Spec> {
        RCck11mTxDiff7dBRateOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck11m_tx_diff_8d_b_rate_offset(
        &mut self,
    ) -> RCck11mTxDiff8dBRateOffsetW<'_, RegAxPwrCck11mTxdiffTable1Spec> {
        RCck11mTxDiff8dBRateOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck11m_tx_diff_9d_b_rate_offset(
        &mut self,
    ) -> RCck11mTxDiff9dBRateOffsetW<'_, RegAxPwrCck11mTxdiffTable1Spec> {
        RCck11mTxDiff9dBRateOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck11m_tx_diff_10d_b_rate_offset(
        &mut self,
    ) -> RCck11mTxDiff10dBRateOffsetW<'_, RegAxPwrCck11mTxdiffTable1Spec> {
        RCck11mTxDiff10dBRateOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_cck11m_txdiff_table1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_cck11m_txdiff_table1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrCck11mTxdiffTable1Spec;
impl crate::RegisterSpec for RegAxPwrCck11mTxdiffTable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_cck11m_txdiff_table1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrCck11mTxdiffTable1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_cck11m_txdiff_table1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrCck11mTxdiffTable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_cck11M_TXDIFF_TABLE1 to value 0"]
impl crate::Resettable for RegAxPwrCck11mTxdiffTable1Spec {}
