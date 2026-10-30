#[doc = "Register `REG_ISO` reader"]
pub type R = crate::R<RegIsoSpec>;
#[doc = "Register `REG_ISO` writer"]
pub type W = crate::W<RegIsoSpec>;
#[doc = "Field `iso_lpon` reader - When LDO core power on, PD_LPON is on. 1: ISO signal of LPON power domain 0: Release"]
pub type IsoLponR = crate::BitReader;
#[doc = "Field `iso_lpon` writer - When LDO core power on, PD_LPON is on. 1: ISO signal of LPON power domain 0: Release"]
pub type IsoLponW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iso_aon_otp` reader - 1: ISO signals from otp core power domain 0: Release"]
pub type IsoAonOtpR = crate::BitReader;
#[doc = "Field `iso_aon_otp` writer - 1: ISO signals from otp core power domain 0: Release"]
pub type IsoAonOtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iso_r2a` reader - 1: ISO signals from RTC to AON 0: Release"]
pub type IsoR2aR = crate::BitReader;
#[doc = "Field `iso_r2a` writer - 1: ISO signals from RTC to AON 0: Release"]
pub type IsoR2aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G0_PAD_SHDN_CORE` reader - 1: Enable G0 group pad 0: Shutdown PAD It will be deassert by aon_det_rtc automaticly"]
pub type G0PadShdnCoreR = crate::BitReader;
#[doc = "Field `G0_PAD_SHDN_CORE` writer - 1: Enable G0 group pad 0: Shutdown PAD It will be deassert by aon_det_rtc automaticly"]
pub type G0PadShdnCoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G1_PAD_SHDN_CORE` reader - 1: Enable G1 group pad 0: Shutdown PAD Note:default is shutdown, wait pmc enable MLDO power"]
pub type G1PadShdnCoreR = crate::BitReader;
#[doc = "Field `G1_PAD_SHDN_CORE` writer - 1: Enable G1 group pad 0: Shutdown PAD Note:default is shutdown, wait pmc enable MLDO power"]
pub type G1PadShdnCoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G2_PAD_SHDN_CORE` reader - 1: Enable G2 group pad 0: Shutdown PAD"]
pub type G2PadShdnCoreR = crate::BitReader;
#[doc = "Field `G2_PAD_SHDN_CORE` writer - 1: Enable G2 group pad 0: Shutdown PAD"]
pub type G2PadShdnCoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G3_PAD_SHDN_CORE` reader - 1: Enable G3 group pad 0: Shutdown PAD"]
pub type G3PadShdnCoreR = crate::BitReader;
#[doc = "Field `G3_PAD_SHDN_CORE` writer - 1: Enable G3 group pad 0: Shutdown PAD"]
pub type G3PadShdnCoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G4_PAD_SHDN_CORE` reader - 1: Enable G4 group pad 0: Shutdown PAD"]
pub type G4PadShdnCoreR = crate::BitReader;
#[doc = "Field `G4_PAD_SHDN_CORE` writer - 1: Enable G4 group pad 0: Shutdown PAD"]
pub type G4PadShdnCoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G5_PAD_SHDN_CORE` reader - 1: Enable G5 group pad 0: Shutdown PAD Note: This maybe nouse , for AON pad , IO power always on before core power."]
pub type G5PadShdnCoreR = crate::BitReader;
#[doc = "Field `G5_PAD_SHDN_CORE` writer - 1: Enable G5 group pad 0: Shutdown PAD Note: This maybe nouse , for AON pad , IO power always on before core power."]
pub type G5PadShdnCoreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When LDO core power on, PD_LPON is on. 1: ISO signal of LPON power domain 0: Release"]
    #[inline(always)]
    pub fn iso_lpon(&self) -> IsoLponR {
        IsoLponR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: ISO signals from otp core power domain 0: Release"]
    #[inline(always)]
    pub fn iso_aon_otp(&self) -> IsoAonOtpR {
        IsoAonOtpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: ISO signals from RTC to AON 0: Release"]
    #[inline(always)]
    pub fn iso_r2a(&self) -> IsoR2aR {
        IsoR2aR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Enable G0 group pad 0: Shutdown PAD It will be deassert by aon_det_rtc automaticly"]
    #[inline(always)]
    pub fn g0_pad_shdn_core(&self) -> G0PadShdnCoreR {
        G0PadShdnCoreR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Enable G1 group pad 0: Shutdown PAD Note:default is shutdown, wait pmc enable MLDO power"]
    #[inline(always)]
    pub fn g1_pad_shdn_core(&self) -> G1PadShdnCoreR {
        G1PadShdnCoreR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Enable G2 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g2_pad_shdn_core(&self) -> G2PadShdnCoreR {
        G2PadShdnCoreR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Enable G3 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g3_pad_shdn_core(&self) -> G3PadShdnCoreR {
        G3PadShdnCoreR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Enable G4 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g4_pad_shdn_core(&self) -> G4PadShdnCoreR {
        G4PadShdnCoreR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Enable G5 group pad 0: Shutdown PAD Note: This maybe nouse , for AON pad , IO power always on before core power."]
    #[inline(always)]
    pub fn g5_pad_shdn_core(&self) -> G5PadShdnCoreR {
        G5PadShdnCoreR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When LDO core power on, PD_LPON is on. 1: ISO signal of LPON power domain 0: Release"]
    #[inline(always)]
    pub fn iso_lpon(&mut self) -> IsoLponW<'_, RegIsoSpec> {
        IsoLponW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: ISO signals from otp core power domain 0: Release"]
    #[inline(always)]
    pub fn iso_aon_otp(&mut self) -> IsoAonOtpW<'_, RegIsoSpec> {
        IsoAonOtpW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: ISO signals from RTC to AON 0: Release"]
    #[inline(always)]
    pub fn iso_r2a(&mut self) -> IsoR2aW<'_, RegIsoSpec> {
        IsoR2aW::new(self, 2)
    }
    #[doc = "Bit 8 - 1: Enable G0 group pad 0: Shutdown PAD It will be deassert by aon_det_rtc automaticly"]
    #[inline(always)]
    pub fn g0_pad_shdn_core(&mut self) -> G0PadShdnCoreW<'_, RegIsoSpec> {
        G0PadShdnCoreW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Enable G1 group pad 0: Shutdown PAD Note:default is shutdown, wait pmc enable MLDO power"]
    #[inline(always)]
    pub fn g1_pad_shdn_core(&mut self) -> G1PadShdnCoreW<'_, RegIsoSpec> {
        G1PadShdnCoreW::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Enable G2 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g2_pad_shdn_core(&mut self) -> G2PadShdnCoreW<'_, RegIsoSpec> {
        G2PadShdnCoreW::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Enable G3 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g3_pad_shdn_core(&mut self) -> G3PadShdnCoreW<'_, RegIsoSpec> {
        G3PadShdnCoreW::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Enable G4 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g4_pad_shdn_core(&mut self) -> G4PadShdnCoreW<'_, RegIsoSpec> {
        G4PadShdnCoreW::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Enable G5 group pad 0: Shutdown PAD Note: This maybe nouse , for AON pad , IO power always on before core power."]
    #[inline(always)]
    pub fn g5_pad_shdn_core(&mut self) -> G5PadShdnCoreW<'_, RegIsoSpec> {
        G5PadShdnCoreW::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_iso::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_iso::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegIsoSpec;
impl crate::RegisterSpec for RegIsoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_iso::R`](R) reader structure"]
impl crate::Readable for RegIsoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_iso::W`](W) writer structure"]
impl crate::Writable for RegIsoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ISO to value 0x1f07"]
impl crate::Resettable for RegIsoSpec {
    const RESET_VALUE: u32 = 0x1f07;
}
