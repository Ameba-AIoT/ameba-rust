#[doc = "Register `ANAPAR_XTAL_PDCK` reader"]
pub type R = crate::R<AnaparXtalPdckSpec>;
#[doc = "Register `ANAPAR_XTAL_PDCK` writer"]
pub type W = crate::W<AnaparXtalPdckSpec>;
#[doc = "Field `reset_n_pdck` reader - PDCK digital circuit reset (low to high),not used"]
pub type ResetNPdckR = crate::BitReader;
#[doc = "Field `reset_n_pdck` writer - PDCK digital circuit reset (low to high),not used"]
pub type ResetNPdckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_PDCK_DIGI` reader - Default set 0 due to POW_XTAL will 1 before do PDCK"]
pub type EnXtalPdckDigiR = crate::BitReader;
#[doc = "Field `EN_XTAL_PDCK_DIGI` writer - Default set 0 due to POW_XTAL will 1 before do PDCK"]
pub type EnXtalPdckDigiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCK_SEARCH_MODE` reader - 0: binary search, 1: full search from 31"]
pub type PdckSearchModeR = crate::BitReader;
#[doc = "Field `PDCK_SEARCH_MODE` writer - 0: binary search, 1: full search from 31"]
pub type PdckSearchModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCK_WAIT_CYC` reader - Enable PDCK waiting (4M CLK) 00: 64us 01:128us 10:256us 11:512us"]
pub type PdckWaitCycR = crate::FieldReader;
#[doc = "Field `PDCK_WAIT_CYC` writer - Enable PDCK waiting (4M CLK) 00: 64us 01:128us 10:256us 11:512us"]
pub type PdckWaitCycW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VREF_MANUAL` reader - VREF manual at EN_XTAL_PDCK_DIGI=0 When pdck done, it will auto set this value to pdck result"]
pub type VrefManualR = crate::FieldReader;
#[doc = "Field `VREF_MANUAL` writer - VREF manual at EN_XTAL_PDCK_DIGI=0 When pdck done, it will auto set this value to pdck result"]
pub type VrefManualW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VREF_INIT` reader - PDCK VREF initial setting"]
pub type VrefInitR = crate::FieldReader;
#[doc = "Field `VREF_INIT` writer - PDCK VREF initial setting"]
pub type VrefInitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `XTAL_PDCK_UNIT` reader - PDCK settling cycles (4M CLK) 00: 4us 01: 8us 10: 16us 11: 32us"]
pub type XtalPdckUnitR = crate::FieldReader;
#[doc = "Field `XTAL_PDCK_UNIT` writer - PDCK settling cycles (4M CLK) 00: 4us 01: 8us 10: 16us 11: 32us"]
pub type XtalPdckUnitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XPDCK_VREF_SEL` reader - PDCK calibrated result +offset For AAC robust +3"]
pub type XpdckVrefSelR = crate::FieldReader;
#[doc = "Field `XPDCK_VREF_SEL` writer - PDCK calibrated result +offset For AAC robust +3"]
pub type XpdckVrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PDCK_LPOW` reader - PDCK low power mode, 1:gating input clk Set to 1 when PDCK finished"]
pub type PdckLpowR = crate::BitReader;
#[doc = "Field `PDCK_LPOW` writer - PDCK low power mode, 1:gating input clk Set to 1 when PDCK finished"]
pub type PdckLpowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCK_STATE` reader - PDCK state for debug"]
pub type PdckStateR = crate::FieldReader;
impl R {
    #[doc = "Bit 1 - PDCK digital circuit reset (low to high),not used"]
    #[inline(always)]
    pub fn reset_n_pdck(&self) -> ResetNPdckR {
        ResetNPdckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Default set 0 due to POW_XTAL will 1 before do PDCK"]
    #[inline(always)]
    pub fn en_xtal_pdck_digi(&self) -> EnXtalPdckDigiR {
        EnXtalPdckDigiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: binary search, 1: full search from 31"]
    #[inline(always)]
    pub fn pdck_search_mode(&self) -> PdckSearchModeR {
        PdckSearchModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Enable PDCK waiting (4M CLK) 00: 64us 01:128us 10:256us 11:512us"]
    #[inline(always)]
    pub fn pdck_wait_cyc(&self) -> PdckWaitCycR {
        PdckWaitCycR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:10 - VREF manual at EN_XTAL_PDCK_DIGI=0 When pdck done, it will auto set this value to pdck result"]
    #[inline(always)]
    pub fn vref_manual(&self) -> VrefManualR {
        VrefManualR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PDCK VREF initial setting"]
    #[inline(always)]
    pub fn vref_init(&self) -> VrefInitR {
        VrefInitR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - PDCK settling cycles (4M CLK) 00: 4us 01: 8us 10: 16us 11: 32us"]
    #[inline(always)]
    pub fn xtal_pdck_unit(&self) -> XtalPdckUnitR {
        XtalPdckUnitR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - PDCK calibrated result +offset For AAC robust +3"]
    #[inline(always)]
    pub fn xpdck_vref_sel(&self) -> XpdckVrefSelR {
        XpdckVrefSelR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - PDCK low power mode, 1:gating input clk Set to 1 when PDCK finished"]
    #[inline(always)]
    pub fn pdck_lpow(&self) -> PdckLpowR {
        PdckLpowR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 28:31 - PDCK state for debug"]
    #[inline(always)]
    pub fn pdck_state(&self) -> PdckStateR {
        PdckStateR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - PDCK digital circuit reset (low to high),not used"]
    #[inline(always)]
    pub fn reset_n_pdck(&mut self) -> ResetNPdckW<'_, AnaparXtalPdckSpec> {
        ResetNPdckW::new(self, 1)
    }
    #[doc = "Bit 2 - Default set 0 due to POW_XTAL will 1 before do PDCK"]
    #[inline(always)]
    pub fn en_xtal_pdck_digi(&mut self) -> EnXtalPdckDigiW<'_, AnaparXtalPdckSpec> {
        EnXtalPdckDigiW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: binary search, 1: full search from 31"]
    #[inline(always)]
    pub fn pdck_search_mode(&mut self) -> PdckSearchModeW<'_, AnaparXtalPdckSpec> {
        PdckSearchModeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Enable PDCK waiting (4M CLK) 00: 64us 01:128us 10:256us 11:512us"]
    #[inline(always)]
    pub fn pdck_wait_cyc(&mut self) -> PdckWaitCycW<'_, AnaparXtalPdckSpec> {
        PdckWaitCycW::new(self, 4)
    }
    #[doc = "Bits 6:10 - VREF manual at EN_XTAL_PDCK_DIGI=0 When pdck done, it will auto set this value to pdck result"]
    #[inline(always)]
    pub fn vref_manual(&mut self) -> VrefManualW<'_, AnaparXtalPdckSpec> {
        VrefManualW::new(self, 6)
    }
    #[doc = "Bits 11:15 - PDCK VREF initial setting"]
    #[inline(always)]
    pub fn vref_init(&mut self) -> VrefInitW<'_, AnaparXtalPdckSpec> {
        VrefInitW::new(self, 11)
    }
    #[doc = "Bits 16:17 - PDCK settling cycles (4M CLK) 00: 4us 01: 8us 10: 16us 11: 32us"]
    #[inline(always)]
    pub fn xtal_pdck_unit(&mut self) -> XtalPdckUnitW<'_, AnaparXtalPdckSpec> {
        XtalPdckUnitW::new(self, 16)
    }
    #[doc = "Bits 18:22 - PDCK calibrated result +offset For AAC robust +3"]
    #[inline(always)]
    pub fn xpdck_vref_sel(&mut self) -> XpdckVrefSelW<'_, AnaparXtalPdckSpec> {
        XpdckVrefSelW::new(self, 18)
    }
    #[doc = "Bit 23 - PDCK low power mode, 1:gating input clk Set to 1 when PDCK finished"]
    #[inline(always)]
    pub fn pdck_lpow(&mut self) -> PdckLpowW<'_, AnaparXtalPdckSpec> {
        PdckLpowW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_pdck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_pdck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalPdckSpec;
impl crate::RegisterSpec for AnaparXtalPdckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_pdck::R`](R) reader structure"]
impl crate::Readable for AnaparXtalPdckSpec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_pdck::W`](W) writer structure"]
impl crate::Writable for AnaparXtalPdckSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_PDCK to value 0x0025_39d0"]
impl crate::Resettable for AnaparXtalPdckSpec {
    const RESET_VALUE: u32 = 0x0025_39d0;
}
