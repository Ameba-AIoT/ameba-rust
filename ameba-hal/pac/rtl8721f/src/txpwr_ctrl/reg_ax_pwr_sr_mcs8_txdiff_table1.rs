#[doc = "Register `REG_AX_PWR_SR_MCS8_TXDIFF_TABLE1` reader"]
pub type R = crate::R<RegAxPwrSrMcs8TxdiffTable1Spec>;
#[doc = "Register `REG_AX_PWR_SR_MCS8_TXDIFF_TABLE1` writer"]
pub type W = crate::W<RegAxPwrSrMcs8TxdiffTable1Spec>;
#[doc = "Field `r_MCS8_TxDiff_6dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs8TxDiff6dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS8_TxDiff_6dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs8TxDiff6dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS8_TxDiff_7dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs8TxDiff7dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS8_TxDiff_7dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs8TxDiff7dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS8_TxDiff_8dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs8TxDiff8dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS8_TxDiff_8dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs8TxDiff8dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS8_TxDiff_9dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs8TxDiff9dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS8_TxDiff_9dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs8TxDiff9dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS8_TxDiff_10dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs8TxDiff10dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS8_TxDiff_10dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs8TxDiff10dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_6d_b_mcs_offset(&self) -> RMcs8TxDiff6dBMcsOffsetR {
        RMcs8TxDiff6dBMcsOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_7d_b_mcs_offset(&self) -> RMcs8TxDiff7dBMcsOffsetR {
        RMcs8TxDiff7dBMcsOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_8d_b_mcs_offset(&self) -> RMcs8TxDiff8dBMcsOffsetR {
        RMcs8TxDiff8dBMcsOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_9d_b_mcs_offset(&self) -> RMcs8TxDiff9dBMcsOffsetR {
        RMcs8TxDiff9dBMcsOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_10d_b_mcs_offset(&self) -> RMcs8TxDiff10dBMcsOffsetR {
        RMcs8TxDiff10dBMcsOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_6d_b_mcs_offset(
        &mut self,
    ) -> RMcs8TxDiff6dBMcsOffsetW<'_, RegAxPwrSrMcs8TxdiffTable1Spec> {
        RMcs8TxDiff6dBMcsOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_7d_b_mcs_offset(
        &mut self,
    ) -> RMcs8TxDiff7dBMcsOffsetW<'_, RegAxPwrSrMcs8TxdiffTable1Spec> {
        RMcs8TxDiff7dBMcsOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_8d_b_mcs_offset(
        &mut self,
    ) -> RMcs8TxDiff8dBMcsOffsetW<'_, RegAxPwrSrMcs8TxdiffTable1Spec> {
        RMcs8TxDiff8dBMcsOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_9d_b_mcs_offset(
        &mut self,
    ) -> RMcs8TxDiff9dBMcsOffsetW<'_, RegAxPwrSrMcs8TxdiffTable1Spec> {
        RMcs8TxDiff9dBMcsOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs8_tx_diff_10d_b_mcs_offset(
        &mut self,
    ) -> RMcs8TxDiff10dBMcsOffsetW<'_, RegAxPwrSrMcs8TxdiffTable1Spec> {
        RMcs8TxDiff10dBMcsOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_mcs8_txdiff_table1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_mcs8_txdiff_table1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrMcs8TxdiffTable1Spec;
impl crate::RegisterSpec for RegAxPwrSrMcs8TxdiffTable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_mcs8_txdiff_table1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrMcs8TxdiffTable1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_mcs8_txdiff_table1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrMcs8TxdiffTable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_MCS8_TXDIFF_TABLE1 to value 0"]
impl crate::Resettable for RegAxPwrSrMcs8TxdiffTable1Spec {}
