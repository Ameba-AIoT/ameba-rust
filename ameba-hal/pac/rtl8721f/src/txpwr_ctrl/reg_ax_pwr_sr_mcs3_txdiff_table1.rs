#[doc = "Register `REG_AX_PWR_SR_MCS3_TXDIFF_TABLE1` reader"]
pub type R = crate::R<RegAxPwrSrMcs3TxdiffTable1Spec>;
#[doc = "Register `REG_AX_PWR_SR_MCS3_TXDIFF_TABLE1` writer"]
pub type W = crate::W<RegAxPwrSrMcs3TxdiffTable1Spec>;
#[doc = "Field `r_MCS3_TxDiff_6dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs3TxDiff6dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS3_TxDiff_6dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs3TxDiff6dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS3_TxDiff_7dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs3TxDiff7dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS3_TxDiff_7dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs3TxDiff7dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS3_TxDiff_8dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs3TxDiff8dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS3_TxDiff_8dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs3TxDiff8dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS3_TxDiff_9dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs3TxDiff9dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS3_TxDiff_9dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs3TxDiff9dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS3_TxDiff_10dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs3TxDiff10dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS3_TxDiff_10dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs3TxDiff10dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs3_tx_diff_6d_b_mcs_offset(&self) -> RMcs3TxDiff6dBMcsOffsetR {
        RMcs3TxDiff6dBMcsOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs3_tx_diff_7d_b_mcs_offset(&self) -> RMcs3TxDiff7dBMcsOffsetR {
        RMcs3TxDiff7dBMcsOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs3_tx_diff_8d_b_mcs_offset(&self) -> RMcs3TxDiff8dBMcsOffsetR {
        RMcs3TxDiff8dBMcsOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs3_tx_diff_9d_b_mcs_offset(&self) -> RMcs3TxDiff9dBMcsOffsetR {
        RMcs3TxDiff9dBMcsOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs3_tx_diff_10d_b_mcs_offset(&self) -> RMcs3TxDiff10dBMcsOffsetR {
        RMcs3TxDiff10dBMcsOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs3_tx_diff_6d_b_mcs_offset(
        &mut self,
    ) -> RMcs3TxDiff6dBMcsOffsetW<'_, RegAxPwrSrMcs3TxdiffTable1Spec> {
        RMcs3TxDiff6dBMcsOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs3_tx_diff_7d_b_mcs_offset(
        &mut self,
    ) -> RMcs3TxDiff7dBMcsOffsetW<'_, RegAxPwrSrMcs3TxdiffTable1Spec> {
        RMcs3TxDiff7dBMcsOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs3_tx_diff_8d_b_mcs_offset(
        &mut self,
    ) -> RMcs3TxDiff8dBMcsOffsetW<'_, RegAxPwrSrMcs3TxdiffTable1Spec> {
        RMcs3TxDiff8dBMcsOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs3_tx_diff_9d_b_mcs_offset(
        &mut self,
    ) -> RMcs3TxDiff9dBMcsOffsetW<'_, RegAxPwrSrMcs3TxdiffTable1Spec> {
        RMcs3TxDiff9dBMcsOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs3_tx_diff_10d_b_mcs_offset(
        &mut self,
    ) -> RMcs3TxDiff10dBMcsOffsetW<'_, RegAxPwrSrMcs3TxdiffTable1Spec> {
        RMcs3TxDiff10dBMcsOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_mcs3_txdiff_table1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_mcs3_txdiff_table1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrMcs3TxdiffTable1Spec;
impl crate::RegisterSpec for RegAxPwrSrMcs3TxdiffTable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_mcs3_txdiff_table1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrMcs3TxdiffTable1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_mcs3_txdiff_table1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrMcs3TxdiffTable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_MCS3_TXDIFF_TABLE1 to value 0"]
impl crate::Resettable for RegAxPwrSrMcs3TxdiffTable1Spec {}
