#[doc = "Register `REG_AX_PWR_SR_MCS7_TXDIFF_TABLE2` reader"]
pub type R = crate::R<RegAxPwrSrMcs7TxdiffTable2Spec>;
#[doc = "Register `REG_AX_PWR_SR_MCS7_TXDIFF_TABLE2` writer"]
pub type W = crate::W<RegAxPwrSrMcs7TxdiffTable2Spec>;
#[doc = "Field `r_MCS7_TxDiff_11dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs7TxDiff11dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS7_TxDiff_11dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs7TxDiff11dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS7_TxDiff_12dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs7TxDiff12dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS7_TxDiff_12dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs7TxDiff12dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS7_TxDiff_13dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs7TxDiff13dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS7_TxDiff_13dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs7TxDiff13dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS7_TxDiff_14dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs7TxDiff14dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS7_TxDiff_14dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs7TxDiff14dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS7_TxDiff_15dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs7TxDiff15dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS7_TxDiff_15dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs7TxDiff15dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs7_tx_diff_11d_b_mcs_offset(&self) -> RMcs7TxDiff11dBMcsOffsetR {
        RMcs7TxDiff11dBMcsOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs7_tx_diff_12d_b_mcs_offset(&self) -> RMcs7TxDiff12dBMcsOffsetR {
        RMcs7TxDiff12dBMcsOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs7_tx_diff_13d_b_mcs_offset(&self) -> RMcs7TxDiff13dBMcsOffsetR {
        RMcs7TxDiff13dBMcsOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs7_tx_diff_14d_b_mcs_offset(&self) -> RMcs7TxDiff14dBMcsOffsetR {
        RMcs7TxDiff14dBMcsOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs7_tx_diff_15d_b_mcs_offset(&self) -> RMcs7TxDiff15dBMcsOffsetR {
        RMcs7TxDiff15dBMcsOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs7_tx_diff_11d_b_mcs_offset(
        &mut self,
    ) -> RMcs7TxDiff11dBMcsOffsetW<'_, RegAxPwrSrMcs7TxdiffTable2Spec> {
        RMcs7TxDiff11dBMcsOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs7_tx_diff_12d_b_mcs_offset(
        &mut self,
    ) -> RMcs7TxDiff12dBMcsOffsetW<'_, RegAxPwrSrMcs7TxdiffTable2Spec> {
        RMcs7TxDiff12dBMcsOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs7_tx_diff_13d_b_mcs_offset(
        &mut self,
    ) -> RMcs7TxDiff13dBMcsOffsetW<'_, RegAxPwrSrMcs7TxdiffTable2Spec> {
        RMcs7TxDiff13dBMcsOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs7_tx_diff_14d_b_mcs_offset(
        &mut self,
    ) -> RMcs7TxDiff14dBMcsOffsetW<'_, RegAxPwrSrMcs7TxdiffTable2Spec> {
        RMcs7TxDiff14dBMcsOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs7_tx_diff_15d_b_mcs_offset(
        &mut self,
    ) -> RMcs7TxDiff15dBMcsOffsetW<'_, RegAxPwrSrMcs7TxdiffTable2Spec> {
        RMcs7TxDiff15dBMcsOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_mcs7_txdiff_table2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_mcs7_txdiff_table2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrMcs7TxdiffTable2Spec;
impl crate::RegisterSpec for RegAxPwrSrMcs7TxdiffTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_mcs7_txdiff_table2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrMcs7TxdiffTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_mcs7_txdiff_table2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrMcs7TxdiffTable2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_MCS7_TXDIFF_TABLE2 to value 0"]
impl crate::Resettable for RegAxPwrSrMcs7TxdiffTable2Spec {}
