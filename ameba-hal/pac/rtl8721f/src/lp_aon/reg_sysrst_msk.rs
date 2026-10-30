#[doc = "Register `REG_SYSRST_MSK` reader"]
pub type R = crate::R<RegSysrstMskSpec>;
#[doc = "Register `REG_SYSRST_MSK` writer"]
pub type W = crate::W<RegSysrstMskSpec>;
#[doc = "Field `sysrst_msk_lpon` reader - Lpon global reset 1: mask LS reset when sys reset happen 0: reset by sys reset"]
pub type SysrstMskLponR = crate::BitReader;
#[doc = "Field `sysrst_msk_lpon` writer - Lpon global reset 1: mask LS reset when sys reset happen 0: reset by sys reset"]
pub type SysrstMskLponW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sysrst_msk_px` reader - Peri global reset 1: mask pinmux and led pro reset when sys reset happen 0: reset by sys reset"]
pub type SysrstMskPxR = crate::BitReader;
#[doc = "Field `sysrst_msk_px` writer - Peri global reset 1: mask pinmux and led pro reset when sys reset happen 0: reset by sys reset"]
pub type SysrstMskPxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lpon global reset 1: mask LS reset when sys reset happen 0: reset by sys reset"]
    #[inline(always)]
    pub fn sysrst_msk_lpon(&self) -> SysrstMskLponR {
        SysrstMskLponR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peri global reset 1: mask pinmux and led pro reset when sys reset happen 0: reset by sys reset"]
    #[inline(always)]
    pub fn sysrst_msk_px(&self) -> SysrstMskPxR {
        SysrstMskPxR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lpon global reset 1: mask LS reset when sys reset happen 0: reset by sys reset"]
    #[inline(always)]
    pub fn sysrst_msk_lpon(&mut self) -> SysrstMskLponW<'_, RegSysrstMskSpec> {
        SysrstMskLponW::new(self, 0)
    }
    #[doc = "Bit 1 - Peri global reset 1: mask pinmux and led pro reset when sys reset happen 0: reset by sys reset"]
    #[inline(always)]
    pub fn sysrst_msk_px(&mut self) -> SysrstMskPxW<'_, RegSysrstMskSpec> {
        SysrstMskPxW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sysrst_msk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sysrst_msk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSysrstMskSpec;
impl crate::RegisterSpec for RegSysrstMskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sysrst_msk::R`](R) reader structure"]
impl crate::Readable for RegSysrstMskSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sysrst_msk::W`](W) writer structure"]
impl crate::Writable for RegSysrstMskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SYSRST_MSK to value 0x02"]
impl crate::Resettable for RegSysrstMskSpec {
    const RESET_VALUE: u32 = 0x02;
}
