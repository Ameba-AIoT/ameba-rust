#[doc = "Register `SCAN_OCC_CTRL3` reader"]
pub type R = crate::R<ScanOccCtrl3Spec>;
#[doc = "Register `SCAN_OCC_CTRL3` writer"]
pub type W = crate::W<ScanOccCtrl3Spec>;
#[doc = "Field `SCAN_OCC_DBG_EN_UART_R_0` reader - "]
pub type ScanOccDbgEnUartR0R = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_UART_R_0` writer - "]
pub type ScanOccDbgEnUartR0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_UART_R_1` reader - "]
pub type ScanOccDbgEnUartR1R = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_UART_R_1` writer - "]
pub type ScanOccDbgEnUartR1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_UART_R_2` reader - "]
pub type ScanOccDbgEnUartR2R = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_UART_R_2` writer - "]
pub type ScanOccDbgEnUartR2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_UART_R_3` reader - "]
pub type ScanOccDbgEnUartR3R = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_UART_R_3` writer - "]
pub type ScanOccDbgEnUartR3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_LTIM_0` reader - "]
pub type ScanOccDbgEnLtim0R = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_LTIM_0` writer - "]
pub type ScanOccDbgEnLtim0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_LTIM_1` reader - "]
pub type ScanOccDbgEnLtim1R = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_LTIM_1` writer - "]
pub type ScanOccDbgEnLtim1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_LTIM_2` reader - "]
pub type ScanOccDbgEnLtim2R = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_LTIM_2` writer - "]
pub type ScanOccDbgEnLtim2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCAN_OCC_DBG_EN_LTIM_3` reader - "]
pub type ScanOccDbgEnLtim3R = crate::FieldReader;
#[doc = "Field `SCAN_OCC_DBG_EN_LTIM_3` writer - "]
pub type ScanOccDbgEnLtim3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uart_r_0(&self) -> ScanOccDbgEnUartR0R {
        ScanOccDbgEnUartR0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uart_r_1(&self) -> ScanOccDbgEnUartR1R {
        ScanOccDbgEnUartR1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uart_r_2(&self) -> ScanOccDbgEnUartR2R {
        ScanOccDbgEnUartR2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uart_r_3(&self) -> ScanOccDbgEnUartR3R {
        ScanOccDbgEnUartR3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_ltim_0(&self) -> ScanOccDbgEnLtim0R {
        ScanOccDbgEnLtim0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_ltim_1(&self) -> ScanOccDbgEnLtim1R {
        ScanOccDbgEnLtim1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_ltim_2(&self) -> ScanOccDbgEnLtim2R {
        ScanOccDbgEnLtim2R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_ltim_3(&self) -> ScanOccDbgEnLtim3R {
        ScanOccDbgEnLtim3R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uart_r_0(&mut self) -> ScanOccDbgEnUartR0W<'_, ScanOccCtrl3Spec> {
        ScanOccDbgEnUartR0W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uart_r_1(&mut self) -> ScanOccDbgEnUartR1W<'_, ScanOccCtrl3Spec> {
        ScanOccDbgEnUartR1W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uart_r_2(&mut self) -> ScanOccDbgEnUartR2W<'_, ScanOccCtrl3Spec> {
        ScanOccDbgEnUartR2W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_uart_r_3(&mut self) -> ScanOccDbgEnUartR3W<'_, ScanOccCtrl3Spec> {
        ScanOccDbgEnUartR3W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_ltim_0(&mut self) -> ScanOccDbgEnLtim0W<'_, ScanOccCtrl3Spec> {
        ScanOccDbgEnLtim0W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_ltim_1(&mut self) -> ScanOccDbgEnLtim1W<'_, ScanOccCtrl3Spec> {
        ScanOccDbgEnLtim1W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_ltim_2(&mut self) -> ScanOccDbgEnLtim2W<'_, ScanOccCtrl3Spec> {
        ScanOccDbgEnLtim2W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn scan_occ_dbg_en_ltim_3(&mut self) -> ScanOccDbgEnLtim3W<'_, ScanOccCtrl3Spec> {
        ScanOccDbgEnLtim3W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`scan_occ_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan_occ_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanOccCtrl3Spec;
impl crate::RegisterSpec for ScanOccCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scan_occ_ctrl3::R`](R) reader structure"]
impl crate::Readable for ScanOccCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`scan_occ_ctrl3::W`](W) writer structure"]
impl crate::Writable for ScanOccCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCAN_OCC_CTRL3 to value 0"]
impl crate::Resettable for ScanOccCtrl3Spec {}
