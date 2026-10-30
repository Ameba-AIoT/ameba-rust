#[doc = "Register `REG_SYSPMC_OPT` reader"]
pub type R = crate::R<RegSyspmcOptSpec>;
#[doc = "Register `REG_SYSPMC_OPT` writer"]
pub type W = crate::W<RegSyspmcOptSpec>;
#[doc = "Field `PST_SLEP_ESOC` reader - 1: power enable SOC platform when PMC enter into sleep mode (clock-gating) 0: power off SoC domain when PMC enter into sleep mode (power-gating)"]
pub type PstSlepEsocR = crate::BitReader;
#[doc = "Field `PST_SLEP_ESOC` writer - 1: power enable SOC platform when PMC enter into sleep mode (clock-gating) 0: power off SoC domain when PMC enter into sleep mode (power-gating)"]
pub type PstSlepEsocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_SLEP_ERCK` reader - 1: Enable lpon clock when PMC enter into sleep mode 0: Gate lpon clock when PMC enter into sleep mode"]
pub type PstSlepErckR = crate::BitReader;
#[doc = "Field `PST_SLEP_ERCK` writer - 1: Enable lpon clock when PMC enter into sleep mode 0: Gate lpon clock when PMC enter into sleep mode"]
pub type PstSlepErckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_SLEP_G5PD` reader - 1: G5 pad used for 1.8v IO power , and will shutdown when LDO3318 off in slep,, when dslp , it will always shutoff 0: G2 pad not used as 1.8v IO power"]
pub type PstSlepG5pdR = crate::BitReader;
#[doc = "Field `PST_SLEP_G5PD` writer - 1: G5 pad used for 1.8v IO power , and will shutdown when LDO3318 off in slep,, when dslp , it will always shutoff 0: G2 pad not used as 1.8v IO power"]
pub type PstSlepG5pdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_SLEP_G4PD` reader - 1: G4 pad used for 1.8v IO power , and will shutdown when LDO3318 off in slep, when dslp , it will always shutoff 0: G1 pad not used as 1.8v IO power"]
pub type PstSlepG4pdR = crate::BitReader;
#[doc = "Field `PST_SLEP_G4PD` writer - 1: G4 pad used for 1.8v IO power , and will shutdown when LDO3318 off in slep, when dslp , it will always shutoff 0: G1 pad not used as 1.8v IO power"]
pub type PstSlepG4pdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_NORM_SDML` reader - normal setting for system request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
pub type PstNormSdmlR = crate::BitReader;
#[doc = "Field `PST_NORM_SDML` writer - normal setting for system request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
pub type PstNormSdmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_NORM_BDML` reader - normal setting for BT request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
pub type PstNormBdmlR = crate::BitReader;
#[doc = "Field `PST_NORM_BDML` writer - normal setting for BT request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
pub type PstNormBdmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_NORM_WDML` reader - normal setting for WMAC request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
pub type PstNormWdmlR = crate::BitReader;
#[doc = "Field `PST_NORM_WDML` writer - normal setting for WMAC request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
pub type PstNormWdmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_SLEP_SDML` reader - normal setting for system request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
pub type PstSlepSdmlR = crate::BitReader;
#[doc = "Field `PST_SLEP_SDML` writer - normal setting for system request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
pub type PstSlepSdmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_SLEP_BDML` reader - normal setting for BT request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
pub type PstSlepBdmlR = crate::BitReader;
#[doc = "Field `PST_SLEP_BDML` writer - normal setting for BT request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
pub type PstSlepBdmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU1_IRQ_MASK` reader - 1.mask all CPU1 interrupt 0:release all CPU1 interrupt"]
pub type Cpu1IrqMaskR = crate::BitReader;
#[doc = "Field `CPU1_IRQ_MASK` writer - 1.mask all CPU1 interrupt 0:release all CPU1 interrupt"]
pub type Cpu1IrqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_G5_USE_MLDO` reader - 1: G5 pad use MLDO 1.8v IO power(psram only) 0: G5 pad use external power and can't shutdown by pmc"]
pub type PstG5UseMldoR = crate::BitReader;
#[doc = "Field `PST_G5_USE_MLDO` writer - 1: G5 pad use MLDO 1.8v IO power(psram only) 0: G5 pad use external power and can't shutdown by pmc"]
pub type PstG5UseMldoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_G4_USE_MLDO` reader - 1: G4 pad use MLDO 1.8v IO power 0: G4 pad use external power and can't shutdown by pmc"]
pub type PstG4UseMldoR = crate::BitReader;
#[doc = "Field `PST_G4_USE_MLDO` writer - 1: G4 pad use MLDO 1.8v IO power 0: G4 pad use external power and can't shutdown by pmc"]
pub type PstG4UseMldoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_CHG_BDML` reader - normal setting for BT request Core LDO dummy load selection 1: enable dummy load during BT PMC change to sleep mode / normal mode 0: disable dummy load during BT PMC change to sleep mode / normal mode"]
pub type PstChgBdmlR = crate::BitReader;
#[doc = "Field `PST_CHG_BDML` writer - normal setting for BT request Core LDO dummy load selection 1: enable dummy load during BT PMC change to sleep mode / normal mode 0: disable dummy load during BT PMC change to sleep mode / normal mode"]
pub type PstChgBdmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cke_xtal40m_slep` reader - when pmc is sleep status, this bit control u_ckg_xtal40m_slep 1:Enable xtal40m_slep 0:Disable Note:when pmc is not sleep, u_ckg_xtal40m_slep will be enabled whether value this bit is Remove to lp_pmc option register"]
pub type CkeXtal40mSlepR = crate::BitReader;
#[doc = "Field `cke_xtal40m_slep` writer - when pmc is sleep status, this bit control u_ckg_xtal40m_slep 1:Enable xtal40m_slep 0:Disable Note:when pmc is not sleep, u_ckg_xtal40m_slep will be enabled whether value this bit is Remove to lp_pmc option register"]
pub type CkeXtal40mSlepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: power enable SOC platform when PMC enter into sleep mode (clock-gating) 0: power off SoC domain when PMC enter into sleep mode (power-gating)"]
    #[inline(always)]
    pub fn pst_slep_esoc(&self) -> PstSlepEsocR {
        PstSlepEsocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Enable lpon clock when PMC enter into sleep mode 0: Gate lpon clock when PMC enter into sleep mode"]
    #[inline(always)]
    pub fn pst_slep_erck(&self) -> PstSlepErckR {
        PstSlepErckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: G5 pad used for 1.8v IO power , and will shutdown when LDO3318 off in slep,, when dslp , it will always shutoff 0: G2 pad not used as 1.8v IO power"]
    #[inline(always)]
    pub fn pst_slep_g5pd(&self) -> PstSlepG5pdR {
        PstSlepG5pdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: G4 pad used for 1.8v IO power , and will shutdown when LDO3318 off in slep, when dslp , it will always shutoff 0: G1 pad not used as 1.8v IO power"]
    #[inline(always)]
    pub fn pst_slep_g4pd(&self) -> PstSlepG4pdR {
        PstSlepG4pdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 18 - normal setting for system request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
    #[inline(always)]
    pub fn pst_norm_sdml(&self) -> PstNormSdmlR {
        PstNormSdmlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - normal setting for BT request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
    #[inline(always)]
    pub fn pst_norm_bdml(&self) -> PstNormBdmlR {
        PstNormBdmlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - normal setting for WMAC request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
    #[inline(always)]
    pub fn pst_norm_wdml(&self) -> PstNormWdmlR {
        PstNormWdmlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - normal setting for system request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
    #[inline(always)]
    pub fn pst_slep_sdml(&self) -> PstSlepSdmlR {
        PstSlepSdmlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - normal setting for BT request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
    #[inline(always)]
    pub fn pst_slep_bdml(&self) -> PstSlepBdmlR {
        PstSlepBdmlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1.mask all CPU1 interrupt 0:release all CPU1 interrupt"]
    #[inline(always)]
    pub fn cpu1_irq_mask(&self) -> Cpu1IrqMaskR {
        Cpu1IrqMaskR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: G5 pad use MLDO 1.8v IO power(psram only) 0: G5 pad use external power and can't shutdown by pmc"]
    #[inline(always)]
    pub fn pst_g5_use_mldo(&self) -> PstG5UseMldoR {
        PstG5UseMldoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: G4 pad use MLDO 1.8v IO power 0: G4 pad use external power and can't shutdown by pmc"]
    #[inline(always)]
    pub fn pst_g4_use_mldo(&self) -> PstG4UseMldoR {
        PstG4UseMldoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - normal setting for BT request Core LDO dummy load selection 1: enable dummy load during BT PMC change to sleep mode / normal mode 0: disable dummy load during BT PMC change to sleep mode / normal mode"]
    #[inline(always)]
    pub fn pst_chg_bdml(&self) -> PstChgBdmlR {
        PstChgBdmlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - when pmc is sleep status, this bit control u_ckg_xtal40m_slep 1:Enable xtal40m_slep 0:Disable Note:when pmc is not sleep, u_ckg_xtal40m_slep will be enabled whether value this bit is Remove to lp_pmc option register"]
    #[inline(always)]
    pub fn cke_xtal40m_slep(&self) -> CkeXtal40mSlepR {
        CkeXtal40mSlepR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: power enable SOC platform when PMC enter into sleep mode (clock-gating) 0: power off SoC domain when PMC enter into sleep mode (power-gating)"]
    #[inline(always)]
    pub fn pst_slep_esoc(&mut self) -> PstSlepEsocW<'_, RegSyspmcOptSpec> {
        PstSlepEsocW::new(self, 0)
    }
    #[doc = "Bit 3 - 1: Enable lpon clock when PMC enter into sleep mode 0: Gate lpon clock when PMC enter into sleep mode"]
    #[inline(always)]
    pub fn pst_slep_erck(&mut self) -> PstSlepErckW<'_, RegSyspmcOptSpec> {
        PstSlepErckW::new(self, 3)
    }
    #[doc = "Bit 6 - 1: G5 pad used for 1.8v IO power , and will shutdown when LDO3318 off in slep,, when dslp , it will always shutoff 0: G2 pad not used as 1.8v IO power"]
    #[inline(always)]
    pub fn pst_slep_g5pd(&mut self) -> PstSlepG5pdW<'_, RegSyspmcOptSpec> {
        PstSlepG5pdW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: G4 pad used for 1.8v IO power , and will shutdown when LDO3318 off in slep, when dslp , it will always shutoff 0: G1 pad not used as 1.8v IO power"]
    #[inline(always)]
    pub fn pst_slep_g4pd(&mut self) -> PstSlepG4pdW<'_, RegSyspmcOptSpec> {
        PstSlepG4pdW::new(self, 7)
    }
    #[doc = "Bit 18 - normal setting for system request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
    #[inline(always)]
    pub fn pst_norm_sdml(&mut self) -> PstNormSdmlW<'_, RegSyspmcOptSpec> {
        PstNormSdmlW::new(self, 18)
    }
    #[doc = "Bit 19 - normal setting for BT request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
    #[inline(always)]
    pub fn pst_norm_bdml(&mut self) -> PstNormBdmlW<'_, RegSyspmcOptSpec> {
        PstNormBdmlW::new(self, 19)
    }
    #[doc = "Bit 20 - normal setting for WMAC request Core LDO dummy load selection 1: enable dummy load when PMC enter into normal mode 0: disable dummy load when PMC enter into normal mode"]
    #[inline(always)]
    pub fn pst_norm_wdml(&mut self) -> PstNormWdmlW<'_, RegSyspmcOptSpec> {
        PstNormWdmlW::new(self, 20)
    }
    #[doc = "Bit 21 - normal setting for system request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
    #[inline(always)]
    pub fn pst_slep_sdml(&mut self) -> PstSlepSdmlW<'_, RegSyspmcOptSpec> {
        PstSlepSdmlW::new(self, 21)
    }
    #[doc = "Bit 22 - normal setting for BT request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
    #[inline(always)]
    pub fn pst_slep_bdml(&mut self) -> PstSlepBdmlW<'_, RegSyspmcOptSpec> {
        PstSlepBdmlW::new(self, 22)
    }
    #[doc = "Bit 23 - 1.mask all CPU1 interrupt 0:release all CPU1 interrupt"]
    #[inline(always)]
    pub fn cpu1_irq_mask(&mut self) -> Cpu1IrqMaskW<'_, RegSyspmcOptSpec> {
        Cpu1IrqMaskW::new(self, 23)
    }
    #[doc = "Bit 24 - 1: G5 pad use MLDO 1.8v IO power(psram only) 0: G5 pad use external power and can't shutdown by pmc"]
    #[inline(always)]
    pub fn pst_g5_use_mldo(&mut self) -> PstG5UseMldoW<'_, RegSyspmcOptSpec> {
        PstG5UseMldoW::new(self, 24)
    }
    #[doc = "Bit 25 - 1: G4 pad use MLDO 1.8v IO power 0: G4 pad use external power and can't shutdown by pmc"]
    #[inline(always)]
    pub fn pst_g4_use_mldo(&mut self) -> PstG4UseMldoW<'_, RegSyspmcOptSpec> {
        PstG4UseMldoW::new(self, 25)
    }
    #[doc = "Bit 26 - normal setting for BT request Core LDO dummy load selection 1: enable dummy load during BT PMC change to sleep mode / normal mode 0: disable dummy load during BT PMC change to sleep mode / normal mode"]
    #[inline(always)]
    pub fn pst_chg_bdml(&mut self) -> PstChgBdmlW<'_, RegSyspmcOptSpec> {
        PstChgBdmlW::new(self, 26)
    }
    #[doc = "Bit 28 - when pmc is sleep status, this bit control u_ckg_xtal40m_slep 1:Enable xtal40m_slep 0:Disable Note:when pmc is not sleep, u_ckg_xtal40m_slep will be enabled whether value this bit is Remove to lp_pmc option register"]
    #[inline(always)]
    pub fn cke_xtal40m_slep(&mut self) -> CkeXtal40mSlepW<'_, RegSyspmcOptSpec> {
        CkeXtal40mSlepW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_syspmc_opt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_syspmc_opt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSyspmcOptSpec;
impl crate::RegisterSpec for RegSyspmcOptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_syspmc_opt::R`](R) reader structure"]
impl crate::Readable for RegSyspmcOptSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_syspmc_opt::W`](W) writer structure"]
impl crate::Writable for RegSyspmcOptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SYSPMC_OPT to value 0x0700_00c0"]
impl crate::Resettable for RegSyspmcOptSpec {
    const RESET_VALUE: u32 = 0x0700_00c0;
}
