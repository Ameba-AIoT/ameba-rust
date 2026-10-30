#[doc = "Register `ANAPAR_XTAL_MODE_DEC_ON_0` reader"]
pub type R = crate::R<AnaparXtalModeDecOn0Spec>;
#[doc = "Register `ANAPAR_XTAL_MODE_DEC_ON_0` writer"]
pub type W = crate::W<AnaparXtalModeDecOn0Spec>;
#[doc = "Field `reset_n_xmd` reader - Decoder digital circuit reset (low to high) Default 1&POR then assert reset_n. (not used)"]
pub type ResetNXmdR = crate::BitReader;
#[doc = "Field `reset_n_xmd` writer - Decoder digital circuit reset (low to high) Default 1&POR then assert reset_n. (not used)"]
pub type ResetNXmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_MODE` reader - 22C xtal mode define: {POW_XTAL, XTAL_MODE, POW_XTAL_LPS} \\[efuse for power on \\]"]
pub type XtalModeR = crate::BitReader;
#[doc = "Field `POW_XTAL_LPS` reader - POW_XTAL_LPS"]
pub type PowXtalLpsR = crate::BitReader;
#[doc = "Field `XTAL_MANU_SEL` reader - 1: enable xtal mode manual \\[efuse for power on \\]"]
pub type XtalManuSelR = crate::BitReader;
#[doc = "Field `XTAL_MANU_SEL` writer - 1: enable xtal mode manual \\[efuse for power on \\]"]
pub type XtalManuSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_MODE_MANUAL` reader - 01: LPS mode 10: Normal mode 11: high performance mode \\[efuse for power on \\]"]
pub type XtalModeManualR = crate::FieldReader;
#[doc = "Field `XTAL_MODE_MANUAL` writer - 01: LPS mode 10: Normal mode 11: high performance mode \\[efuse for power on \\]"]
pub type XtalModeManualW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_OV_UNIT` reader - overlap time settling cycles (4M CLK) 000: 10us 001: 20us 010: 40us 011: 80us 100: 160us 101: 320us 110: 640us 111: 1280us"]
pub type XtalOvUnitR = crate::FieldReader;
#[doc = "Field `XTAL_OV_UNIT` writer - overlap time settling cycles (4M CLK) 000: 10us 001: 20us 010: 40us 011: 80us 100: 160us 101: 320us 110: 640us 111: 1280us"]
pub type XtalOvUnitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XTAL_OV_RATIO` reader - overlap unit times (4M CLK) 00: 128us 01: 256us 10: 512us 11: 1024us"]
pub type XtalOvRatioR = crate::FieldReader;
#[doc = "Field `XTAL_OV_RATIO` writer - overlap unit times (4M CLK) 00: 128us 01: 256us 10: 512us 11: 1024us"]
pub type XtalOvRatioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_MD_LPOW` reader - XTAL mode decoder low power mode 1: gated input clock"]
pub type XtalMdLpowR = crate::BitReader;
#[doc = "Field `XTAL_MD_LPOW` writer - XTAL mode decoder low power mode 1: gated input clock"]
pub type XtalMdLpowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_LDO_OK` reader - (after start-up) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
pub type XtalLdoOkR = crate::FieldReader;
#[doc = "Field `XTAL_LDO_OK` writer - (after start-up) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
pub type XtalLdoOkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XTAL_WAIT_CYC` reader - Next state wait cycle, 83ns/unit"]
pub type XtalWaitCycR = crate::FieldReader;
#[doc = "Field `XTAL_WAIT_CYC` writer - Next state wait cycle, 83ns/unit"]
pub type XtalWaitCycW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `XTAL_LDO_LPS` reader - (after start-up, when entering LPS) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
pub type XtalLdoLpsR = crate::FieldReader;
#[doc = "Field `XTAL_LDO_LPS` writer - (after start-up, when entering LPS) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
pub type XtalLdoLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCAN_MODE_XMD` reader - 1:scan mode"]
pub type ScanModeXmdR = crate::BitReader;
#[doc = "Field `SCAN_MODE_XMD` writer - 1:scan mode"]
pub type ScanModeXmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMD_SEL` reader - 00:100K/01:500K/10:4M/11:12M Must be changed as the system frequency changes"]
pub type XmdSelR = crate::FieldReader;
#[doc = "Field `XMD_SEL` writer - 00:100K/01:500K/10:4M/11:12M Must be changed as the system frequency changes"]
pub type XmdSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_LPS_CAP_CYC` reader - 00:8u/01:16u/10:32u/11:64u How often a cap step is dropped"]
pub type XtalLpsCapCycR = crate::FieldReader;
#[doc = "Field `XTAL_LPS_CAP_CYC` writer - 00:8u/01:16u/10:32u/11:64u How often a cap step is dropped"]
pub type XtalLpsCapCycW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_LPS_CAP_STEP` reader - 00:1 step/01:2 step/10:4 step /11:8 step How many cap steps to drop at a time"]
pub type XtalLpsCapStepR = crate::FieldReader;
#[doc = "Field `XTAL_LPS_CAP_STEP` writer - 00:1 step/01:2 step/10:4 step /11:8 step How many cap steps to drop at a time"]
pub type XtalLpsCapStepW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Decoder digital circuit reset (low to high) Default 1&POR then assert reset_n. (not used)"]
    #[inline(always)]
    pub fn reset_n_xmd(&self) -> ResetNXmdR {
        ResetNXmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 22C xtal mode define: {POW_XTAL, XTAL_MODE, POW_XTAL_LPS} \\[efuse for power on \\]"]
    #[inline(always)]
    pub fn xtal_mode(&self) -> XtalModeR {
        XtalModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - POW_XTAL_LPS"]
    #[inline(always)]
    pub fn pow_xtal_lps(&self) -> PowXtalLpsR {
        PowXtalLpsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: enable xtal mode manual \\[efuse for power on \\]"]
    #[inline(always)]
    pub fn xtal_manu_sel(&self) -> XtalManuSelR {
        XtalManuSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 01: LPS mode 10: Normal mode 11: high performance mode \\[efuse for power on \\]"]
    #[inline(always)]
    pub fn xtal_mode_manual(&self) -> XtalModeManualR {
        XtalModeManualR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:9 - overlap time settling cycles (4M CLK) 000: 10us 001: 20us 010: 40us 011: 80us 100: 160us 101: 320us 110: 640us 111: 1280us"]
    #[inline(always)]
    pub fn xtal_ov_unit(&self) -> XtalOvUnitR {
        XtalOvUnitR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:11 - overlap unit times (4M CLK) 00: 128us 01: 256us 10: 512us 11: 1024us"]
    #[inline(always)]
    pub fn xtal_ov_ratio(&self) -> XtalOvRatioR {
        XtalOvRatioR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - XTAL mode decoder low power mode 1: gated input clock"]
    #[inline(always)]
    pub fn xtal_md_lpow(&self) -> XtalMdLpowR {
        XtalMdLpowR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - (after start-up) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
    #[inline(always)]
    pub fn xtal_ldo_ok(&self) -> XtalLdoOkR {
        XtalLdoOkR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - Next state wait cycle, 83ns/unit"]
    #[inline(always)]
    pub fn xtal_wait_cyc(&self) -> XtalWaitCycR {
        XtalWaitCycR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:24 - (after start-up, when entering LPS) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
    #[inline(always)]
    pub fn xtal_ldo_lps(&self) -> XtalLdoLpsR {
        XtalLdoLpsR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - 1:scan mode"]
    #[inline(always)]
    pub fn scan_mode_xmd(&self) -> ScanModeXmdR {
        ScanModeXmdR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - 00:100K/01:500K/10:4M/11:12M Must be changed as the system frequency changes"]
    #[inline(always)]
    pub fn xmd_sel(&self) -> XmdSelR {
        XmdSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - 00:8u/01:16u/10:32u/11:64u How often a cap step is dropped"]
    #[inline(always)]
    pub fn xtal_lps_cap_cyc(&self) -> XtalLpsCapCycR {
        XtalLpsCapCycR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 00:1 step/01:2 step/10:4 step /11:8 step How many cap steps to drop at a time"]
    #[inline(always)]
    pub fn xtal_lps_cap_step(&self) -> XtalLpsCapStepR {
        XtalLpsCapStepR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Decoder digital circuit reset (low to high) Default 1&POR then assert reset_n. (not used)"]
    #[inline(always)]
    pub fn reset_n_xmd(&mut self) -> ResetNXmdW<'_, AnaparXtalModeDecOn0Spec> {
        ResetNXmdW::new(self, 1)
    }
    #[doc = "Bit 4 - 1: enable xtal mode manual \\[efuse for power on \\]"]
    #[inline(always)]
    pub fn xtal_manu_sel(&mut self) -> XtalManuSelW<'_, AnaparXtalModeDecOn0Spec> {
        XtalManuSelW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 01: LPS mode 10: Normal mode 11: high performance mode \\[efuse for power on \\]"]
    #[inline(always)]
    pub fn xtal_mode_manual(&mut self) -> XtalModeManualW<'_, AnaparXtalModeDecOn0Spec> {
        XtalModeManualW::new(self, 5)
    }
    #[doc = "Bits 7:9 - overlap time settling cycles (4M CLK) 000: 10us 001: 20us 010: 40us 011: 80us 100: 160us 101: 320us 110: 640us 111: 1280us"]
    #[inline(always)]
    pub fn xtal_ov_unit(&mut self) -> XtalOvUnitW<'_, AnaparXtalModeDecOn0Spec> {
        XtalOvUnitW::new(self, 7)
    }
    #[doc = "Bits 10:11 - overlap unit times (4M CLK) 00: 128us 01: 256us 10: 512us 11: 1024us"]
    #[inline(always)]
    pub fn xtal_ov_ratio(&mut self) -> XtalOvRatioW<'_, AnaparXtalModeDecOn0Spec> {
        XtalOvRatioW::new(self, 10)
    }
    #[doc = "Bit 12 - XTAL mode decoder low power mode 1: gated input clock"]
    #[inline(always)]
    pub fn xtal_md_lpow(&mut self) -> XtalMdLpowW<'_, AnaparXtalModeDecOn0Spec> {
        XtalMdLpowW::new(self, 12)
    }
    #[doc = "Bits 13:15 - (after start-up) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
    #[inline(always)]
    pub fn xtal_ldo_ok(&mut self) -> XtalLdoOkW<'_, AnaparXtalModeDecOn0Spec> {
        XtalLdoOkW::new(self, 13)
    }
    #[doc = "Bits 16:21 - Next state wait cycle, 83ns/unit"]
    #[inline(always)]
    pub fn xtal_wait_cyc(&mut self) -> XtalWaitCycW<'_, AnaparXtalModeDecOn0Spec> {
        XtalWaitCycW::new(self, 16)
    }
    #[doc = "Bits 22:24 - (after start-up, when entering LPS) 111: 1.1V 110: 1.05V 101:1V 100:0.95V 011: 0.9V 010: 0.85V 001: 0.8V 000: 0.75V"]
    #[inline(always)]
    pub fn xtal_ldo_lps(&mut self) -> XtalLdoLpsW<'_, AnaparXtalModeDecOn0Spec> {
        XtalLdoLpsW::new(self, 22)
    }
    #[doc = "Bit 25 - 1:scan mode"]
    #[inline(always)]
    pub fn scan_mode_xmd(&mut self) -> ScanModeXmdW<'_, AnaparXtalModeDecOn0Spec> {
        ScanModeXmdW::new(self, 25)
    }
    #[doc = "Bits 26:27 - 00:100K/01:500K/10:4M/11:12M Must be changed as the system frequency changes"]
    #[inline(always)]
    pub fn xmd_sel(&mut self) -> XmdSelW<'_, AnaparXtalModeDecOn0Spec> {
        XmdSelW::new(self, 26)
    }
    #[doc = "Bits 28:29 - 00:8u/01:16u/10:32u/11:64u How often a cap step is dropped"]
    #[inline(always)]
    pub fn xtal_lps_cap_cyc(&mut self) -> XtalLpsCapCycW<'_, AnaparXtalModeDecOn0Spec> {
        XtalLpsCapCycW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 00:1 step/01:2 step/10:4 step /11:8 step How many cap steps to drop at a time"]
    #[inline(always)]
    pub fn xtal_lps_cap_step(&mut self) -> XtalLpsCapStepW<'_, AnaparXtalModeDecOn0Spec> {
        XtalLpsCapStepW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_mode_dec_on_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_mode_dec_on_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalModeDecOn0Spec;
impl crate::RegisterSpec for AnaparXtalModeDecOn0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_mode_dec_on_0::R`](R) reader structure"]
impl crate::Readable for AnaparXtalModeDecOn0Spec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_mode_dec_on_0::W`](W) writer structure"]
impl crate::Writable for AnaparXtalModeDecOn0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_MODE_DEC_ON_0 to value 0x9809_24e2"]
impl crate::Resettable for AnaparXtalModeDecOn0Spec {
    const RESET_VALUE: u32 = 0x9809_24e2;
}
