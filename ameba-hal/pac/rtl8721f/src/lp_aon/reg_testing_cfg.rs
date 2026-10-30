#[doc = "Register `REG_TESTING_CFG` reader"]
pub type R = crate::R<RegTestingCfgSpec>;
#[doc = "Register `REG_TESTING_CFG` writer"]
pub type W = crate::W<RegTestingCfgSpec>;
#[doc = "Field `iboot_sel` reader - If 1, using boot_int or boot_nor to replace trapped iboot value."]
pub type IbootSelR = crate::BitReader;
#[doc = "Field `iboot_sel` writer - If 1, using boot_int or boot_nor to replace trapped iboot value."]
pub type IbootSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iboot_sw` reader - No use"]
pub type IbootSwR = crate::BitReader;
#[doc = "Field `iboot_sw` writer - No use"]
pub type IbootSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `boot_nor` reader - config by SW, boot from external"]
pub type BootNorR = crate::BitReader;
#[doc = "Field `boot_nor` writer - config by SW, boot from external"]
pub type BootNorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `boot_int` reader - config by SW, boot from internal MCM flash"]
pub type BootIntR = crate::BitReader;
#[doc = "Field `boot_int` writer - config by SW, boot from internal MCM flash"]
pub type BootIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `icfg_sel` reader - If 1, using icfg_sw to replace trapped icfg value."]
pub type IcfgSelR = crate::BitReader;
#[doc = "Field `icfg_sel` writer - If 1, using icfg_sw to replace trapped icfg value."]
pub type IcfgSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `icfg_sw` reader - icfg configured by SIC/SW"]
pub type IcfgSwR = crate::FieldReader;
#[doc = "Field `icfg_sw` writer - icfg configured by SIC/SW"]
pub type IcfgSwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 16 - If 1, using boot_int or boot_nor to replace trapped iboot value."]
    #[inline(always)]
    pub fn iboot_sel(&self) -> IbootSelR {
        IbootSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - No use"]
    #[inline(always)]
    pub fn iboot_sw(&self) -> IbootSwR {
        IbootSwR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - config by SW, boot from external"]
    #[inline(always)]
    pub fn boot_nor(&self) -> BootNorR {
        BootNorR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - config by SW, boot from internal MCM flash"]
    #[inline(always)]
    pub fn boot_int(&self) -> BootIntR {
        BootIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If 1, using icfg_sw to replace trapped icfg value."]
    #[inline(always)]
    pub fn icfg_sel(&self) -> IcfgSelR {
        IcfgSelR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:27 - icfg configured by SIC/SW"]
    #[inline(always)]
    pub fn icfg_sw(&self) -> IcfgSwR {
        IcfgSwR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - If 1, using boot_int or boot_nor to replace trapped iboot value."]
    #[inline(always)]
    pub fn iboot_sel(&mut self) -> IbootSelW<'_, RegTestingCfgSpec> {
        IbootSelW::new(self, 16)
    }
    #[doc = "Bit 17 - No use"]
    #[inline(always)]
    pub fn iboot_sw(&mut self) -> IbootSwW<'_, RegTestingCfgSpec> {
        IbootSwW::new(self, 17)
    }
    #[doc = "Bit 18 - config by SW, boot from external"]
    #[inline(always)]
    pub fn boot_nor(&mut self) -> BootNorW<'_, RegTestingCfgSpec> {
        BootNorW::new(self, 18)
    }
    #[doc = "Bit 19 - config by SW, boot from internal MCM flash"]
    #[inline(always)]
    pub fn boot_int(&mut self) -> BootIntW<'_, RegTestingCfgSpec> {
        BootIntW::new(self, 19)
    }
    #[doc = "Bit 20 - If 1, using icfg_sw to replace trapped icfg value."]
    #[inline(always)]
    pub fn icfg_sel(&mut self) -> IcfgSelW<'_, RegTestingCfgSpec> {
        IcfgSelW::new(self, 20)
    }
    #[doc = "Bits 24:27 - icfg configured by SIC/SW"]
    #[inline(always)]
    pub fn icfg_sw(&mut self) -> IcfgSwW<'_, RegTestingCfgSpec> {
        IcfgSwW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_testing_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_testing_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTestingCfgSpec;
impl crate::RegisterSpec for RegTestingCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_testing_cfg::R`](R) reader structure"]
impl crate::Readable for RegTestingCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_testing_cfg::W`](W) writer structure"]
impl crate::Writable for RegTestingCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TESTING_CFG to value 0x0200_0000"]
impl crate::Resettable for RegTestingCfgSpec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
