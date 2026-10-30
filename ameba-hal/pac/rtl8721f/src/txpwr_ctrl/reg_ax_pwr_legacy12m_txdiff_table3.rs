#[doc = "Register `REG_AX_PWR_legacy12M_TXDIFF_TABLE3` reader"]
pub type R = crate::R<RegAxPwrLegacy12mTxdiffTable3Spec>;
#[doc = "Register `REG_AX_PWR_legacy12M_TXDIFF_TABLE3` writer"]
pub type W = crate::W<RegAxPwrLegacy12mTxdiffTable3Spec>;
#[doc = "Field `r_legacy12M_TxDiff_16dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy12mTxDiff16dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy12M_TxDiff_16dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy12mTxDiff16dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy12M_TxDiff_17dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy12mTxDiff17dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy12M_TxDiff_17dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy12mTxDiff17dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy12M_TxDiff_18dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy12mTxDiff18dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy12M_TxDiff_18dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy12mTxDiff18dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy12M_TxDiff_19dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy12mTxDiff19dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy12M_TxDiff_19dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy12mTxDiff19dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy12M_TxDiff_20dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy12mTxDiff20dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy12M_TxDiff_20dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy12mTxDiff20dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy12m_tx_diff_16d_b_rate_offset(&self) -> RLegacy12mTxDiff16dBRateOffsetR {
        RLegacy12mTxDiff16dBRateOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy12m_tx_diff_17d_b_rate_offset(&self) -> RLegacy12mTxDiff17dBRateOffsetR {
        RLegacy12mTxDiff17dBRateOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy12m_tx_diff_18d_b_rate_offset(&self) -> RLegacy12mTxDiff18dBRateOffsetR {
        RLegacy12mTxDiff18dBRateOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy12m_tx_diff_19d_b_rate_offset(&self) -> RLegacy12mTxDiff19dBRateOffsetR {
        RLegacy12mTxDiff19dBRateOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy12m_tx_diff_20d_b_rate_offset(&self) -> RLegacy12mTxDiff20dBRateOffsetR {
        RLegacy12mTxDiff20dBRateOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy12m_tx_diff_16d_b_rate_offset(
        &mut self,
    ) -> RLegacy12mTxDiff16dBRateOffsetW<'_, RegAxPwrLegacy12mTxdiffTable3Spec> {
        RLegacy12mTxDiff16dBRateOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy12m_tx_diff_17d_b_rate_offset(
        &mut self,
    ) -> RLegacy12mTxDiff17dBRateOffsetW<'_, RegAxPwrLegacy12mTxdiffTable3Spec> {
        RLegacy12mTxDiff17dBRateOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy12m_tx_diff_18d_b_rate_offset(
        &mut self,
    ) -> RLegacy12mTxDiff18dBRateOffsetW<'_, RegAxPwrLegacy12mTxdiffTable3Spec> {
        RLegacy12mTxDiff18dBRateOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy12m_tx_diff_19d_b_rate_offset(
        &mut self,
    ) -> RLegacy12mTxDiff19dBRateOffsetW<'_, RegAxPwrLegacy12mTxdiffTable3Spec> {
        RLegacy12mTxDiff19dBRateOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy12m_tx_diff_20d_b_rate_offset(
        &mut self,
    ) -> RLegacy12mTxDiff20dBRateOffsetW<'_, RegAxPwrLegacy12mTxdiffTable3Spec> {
        RLegacy12mTxDiff20dBRateOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_legacy12m_txdiff_table3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_legacy12m_txdiff_table3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrLegacy12mTxdiffTable3Spec;
impl crate::RegisterSpec for RegAxPwrLegacy12mTxdiffTable3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_legacy12m_txdiff_table3::R`](R) reader structure"]
impl crate::Readable for RegAxPwrLegacy12mTxdiffTable3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_legacy12m_txdiff_table3::W`](W) writer structure"]
impl crate::Writable for RegAxPwrLegacy12mTxdiffTable3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_legacy12M_TXDIFF_TABLE3 to value 0"]
impl crate::Resettable for RegAxPwrLegacy12mTxdiffTable3Spec {}
