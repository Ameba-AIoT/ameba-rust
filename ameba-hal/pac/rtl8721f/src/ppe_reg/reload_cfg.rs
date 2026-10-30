#[doc = "Register `RELOAD_CFG` reader"]
pub type R = crate::R<ReloadCfgSpec>;
#[doc = "Register `RELOAD_CFG` writer"]
pub type W = crate::W<ReloadCfgSpec>;
#[doc = "Field `LYR_RELOAD_EN` reader - Layers setting reload enable Bit0~15: Layer0~15"]
pub type LyrReloadEnR = crate::FieldReader;
#[doc = "Field `LYR_RELOAD_EN` writer - Layers setting reload enable Bit0~15: Layer0~15"]
pub type LyrReloadEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SET_VALID` reader - Setting is valid Will be auto set to 1 when writing 1 to GLB_EN"]
pub type SetValidR = crate::BitReader;
#[doc = "Field `SET_VALID` writer - Setting is valid Will be auto set to 1 when writing 1 to GLB_EN"]
pub type SetValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CLR` reader - Auto clear \"SET_VALID\" after loading layer settings"]
pub type AutoClrR = crate::BitReader;
#[doc = "Field `AUTO_CLR` writer - Auto clear \"SET_VALID\" after loading layer settings"]
pub type AutoClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Layers setting reload enable Bit0~15: Layer0~15"]
    #[inline(always)]
    pub fn lyr_reload_en(&self) -> LyrReloadEnR {
        LyrReloadEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Setting is valid Will be auto set to 1 when writing 1 to GLB_EN"]
    #[inline(always)]
    pub fn set_valid(&self) -> SetValidR {
        SetValidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Auto clear \"SET_VALID\" after loading layer settings"]
    #[inline(always)]
    pub fn auto_clr(&self) -> AutoClrR {
        AutoClrR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Layers setting reload enable Bit0~15: Layer0~15"]
    #[inline(always)]
    pub fn lyr_reload_en(&mut self) -> LyrReloadEnW<'_, ReloadCfgSpec> {
        LyrReloadEnW::new(self, 0)
    }
    #[doc = "Bit 16 - Setting is valid Will be auto set to 1 when writing 1 to GLB_EN"]
    #[inline(always)]
    pub fn set_valid(&mut self) -> SetValidW<'_, ReloadCfgSpec> {
        SetValidW::new(self, 16)
    }
    #[doc = "Bit 17 - Auto clear \"SET_VALID\" after loading layer settings"]
    #[inline(always)]
    pub fn auto_clr(&mut self) -> AutoClrW<'_, ReloadCfgSpec> {
        AutoClrW::new(self, 17)
    }
}
#[doc = "Reload configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reload_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reload_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReloadCfgSpec;
impl crate::RegisterSpec for ReloadCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reload_cfg::R`](R) reader structure"]
impl crate::Readable for ReloadCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reload_cfg::W`](W) writer structure"]
impl crate::Writable for ReloadCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RELOAD_CFG to value 0x0002_0000"]
impl crate::Resettable for ReloadCfgSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
