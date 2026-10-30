#[doc = "Register `REG_PWC` reader"]
pub type R = crate::R<RegPwcSpec>;
#[doc = "Register `REG_PWC` writer"]
pub type W = crate::W<RegPwcSpec>;
#[doc = "Field `pwc_aon_otp` reader - OTP power cut enable for core power, high active. 1:enable 0:disable"]
pub type PwcAonOtpR = crate::BitReader;
#[doc = "Field `pwc_aon_otp` writer - OTP power cut enable for core power, high active. 1:enable 0:disable"]
pub type PwcAonOtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `psw_out_aon_otp` reader - aon otp domain power switch status 1: power on 0: power off"]
pub type PswOutAonOtpR = crate::BitReader;
#[doc = "Field `pc_step` reader - power cut enable time step. 0: not valid 1: 10us 2: 20us 3: 30us"]
pub type PcStepR = crate::FieldReader;
#[doc = "Field `pc_step` writer - power cut enable time step. 0: not valid 1: 10us 2: 20us 3: 30us"]
pub type PcStepW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `G0_PAD_SHDN` reader - 1: Enable G0 group pad 0: Shutdown PAD"]
pub type G0PadShdnR = crate::BitReader;
#[doc = "Field `G0_PAD_SHDN` writer - 1: Enable G0 group pad 0: Shutdown PAD"]
pub type G0PadShdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G1_PAD_SHDN` reader - 1: Enable G1 group pad 0: Shutdown PAD"]
pub type G1PadShdnR = crate::BitReader;
#[doc = "Field `G1_PAD_SHDN` writer - 1: Enable G1 group pad 0: Shutdown PAD"]
pub type G1PadShdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G2_PAD_SHDN` reader - 1: Enable G2 group pad 0: Shutdown PAD"]
pub type G2PadShdnR = crate::BitReader;
#[doc = "Field `G2_PAD_SHDN` writer - 1: Enable G2 group pad 0: Shutdown PAD"]
pub type G2PadShdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G3_PAD_SHDN` reader - 1: Enable G3 group pad 0: Shutdown PAD"]
pub type G3PadShdnR = crate::BitReader;
#[doc = "Field `G3_PAD_SHDN` writer - 1: Enable G3 group pad 0: Shutdown PAD"]
pub type G3PadShdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G4_PAD_SHDN` reader - 1: Enable G4 group pad 0: Shutdown PAD"]
pub type G4PadShdnR = crate::BitReader;
#[doc = "Field `G4_PAD_SHDN` writer - 1: Enable G4 group pad 0: Shutdown PAD"]
pub type G4PadShdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G5_PAD_SHDN` reader - 1: Enable G5 group pad 0: Shutdown PAD Note: This is AON pad group, aon pmc will auto release this bit in power on init seq"]
pub type G5PadShdnR = crate::BitReader;
#[doc = "Field `G5_PAD_SHDN` writer - 1: Enable G5 group pad 0: Shutdown PAD Note: This is AON pad group, aon pmc will auto release this bit in power on init seq"]
pub type G5PadShdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G0_PAD_RSTB` reader - 1: Enable G0 group pad 0: Reset PAD"]
pub type G0PadRstbR = crate::BitReader;
#[doc = "Field `G0_PAD_RSTB` writer - 1: Enable G0 group pad 0: Reset PAD"]
pub type G0PadRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G1_PAD_RSTB` reader - 1: Enable G1 group pad 0: Reset PAD"]
pub type G1PadRstbR = crate::BitReader;
#[doc = "Field `G1_PAD_RSTB` writer - 1: Enable G1 group pad 0: Reset PAD"]
pub type G1PadRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G2_PAD_RSTB` reader - 1: Enable G2 group pad 0: Reset PAD"]
pub type G2PadRstbR = crate::BitReader;
#[doc = "Field `G2_PAD_RSTB` writer - 1: Enable G2 group pad 0: Reset PAD"]
pub type G2PadRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G3_PAD_RSTB` reader - 1: Enable G3 group pad 0: Reset PAD"]
pub type G3PadRstbR = crate::BitReader;
#[doc = "Field `G3_PAD_RSTB` writer - 1: Enable G3 group pad 0: Reset PAD"]
pub type G3PadRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G4_PAD_RSTB` reader - 1: Enable G4 group pad 0: Reset PAD"]
pub type G4PadRstbR = crate::BitReader;
#[doc = "Field `G4_PAD_RSTB` writer - 1: Enable G4 group pad 0: Reset PAD"]
pub type G4PadRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G5_PAD_RSTB` reader - 1: Enable G5 group pad 0: Reset PAD Note: This is AON pad group, direct connect to G5 pad, aon pmc will auto release this bit in power on init seq"]
pub type G5PadRstbR = crate::BitReader;
#[doc = "Field `G5_PAD_RSTB` writer - 1: Enable G5 group pad 0: Reset PAD Note: This is AON pad group, direct connect to G5 pad, aon pmc will auto release this bit in power on init seq"]
pub type G5PadRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPPAD_SHDN` reader - 1: Global Enable LP PAD (except G5) 0: Shutdown LP PAD Note: aon pmc will auto release this bit in power on init seq"]
pub type LppadShdnR = crate::BitReader;
#[doc = "Field `LPPAD_SHDN` writer - 1: Global Enable LP PAD (except G5) 0: Shutdown LP PAD Note: aon pmc will auto release this bit in power on init seq"]
pub type LppadShdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPPAD_RSTB` reader - 1: Release LP PAD RSTB33 (except G5) 0: Global Assert LP PAD RSTB33 Note: aon pmc will auto release this bit in power on init seq"]
pub type LppadRstbR = crate::BitReader;
#[doc = "Field `LPPAD_RSTB` writer - 1: Release LP PAD RSTB33 (except G5) 0: Global Assert LP PAD RSTB33 Note: aon pmc will auto release this bit in power on init seq"]
pub type LppadRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPPAD_SHDN_CORE` reader - 1: Release LP PAD SHDN CORE (except G5) 0: Global Assert LP PAD RSTB33 Note:G5 is aon pad, when all LP pad's IO power is ready , aon pmc will release this bit , only in power on init seq ."]
pub type LppadShdnCoreR = crate::BitReader;
#[doc = "Field `LPPAD_SHDN_CORE` writer - 1: Release LP PAD SHDN CORE (except G5) 0: Global Assert LP PAD RSTB33 Note:G5 is aon pad, when all LP pad's IO power is ready , aon pmc will release this bit , only in power on init seq ."]
pub type LppadShdnCoreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - OTP power cut enable for core power, high active. 1:enable 0:disable"]
    #[inline(always)]
    pub fn pwc_aon_otp(&self) -> PwcAonOtpR {
        PwcAonOtpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - aon otp domain power switch status 1: power on 0: power off"]
    #[inline(always)]
    pub fn psw_out_aon_otp(&self) -> PswOutAonOtpR {
        PswOutAonOtpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - power cut enable time step. 0: not valid 1: 10us 2: 20us 3: 30us"]
    #[inline(always)]
    pub fn pc_step(&self) -> PcStepR {
        PcStepR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - 1: Enable G0 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g0_pad_shdn(&self) -> G0PadShdnR {
        G0PadShdnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Enable G1 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g1_pad_shdn(&self) -> G1PadShdnR {
        G1PadShdnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Enable G2 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g2_pad_shdn(&self) -> G2PadShdnR {
        G2PadShdnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Enable G3 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g3_pad_shdn(&self) -> G3PadShdnR {
        G3PadShdnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Enable G4 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g4_pad_shdn(&self) -> G4PadShdnR {
        G4PadShdnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Enable G5 group pad 0: Shutdown PAD Note: This is AON pad group, aon pmc will auto release this bit in power on init seq"]
    #[inline(always)]
    pub fn g5_pad_shdn(&self) -> G5PadShdnR {
        G5PadShdnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Enable G0 group pad 0: Reset PAD"]
    #[inline(always)]
    pub fn g0_pad_rstb(&self) -> G0PadRstbR {
        G0PadRstbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Enable G1 group pad 0: Reset PAD"]
    #[inline(always)]
    pub fn g1_pad_rstb(&self) -> G1PadRstbR {
        G1PadRstbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Enable G2 group pad 0: Reset PAD"]
    #[inline(always)]
    pub fn g2_pad_rstb(&self) -> G2PadRstbR {
        G2PadRstbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Enable G3 group pad 0: Reset PAD"]
    #[inline(always)]
    pub fn g3_pad_rstb(&self) -> G3PadRstbR {
        G3PadRstbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Enable G4 group pad 0: Reset PAD"]
    #[inline(always)]
    pub fn g4_pad_rstb(&self) -> G4PadRstbR {
        G4PadRstbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Enable G5 group pad 0: Reset PAD Note: This is AON pad group, direct connect to G5 pad, aon pmc will auto release this bit in power on init seq"]
    #[inline(always)]
    pub fn g5_pad_rstb(&self) -> G5PadRstbR {
        G5PadRstbR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Global Enable LP PAD (except G5) 0: Shutdown LP PAD Note: aon pmc will auto release this bit in power on init seq"]
    #[inline(always)]
    pub fn lppad_shdn(&self) -> LppadShdnR {
        LppadShdnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Release LP PAD RSTB33 (except G5) 0: Global Assert LP PAD RSTB33 Note: aon pmc will auto release this bit in power on init seq"]
    #[inline(always)]
    pub fn lppad_rstb(&self) -> LppadRstbR {
        LppadRstbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Release LP PAD SHDN CORE (except G5) 0: Global Assert LP PAD RSTB33 Note:G5 is aon pad, when all LP pad's IO power is ready , aon pmc will release this bit , only in power on init seq ."]
    #[inline(always)]
    pub fn lppad_shdn_core(&self) -> LppadShdnCoreR {
        LppadShdnCoreR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - OTP power cut enable for core power, high active. 1:enable 0:disable"]
    #[inline(always)]
    pub fn pwc_aon_otp(&mut self) -> PwcAonOtpW<'_, RegPwcSpec> {
        PwcAonOtpW::new(self, 1)
    }
    #[doc = "Bits 4:5 - power cut enable time step. 0: not valid 1: 10us 2: 20us 3: 30us"]
    #[inline(always)]
    pub fn pc_step(&mut self) -> PcStepW<'_, RegPwcSpec> {
        PcStepW::new(self, 4)
    }
    #[doc = "Bit 8 - 1: Enable G0 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g0_pad_shdn(&mut self) -> G0PadShdnW<'_, RegPwcSpec> {
        G0PadShdnW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Enable G1 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g1_pad_shdn(&mut self) -> G1PadShdnW<'_, RegPwcSpec> {
        G1PadShdnW::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Enable G2 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g2_pad_shdn(&mut self) -> G2PadShdnW<'_, RegPwcSpec> {
        G2PadShdnW::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Enable G3 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g3_pad_shdn(&mut self) -> G3PadShdnW<'_, RegPwcSpec> {
        G3PadShdnW::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Enable G4 group pad 0: Shutdown PAD"]
    #[inline(always)]
    pub fn g4_pad_shdn(&mut self) -> G4PadShdnW<'_, RegPwcSpec> {
        G4PadShdnW::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Enable G5 group pad 0: Shutdown PAD Note: This is AON pad group, aon pmc will auto release this bit in power on init seq"]
    #[inline(always)]
    pub fn g5_pad_shdn(&mut self) -> G5PadShdnW<'_, RegPwcSpec> {
        G5PadShdnW::new(self, 13)
    }
    #[doc = "Bit 16 - 1: Enable G0 group pad 0: Reset PAD"]
    #[inline(always)]
    pub fn g0_pad_rstb(&mut self) -> G0PadRstbW<'_, RegPwcSpec> {
        G0PadRstbW::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Enable G1 group pad 0: Reset PAD"]
    #[inline(always)]
    pub fn g1_pad_rstb(&mut self) -> G1PadRstbW<'_, RegPwcSpec> {
        G1PadRstbW::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Enable G2 group pad 0: Reset PAD"]
    #[inline(always)]
    pub fn g2_pad_rstb(&mut self) -> G2PadRstbW<'_, RegPwcSpec> {
        G2PadRstbW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Enable G3 group pad 0: Reset PAD"]
    #[inline(always)]
    pub fn g3_pad_rstb(&mut self) -> G3PadRstbW<'_, RegPwcSpec> {
        G3PadRstbW::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Enable G4 group pad 0: Reset PAD"]
    #[inline(always)]
    pub fn g4_pad_rstb(&mut self) -> G4PadRstbW<'_, RegPwcSpec> {
        G4PadRstbW::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Enable G5 group pad 0: Reset PAD Note: This is AON pad group, direct connect to G5 pad, aon pmc will auto release this bit in power on init seq"]
    #[inline(always)]
    pub fn g5_pad_rstb(&mut self) -> G5PadRstbW<'_, RegPwcSpec> {
        G5PadRstbW::new(self, 21)
    }
    #[doc = "Bit 24 - 1: Global Enable LP PAD (except G5) 0: Shutdown LP PAD Note: aon pmc will auto release this bit in power on init seq"]
    #[inline(always)]
    pub fn lppad_shdn(&mut self) -> LppadShdnW<'_, RegPwcSpec> {
        LppadShdnW::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Release LP PAD RSTB33 (except G5) 0: Global Assert LP PAD RSTB33 Note: aon pmc will auto release this bit in power on init seq"]
    #[inline(always)]
    pub fn lppad_rstb(&mut self) -> LppadRstbW<'_, RegPwcSpec> {
        LppadRstbW::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Release LP PAD SHDN CORE (except G5) 0: Global Assert LP PAD RSTB33 Note:G5 is aon pad, when all LP pad's IO power is ready , aon pmc will release this bit , only in power on init seq ."]
    #[inline(always)]
    pub fn lppad_shdn_core(&mut self) -> LppadShdnCoreW<'_, RegPwcSpec> {
        LppadShdnCoreW::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPwcSpec;
impl crate::RegisterSpec for RegPwcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pwc::R`](R) reader structure"]
impl crate::Readable for RegPwcSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pwc::W`](W) writer structure"]
impl crate::Writable for RegPwcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PWC to value 0x043e_3e10"]
impl crate::Resettable for RegPwcSpec {
    const RESET_VALUE: u32 = 0x043e_3e10;
}
