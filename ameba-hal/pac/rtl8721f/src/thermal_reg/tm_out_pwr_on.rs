#[doc = "Register `TM_OUT_PWR_ON` reader"]
pub type R = crate::R<TmOutPwrOnSpec>;
#[doc = "Field `temp_out_poweron` reader - This bit defines output temperature when power on reset. * Bit18: Signed bit * Bit\\[17:10\\]: Integer part * Bit\\[9:0\\]: Decimal part"]
pub type TempOutPoweronR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:18 - This bit defines output temperature when power on reset. * Bit18: Signed bit * Bit\\[17:10\\]: Integer part * Bit\\[9:0\\]: Decimal part"]
    #[inline(always)]
    pub fn temp_out_poweron(&self) -> TempOutPoweronR {
        TempOutPoweronR::new(self.bits & 0x0007_ffff)
    }
}
#[doc = "Thermal Meter Power On Temperature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_out_pwr_on::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmOutPwrOnSpec;
impl crate::RegisterSpec for TmOutPwrOnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_out_pwr_on::R`](R) reader structure"]
impl crate::Readable for TmOutPwrOnSpec {}
#[doc = "`reset()` method sets TM_OUT_PWR_ON to value 0"]
impl crate::Resettable for TmOutPwrOnSpec {}
