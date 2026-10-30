#[doc = "Register `TM_TH_CTRL` reader"]
pub type R = crate::R<TmThCtrlSpec>;
#[doc = "Register `TM_TH_CTRL` writer"]
pub type W = crate::W<TmThCtrlSpec>;
#[doc = "Field `tm_low_wt_thr` reader - Set the Low threshold for comparison with TEMP_OUT. * Bit8: Signed bit * Bit\\[7:0\\]: Integer part"]
pub type TmLowWtThrR = crate::FieldReader<u16>;
#[doc = "Field `tm_low_wt_thr` writer - Set the Low threshold for comparison with TEMP_OUT. * Bit8: Signed bit * Bit\\[7:0\\]: Integer part"]
pub type TmLowWtThrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `tm_lowcmp_wt_en` reader - To enable compare TM_OUT with set Low threshold: When tm_out\\[18:10\\] <= tm_low_thr, it will set Interrupt pending flag ISR_TM_LOW. * 1: Enable the compare * 0: Disable the compare"]
pub type TmLowcmpWtEnR = crate::BitReader;
#[doc = "Field `tm_lowcmp_wt_en` writer - To enable compare TM_OUT with set Low threshold: When tm_out\\[18:10\\] <= tm_low_thr, it will set Interrupt pending flag ISR_TM_LOW. * 1: Enable the compare * 0: Disable the compare"]
pub type TmLowcmpWtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_high_wt_thr` reader - Set the over temperature warning threshold for comparison with TEMP_OUT.Only values greater than 0 are supported * Bit18: Signed bit * Bit\\[17:10\\]: Integer part"]
pub type TmHighWtThrR = crate::FieldReader<u16>;
#[doc = "Field `tm_high_wt_thr` writer - Set the over temperature warning threshold for comparison with TEMP_OUT.Only values greater than 0 are supported * Bit18: Signed bit * Bit\\[17:10\\]: Integer part"]
pub type TmHighWtThrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `tm_highcmp_wt_en` reader - To enable compare tm_out with over temperature warning threshold: When tm_high_thr <= tm_out\\[18:10\\] < tm_high_pt_thr, it will set Interrupt pending flag ISR_TM_HIGH. * 1: Enable the compare; * 0: Disable the compare."]
pub type TmHighcmpWtEnR = crate::BitReader;
#[doc = "Field `tm_highcmp_wt_en` writer - To enable compare tm_out with over temperature warning threshold: When tm_high_thr <= tm_out\\[18:10\\] < tm_high_pt_thr, it will set Interrupt pending flag ISR_TM_HIGH. * 1: Enable the compare; * 0: Disable the compare."]
pub type TmHighcmpWtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tm_high_pt_thr` reader - Set over temperature protection threshold for comparison with TEMP_OUT. Only between 0x046 (70centigrade) and 0x08C (140centigrade) are valid. * Bit28: Signed bit * Bit\\[27:20\\]: Integer part"]
pub type TmHighPtThrR = crate::FieldReader<u16>;
#[doc = "Field `tm_high_pt_thr` writer - Set over temperature protection threshold for comparison with TEMP_OUT. Only between 0x046 (70centigrade) and 0x08C (140centigrade) are valid. * Bit28: Signed bit * Bit\\[27:20\\]: Integer part"]
pub type TmHighPtThrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `tm_highcmp_pt_en` reader - To enable compare tm_out with over temperature protect threshold: When tm_out\\[18:10\\] > tm_high_pt_thr, it will set aon reset. * 1: Enable the compare; * 0: Disable the compare."]
pub type TmHighcmpPtEnR = crate::BitReader;
#[doc = "Field `tm_highcmp_pt_en` writer - To enable compare tm_out with over temperature protect threshold: When tm_out\\[18:10\\] > tm_high_pt_thr, it will set aon reset. * 1: Enable the compare; * 0: Disable the compare."]
pub type TmHighcmpPtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iso_thm` reader - 1: ISO signals from thermal power domain 0: Release"]
pub type IsoThmR = crate::BitReader;
#[doc = "Field `iso_thm` writer - 1: ISO signals from thermal power domain 0: Release"]
pub type IsoThmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Set the Low threshold for comparison with TEMP_OUT. * Bit8: Signed bit * Bit\\[7:0\\]: Integer part"]
    #[inline(always)]
    pub fn tm_low_wt_thr(&self) -> TmLowWtThrR {
        TmLowWtThrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - To enable compare TM_OUT with set Low threshold: When tm_out\\[18:10\\] <= tm_low_thr, it will set Interrupt pending flag ISR_TM_LOW. * 1: Enable the compare * 0: Disable the compare"]
    #[inline(always)]
    pub fn tm_lowcmp_wt_en(&self) -> TmLowcmpWtEnR {
        TmLowcmpWtEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:18 - Set the over temperature warning threshold for comparison with TEMP_OUT.Only values greater than 0 are supported * Bit18: Signed bit * Bit\\[17:10\\]: Integer part"]
    #[inline(always)]
    pub fn tm_high_wt_thr(&self) -> TmHighWtThrR {
        TmHighWtThrR::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bit 19 - To enable compare tm_out with over temperature warning threshold: When tm_high_thr <= tm_out\\[18:10\\] < tm_high_pt_thr, it will set Interrupt pending flag ISR_TM_HIGH. * 1: Enable the compare; * 0: Disable the compare."]
    #[inline(always)]
    pub fn tm_highcmp_wt_en(&self) -> TmHighcmpWtEnR {
        TmHighcmpWtEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:28 - Set over temperature protection threshold for comparison with TEMP_OUT. Only between 0x046 (70centigrade) and 0x08C (140centigrade) are valid. * Bit28: Signed bit * Bit\\[27:20\\]: Integer part"]
    #[inline(always)]
    pub fn tm_high_pt_thr(&self) -> TmHighPtThrR {
        TmHighPtThrR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - To enable compare tm_out with over temperature protect threshold: When tm_out\\[18:10\\] > tm_high_pt_thr, it will set aon reset. * 1: Enable the compare; * 0: Disable the compare."]
    #[inline(always)]
    pub fn tm_highcmp_pt_en(&self) -> TmHighcmpPtEnR {
        TmHighcmpPtEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: ISO signals from thermal power domain 0: Release"]
    #[inline(always)]
    pub fn iso_thm(&self) -> IsoThmR {
        IsoThmR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Set the Low threshold for comparison with TEMP_OUT. * Bit8: Signed bit * Bit\\[7:0\\]: Integer part"]
    #[inline(always)]
    pub fn tm_low_wt_thr(&mut self) -> TmLowWtThrW<'_, TmThCtrlSpec> {
        TmLowWtThrW::new(self, 0)
    }
    #[doc = "Bit 9 - To enable compare TM_OUT with set Low threshold: When tm_out\\[18:10\\] <= tm_low_thr, it will set Interrupt pending flag ISR_TM_LOW. * 1: Enable the compare * 0: Disable the compare"]
    #[inline(always)]
    pub fn tm_lowcmp_wt_en(&mut self) -> TmLowcmpWtEnW<'_, TmThCtrlSpec> {
        TmLowcmpWtEnW::new(self, 9)
    }
    #[doc = "Bits 10:18 - Set the over temperature warning threshold for comparison with TEMP_OUT.Only values greater than 0 are supported * Bit18: Signed bit * Bit\\[17:10\\]: Integer part"]
    #[inline(always)]
    pub fn tm_high_wt_thr(&mut self) -> TmHighWtThrW<'_, TmThCtrlSpec> {
        TmHighWtThrW::new(self, 10)
    }
    #[doc = "Bit 19 - To enable compare tm_out with over temperature warning threshold: When tm_high_thr <= tm_out\\[18:10\\] < tm_high_pt_thr, it will set Interrupt pending flag ISR_TM_HIGH. * 1: Enable the compare; * 0: Disable the compare."]
    #[inline(always)]
    pub fn tm_highcmp_wt_en(&mut self) -> TmHighcmpWtEnW<'_, TmThCtrlSpec> {
        TmHighcmpWtEnW::new(self, 19)
    }
    #[doc = "Bits 20:28 - Set over temperature protection threshold for comparison with TEMP_OUT. Only between 0x046 (70centigrade) and 0x08C (140centigrade) are valid. * Bit28: Signed bit * Bit\\[27:20\\]: Integer part"]
    #[inline(always)]
    pub fn tm_high_pt_thr(&mut self) -> TmHighPtThrW<'_, TmThCtrlSpec> {
        TmHighPtThrW::new(self, 20)
    }
    #[doc = "Bit 29 - To enable compare tm_out with over temperature protect threshold: When tm_out\\[18:10\\] > tm_high_pt_thr, it will set aon reset. * 1: Enable the compare; * 0: Disable the compare."]
    #[inline(always)]
    pub fn tm_highcmp_pt_en(&mut self) -> TmHighcmpPtEnW<'_, TmThCtrlSpec> {
        TmHighcmpPtEnW::new(self, 29)
    }
    #[doc = "Bit 30 - 1: ISO signals from thermal power domain 0: Release"]
    #[inline(always)]
    pub fn iso_thm(&mut self) -> IsoThmW<'_, TmThCtrlSpec> {
        IsoThmW::new(self, 30)
    }
}
#[doc = "Thermal Meter Threshold Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_th_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_th_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmThCtrlSpec;
impl crate::RegisterSpec for TmThCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_th_ctrl::R`](R) reader structure"]
impl crate::Readable for TmThCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tm_th_ctrl::W`](W) writer structure"]
impl crate::Writable for TmThCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TM_TH_CTRL to value 0x07d1_a5d8"]
impl crate::Resettable for TmThCtrlSpec {
    const RESET_VALUE: u32 = 0x07d1_a5d8;
}
