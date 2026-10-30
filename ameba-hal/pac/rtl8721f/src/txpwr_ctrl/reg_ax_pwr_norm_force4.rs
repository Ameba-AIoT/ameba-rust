#[doc = "Register `REG_AX_PWR_NORM_FORCE4` reader"]
pub type R = crate::R<RegAxPwrNormForce4Spec>;
#[doc = "Register `REG_AX_PWR_NORM_FORCE4` writer"]
pub type W = crate::W<RegAxPwrNormForce4Spec>;
#[doc = "Field `r_force_out_DPD_by_rate_off` reader - (RTL_only) Force output to MAC"]
pub type RForceOutDpdByRateOffR = crate::BitReader;
#[doc = "Field `r_force_out_DPD_by_rate_off` writer - (RTL_only) Force output to MAC"]
pub type RForceOutDpdByRateOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_DPD_by_rate_off_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutDpdByRateOffOnR = crate::BitReader;
#[doc = "Field `r_force_out_DPD_by_rate_off_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutDpdByRateOffOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_CFIR_by_rate_off` reader - (RTL_only) Force output to MAC"]
pub type RForceOutCfirByRateOffR = crate::BitReader;
#[doc = "Field `r_force_out_CFIR_by_rate_off` writer - (RTL_only) Force output to MAC"]
pub type RForceOutCfirByRateOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_CFIR_by_rate_off_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutCfirByRateOffOnR = crate::BitReader;
#[doc = "Field `r_force_out_CFIR_by_rate_off_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutCfirByRateOffOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_SR_pd_threhold` reader - (RTL_only) Force output to MAC"]
pub type RForceOutSrPdThreholdR = crate::FieldReader;
#[doc = "Field `r_force_out_SR_pd_threhold` writer - (RTL_only) Force output to MAC"]
pub type RForceOutSrPdThreholdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_force_out_SR_pd_threhold_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutSrPdThreholdOnR = crate::BitReader;
#[doc = "Field `r_force_out_SR_pd_threhold_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutSrPdThreholdOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_SR_mcs` reader - (RTL_only) Force output to MAC"]
pub type RForceOutSrMcsR = crate::FieldReader<u16>;
#[doc = "Field `r_force_out_SR_mcs` writer - (RTL_only) Force output to MAC"]
pub type RForceOutSrMcsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_force_out_SR_mcs_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutSrMcsOnR = crate::BitReader;
#[doc = "Field `r_force_out_SR_mcs_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutSrMcsOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_SR_dcm` reader - (RTL_only) Force output to MAC"]
pub type RForceOutSrDcmR = crate::BitReader;
#[doc = "Field `r_force_out_SR_dcm` writer - (RTL_only) Force output to MAC"]
pub type RForceOutSrDcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_SR_dcm_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutSrDcmOnR = crate::BitReader;
#[doc = "Field `r_force_out_SR_dcm_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutSrDcmOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BT_grant_rx_val` reader - (RTL_only) grant_rx flag=1 means coexist act"]
pub type RBtGrantRxValR = crate::BitReader;
#[doc = "Field `r_BT_grant_rx_val` writer - (RTL_only) grant_rx flag=1 means coexist act"]
pub type RBtGrantRxValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BT_grant_tx_val` reader - (RTL_only) grant_rx flag=1 means coexist act"]
pub type RBtGrantTxValR = crate::BitReader;
#[doc = "Field `r_BT_grant_tx_val` writer - (RTL_only) grant_rx flag=1 means coexist act"]
pub type RBtGrantTxValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BT_grant_wl_val` reader - (RTL_only) grant_wl flag=0 means coexist act"]
pub type RBtGrantWlValR = crate::BitReader;
#[doc = "Field `r_BT_grant_wl_val` writer - (RTL_only) grant_wl flag=0 means coexist act"]
pub type RBtGrantWlValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BT_grant_rx_force_off` reader - (RTL_only) Force off grant_rx"]
pub type RBtGrantRxForceOffR = crate::BitReader;
#[doc = "Field `r_BT_grant_rx_force_off` writer - (RTL_only) Force off grant_rx"]
pub type RBtGrantRxForceOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BT_grant_tx_force_off` reader - (RTL_only) Force off grant_tx"]
pub type RBtGrantTxForceOffR = crate::BitReader;
#[doc = "Field `r_BT_grant_tx_force_off` writer - (RTL_only) Force off grant_tx"]
pub type RBtGrantTxForceOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_BT_grant_wl_force_off` reader - (RTL_only) Force off grant_wl"]
pub type RBtGrantWlForceOffR = crate::BitReader;
#[doc = "Field `r_BT_grant_wl_force_off` writer - (RTL_only) Force off grant_wl"]
pub type RBtGrantWlForceOffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_dpd_by_rate_off(&self) -> RForceOutDpdByRateOffR {
        RForceOutDpdByRateOffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_dpd_by_rate_off_on(&self) -> RForceOutDpdByRateOffOnR {
        RForceOutDpdByRateOffOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_cfir_by_rate_off(&self) -> RForceOutCfirByRateOffR {
        RForceOutCfirByRateOffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_cfir_by_rate_off_on(&self) -> RForceOutCfirByRateOffOnR {
        RForceOutCfirByRateOffOnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_sr_pd_threhold(&self) -> RForceOutSrPdThreholdR {
        RForceOutSrPdThreholdR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_sr_pd_threhold_on(&self) -> RForceOutSrPdThreholdOnR {
        RForceOutSrPdThreholdOnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:22 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_sr_mcs(&self) -> RForceOutSrMcsR {
        RForceOutSrMcsR::new(((self.bits >> 11) & 0x0fff) as u16)
    }
    #[doc = "Bit 23 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_sr_mcs_on(&self) -> RForceOutSrMcsOnR {
        RForceOutSrMcsOnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_sr_dcm(&self) -> RForceOutSrDcmR {
        RForceOutSrDcmR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_sr_dcm_on(&self) -> RForceOutSrDcmOnR {
        RForceOutSrDcmOnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - (RTL_only) grant_rx flag=1 means coexist act"]
    #[inline(always)]
    pub fn r_bt_grant_rx_val(&self) -> RBtGrantRxValR {
        RBtGrantRxValR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - (RTL_only) grant_rx flag=1 means coexist act"]
    #[inline(always)]
    pub fn r_bt_grant_tx_val(&self) -> RBtGrantTxValR {
        RBtGrantTxValR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - (RTL_only) grant_wl flag=0 means coexist act"]
    #[inline(always)]
    pub fn r_bt_grant_wl_val(&self) -> RBtGrantWlValR {
        RBtGrantWlValR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - (RTL_only) Force off grant_rx"]
    #[inline(always)]
    pub fn r_bt_grant_rx_force_off(&self) -> RBtGrantRxForceOffR {
        RBtGrantRxForceOffR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - (RTL_only) Force off grant_tx"]
    #[inline(always)]
    pub fn r_bt_grant_tx_force_off(&self) -> RBtGrantTxForceOffR {
        RBtGrantTxForceOffR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - (RTL_only) Force off grant_wl"]
    #[inline(always)]
    pub fn r_bt_grant_wl_force_off(&self) -> RBtGrantWlForceOffR {
        RBtGrantWlForceOffR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_dpd_by_rate_off(
        &mut self,
    ) -> RForceOutDpdByRateOffW<'_, RegAxPwrNormForce4Spec> {
        RForceOutDpdByRateOffW::new(self, 0)
    }
    #[doc = "Bit 1 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_dpd_by_rate_off_on(
        &mut self,
    ) -> RForceOutDpdByRateOffOnW<'_, RegAxPwrNormForce4Spec> {
        RForceOutDpdByRateOffOnW::new(self, 1)
    }
    #[doc = "Bit 2 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_cfir_by_rate_off(
        &mut self,
    ) -> RForceOutCfirByRateOffW<'_, RegAxPwrNormForce4Spec> {
        RForceOutCfirByRateOffW::new(self, 2)
    }
    #[doc = "Bit 3 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_cfir_by_rate_off_on(
        &mut self,
    ) -> RForceOutCfirByRateOffOnW<'_, RegAxPwrNormForce4Spec> {
        RForceOutCfirByRateOffOnW::new(self, 3)
    }
    #[doc = "Bits 4:9 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_sr_pd_threhold(
        &mut self,
    ) -> RForceOutSrPdThreholdW<'_, RegAxPwrNormForce4Spec> {
        RForceOutSrPdThreholdW::new(self, 4)
    }
    #[doc = "Bit 10 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_sr_pd_threhold_on(
        &mut self,
    ) -> RForceOutSrPdThreholdOnW<'_, RegAxPwrNormForce4Spec> {
        RForceOutSrPdThreholdOnW::new(self, 10)
    }
    #[doc = "Bits 11:22 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_sr_mcs(&mut self) -> RForceOutSrMcsW<'_, RegAxPwrNormForce4Spec> {
        RForceOutSrMcsW::new(self, 11)
    }
    #[doc = "Bit 23 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_sr_mcs_on(&mut self) -> RForceOutSrMcsOnW<'_, RegAxPwrNormForce4Spec> {
        RForceOutSrMcsOnW::new(self, 23)
    }
    #[doc = "Bit 24 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_sr_dcm(&mut self) -> RForceOutSrDcmW<'_, RegAxPwrNormForce4Spec> {
        RForceOutSrDcmW::new(self, 24)
    }
    #[doc = "Bit 25 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_sr_dcm_on(&mut self) -> RForceOutSrDcmOnW<'_, RegAxPwrNormForce4Spec> {
        RForceOutSrDcmOnW::new(self, 25)
    }
    #[doc = "Bit 26 - (RTL_only) grant_rx flag=1 means coexist act"]
    #[inline(always)]
    pub fn r_bt_grant_rx_val(&mut self) -> RBtGrantRxValW<'_, RegAxPwrNormForce4Spec> {
        RBtGrantRxValW::new(self, 26)
    }
    #[doc = "Bit 27 - (RTL_only) grant_rx flag=1 means coexist act"]
    #[inline(always)]
    pub fn r_bt_grant_tx_val(&mut self) -> RBtGrantTxValW<'_, RegAxPwrNormForce4Spec> {
        RBtGrantTxValW::new(self, 27)
    }
    #[doc = "Bit 28 - (RTL_only) grant_wl flag=0 means coexist act"]
    #[inline(always)]
    pub fn r_bt_grant_wl_val(&mut self) -> RBtGrantWlValW<'_, RegAxPwrNormForce4Spec> {
        RBtGrantWlValW::new(self, 28)
    }
    #[doc = "Bit 29 - (RTL_only) Force off grant_rx"]
    #[inline(always)]
    pub fn r_bt_grant_rx_force_off(&mut self) -> RBtGrantRxForceOffW<'_, RegAxPwrNormForce4Spec> {
        RBtGrantRxForceOffW::new(self, 29)
    }
    #[doc = "Bit 30 - (RTL_only) Force off grant_tx"]
    #[inline(always)]
    pub fn r_bt_grant_tx_force_off(&mut self) -> RBtGrantTxForceOffW<'_, RegAxPwrNormForce4Spec> {
        RBtGrantTxForceOffW::new(self, 30)
    }
    #[doc = "Bit 31 - (RTL_only) Force off grant_wl"]
    #[inline(always)]
    pub fn r_bt_grant_wl_force_off(&mut self) -> RBtGrantWlForceOffW<'_, RegAxPwrNormForce4Spec> {
        RBtGrantWlForceOffW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_norm_force4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_norm_force4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrNormForce4Spec;
impl crate::RegisterSpec for RegAxPwrNormForce4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_norm_force4::R`](R) reader structure"]
impl crate::Readable for RegAxPwrNormForce4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_norm_force4::W`](W) writer structure"]
impl crate::Writable for RegAxPwrNormForce4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_NORM_FORCE4 to value 0x0c00_0000"]
impl crate::Resettable for RegAxPwrNormForce4Spec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
