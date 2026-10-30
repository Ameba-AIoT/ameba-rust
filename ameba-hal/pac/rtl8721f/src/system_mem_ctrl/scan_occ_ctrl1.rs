#[doc = "Register `SCAN_OCC_CTRL1` reader"]
pub type R = crate::R<ScanOccCtrl1Spec>;
#[doc = "Register `SCAN_OCC_CTRL1` writer"]
pub type W = crate::W<ScanOccCtrl1Spec>;
#[doc = "Field `SCAN_OCC_DBG_EN_HPERI` reader - "]
pub type ScanOccDbgEnHperiR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_HPERI` writer - "]
pub type ScanOccDbgEnHperiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_IRDA` reader - "]
pub type ScanOccDbgEnIrdaR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_IRDA` writer - "]
pub type ScanOccDbgEnIrdaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_LPERI` reader - "]
pub type ScanOccDbgEnLperiR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_LPERI` writer - "]
pub type ScanOccDbgEnLperiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_SPORT` reader - "]
pub type ScanOccDbgEnSportR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_SPORT` writer - "]
pub type ScanOccDbgEnSportW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_SHPERI` reader - "]
pub type ScanOccDbgEnShperiR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_SHPERI` writer - "]
pub type ScanOccDbgEnShperiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_VO` reader - "]
pub type ScanOccDbgEnVoR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_VO` writer - "]
pub type ScanOccDbgEnVoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_CAN` reader - "]
pub type ScanOccDbgEnCanR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_CAN` writer - "]
pub type ScanOccDbgEnCanW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_AC` reader - "]
pub type ScanOccDbgEnAcR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_AC` writer - "]
pub type ScanOccDbgEnAcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_SDIO` reader - "]
pub type ScanOccDbgEnSdioR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_SDIO` writer - "]
pub type ScanOccDbgEnSdioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_hperi(&self) -> ScanOccDbgEnHperiR {
        ScanOccDbgEnHperiR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_irda(&self) -> ScanOccDbgEnIrdaR {
        ScanOccDbgEnIrdaR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_lperi(&self) -> ScanOccDbgEnLperiR {
        ScanOccDbgEnLperiR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_sport(&self) -> ScanOccDbgEnSportR {
        ScanOccDbgEnSportR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_shperi(&self) -> ScanOccDbgEnShperiR {
        ScanOccDbgEnShperiR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_vo(&self) -> ScanOccDbgEnVoR {
        ScanOccDbgEnVoR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_can(&self) -> ScanOccDbgEnCanR {
        ScanOccDbgEnCanR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_ac(&self) -> ScanOccDbgEnAcR {
        ScanOccDbgEnAcR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_sdio(&self) -> ScanOccDbgEnSdioR {
        ScanOccDbgEnSdioR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_hperi(&mut self) -> ScanOccDbgEnHperiW<'_, ScanOccCtrl1Spec> {
        ScanOccDbgEnHperiW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_irda(&mut self) -> ScanOccDbgEnIrdaW<'_, ScanOccCtrl1Spec> {
        ScanOccDbgEnIrdaW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_lperi(&mut self) -> ScanOccDbgEnLperiW<'_, ScanOccCtrl1Spec> {
        ScanOccDbgEnLperiW::new(self, 4)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_sport(&mut self) -> ScanOccDbgEnSportW<'_, ScanOccCtrl1Spec> {
        ScanOccDbgEnSportW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_shperi(&mut self) -> ScanOccDbgEnShperiW<'_, ScanOccCtrl1Spec> {
        ScanOccDbgEnShperiW::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_vo(&mut self) -> ScanOccDbgEnVoW<'_, ScanOccCtrl1Spec> {
        ScanOccDbgEnVoW::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_can(&mut self) -> ScanOccDbgEnCanW<'_, ScanOccCtrl1Spec> {
        ScanOccDbgEnCanW::new(self, 20)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_ac(&mut self) -> ScanOccDbgEnAcW<'_, ScanOccCtrl1Spec> {
        ScanOccDbgEnAcW::new(self, 26)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_sdio(&mut self) -> ScanOccDbgEnSdioW<'_, ScanOccCtrl1Spec> {
        ScanOccDbgEnSdioW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`scan_occ_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan_occ_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanOccCtrl1Spec;
impl crate::RegisterSpec for ScanOccCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scan_occ_ctrl1::R`](R) reader structure"]
impl crate::Readable for ScanOccCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`scan_occ_ctrl1::W`](W) writer structure"]
impl crate::Writable for ScanOccCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCAN_OCC_CTRL1 to value 0"]
impl crate::Resettable for ScanOccCtrl1Spec {}
