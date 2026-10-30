#[doc = "Register `REG_AX_PWR_legacy6M_TXDIFF_TABLE0` reader"]
pub type R = crate::R<RegAxPwrLegacy6mTxdiffTable0Spec>;
#[doc = "Register `REG_AX_PWR_legacy6M_TXDIFF_TABLE0` writer"]
pub type W = crate::W<RegAxPwrLegacy6mTxdiffTable0Spec>;
#[doc = "Field `r_legacy6M_TxDiff_1dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff1dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy6M_TxDiff_1dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff1dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy6M_TxDiff_2dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff2dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy6M_TxDiff_2dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff2dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy6M_TxDiff_3dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff3dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy6M_TxDiff_3dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff3dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy6M_TxDiff_4dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff4dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy6M_TxDiff_4dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff4dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy6M_TxDiff_5dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff5dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy6M_TxDiff_5dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy6mTxDiff5dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_1d_b_rate_offset(&self) -> RLegacy6mTxDiff1dBRateOffsetR {
        RLegacy6mTxDiff1dBRateOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_2d_b_rate_offset(&self) -> RLegacy6mTxDiff2dBRateOffsetR {
        RLegacy6mTxDiff2dBRateOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_3d_b_rate_offset(&self) -> RLegacy6mTxDiff3dBRateOffsetR {
        RLegacy6mTxDiff3dBRateOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_4d_b_rate_offset(&self) -> RLegacy6mTxDiff4dBRateOffsetR {
        RLegacy6mTxDiff4dBRateOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_5d_b_rate_offset(&self) -> RLegacy6mTxDiff5dBRateOffsetR {
        RLegacy6mTxDiff5dBRateOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_1d_b_rate_offset(
        &mut self,
    ) -> RLegacy6mTxDiff1dBRateOffsetW<'_, RegAxPwrLegacy6mTxdiffTable0Spec> {
        RLegacy6mTxDiff1dBRateOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_2d_b_rate_offset(
        &mut self,
    ) -> RLegacy6mTxDiff2dBRateOffsetW<'_, RegAxPwrLegacy6mTxdiffTable0Spec> {
        RLegacy6mTxDiff2dBRateOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_3d_b_rate_offset(
        &mut self,
    ) -> RLegacy6mTxDiff3dBRateOffsetW<'_, RegAxPwrLegacy6mTxdiffTable0Spec> {
        RLegacy6mTxDiff3dBRateOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_4d_b_rate_offset(
        &mut self,
    ) -> RLegacy6mTxDiff4dBRateOffsetW<'_, RegAxPwrLegacy6mTxdiffTable0Spec> {
        RLegacy6mTxDiff4dBRateOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy6m_tx_diff_5d_b_rate_offset(
        &mut self,
    ) -> RLegacy6mTxDiff5dBRateOffsetW<'_, RegAxPwrLegacy6mTxdiffTable0Spec> {
        RLegacy6mTxDiff5dBRateOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_legacy6m_txdiff_table0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_legacy6m_txdiff_table0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrLegacy6mTxdiffTable0Spec;
impl crate::RegisterSpec for RegAxPwrLegacy6mTxdiffTable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_legacy6m_txdiff_table0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrLegacy6mTxdiffTable0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_legacy6m_txdiff_table0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrLegacy6mTxdiffTable0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_legacy6M_TXDIFF_TABLE0 to value 0"]
impl crate::Resettable for RegAxPwrLegacy6mTxdiffTable0Spec {}
