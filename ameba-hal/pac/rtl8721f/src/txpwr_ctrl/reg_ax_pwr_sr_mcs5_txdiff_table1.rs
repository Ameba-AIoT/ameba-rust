#[doc = "Register `REG_AX_PWR_SR_MCS5_TXDIFF_TABLE1` reader"]
pub type R = crate::R<RegAxPwrSrMcs5TxdiffTable1Spec>;
#[doc = "Register `REG_AX_PWR_SR_MCS5_TXDIFF_TABLE1` writer"]
pub type W = crate::W<RegAxPwrSrMcs5TxdiffTable1Spec>;
#[doc = "Field `r_MCS5_TxDiff_6dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs5TxDiff6dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS5_TxDiff_6dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs5TxDiff6dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS5_TxDiff_7dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs5TxDiff7dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS5_TxDiff_7dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs5TxDiff7dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS5_TxDiff_8dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs5TxDiff8dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS5_TxDiff_8dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs5TxDiff8dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS5_TxDiff_9dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs5TxDiff9dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS5_TxDiff_9dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs5TxDiff9dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS5_TxDiff_10dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs5TxDiff10dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS5_TxDiff_10dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs5TxDiff10dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs5_tx_diff_6d_b_mcs_offset(&self) -> RMcs5TxDiff6dBMcsOffsetR {
        RMcs5TxDiff6dBMcsOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs5_tx_diff_7d_b_mcs_offset(&self) -> RMcs5TxDiff7dBMcsOffsetR {
        RMcs5TxDiff7dBMcsOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs5_tx_diff_8d_b_mcs_offset(&self) -> RMcs5TxDiff8dBMcsOffsetR {
        RMcs5TxDiff8dBMcsOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs5_tx_diff_9d_b_mcs_offset(&self) -> RMcs5TxDiff9dBMcsOffsetR {
        RMcs5TxDiff9dBMcsOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs5_tx_diff_10d_b_mcs_offset(&self) -> RMcs5TxDiff10dBMcsOffsetR {
        RMcs5TxDiff10dBMcsOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs5_tx_diff_6d_b_mcs_offset(
        &mut self,
    ) -> RMcs5TxDiff6dBMcsOffsetW<'_, RegAxPwrSrMcs5TxdiffTable1Spec> {
        RMcs5TxDiff6dBMcsOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs5_tx_diff_7d_b_mcs_offset(
        &mut self,
    ) -> RMcs5TxDiff7dBMcsOffsetW<'_, RegAxPwrSrMcs5TxdiffTable1Spec> {
        RMcs5TxDiff7dBMcsOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs5_tx_diff_8d_b_mcs_offset(
        &mut self,
    ) -> RMcs5TxDiff8dBMcsOffsetW<'_, RegAxPwrSrMcs5TxdiffTable1Spec> {
        RMcs5TxDiff8dBMcsOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs5_tx_diff_9d_b_mcs_offset(
        &mut self,
    ) -> RMcs5TxDiff9dBMcsOffsetW<'_, RegAxPwrSrMcs5TxdiffTable1Spec> {
        RMcs5TxDiff9dBMcsOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs5_tx_diff_10d_b_mcs_offset(
        &mut self,
    ) -> RMcs5TxDiff10dBMcsOffsetW<'_, RegAxPwrSrMcs5TxdiffTable1Spec> {
        RMcs5TxDiff10dBMcsOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_mcs5_txdiff_table1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_mcs5_txdiff_table1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrMcs5TxdiffTable1Spec;
impl crate::RegisterSpec for RegAxPwrSrMcs5TxdiffTable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_mcs5_txdiff_table1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrMcs5TxdiffTable1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_mcs5_txdiff_table1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrMcs5TxdiffTable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_MCS5_TXDIFF_TABLE1 to value 0"]
impl crate::Resettable for RegAxPwrSrMcs5TxdiffTable1Spec {}
