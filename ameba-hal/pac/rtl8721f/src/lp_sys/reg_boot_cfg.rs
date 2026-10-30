#[doc = "Register `REG_BOOT_CFG` reader"]
pub type R = crate::R<RegBootCfgSpec>;
#[doc = "Register `REG_BOOT_CFG` writer"]
pub type W = crate::W<RegBootCfgSpec>;
#[doc = "Field `ROM_VERSION_SW` reader - "]
pub type RomVersionSwR = crate::FieldReader<u16>;
#[doc = "Field `ROM_VERSION_SW` writer - "]
pub type RomVersionSwW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BOOT_RSVD_SW` reader - "]
pub type BootRsvdSwR = crate::FieldReader<u16>;
#[doc = "Field `BOOT_RSVD_SW` writer - "]
pub type BootRsvdSwW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BOOT_PATCH_FUNC0` reader - "]
pub type BootPatchFunc0R = crate::BitReader;
#[doc = "Field `BOOT_PATCH_FUNC0` writer - "]
pub type BootPatchFunc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_WAKE_FROM_PS_HS` reader - "]
pub type BootWakeFromPsHsR = crate::BitReader;
#[doc = "Field `BOOT_WAKE_FROM_PS_HS` writer - "]
pub type BootWakeFromPsHsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_WAKE_FROM_PS_LS` reader - "]
pub type BootWakeFromPsLsR = crate::BitReader;
#[doc = "Field `BOOT_WAKE_FROM_PS_LS` writer - "]
pub type BootWakeFromPsLsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_CPU1_RUN` reader - "]
pub type BootCpu1RunR = crate::BitReader;
#[doc = "Field `BOOT_CPU1_RUN` writer - "]
pub type BootCpu1RunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rom_version_sw(&self) -> RomVersionSwR {
        RomVersionSwR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn boot_rsvd_sw(&self) -> BootRsvdSwR {
        BootRsvdSwR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn boot_patch_func0(&self) -> BootPatchFunc0R {
        BootPatchFunc0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn boot_wake_from_ps_hs(&self) -> BootWakeFromPsHsR {
        BootWakeFromPsHsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn boot_wake_from_ps_ls(&self) -> BootWakeFromPsLsR {
        BootWakeFromPsLsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn boot_cpu1_run(&self) -> BootCpu1RunR {
        BootCpu1RunR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rom_version_sw(&mut self) -> RomVersionSwW<'_, RegBootCfgSpec> {
        RomVersionSwW::new(self, 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn boot_rsvd_sw(&mut self) -> BootRsvdSwW<'_, RegBootCfgSpec> {
        BootRsvdSwW::new(self, 16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn boot_patch_func0(&mut self) -> BootPatchFunc0W<'_, RegBootCfgSpec> {
        BootPatchFunc0W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn boot_wake_from_ps_hs(&mut self) -> BootWakeFromPsHsW<'_, RegBootCfgSpec> {
        BootWakeFromPsHsW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn boot_wake_from_ps_ls(&mut self) -> BootWakeFromPsLsW<'_, RegBootCfgSpec> {
        BootWakeFromPsLsW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn boot_cpu1_run(&mut self) -> BootCpu1RunW<'_, RegBootCfgSpec> {
        BootCpu1RunW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_boot_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_boot_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBootCfgSpec;
impl crate::RegisterSpec for RegBootCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_boot_cfg::R`](R) reader structure"]
impl crate::Readable for RegBootCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_boot_cfg::W`](W) writer structure"]
impl crate::Writable for RegBootCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BOOT_CFG to value 0"]
impl crate::Resettable for RegBootCfgSpec {}
