#[doc = "Register `REG_AX_PWR_legacy36M_TXDIFF_TABLE1` reader"]
pub type R = crate::R<RegAxPwrLegacy36mTxdiffTable1Spec>;
#[doc = "Register `REG_AX_PWR_legacy36M_TXDIFF_TABLE1` writer"]
pub type W = crate::W<RegAxPwrLegacy36mTxdiffTable1Spec>;
#[doc = "Field `r_legacy36M_TxDiff_6dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy36mTxDiff6dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy36M_TxDiff_6dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy36mTxDiff6dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy36M_TxDiff_7dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy36mTxDiff7dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy36M_TxDiff_7dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy36mTxDiff7dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy36M_TxDiff_8dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy36mTxDiff8dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy36M_TxDiff_8dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy36mTxDiff8dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy36M_TxDiff_9dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy36mTxDiff9dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy36M_TxDiff_9dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy36mTxDiff9dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy36M_TxDiff_10dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy36mTxDiff10dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy36M_TxDiff_10dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy36mTxDiff10dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy36m_tx_diff_6d_b_rate_offset(&self) -> RLegacy36mTxDiff6dBRateOffsetR {
        RLegacy36mTxDiff6dBRateOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy36m_tx_diff_7d_b_rate_offset(&self) -> RLegacy36mTxDiff7dBRateOffsetR {
        RLegacy36mTxDiff7dBRateOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy36m_tx_diff_8d_b_rate_offset(&self) -> RLegacy36mTxDiff8dBRateOffsetR {
        RLegacy36mTxDiff8dBRateOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy36m_tx_diff_9d_b_rate_offset(&self) -> RLegacy36mTxDiff9dBRateOffsetR {
        RLegacy36mTxDiff9dBRateOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy36m_tx_diff_10d_b_rate_offset(&self) -> RLegacy36mTxDiff10dBRateOffsetR {
        RLegacy36mTxDiff10dBRateOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy36m_tx_diff_6d_b_rate_offset(
        &mut self,
    ) -> RLegacy36mTxDiff6dBRateOffsetW<'_, RegAxPwrLegacy36mTxdiffTable1Spec> {
        RLegacy36mTxDiff6dBRateOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy36m_tx_diff_7d_b_rate_offset(
        &mut self,
    ) -> RLegacy36mTxDiff7dBRateOffsetW<'_, RegAxPwrLegacy36mTxdiffTable1Spec> {
        RLegacy36mTxDiff7dBRateOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy36m_tx_diff_8d_b_rate_offset(
        &mut self,
    ) -> RLegacy36mTxDiff8dBRateOffsetW<'_, RegAxPwrLegacy36mTxdiffTable1Spec> {
        RLegacy36mTxDiff8dBRateOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy36m_tx_diff_9d_b_rate_offset(
        &mut self,
    ) -> RLegacy36mTxDiff9dBRateOffsetW<'_, RegAxPwrLegacy36mTxdiffTable1Spec> {
        RLegacy36mTxDiff9dBRateOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy36m_tx_diff_10d_b_rate_offset(
        &mut self,
    ) -> RLegacy36mTxDiff10dBRateOffsetW<'_, RegAxPwrLegacy36mTxdiffTable1Spec> {
        RLegacy36mTxDiff10dBRateOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_legacy36m_txdiff_table1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_legacy36m_txdiff_table1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrLegacy36mTxdiffTable1Spec;
impl crate::RegisterSpec for RegAxPwrLegacy36mTxdiffTable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_legacy36m_txdiff_table1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrLegacy36mTxdiffTable1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_legacy36m_txdiff_table1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrLegacy36mTxdiffTable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_legacy36M_TXDIFF_TABLE1 to value 0"]
impl crate::Resettable for RegAxPwrLegacy36mTxdiffTable1Spec {}
