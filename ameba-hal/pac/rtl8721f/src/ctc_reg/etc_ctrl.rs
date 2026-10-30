#[doc = "Register `ETC_CTRL` reader"]
pub type R = crate::R<EtcCtrlSpec>;
#[doc = "Register `ETC_CTRL` writer"]
pub type W = crate::W<EtcCtrlSpec>;
#[doc = "Field `etc_func_ctrl` reader - Environmental cap tracking calibration function 0: Disable 1: Enable"]
pub type EtcFuncCtrlR = crate::BitReader;
#[doc = "Field `etc_func_ctrl` writer - Environmental cap tracking calibration function 0: Disable 1: Enable"]
pub type EtcFuncCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `etc_scan_interval` reader - ETC update interval between scan period (sleep time) for all channel Interval=(etc_scan_interval+1)*scan_period"]
pub type EtcScanIntervalR = crate::FieldReader;
#[doc = "Field `etc_scan_interval` writer - ETC update interval between scan period (sleep time) for all channel Interval=(etc_scan_interval+1)*scan_period"]
pub type EtcScanIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `baseline_wt_factor` reader - Baseline update weight factor (ETC_factor_th) for all channel ETC_factor_th=2^baseline_wt_factor"]
pub type BaselineWtFactorR = crate::FieldReader;
#[doc = "Field `baseline_wt_factor` writer - Baseline update weight factor (ETC_factor_th) for all channel ETC_factor_th=2^baseline_wt_factor"]
pub type BaselineWtFactorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `baseline_upd_step` reader - Baseline update step for all channel"]
pub type BaselineUpdStepR = crate::FieldReader;
#[doc = "Field `baseline_upd_step` writer - Baseline update step for all channel"]
pub type BaselineUpdStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Environmental cap tracking calibration function 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn etc_func_ctrl(&self) -> EtcFuncCtrlR {
        EtcFuncCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - ETC update interval between scan period (sleep time) for all channel Interval=(etc_scan_interval+1)*scan_period"]
    #[inline(always)]
    pub fn etc_scan_interval(&self) -> EtcScanIntervalR {
        EtcScanIntervalR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - Baseline update weight factor (ETC_factor_th) for all channel ETC_factor_th=2^baseline_wt_factor"]
    #[inline(always)]
    pub fn baseline_wt_factor(&self) -> BaselineWtFactorR {
        BaselineWtFactorR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Baseline update step for all channel"]
    #[inline(always)]
    pub fn baseline_upd_step(&self) -> BaselineUpdStepR {
        BaselineUpdStepR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Environmental cap tracking calibration function 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn etc_func_ctrl(&mut self) -> EtcFuncCtrlW<'_, EtcCtrlSpec> {
        EtcFuncCtrlW::new(self, 0)
    }
    #[doc = "Bits 1:7 - ETC update interval between scan period (sleep time) for all channel Interval=(etc_scan_interval+1)*scan_period"]
    #[inline(always)]
    pub fn etc_scan_interval(&mut self) -> EtcScanIntervalW<'_, EtcCtrlSpec> {
        EtcScanIntervalW::new(self, 1)
    }
    #[doc = "Bits 8:11 - Baseline update weight factor (ETC_factor_th) for all channel ETC_factor_th=2^baseline_wt_factor"]
    #[inline(always)]
    pub fn baseline_wt_factor(&mut self) -> BaselineWtFactorW<'_, EtcCtrlSpec> {
        BaselineWtFactorW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Baseline update step for all channel"]
    #[inline(always)]
    pub fn baseline_upd_step(&mut self) -> BaselineUpdStepW<'_, EtcCtrlSpec> {
        BaselineUpdStepW::new(self, 12)
    }
}
#[doc = "Environment Tracking Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtcCtrlSpec;
impl crate::RegisterSpec for EtcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etc_ctrl::R`](R) reader structure"]
impl crate::Readable for EtcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`etc_ctrl::W`](W) writer structure"]
impl crate::Writable for EtcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETC_CTRL to value 0x1204"]
impl crate::Resettable for EtcCtrlSpec {
    const RESET_VALUE: u32 = 0x1204;
}
