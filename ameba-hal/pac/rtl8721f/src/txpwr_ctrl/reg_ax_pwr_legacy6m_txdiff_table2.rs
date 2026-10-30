#[doc = "Register `REG_AX_PWR_legacy6M_TXDIFF_TABLE2` reader"]
pub type R = crate::R<RegAxPwrLegacy6mTxdiffTable2Spec>;
#[doc = "Register `REG_AX_PWR_legacy6M_TXDIFF_TABLE2` writer"]
pub type W = crate::W<RegAxPwrLegacy6mTxdiffTable2Spec>;
#[doc = "Field `r_legacy6M_TxDiff_11dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff11dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy6M_TxDiff_11dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff11dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy6M_TxDiff_12dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff12dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy6M_TxDiff_12dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff12dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy6M_TxDiff_13dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff13dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy6M_TxDiff_13dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff13dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy6M_TxDiff_14dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff14dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy6M_TxDiff_14dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff14dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy6M_TxDiff_15dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff15dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy6M_TxDiff_15dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff15dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_11d_b_rate_offset(&self) -> RLegacy6mTxDiff11dBRateOffsetR {
        RLegacy6mTxDiff11dBRateOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_12d_b_rate_offset(&self) -> RLegacy6mTxDiff12dBRateOffsetR {
        RLegacy6mTxDiff12dBRateOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_13d_b_rate_offset(&self) -> RLegacy6mTxDiff13dBRateOffsetR {
        RLegacy6mTxDiff13dBRateOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_14d_b_rate_offset(&self) -> RLegacy6mTxDiff14dBRateOffsetR {
        RLegacy6mTxDiff14dBRateOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_15d_b_rate_offset(&self) -> RLegacy6mTxDiff15dBRateOffsetR {
        RLegacy6mTxDiff15dBRateOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_11d_b_rate_offset(
        &mut self,
    ) -> RLegacy6mTxDiff11dBRateOffsetW<'_, RegAxPwrLegacy6mTxdiffTable2Spec> {
        RLegacy6mTxDiff11dBRateOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_12d_b_rate_offset(
        &mut self,
    ) -> RLegacy6mTxDiff12dBRateOffsetW<'_, RegAxPwrLegacy6mTxdiffTable2Spec> {
        RLegacy6mTxDiff12dBRateOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_13d_b_rate_offset(
        &mut self,
    ) -> RLegacy6mTxDiff13dBRateOffsetW<'_, RegAxPwrLegacy6mTxdiffTable2Spec> {
        RLegacy6mTxDiff13dBRateOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_14d_b_rate_offset(
        &mut self,
    ) -> RLegacy6mTxDiff14dBRateOffsetW<'_, RegAxPwrLegacy6mTxdiffTable2Spec> {
        RLegacy6mTxDiff14dBRateOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_15d_b_rate_offset(
        &mut self,
    ) -> RLegacy6mTxDiff15dBRateOffsetW<'_, RegAxPwrLegacy6mTxdiffTable2Spec> {
        RLegacy6mTxDiff15dBRateOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_legacy6m_txdiff_table2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_legacy6m_txdiff_table2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrLegacy6mTxdiffTable2Spec;
impl crate::RegisterSpec for RegAxPwrLegacy6mTxdiffTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_legacy6m_txdiff_table2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrLegacy6mTxdiffTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_legacy6m_txdiff_table2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrLegacy6mTxdiffTable2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_legacy6M_TXDIFF_TABLE2 to value 0"]
impl crate::Resettable for RegAxPwrLegacy6mTxdiffTable2Spec {}
