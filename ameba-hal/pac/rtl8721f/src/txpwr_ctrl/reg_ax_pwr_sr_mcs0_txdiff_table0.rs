#[doc = "Register `REG_AX_PWR_SR_MCS0_TXDIFF_TABLE0` reader"]
pub type R = crate::R<RegAxPwrSrMcs0TxdiffTable0Spec>;
#[doc = "Register `REG_AX_PWR_SR_MCS0_TXDIFF_TABLE0` writer"]
pub type W = crate::W<RegAxPwrSrMcs0TxdiffTable0Spec>;
#[doc = "Field `r_MCS0_TxDiff_1dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs0TxDiff1dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS0_TxDiff_1dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs0TxDiff1dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS0_TxDiff_2dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs0TxDiff2dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS0_TxDiff_2dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs0TxDiff2dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS0_TxDiff_3dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs0TxDiff3dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS0_TxDiff_3dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs0TxDiff3dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS0_TxDiff_4dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs0TxDiff4dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS0_TxDiff_4dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs0TxDiff4dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS0_TxDiff_5dB_mcs_offset` reader - U(4.0) mcs0~11"]
pub type RMcs0TxDiff5dBMcsOffsetR = crate::FieldReader;
#[doc = "Field `r_MCS0_TxDiff_5dB_mcs_offset` writer - U(4.0) mcs0~11"]
pub type RMcs0TxDiff5dBMcsOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs0_tx_diff_1d_b_mcs_offset(&self) -> RMcs0TxDiff1dBMcsOffsetR {
        RMcs0TxDiff1dBMcsOffsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs0_tx_diff_2d_b_mcs_offset(&self) -> RMcs0TxDiff2dBMcsOffsetR {
        RMcs0TxDiff2dBMcsOffsetR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs0_tx_diff_3d_b_mcs_offset(&self) -> RMcs0TxDiff3dBMcsOffsetR {
        RMcs0TxDiff3dBMcsOffsetR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs0_tx_diff_4d_b_mcs_offset(&self) -> RMcs0TxDiff4dBMcsOffsetR {
        RMcs0TxDiff4dBMcsOffsetR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs0_tx_diff_5d_b_mcs_offset(&self) -> RMcs0TxDiff5dBMcsOffsetR {
        RMcs0TxDiff5dBMcsOffsetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs0_tx_diff_1d_b_mcs_offset(
        &mut self,
    ) -> RMcs0TxDiff1dBMcsOffsetW<'_, RegAxPwrSrMcs0TxdiffTable0Spec> {
        RMcs0TxDiff1dBMcsOffsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs0_tx_diff_2d_b_mcs_offset(
        &mut self,
    ) -> RMcs0TxDiff2dBMcsOffsetW<'_, RegAxPwrSrMcs0TxdiffTable0Spec> {
        RMcs0TxDiff2dBMcsOffsetW::new(self, 4)
    }
    #[doc = "Bits 8:11 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs0_tx_diff_3d_b_mcs_offset(
        &mut self,
    ) -> RMcs0TxDiff3dBMcsOffsetW<'_, RegAxPwrSrMcs0TxdiffTable0Spec> {
        RMcs0TxDiff3dBMcsOffsetW::new(self, 8)
    }
    #[doc = "Bits 12:15 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs0_tx_diff_4d_b_mcs_offset(
        &mut self,
    ) -> RMcs0TxDiff4dBMcsOffsetW<'_, RegAxPwrSrMcs0TxdiffTable0Spec> {
        RMcs0TxDiff4dBMcsOffsetW::new(self, 12)
    }
    #[doc = "Bits 16:19 - U(4.0) mcs0~11"]
    #[inline(always)]
    pub fn r_mcs0_tx_diff_5d_b_mcs_offset(
        &mut self,
    ) -> RMcs0TxDiff5dBMcsOffsetW<'_, RegAxPwrSrMcs0TxdiffTable0Spec> {
        RMcs0TxDiff5dBMcsOffsetW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_mcs0_txdiff_table0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_mcs0_txdiff_table0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrMcs0TxdiffTable0Spec;
impl crate::RegisterSpec for RegAxPwrSrMcs0TxdiffTable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_mcs0_txdiff_table0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrMcs0TxdiffTable0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_mcs0_txdiff_table0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrMcs0TxdiffTable0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_MCS0_TXDIFF_TABLE0 to value 0"]
impl crate::Resettable for RegAxPwrSrMcs0TxdiffTable0Spec {}
