#[doc = "Register `REG_BOOT_REASON_HW` reader"]
pub type R = crate::R<RegBootReasonHwSpec>;
#[doc = "Field `rstf_bor` reader - This bit will be set to 1 when BOR resets the system. Write 1 to clear."]
pub type RstfBorR = crate::BitReader;
#[doc = "Field `rstf_dslp` reader - This is boot reason for dslp exit, clear by SW write to 1."]
pub type RstfDslpR = crate::BitReader;
#[doc = "Field `rstf_sys0_glb` reader - This bit will be set to 1 when cpu0 resets the system. Write 1 to clear."]
pub type RstfSys0GlbR = crate::BitReader;
#[doc = "Field `rstf_sys1_glb` reader - This bit will be set to 1 when cpu1 resets the system. Write 1 to clear."]
pub type RstfSys1GlbR = crate::BitReader;
#[doc = "Field `rstf_iwdg` reader - This bit will be set to 1 when WDG resets the system. Write 1 to clear."]
pub type RstfIwdgR = crate::BitReader;
#[doc = "Field `rstf_wdg0_glb` reader - This bit will be set to 1 when WDG resets the system. Write 1 to clear."]
pub type RstfWdg0GlbR = crate::BitReader;
#[doc = "Field `rstf_wdg1_glb` reader - This bit will be set to 1 when WDG resets the system. Write 1 to clear."]
pub type RstfWdg1GlbR = crate::BitReader;
#[doc = "Field `rstf_wdg2_glb` reader - This bit will be set to 1 when WDG resets the system. Write 1 to clear."]
pub type RstfWdg2GlbR = crate::BitReader;
#[doc = "Field `rstf_warm_km4tz` reader - This bit will be set to 1 when km4 warm resets the system. Write 1 to clear."]
pub type RstfWarmKm4tzR = crate::BitReader;
#[doc = "Field `rstf_warm_km4ns` reader - This bit will be set to 1 when kr4 warm resets the system. Write 1 to clear."]
pub type RstfWarmKm4nsR = crate::BitReader;
#[doc = "Field `rstf_ocp` reader - This bit will be set to 1 when OCP happen, it will let system enter dslp mode. Write 1 to clear."]
pub type RstfOcpR = crate::BitReader;
#[doc = "Field `rstf_sys0_cpu` reader - This bit will be set to 1 when cpu0 resets local CPU. Write 1 to clear."]
pub type RstfSys0CpuR = crate::BitReader;
#[doc = "Field `rstf_sys1_cpu` reader - This bit will be set to 1 when cpu1 resets local CPU. Write 1 to clear."]
pub type RstfSys1CpuR = crate::BitReader;
#[doc = "Field `rstf_wdg0_cpu` reader - This bit will be set to 1 when WDG resets local CPU only. Write 1 to clear."]
pub type RstfWdg0CpuR = crate::BitReader;
#[doc = "Field `rstf_wdg1_cpu` reader - This bit will be set to 1 when WDG resets local CPU only. Write 1 to clear."]
pub type RstfWdg1CpuR = crate::BitReader;
#[doc = "Field `rstf_wdg2_cpu` reader - This bit will be set to 1 when WDG resets local CPU only. Write 1 to clear."]
pub type RstfWdg2CpuR = crate::BitReader;
#[doc = "Field `rstf_thm` reader - This bit will be set to 1 when Thermal resets the system. Write 1 to clear."]
pub type RstfThmR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit will be set to 1 when BOR resets the system. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_bor(&self) -> RstfBorR {
        RstfBorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is boot reason for dslp exit, clear by SW write to 1."]
    #[inline(always)]
    pub fn rstf_dslp(&self) -> RstfDslpR {
        RstfDslpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit will be set to 1 when cpu0 resets the system. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_sys0_glb(&self) -> RstfSys0GlbR {
        RstfSys0GlbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit will be set to 1 when cpu1 resets the system. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_sys1_glb(&self) -> RstfSys1GlbR {
        RstfSys1GlbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit will be set to 1 when WDG resets the system. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_iwdg(&self) -> RstfIwdgR {
        RstfIwdgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit will be set to 1 when WDG resets the system. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_wdg0_glb(&self) -> RstfWdg0GlbR {
        RstfWdg0GlbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit will be set to 1 when WDG resets the system. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_wdg1_glb(&self) -> RstfWdg1GlbR {
        RstfWdg1GlbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit will be set to 1 when WDG resets the system. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_wdg2_glb(&self) -> RstfWdg2GlbR {
        RstfWdg2GlbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit will be set to 1 when km4 warm resets the system. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_warm_km4tz(&self) -> RstfWarmKm4tzR {
        RstfWarmKm4tzR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit will be set to 1 when kr4 warm resets the system. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_warm_km4ns(&self) -> RstfWarmKm4nsR {
        RstfWarmKm4nsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit will be set to 1 when OCP happen, it will let system enter dslp mode. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_ocp(&self) -> RstfOcpR {
        RstfOcpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit will be set to 1 when cpu0 resets local CPU. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_sys0_cpu(&self) -> RstfSys0CpuR {
        RstfSys0CpuR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit will be set to 1 when cpu1 resets local CPU. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_sys1_cpu(&self) -> RstfSys1CpuR {
        RstfSys1CpuR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit will be set to 1 when WDG resets local CPU only. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_wdg0_cpu(&self) -> RstfWdg0CpuR {
        RstfWdg0CpuR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit will be set to 1 when WDG resets local CPU only. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_wdg1_cpu(&self) -> RstfWdg1CpuR {
        RstfWdg1CpuR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit will be set to 1 when WDG resets local CPU only. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_wdg2_cpu(&self) -> RstfWdg2CpuR {
        RstfWdg2CpuR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit will be set to 1 when Thermal resets the system. Write 1 to clear."]
    #[inline(always)]
    pub fn rstf_thm(&self) -> RstfThmR {
        RstfThmR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_boot_reason_hw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBootReasonHwSpec;
impl crate::RegisterSpec for RegBootReasonHwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_boot_reason_hw::R`](R) reader structure"]
impl crate::Readable for RegBootReasonHwSpec {}
#[doc = "`reset()` method sets REG_BOOT_REASON_HW to value 0"]
impl crate::Resettable for RegBootReasonHwSpec {}
