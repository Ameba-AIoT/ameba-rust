#[doc = "Register `REG_SYSPMC_CTRL` reader"]
pub type R = crate::R<RegSyspmcCtrlSpec>;
#[doc = "Register `REG_SYSPMC_CTRL` writer"]
pub type W = crate::W<RegSyspmcCtrlSpec>;
#[doc = "Field `PMC_PMEN_DSLP` reader - 1: Request PMC to enter deep-sleep state, auto clear to 0 after PMC finishes"]
pub type PmcPmenDslpR = crate::BitReader;
#[doc = "Field `PMC_PMEN_DSLP` writer - 1: Request PMC to enter deep-sleep state, auto clear to 0 after PMC finishes"]
pub type PmcPmenDslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMC_PMEN_SLEP` reader - 1: Request PMC to enter sleep state, auto clear to 0 after PMC finishes"]
pub type PmcPmenSlepR = crate::BitReader;
#[doc = "Field `PMC_PMEN_SLEP` writer - 1: Request PMC to enter sleep state, auto clear to 0 after PMC finishes"]
pub type PmcPmenSlepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU0_IRQ_MASK` reader - 1.mask all KM4 interrupt 0:release all KM4 interrupt"]
pub type Cpu0IrqMaskR = crate::BitReader;
#[doc = "Field `CPU0_IRQ_MASK` writer - 1.mask all KM4 interrupt 0:release all KM4 interrupt"]
pub type Cpu0IrqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PST_SLEP_WDML` reader - normal setting for WMAC request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
pub type PstSlepWdmlR = crate::BitReader;
#[doc = "Field `PST_SLEP_WDML` writer - normal setting for WMAC request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
pub type PstSlepWdmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMC_POF_WAIT_SLEP` reader - 1.when SW set pmen_slep or pmen_dslp , hw pmc will wait cpu0 enter sleep mode , then begin to pof seq 0.Not wait cpu sleep"]
pub type PmcPofWaitSlepR = crate::BitReader;
#[doc = "Field `PMC_POF_WAIT_SLEP` writer - 1.when SW set pmen_slep or pmen_dslp , hw pmc will wait cpu0 enter sleep mode , then begin to pof seq 0.Not wait cpu sleep"]
pub type PmcPofWaitSlepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pmc_mem_slp_mode` reader - Select memory parameter option 1: sleep setting 0: normal setting Note:defautl set to sleep setting to save power when snooze"]
pub type PmcMemSlpModeR = crate::BitReader;
#[doc = "Field `pmc_mem_slp_mode` writer - Select memory parameter option 1: sleep setting 0: normal setting Note:defautl set to sleep setting to save power when snooze"]
pub type PmcMemSlpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMC_PLL_PON` reader - 1: Request PLL pon by software,plfm and system cpu will auto select to pll path . Which one pll will be active is determined by pst_cksl_sys_cpu and pst_cksl_sys_hperi. This bit auto clear to 0 after by HW PMC Note: HW will pon one pll or two if cpu pll and hperi cpu are different"]
pub type PmcPllPonR = crate::BitReader;
#[doc = "Field `PMC_PLL_PON` writer - 1: Request PLL pon by software,plfm and system cpu will auto select to pll path . Which one pll will be active is determined by pst_cksl_sys_cpu and pst_cksl_sys_hperi. This bit auto clear to 0 after by HW PMC Note: HW will pon one pll or two if cpu pll and hperi cpu are different"]
pub type PmcPllPonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMC_PLL_POF` reader - 1: Request PLL pof by software,plfm and system cpu will auto select to pll path . Which one pll will be active is determined by pst_cksl_sys_cpu and pst_cksl_sys_hperi. This bit auto clear to 0 after by HW PMC Note: HW will pof one pll or two if cpu pll and hperi cpu are different after arbitration with bt ."]
pub type PmcPllPofR = crate::BitReader;
#[doc = "Field `PMC_PLL_POF` writer - 1: Request PLL pof by software,plfm and system cpu will auto select to pll path . Which one pll will be active is determined by pst_cksl_sys_cpu and pst_cksl_sys_hperi. This bit auto clear to 0 after by HW PMC Note: HW will pof one pll or two if cpu pll and hperi cpu are different after arbitration with bt ."]
pub type PmcPllPofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMC_TUTIME` reader - PMC timeunit setting, in cycles of osc4M e.g, default 0x1F, timeunit = 8us"]
pub type PmcTutimeR = crate::FieldReader;
#[doc = "Field `PMC_TUTIME` writer - PMC timeunit setting, in cycles of osc4M e.g, default 0x1F, timeunit = 8us"]
pub type PmcTutimeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PMC_STL_PDSOC` reader - timer wait for core power power-cut power stable . 000:8us * 2^1 001:8us * 2^2 010:8us * 2^3 // 64us 011:8us * 2^4 100:8us * 2^5 101:8us * 2^6 110:8us * 2^7 // pon default 111:8us * 2^8"]
pub type PmcStlPdsocR = crate::FieldReader;
#[doc = "Field `PMC_STL_PDSOC` writer - timer wait for core power power-cut power stable . 000:8us * 2^1 001:8us * 2^2 010:8us * 2^3 // 64us 011:8us * 2^4 100:8us * 2^5 101:8us * 2^6 110:8us * 2^7 // pon default 111:8us * 2^8"]
pub type PmcStlPdsocW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `np_cpu_id` reader - 0:cpu0(km4tz) used as NP, pmc will enable it after system wake 1:cpu1(km4ns) used as NP."]
pub type NpCpuIdR = crate::BitReader;
#[doc = "Field `np_cpu_id` writer - 0:cpu0(km4tz) used as NP, pmc will enable it after system wake 1:cpu1(km4ns) used as NP."]
pub type NpCpuIdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_is_ap` reader - This bit indicate if current cpu who read this bit is used as AP 1: is AP 0: is NP"]
pub type CpuIsApR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: Request PMC to enter deep-sleep state, auto clear to 0 after PMC finishes"]
    #[inline(always)]
    pub fn pmc_pmen_dslp(&self) -> PmcPmenDslpR {
        PmcPmenDslpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Request PMC to enter sleep state, auto clear to 0 after PMC finishes"]
    #[inline(always)]
    pub fn pmc_pmen_slep(&self) -> PmcPmenSlepR {
        PmcPmenSlepR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1.mask all KM4 interrupt 0:release all KM4 interrupt"]
    #[inline(always)]
    pub fn cpu0_irq_mask(&self) -> Cpu0IrqMaskR {
        Cpu0IrqMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - normal setting for WMAC request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
    #[inline(always)]
    pub fn pst_slep_wdml(&self) -> PstSlepWdmlR {
        PstSlepWdmlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1.when SW set pmen_slep or pmen_dslp , hw pmc will wait cpu0 enter sleep mode , then begin to pof seq 0.Not wait cpu sleep"]
    #[inline(always)]
    pub fn pmc_pof_wait_slep(&self) -> PmcPofWaitSlepR {
        PmcPofWaitSlepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select memory parameter option 1: sleep setting 0: normal setting Note:defautl set to sleep setting to save power when snooze"]
    #[inline(always)]
    pub fn pmc_mem_slp_mode(&self) -> PmcMemSlpModeR {
        PmcMemSlpModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Request PLL pon by software,plfm and system cpu will auto select to pll path . Which one pll will be active is determined by pst_cksl_sys_cpu and pst_cksl_sys_hperi. This bit auto clear to 0 after by HW PMC Note: HW will pon one pll or two if cpu pll and hperi cpu are different"]
    #[inline(always)]
    pub fn pmc_pll_pon(&self) -> PmcPllPonR {
        PmcPllPonR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Request PLL pof by software,plfm and system cpu will auto select to pll path . Which one pll will be active is determined by pst_cksl_sys_cpu and pst_cksl_sys_hperi. This bit auto clear to 0 after by HW PMC Note: HW will pof one pll or two if cpu pll and hperi cpu are different after arbitration with bt ."]
    #[inline(always)]
    pub fn pmc_pll_pof(&self) -> PmcPllPofR {
        PmcPllPofR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:21 - PMC timeunit setting, in cycles of osc4M e.g, default 0x1F, timeunit = 8us"]
    #[inline(always)]
    pub fn pmc_tutime(&self) -> PmcTutimeR {
        PmcTutimeR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - timer wait for core power power-cut power stable . 000:8us * 2^1 001:8us * 2^2 010:8us * 2^3 // 64us 011:8us * 2^4 100:8us * 2^5 101:8us * 2^6 110:8us * 2^7 // pon default 111:8us * 2^8"]
    #[inline(always)]
    pub fn pmc_stl_pdsoc(&self) -> PmcStlPdsocR {
        PmcStlPdsocR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 0:cpu0(km4tz) used as NP, pmc will enable it after system wake 1:cpu1(km4ns) used as NP."]
    #[inline(always)]
    pub fn np_cpu_id(&self) -> NpCpuIdR {
        NpCpuIdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit indicate if current cpu who read this bit is used as AP 1: is AP 0: is NP"]
    #[inline(always)]
    pub fn cpu_is_ap(&self) -> CpuIsApR {
        CpuIsApR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Request PMC to enter deep-sleep state, auto clear to 0 after PMC finishes"]
    #[inline(always)]
    pub fn pmc_pmen_dslp(&mut self) -> PmcPmenDslpW<'_, RegSyspmcCtrlSpec> {
        PmcPmenDslpW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Request PMC to enter sleep state, auto clear to 0 after PMC finishes"]
    #[inline(always)]
    pub fn pmc_pmen_slep(&mut self) -> PmcPmenSlepW<'_, RegSyspmcCtrlSpec> {
        PmcPmenSlepW::new(self, 1)
    }
    #[doc = "Bit 2 - 1.mask all KM4 interrupt 0:release all KM4 interrupt"]
    #[inline(always)]
    pub fn cpu0_irq_mask(&mut self) -> Cpu0IrqMaskW<'_, RegSyspmcCtrlSpec> {
        Cpu0IrqMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - normal setting for WMAC request Core LDO dummy load selection 1: enable dummy load when PMC enter into sleep mode 0: disable dummy load when PMC enter into sleep mode"]
    #[inline(always)]
    pub fn pst_slep_wdml(&mut self) -> PstSlepWdmlW<'_, RegSyspmcCtrlSpec> {
        PstSlepWdmlW::new(self, 3)
    }
    #[doc = "Bit 4 - 1.when SW set pmen_slep or pmen_dslp , hw pmc will wait cpu0 enter sleep mode , then begin to pof seq 0.Not wait cpu sleep"]
    #[inline(always)]
    pub fn pmc_pof_wait_slep(&mut self) -> PmcPofWaitSlepW<'_, RegSyspmcCtrlSpec> {
        PmcPofWaitSlepW::new(self, 4)
    }
    #[doc = "Bit 5 - Select memory parameter option 1: sleep setting 0: normal setting Note:defautl set to sleep setting to save power when snooze"]
    #[inline(always)]
    pub fn pmc_mem_slp_mode(&mut self) -> PmcMemSlpModeW<'_, RegSyspmcCtrlSpec> {
        PmcMemSlpModeW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Request PLL pon by software,plfm and system cpu will auto select to pll path . Which one pll will be active is determined by pst_cksl_sys_cpu and pst_cksl_sys_hperi. This bit auto clear to 0 after by HW PMC Note: HW will pon one pll or two if cpu pll and hperi cpu are different"]
    #[inline(always)]
    pub fn pmc_pll_pon(&mut self) -> PmcPllPonW<'_, RegSyspmcCtrlSpec> {
        PmcPllPonW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Request PLL pof by software,plfm and system cpu will auto select to pll path . Which one pll will be active is determined by pst_cksl_sys_cpu and pst_cksl_sys_hperi. This bit auto clear to 0 after by HW PMC Note: HW will pof one pll or two if cpu pll and hperi cpu are different after arbitration with bt ."]
    #[inline(always)]
    pub fn pmc_pll_pof(&mut self) -> PmcPllPofW<'_, RegSyspmcCtrlSpec> {
        PmcPllPofW::new(self, 7)
    }
    #[doc = "Bits 16:21 - PMC timeunit setting, in cycles of osc4M e.g, default 0x1F, timeunit = 8us"]
    #[inline(always)]
    pub fn pmc_tutime(&mut self) -> PmcTutimeW<'_, RegSyspmcCtrlSpec> {
        PmcTutimeW::new(self, 16)
    }
    #[doc = "Bits 24:26 - timer wait for core power power-cut power stable . 000:8us * 2^1 001:8us * 2^2 010:8us * 2^3 // 64us 011:8us * 2^4 100:8us * 2^5 101:8us * 2^6 110:8us * 2^7 // pon default 111:8us * 2^8"]
    #[inline(always)]
    pub fn pmc_stl_pdsoc(&mut self) -> PmcStlPdsocW<'_, RegSyspmcCtrlSpec> {
        PmcStlPdsocW::new(self, 24)
    }
    #[doc = "Bit 27 - 0:cpu0(km4tz) used as NP, pmc will enable it after system wake 1:cpu1(km4ns) used as NP."]
    #[inline(always)]
    pub fn np_cpu_id(&mut self) -> NpCpuIdW<'_, RegSyspmcCtrlSpec> {
        NpCpuIdW::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_syspmc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_syspmc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSyspmcCtrlSpec;
impl crate::RegisterSpec for RegSyspmcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_syspmc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSyspmcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_syspmc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSyspmcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SYSPMC_CTRL to value 0x0e1f_0010"]
impl crate::Resettable for RegSyspmcCtrlSpec {
    const RESET_VALUE: u32 = 0x0e1f_0010;
}
