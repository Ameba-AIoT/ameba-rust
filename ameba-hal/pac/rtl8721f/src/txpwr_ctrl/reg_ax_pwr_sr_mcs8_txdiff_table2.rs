#[doc = "Register `REG_AX_PWR_SR_MCS8_TXDIFF_TABLE2` reader"]
pub type R = crate::R<RegAxPwrSrMcs8TxdiffTable2Spec>;
#[doc = "Register `REG_AX_PWR_SR_MCS8_TXDIFF_TABLE2` writer"]
pub type W = crate::W<RegAxPwrSrMcs8TxdiffTable2Spec>;
#[doc = "Field `r_MCS8_TxDiff_11dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs8TxDiff11dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS8_TxDiff_11dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs8TxDiff11dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS8_TxDiff_12dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs8TxDiff12dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS8_TxDiff_12dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs8TxDiff12dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS8_TxDiff_13dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs8TxDiff13dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS8_TxDiff_13dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs8TxDiff13dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS8_TxDiff_14dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs8TxDiff14dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS8_TxDiff_14dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs8TxDiff14dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS8_TxDiff_15dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs8TxDiff15dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS8_TxDiff_15dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs8TxDiff15dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_11d_b_mcs_offset(&self) -> RMcs8TxDiff11dBMcsOffsetR {
        RMcs8TxDiff11dBMcsOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_12d_b_mcs_offset(&self) -> RMcs8TxDiff12dBMcsOffsetR {
        RMcs8TxDiff12dBMcsOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_13d_b_mcs_offset(&self) -> RMcs8TxDiff13dBMcsOffsetR {
        RMcs8TxDiff13dBMcsOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_14d_b_mcs_offset(&self) -> RMcs8TxDiff14dBMcsOffsetR {
        RMcs8TxDiff14dBMcsOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_15d_b_mcs_offset(&self) -> RMcs8TxDiff15dBMcsOffsetR {
        RMcs8TxDiff15dBMcsOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_11d_b_mcs_offset(
        &mut self,
    ) -> RMcs8TxDiff11dBMcsOffsetW<'_, RegAxPwrSrMcs8TxdiffTable2Spec> {
        RMcs8TxDiff11dBMcsOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_12d_b_mcs_offset(
        &mut self,
    ) -> RMcs8TxDiff12dBMcsOffsetW<'_, RegAxPwrSrMcs8TxdiffTable2Spec> {
        RMcs8TxDiff12dBMcsOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_13d_b_mcs_offset(
        &mut self,
    ) -> RMcs8TxDiff13dBMcsOffsetW<'_, RegAxPwrSrMcs8TxdiffTable2Spec> {
        RMcs8TxDiff13dBMcsOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_14d_b_mcs_offset(
        &mut self,
    ) -> RMcs8TxDiff14dBMcsOffsetW<'_, RegAxPwrSrMcs8TxdiffTable2Spec> {
        RMcs8TxDiff14dBMcsOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_15d_b_mcs_offset(
        &mut self,
    ) -> RMcs8TxDiff15dBMcsOffsetW<'_, RegAxPwrSrMcs8TxdiffTable2Spec> {
        RMcs8TxDiff15dBMcsOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_mcs8_txdiff_table2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_mcs8_txdiff_table2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrMcs8TxdiffTable2Spec;
impl crate::RegisterSpec for RegAxPwrSrMcs8TxdiffTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_mcs8_txdiff_table2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrMcs8TxdiffTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_mcs8_txdiff_table2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrMcs8TxdiffTable2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_MCS8_TXDIFF_TABLE2 to value 0"]
impl crate::Resettable for RegAxPwrSrMcs8TxdiffTable2Spec {}
