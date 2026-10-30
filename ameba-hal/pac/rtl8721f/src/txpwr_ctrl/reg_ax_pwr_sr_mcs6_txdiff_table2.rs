#[doc = "Register `REG_AX_PWR_SR_MCS6_TXDIFF_TABLE2` reader"]
pub type R = crate::R<RegAxPwrSrMcs6TxdiffTable2Spec>;
#[doc = "Register `REG_AX_PWR_SR_MCS6_TXDIFF_TABLE2` writer"]
pub type W = crate::W<RegAxPwrSrMcs6TxdiffTable2Spec>;
#[doc = "Field `r_MCS6_TxDiff_11dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs6TxDiff11dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS6_TxDiff_11dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs6TxDiff11dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS6_TxDiff_12dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs6TxDiff12dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS6_TxDiff_12dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs6TxDiff12dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS6_TxDiff_13dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs6TxDiff13dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS6_TxDiff_13dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs6TxDiff13dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS6_TxDiff_14dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs6TxDiff14dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS6_TxDiff_14dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs6TxDiff14dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS6_TxDiff_15dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs6TxDiff15dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS6_TxDiff_15dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs6TxDiff15dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs6_tx_diff_11d_b_mcs_offset(&self) -> RMcs6TxDiff11dBMcsOffsetR {
        RMcs6TxDiff11dBMcsOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs6_tx_diff_12d_b_mcs_offset(&self) -> RMcs6TxDiff12dBMcsOffsetR {
        RMcs6TxDiff12dBMcsOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs6_tx_diff_13d_b_mcs_offset(&self) -> RMcs6TxDiff13dBMcsOffsetR {
        RMcs6TxDiff13dBMcsOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs6_tx_diff_14d_b_mcs_offset(&self) -> RMcs6TxDiff14dBMcsOffsetR {
        RMcs6TxDiff14dBMcsOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs6_tx_diff_15d_b_mcs_offset(&self) -> RMcs6TxDiff15dBMcsOffsetR {
        RMcs6TxDiff15dBMcsOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs6_tx_diff_11d_b_mcs_offset(
        &mut self,
    ) -> RMcs6TxDiff11dBMcsOffsetW<'_, RegAxPwrSrMcs6TxdiffTable2Spec> {
        RMcs6TxDiff11dBMcsOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs6_tx_diff_12d_b_mcs_offset(
        &mut self,
    ) -> RMcs6TxDiff12dBMcsOffsetW<'_, RegAxPwrSrMcs6TxdiffTable2Spec> {
        RMcs6TxDiff12dBMcsOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs6_tx_diff_13d_b_mcs_offset(
        &mut self,
    ) -> RMcs6TxDiff13dBMcsOffsetW<'_, RegAxPwrSrMcs6TxdiffTable2Spec> {
        RMcs6TxDiff13dBMcsOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs6_tx_diff_14d_b_mcs_offset(
        &mut self,
    ) -> RMcs6TxDiff14dBMcsOffsetW<'_, RegAxPwrSrMcs6TxdiffTable2Spec> {
        RMcs6TxDiff14dBMcsOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs6_tx_diff_15d_b_mcs_offset(
        &mut self,
    ) -> RMcs6TxDiff15dBMcsOffsetW<'_, RegAxPwrSrMcs6TxdiffTable2Spec> {
        RMcs6TxDiff15dBMcsOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_mcs6_txdiff_table2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_mcs6_txdiff_table2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrMcs6TxdiffTable2Spec;
impl crate::RegisterSpec for RegAxPwrSrMcs6TxdiffTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_mcs6_txdiff_table2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrMcs6TxdiffTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_mcs6_txdiff_table2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrMcs6TxdiffTable2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_MCS6_TXDIFF_TABLE2 to value 0"]
impl crate::Resettable for RegAxPwrSrMcs6TxdiffTable2Spec {}
