#[doc = "Register `SCAN_OCC_CTRL0` reader"]
pub type R = crate::R<ScanOccCtrl0Spec>;
#[doc = "Register `SCAN_OCC_CTRL0` writer"]
pub type W = crate::W<ScanOccCtrl0Spec>;
#[doc = "Field `SCAN_OCC_DBG_EN_HP` reader - scan occ dbg en hp"]
pub type ScanOccDbgEnHpR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_HP` writer - scan occ dbg en hp"]
pub type ScanOccDbgEnHpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_UPHY` reader - scan occ dbg en uphy"]
pub type ScanOccDbgEnUphyR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_UPHY` writer - scan occ dbg en uphy"]
pub type ScanOccDbgEnUphyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_SPIC` reader - scan occ dbg en spic"]
pub type ScanOccDbgEnSpicR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_SPIC` writer - scan occ dbg en spic"]
pub type ScanOccDbgEnSpicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_PSRAM` reader - scan occ dbg en psram"]
pub type ScanOccDbgEnPsramR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_PSRAM` writer - scan occ dbg en psram"]
pub type ScanOccDbgEnPsramW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_SEC` reader - scan occ dbg en sec"]
pub type ScanOccDbgEnSecR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_SEC` writer - scan occ dbg en sec"]
pub type ScanOccDbgEnSecW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_WPHY` reader - scan occ dbg en wphy"]
pub type ScanOccDbgEnWphyR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_WPHY` writer - scan occ dbg en wphy"]
pub type ScanOccDbgEnWphyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_PLL_CTRL_UPHY` reader - scan pll ctrl uphy"]
pub type ScanPllCtrlUphyR = crate::BitReader;
#[doc = "Field `SCAN_PLL_CTRL_UPHY` writer - scan pll ctrl uphy"]
pub type ScanPllCtrlUphyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_PLL_CTRL_MPHY` reader - scan pll ctrl mipiphy"]
pub type ScanPllCtrlMphyR = crate::BitReader;
#[doc = "Field `SCAN_PLL_CTRL_MPHY` writer - scan pll ctrl mipiphy"]
pub type ScanPllCtrlMphyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_OCC_DBG_EN_MAC` reader - scan occ dbg en mac"]
pub type ScanOccDbgEnMacR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_MAC` writer - scan occ dbg en mac"]
pub type ScanOccDbgEnMacW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - scan occ dbg en hp"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_hp(&self) -> ScanOccDbgEnHpR {
        ScanOccDbgEnHpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - scan occ dbg en uphy"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uphy(&self) -> ScanOccDbgEnUphyR {
        ScanOccDbgEnUphyR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - scan occ dbg en spic"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_spic(&self) -> ScanOccDbgEnSpicR {
        ScanOccDbgEnSpicR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - scan occ dbg en psram"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_psram(&self) -> ScanOccDbgEnPsramR {
        ScanOccDbgEnPsramR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - scan occ dbg en sec"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_sec(&self) -> ScanOccDbgEnSecR {
        ScanOccDbgEnSecR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - scan occ dbg en wphy"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_wphy(&self) -> ScanOccDbgEnWphyR {
        ScanOccDbgEnWphyR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - scan pll ctrl uphy"]
    #[inline(always)]
    pub fn scan_pll_ctrl_uphy(&self) -> ScanPllCtrlUphyR {
        ScanPllCtrlUphyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - scan pll ctrl mipiphy"]
    #[inline(always)]
    pub fn scan_pll_ctrl_mphy(&self) -> ScanPllCtrlMphyR {
        ScanPllCtrlMphyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - scan occ dbg en mac"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_mac(&self) -> ScanOccDbgEnMacR {
        ScanOccDbgEnMacR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - scan occ dbg en hp"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_hp(&mut self) -> ScanOccDbgEnHpW<'_, ScanOccCtrl0Spec> {
        ScanOccDbgEnHpW::new(self, 0)
    }
    #[doc = "Bits 2:3 - scan occ dbg en uphy"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uphy(&mut self) -> ScanOccDbgEnUphyW<'_, ScanOccCtrl0Spec> {
        ScanOccDbgEnUphyW::new(self, 2)
    }
    #[doc = "Bits 4:5 - scan occ dbg en spic"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_spic(&mut self) -> ScanOccDbgEnSpicW<'_, ScanOccCtrl0Spec> {
        ScanOccDbgEnSpicW::new(self, 4)
    }
    #[doc = "Bits 6:7 - scan occ dbg en psram"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_psram(&mut self) -> ScanOccDbgEnPsramW<'_, ScanOccCtrl0Spec> {
        ScanOccDbgEnPsramW::new(self, 6)
    }
    #[doc = "Bits 8:9 - scan occ dbg en sec"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_sec(&mut self) -> ScanOccDbgEnSecW<'_, ScanOccCtrl0Spec> {
        ScanOccDbgEnSecW::new(self, 8)
    }
    #[doc = "Bits 10:11 - scan occ dbg en wphy"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_wphy(&mut self) -> ScanOccDbgEnWphyW<'_, ScanOccCtrl0Spec> {
        ScanOccDbgEnWphyW::new(self, 10)
    }
    #[doc = "Bit 12 - scan pll ctrl uphy"]
    #[inline(always)]
    pub fn scan_pll_ctrl_uphy(&mut self) -> ScanPllCtrlUphyW<'_, ScanOccCtrl0Spec> {
        ScanPllCtrlUphyW::new(self, 12)
    }
    #[doc = "Bit 13 - scan pll ctrl mipiphy"]
    #[inline(always)]
    pub fn scan_pll_ctrl_mphy(&mut self) -> ScanPllCtrlMphyW<'_, ScanOccCtrl0Spec> {
        ScanPllCtrlMphyW::new(self, 13)
    }
    #[doc = "Bits 14:15 - scan occ dbg en mac"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_mac(&mut self) -> ScanOccDbgEnMacW<'_, ScanOccCtrl0Spec> {
        ScanOccDbgEnMacW::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`scan_occ_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan_occ_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanOccCtrl0Spec;
impl crate::RegisterSpec for ScanOccCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scan_occ_ctrl0::R`](R) reader structure"]
impl crate::Readable for ScanOccCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`scan_occ_ctrl0::W`](W) writer structure"]
impl crate::Writable for ScanOccCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCAN_OCC_CTRL0 to value 0"]
impl crate::Resettable for ScanOccCtrl0Spec {}
