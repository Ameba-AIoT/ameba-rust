#[doc = "Register `REG_AX_PWR_SR_MCS2_TXDIFF_TABLE3` reader"]
pub type R = crate::R<RegAxPwrSrMcs2TxdiffTable3Spec>;
#[doc = "Register `REG_AX_PWR_SR_MCS2_TXDIFF_TABLE3` writer"]
pub type W = crate::W<RegAxPwrSrMcs2TxdiffTable3Spec>;
#[doc = "Field `r_MCS2_TxDiff_16dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs2TxDiff16dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS2_TxDiff_16dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs2TxDiff16dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS2_TxDiff_17dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs2TxDiff17dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS2_TxDiff_17dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs2TxDiff17dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS2_TxDiff_18dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs2TxDiff18dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS2_TxDiff_18dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs2TxDiff18dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS2_TxDiff_19dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs2TxDiff19dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS2_TxDiff_19dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs2TxDiff19dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS2_TxDiff_20dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs2TxDiff20dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS2_TxDiff_20dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs2TxDiff20dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs2_tx_diff_16d_b_mcs_offset(&self) -> RMcs2TxDiff16dBMcsOffsetR {
        RMcs2TxDiff16dBMcsOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs2_tx_diff_17d_b_mcs_offset(&self) -> RMcs2TxDiff17dBMcsOffsetR {
        RMcs2TxDiff17dBMcsOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs2_tx_diff_18d_b_mcs_offset(&self) -> RMcs2TxDiff18dBMcsOffsetR {
        RMcs2TxDiff18dBMcsOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs2_tx_diff_19d_b_mcs_offset(&self) -> RMcs2TxDiff19dBMcsOffsetR {
        RMcs2TxDiff19dBMcsOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs2_tx_diff_20d_b_mcs_offset(&self) -> RMcs2TxDiff20dBMcsOffsetR {
        RMcs2TxDiff20dBMcsOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs2_tx_diff_16d_b_mcs_offset(
        &mut self,
    ) -> RMcs2TxDiff16dBMcsOffsetW<'_, RegAxPwrSrMcs2TxdiffTable3Spec> {
        RMcs2TxDiff16dBMcsOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs2_tx_diff_17d_b_mcs_offset(
        &mut self,
    ) -> RMcs2TxDiff17dBMcsOffsetW<'_, RegAxPwrSrMcs2TxdiffTable3Spec> {
        RMcs2TxDiff17dBMcsOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs2_tx_diff_18d_b_mcs_offset(
        &mut self,
    ) -> RMcs2TxDiff18dBMcsOffsetW<'_, RegAxPwrSrMcs2TxdiffTable3Spec> {
        RMcs2TxDiff18dBMcsOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs2_tx_diff_19d_b_mcs_offset(
        &mut self,
    ) -> RMcs2TxDiff19dBMcsOffsetW<'_, RegAxPwrSrMcs2TxdiffTable3Spec> {
        RMcs2TxDiff19dBMcsOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs2_tx_diff_20d_b_mcs_offset(
        &mut self,
    ) -> RMcs2TxDiff20dBMcsOffsetW<'_, RegAxPwrSrMcs2TxdiffTable3Spec> {
        RMcs2TxDiff20dBMcsOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_mcs2_txdiff_table3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_mcs2_txdiff_table3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrMcs2TxdiffTable3Spec;
impl crate::RegisterSpec for RegAxPwrSrMcs2TxdiffTable3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_mcs2_txdiff_table3::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrMcs2TxdiffTable3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_mcs2_txdiff_table3::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrMcs2TxdiffTable3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_MCS2_TXDIFF_TABLE3 to value 0"]
impl crate::Resettable for RegAxPwrSrMcs2TxdiffTable3Spec {}
