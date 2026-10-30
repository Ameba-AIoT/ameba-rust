#[doc = "Register `ANAPAR_XTAL_MODE_DEC_ON_1` reader"]
pub type R = crate::R<AnaparXtalModeDecOn1Spec>;
#[doc = "Register `ANAPAR_XTAL_MODE_DEC_ON_1` writer"]
pub type W = crate::W<AnaparXtalModeDecOn1Spec>;
#[doc = "Field `XTAL_LPS_CAP_CTRL` reader - 1:cap step control"]
pub type XtalLpsCapCtrlR = crate::BitReader;
#[doc = "Field `XTAL_LPS_CAP_CTRL` writer - 1:cap step control"]
pub type XtalLpsCapCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_mode_dec` reader - for xmd debug 10:NM 01: LPS 11: HP"]
pub type XtalModeDecR = crate::FieldReader;
#[doc = "Field `xmd_state` reader - for xmd debug"]
pub type XmdStateR = crate::FieldReader;
#[doc = "Field `XTAL_OV_RATIO_FST` reader - overlap unit times (4M CLK), used for HP2NM or NM2HP 00: 128us 01: 256us 10: 512us 11: 1024us"]
pub type XtalOvRatioFstR = crate::FieldReader;
#[doc = "Field `XTAL_OV_RATIO_FST` writer - overlap unit times (4M CLK), used for HP2NM or NM2HP 00: 128us 01: 256us 10: 512us 11: 1024us"]
pub type XtalOvRatioFstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 1:cap step control"]
    #[inline(always)]
    pub fn xtal_lps_cap_ctrl(&self) -> XtalLpsCapCtrlR {
        XtalLpsCapCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - for xmd debug 10:NM 01: LPS 11: HP"]
    #[inline(always)]
    pub fn xtal_mode_dec(&self) -> XtalModeDecR {
        XtalModeDecR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:20 - for xmd debug"]
    #[inline(always)]
    pub fn xmd_state(&self) -> XmdStateR {
        XmdStateR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 30:31 - overlap unit times (4M CLK), used for HP2NM or NM2HP 00: 128us 01: 256us 10: 512us 11: 1024us"]
    #[inline(always)]
    pub fn xtal_ov_ratio_fst(&self) -> XtalOvRatioFstR {
        XtalOvRatioFstR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1:cap step control"]
    #[inline(always)]
    pub fn xtal_lps_cap_ctrl(&mut self) -> XtalLpsCapCtrlW<'_, AnaparXtalModeDecOn1Spec> {
        XtalLpsCapCtrlW::new(self, 0)
    }
    #[doc = "Bits 30:31 - overlap unit times (4M CLK), used for HP2NM or NM2HP 00: 128us 01: 256us 10: 512us 11: 1024us"]
    #[inline(always)]
    pub fn xtal_ov_ratio_fst(&mut self) -> XtalOvRatioFstW<'_, AnaparXtalModeDecOn1Spec> {
        XtalOvRatioFstW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_mode_dec_on_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_mode_dec_on_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalModeDecOn1Spec;
impl crate::RegisterSpec for AnaparXtalModeDecOn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_mode_dec_on_1::R`](R) reader structure"]
impl crate::Readable for AnaparXtalModeDecOn1Spec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_mode_dec_on_1::W`](W) writer structure"]
impl crate::Writable for AnaparXtalModeDecOn1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_MODE_DEC_ON_1 to value 0x01"]
impl crate::Resettable for AnaparXtalModeDecOn1Spec {
    const RESET_VALUE: u32 = 0x01;
}
