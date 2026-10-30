#[doc = "Register `SCAN_OCC_CTRL2` reader"]
pub type R = crate::R<ScanOccCtrl2Spec>;
#[doc = "Register `SCAN_OCC_CTRL2` writer"]
pub type W = crate::W<ScanOccCtrl2Spec>;
#[doc = "Field `SCAN_OCC_DBG_EN_ADCC` reader - "]
pub type ScanOccDbgEnAdccR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_ADCC` writer - "]
pub type ScanOccDbgEnAdccW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_AUX40M` reader - "]
pub type ScanOccDbgEnAux40mR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_AUX40M` writer - "]
pub type ScanOccDbgEnAux40mW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_BT` reader - "]
pub type ScanOccDbgEnBtR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_BT` writer - "]
pub type ScanOccDbgEnBtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_GPIO` reader - "]
pub type ScanOccDbgEnGpioR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_GPIO` writer - "]
pub type ScanOccDbgEnGpioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_HTIM` reader - "]
pub type ScanOccDbgEnHtimR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_HTIM` writer - "]
pub type ScanOccDbgEnHtimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_LOGUART` reader - "]
pub type ScanOccDbgEnLoguartR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_LOGUART` writer - "]
pub type ScanOccDbgEnLoguartW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_LPON` reader - "]
pub type ScanOccDbgEnLponR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_LPON` writer - "]
pub type ScanOccDbgEnLponW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_OTPC` reader - "]
pub type ScanOccDbgEnOtpcR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_OTPC` writer - "]
pub type ScanOccDbgEnOtpcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_SDMX` reader - "]
pub type ScanOccDbgEnSdmxR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_SDMX` writer - "]
pub type ScanOccDbgEnSdmxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_SIC` reader - "]
pub type ScanOccDbgEnSicR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_SIC` writer - "]
pub type ScanOccDbgEnSicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_UART_T` reader - "]
pub type ScanOccDbgEnUartTR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_UART_T` writer - "]
pub type ScanOccDbgEnUartTW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_WL` reader - "]
pub type ScanOccDbgEnWlR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_WL` writer - "]
pub type ScanOccDbgEnWlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_AON4M` reader - "]
pub type ScanOccDbgEnAon4mR = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_AON4M` writer - "]
pub type ScanOccDbgEnAon4mW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_adcc(&self) -> ScanOccDbgEnAdccR {
        ScanOccDbgEnAdccR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_aux40m(&self) -> ScanOccDbgEnAux40mR {
        ScanOccDbgEnAux40mR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_bt(&self) -> ScanOccDbgEnBtR {
        ScanOccDbgEnBtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_gpio(&self) -> ScanOccDbgEnGpioR {
        ScanOccDbgEnGpioR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_htim(&self) -> ScanOccDbgEnHtimR {
        ScanOccDbgEnHtimR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_loguart(&self) -> ScanOccDbgEnLoguartR {
        ScanOccDbgEnLoguartR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_lpon(&self) -> ScanOccDbgEnLponR {
        ScanOccDbgEnLponR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_otpc(&self) -> ScanOccDbgEnOtpcR {
        ScanOccDbgEnOtpcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_sdmx(&self) -> ScanOccDbgEnSdmxR {
        ScanOccDbgEnSdmxR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_sic(&self) -> ScanOccDbgEnSicR {
        ScanOccDbgEnSicR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uart_t(&self) -> ScanOccDbgEnUartTR {
        ScanOccDbgEnUartTR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_wl(&self) -> ScanOccDbgEnWlR {
        ScanOccDbgEnWlR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_aon4m(&self) -> ScanOccDbgEnAon4mR {
        ScanOccDbgEnAon4mR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_adcc(&mut self) -> ScanOccDbgEnAdccW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnAdccW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_aux40m(&mut self) -> ScanOccDbgEnAux40mW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnAux40mW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_bt(&mut self) -> ScanOccDbgEnBtW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnBtW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_gpio(&mut self) -> ScanOccDbgEnGpioW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnGpioW::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_htim(&mut self) -> ScanOccDbgEnHtimW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnHtimW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_loguart(&mut self) -> ScanOccDbgEnLoguartW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnLoguartW::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_lpon(&mut self) -> ScanOccDbgEnLponW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnLponW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_otpc(&mut self) -> ScanOccDbgEnOtpcW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnOtpcW::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_sdmx(&mut self) -> ScanOccDbgEnSdmxW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnSdmxW::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_sic(&mut self) -> ScanOccDbgEnSicW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnSicW::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uart_t(&mut self) -> ScanOccDbgEnUartTW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnUartTW::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_wl(&mut self) -> ScanOccDbgEnWlW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnWlW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_aon4m(&mut self) -> ScanOccDbgEnAon4mW<'_, ScanOccCtrl2Spec> {
        ScanOccDbgEnAon4mW::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`scan_occ_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan_occ_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanOccCtrl2Spec;
impl crate::RegisterSpec for ScanOccCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scan_occ_ctrl2::R`](R) reader structure"]
impl crate::Readable for ScanOccCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`scan_occ_ctrl2::W`](W) writer structure"]
impl crate::Writable for ScanOccCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCAN_OCC_CTRL2 to value 0"]
impl crate::Resettable for ScanOccCtrl2Spec {}
