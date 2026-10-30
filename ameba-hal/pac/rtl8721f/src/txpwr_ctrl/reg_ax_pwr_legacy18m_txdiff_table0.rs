#[doc = "Register `REG_AX_PWR_legacy18M_TXDIFF_TABLE0` reader"]
pub type R = crate::R<RegAxPwrLegacy18mTxdiffTable0Spec>;
#[doc = "Register `REG_AX_PWR_legacy18M_TXDIFF_TABLE0` writer"]
pub type W = crate::W<RegAxPwrLegacy18mTxdiffTable0Spec>;
#[doc = "Field `r_legacy18M_TxDiff_1dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy18mTxDiff1dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy18M_TxDiff_1dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy18mTxDiff1dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy18M_TxDiff_2dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy18mTxDiff2dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy18M_TxDiff_2dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy18mTxDiff2dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy18M_TxDiff_3dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy18mTxDiff3dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy18M_TxDiff_3dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy18mTxDiff3dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy18M_TxDiff_4dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy18mTxDiff4dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy18M_TxDiff_4dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy18mTxDiff4dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_legacy18M_TxDiff_5dB_rate_offset` reader - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy18mTxDiff5dBRateOffsetR = crate::FieldReader;
#[doc = "Field `r_legacy18M_TxDiff_5dB_rate_offset` writer - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
pub type RLegacy18mTxDiff5dBRateOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy18m_tx_diff_1d_b_rate_offset(&self) -> RLegacy18mTxDiff1dBRateOffsetR {
        RLegacy18mTxDiff1dBRateOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy18m_tx_diff_2d_b_rate_offset(&self) -> RLegacy18mTxDiff2dBRateOffsetR {
        RLegacy18mTxDiff2dBRateOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy18m_tx_diff_3d_b_rate_offset(&self) -> RLegacy18mTxDiff3dBRateOffsetR {
        RLegacy18mTxDiff3dBRateOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy18m_tx_diff_4d_b_rate_offset(&self) -> RLegacy18mTxDiff4dBRateOffsetR {
        RLegacy18mTxDiff4dBRateOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy18m_tx_diff_5d_b_rate_offset(&self) -> RLegacy18mTxDiff5dBRateOffsetR {
        RLegacy18mTxDiff5dBRateOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy18m_tx_diff_1d_b_rate_offset(
        &mut self,
    ) -> RLegacy18mTxDiff1dBRateOffsetW<'_, RegAxPwrLegacy18mTxdiffTable0Spec> {
        RLegacy18mTxDiff1dBRateOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy18m_tx_diff_2d_b_rate_offset(
        &mut self,
    ) -> RLegacy18mTxDiff2dBRateOffsetW<'_, RegAxPwrLegacy18mTxdiffTable0Spec> {
        RLegacy18mTxDiff2dBRateOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy18m_tx_diff_3d_b_rate_offset(
        &mut self,
    ) -> RLegacy18mTxDiff3dBRateOffsetW<'_, RegAxPwrLegacy18mTxdiffTable0Spec> {
        RLegacy18mTxDiff3dBRateOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy18m_tx_diff_4d_b_rate_offset(
        &mut self,
    ) -> RLegacy18mTxDiff4dBRateOffsetW<'_, RegAxPwrLegacy18mTxdiffTable0Spec> {
        RLegacy18mTxDiff4dBRateOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) legacy 4~11,4=legacy6M,5=legacy9M,6=legacy12M,7=legacy18M,8=legacy24M,9=legacy36M,10=legacy48M,11=legacy54M"]
    #[inline(always)]
    pub fn r_legacy18m_tx_diff_5d_b_rate_offset(
        &mut self,
    ) -> RLegacy18mTxDiff5dBRateOffsetW<'_, RegAxPwrLegacy18mTxdiffTable0Spec> {
        RLegacy18mTxDiff5dBRateOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_legacy18m_txdiff_table0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_legacy18m_txdiff_table0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrLegacy18mTxdiffTable0Spec;
impl crate::RegisterSpec for RegAxPwrLegacy18mTxdiffTable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_legacy18m_txdiff_table0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrLegacy18mTxdiffTable0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_legacy18m_txdiff_table0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrLegacy18mTxdiffTable0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_legacy18M_TXDIFF_TABLE0 to value 0"]
impl crate::Resettable for RegAxPwrLegacy18mTxdiffTable0Spec {}
