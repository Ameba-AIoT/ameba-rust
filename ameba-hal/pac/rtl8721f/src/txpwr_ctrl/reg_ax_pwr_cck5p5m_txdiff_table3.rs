#[doc = "Register `REG_AX_PWR_cck5p5M_TXDIFF_TABLE3` reader"]
pub type R = crate::R<RegAxPwrCck5p5mTxdiffTable3Spec>;
#[doc = "Register `REG_AX_PWR_cck5p5M_TXDIFF_TABLE3` writer"]
pub type W = crate::W<RegAxPwrCck5p5mTxdiffTable3Spec>;
#[doc = "Field `r_cck5p5M_TxDiff_16dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck5p5mTxDiff16dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck5p5M_TxDiff_16dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck5p5mTxDiff16dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck5p5M_TxDiff_17dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck5p5mTxDiff17dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck5p5M_TxDiff_17dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck5p5mTxDiff17dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck5p5M_TxDiff_18dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck5p5mTxDiff18dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck5p5M_TxDiff_18dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck5p5mTxDiff18dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck5p5M_TxDiff_19dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck5p5mTxDiff19dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck5p5M_TxDiff_19dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck5p5mTxDiff19dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck5p5M_TxDiff_20dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck5p5mTxDiff20dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck5p5M_TxDiff_20dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck5p5mTxDiff20dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck5p5m_tx_diff_16d_b_rate_offset(&self) -> RCck5p5mTxDiff16dBRateOffsetR {
        RCck5p5mTxDiff16dBRateOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck5p5m_tx_diff_17d_b_rate_offset(&self) -> RCck5p5mTxDiff17dBRateOffsetR {
        RCck5p5mTxDiff17dBRateOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck5p5m_tx_diff_18d_b_rate_offset(&self) -> RCck5p5mTxDiff18dBRateOffsetR {
        RCck5p5mTxDiff18dBRateOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck5p5m_tx_diff_19d_b_rate_offset(&self) -> RCck5p5mTxDiff19dBRateOffsetR {
        RCck5p5mTxDiff19dBRateOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck5p5m_tx_diff_20d_b_rate_offset(&self) -> RCck5p5mTxDiff20dBRateOffsetR {
        RCck5p5mTxDiff20dBRateOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck5p5m_tx_diff_16d_b_rate_offset(
        &mut self,
    ) -> RCck5p5mTxDiff16dBRateOffsetW<'_, RegAxPwrCck5p5mTxdiffTable3Spec> {
        RCck5p5mTxDiff16dBRateOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck5p5m_tx_diff_17d_b_rate_offset(
        &mut self,
    ) -> RCck5p5mTxDiff17dBRateOffsetW<'_, RegAxPwrCck5p5mTxdiffTable3Spec> {
        RCck5p5mTxDiff17dBRateOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck5p5m_tx_diff_18d_b_rate_offset(
        &mut self,
    ) -> RCck5p5mTxDiff18dBRateOffsetW<'_, RegAxPwrCck5p5mTxdiffTable3Spec> {
        RCck5p5mTxDiff18dBRateOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck5p5m_tx_diff_19d_b_rate_offset(
        &mut self,
    ) -> RCck5p5mTxDiff19dBRateOffsetW<'_, RegAxPwrCck5p5mTxdiffTable3Spec> {
        RCck5p5mTxDiff19dBRateOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck5p5m_tx_diff_20d_b_rate_offset(
        &mut self,
    ) -> RCck5p5mTxDiff20dBRateOffsetW<'_, RegAxPwrCck5p5mTxdiffTable3Spec> {
        RCck5p5mTxDiff20dBRateOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_cck5p5m_txdiff_table3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_cck5p5m_txdiff_table3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrCck5p5mTxdiffTable3Spec;
impl crate::RegisterSpec for RegAxPwrCck5p5mTxdiffTable3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_cck5p5m_txdiff_table3::R`](R) reader structure"]
impl crate::Readable for RegAxPwrCck5p5mTxdiffTable3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_cck5p5m_txdiff_table3::W`](W) writer structure"]
impl crate::Writable for RegAxPwrCck5p5mTxdiffTable3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_cck5p5M_TXDIFF_TABLE3 to value 0"]
impl crate::Resettable for RegAxPwrCck5p5mTxdiffTable3Spec {}
