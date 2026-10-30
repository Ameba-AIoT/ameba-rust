#[doc = "Register `REG_AX_PWR_cck2M_TXDIFF_TABLE2` reader"]
pub type R = crate::R<RegAxPwrCck2mTxdiffTable2Spec>;
#[doc = "Register `REG_AX_PWR_cck2M_TXDIFF_TABLE2` writer"]
pub type W = crate::W<RegAxPwrCck2mTxdiffTable2Spec>;
#[doc = "Field `r_cck2M_TxDiff_11dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck2mTxDiff11dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck2M_TxDiff_11dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck2mTxDiff11dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck2M_TxDiff_12dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck2mTxDiff12dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck2M_TxDiff_12dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck2mTxDiff12dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck2M_TxDiff_13dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck2mTxDiff13dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck2M_TxDiff_13dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck2mTxDiff13dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck2M_TxDiff_14dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck2mTxDiff14dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck2M_TxDiff_14dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck2mTxDiff14dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_cck2M_TxDiff_15dB_rate_offset` reader - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck2mTxDiff15dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_cck2M_TxDiff_15dB_rate_offset` writer - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
pub type RCck2mTxDiff15dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck2m_tx_diff_11d_b_rate_offset(&self) -> RCck2mTxDiff11dBRateOffsetR {
        RCck2mTxDiff11dBRateOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck2m_tx_diff_12d_b_rate_offset(&self) -> RCck2mTxDiff12dBRateOffsetR {
        RCck2mTxDiff12dBRateOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck2m_tx_diff_13d_b_rate_offset(&self) -> RCck2mTxDiff13dBRateOffsetR {
        RCck2mTxDiff13dBRateOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck2m_tx_diff_14d_b_rate_offset(&self) -> RCck2mTxDiff14dBRateOffsetR {
        RCck2mTxDiff14dBRateOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck2m_tx_diff_15d_b_rate_offset(&self) -> RCck2mTxDiff15dBRateOffsetR {
        RCck2mTxDiff15dBRateOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck2m_tx_diff_11d_b_rate_offset(
        &mut self,
    ) -> RCck2mTxDiff11dBRateOffsetW<'_, RegAxPwrCck2mTxdiffTable2Spec> {
        RCck2mTxDiff11dBRateOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck2m_tx_diff_12d_b_rate_offset(
        &mut self,
    ) -> RCck2mTxDiff12dBRateOffsetW<'_, RegAxPwrCck2mTxdiffTable2Spec> {
        RCck2mTxDiff12dBRateOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck2m_tx_diff_13d_b_rate_offset(
        &mut self,
    ) -> RCck2mTxDiff13dBRateOffsetW<'_, RegAxPwrCck2mTxdiffTable2Spec> {
        RCck2mTxDiff13dBRateOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck2m_tx_diff_14d_b_rate_offset(
        &mut self,
    ) -> RCck2mTxDiff14dBRateOffsetW<'_, RegAxPwrCck2mTxdiffTable2Spec> {
        RCck2mTxDiff14dBRateOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) cck 0~3, 0-cck1M,1=cck2M,2=cck5p5M,3=cck11M"]
    #[inline(always)]
    pub fn r_cck2m_tx_diff_15d_b_rate_offset(
        &mut self,
    ) -> RCck2mTxDiff15dBRateOffsetW<'_, RegAxPwrCck2mTxdiffTable2Spec> {
        RCck2mTxDiff15dBRateOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_cck2m_txdiff_table2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_cck2m_txdiff_table2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrCck2mTxdiffTable2Spec;
impl crate::RegisterSpec for RegAxPwrCck2mTxdiffTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_cck2m_txdiff_table2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrCck2mTxdiffTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_cck2m_txdiff_table2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrCck2mTxdiffTable2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_cck2M_TXDIFF_TABLE2 to value 0"]
impl crate::Resettable for RegAxPwrCck2mTxdiffTable2Spec {}
