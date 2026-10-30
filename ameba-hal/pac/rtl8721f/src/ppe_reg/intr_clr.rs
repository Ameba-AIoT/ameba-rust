#[doc = "Register `INTR_CLR` writer"]
pub type W = crate::W<IntrClrSpec>;
#[doc = "Field `INTR_CLR_ALL_OVER` writer - Wirte 1 to clear the corresponding interrupt"]
pub type IntrClrAllOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_CLR_FR_OVER` writer - Wirte 1 to clear the corresponding interrupt"]
pub type IntrClrFrOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_CLR_LOAD_OVER` writer - Wirte 1 to clear the corresponding interrupt"]
pub type IntrClrLoadOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_CLR_LINE_OVER` writer - Wirte 1 to clear the corresponding interrupt"]
pub type IntrClrLineOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_CLR_SUSP_INAC` writer - Wirte 1 to clear the corresponding interrupt"]
pub type IntrClrSuspInacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_CLR_SECURE_ERR` writer - Wirte 1 to clear the corresponding interrupt"]
pub type IntrClrSecureErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_CLR_SET_ERR` writer - Wirte 1 to clear the corresponding interrupt"]
pub type IntrClrSetErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_CLR_BUS_ERR` writer - Wirte 1 to clear the corresponding interrupt"]
pub type IntrClrBusErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_CLR_DIV0_ERR` writer - Wirte 1 to clear the corresponding interrupt"]
pub type IntrClrDiv0ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Wirte 1 to clear the corresponding interrupt"]
    #[inline(always)]
    pub fn intr_clr_all_over(&mut self) -> IntrClrAllOverW<'_, IntrClrSpec> {
        IntrClrAllOverW::new(self, 0)
    }
    #[doc = "Bit 1 - Wirte 1 to clear the corresponding interrupt"]
    #[inline(always)]
    pub fn intr_clr_fr_over(&mut self) -> IntrClrFrOverW<'_, IntrClrSpec> {
        IntrClrFrOverW::new(self, 1)
    }
    #[doc = "Bit 2 - Wirte 1 to clear the corresponding interrupt"]
    #[inline(always)]
    pub fn intr_clr_load_over(&mut self) -> IntrClrLoadOverW<'_, IntrClrSpec> {
        IntrClrLoadOverW::new(self, 2)
    }
    #[doc = "Bit 3 - Wirte 1 to clear the corresponding interrupt"]
    #[inline(always)]
    pub fn intr_clr_line_over(&mut self) -> IntrClrLineOverW<'_, IntrClrSpec> {
        IntrClrLineOverW::new(self, 3)
    }
    #[doc = "Bit 4 - Wirte 1 to clear the corresponding interrupt"]
    #[inline(always)]
    pub fn intr_clr_susp_inac(&mut self) -> IntrClrSuspInacW<'_, IntrClrSpec> {
        IntrClrSuspInacW::new(self, 4)
    }
    #[doc = "Bit 5 - Wirte 1 to clear the corresponding interrupt"]
    #[inline(always)]
    pub fn intr_clr_secure_err(&mut self) -> IntrClrSecureErrW<'_, IntrClrSpec> {
        IntrClrSecureErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Wirte 1 to clear the corresponding interrupt"]
    #[inline(always)]
    pub fn intr_clr_set_err(&mut self) -> IntrClrSetErrW<'_, IntrClrSpec> {
        IntrClrSetErrW::new(self, 6)
    }
    #[doc = "Bit 7 - Wirte 1 to clear the corresponding interrupt"]
    #[inline(always)]
    pub fn intr_clr_bus_err(&mut self) -> IntrClrBusErrW<'_, IntrClrSpec> {
        IntrClrBusErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Wirte 1 to clear the corresponding interrupt"]
    #[inline(always)]
    pub fn intr_clr_div0_err(&mut self) -> IntrClrDiv0ErrW<'_, IntrClrSpec> {
        IntrClrDiv0ErrW::new(self, 8)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrClrSpec;
impl crate::RegisterSpec for IntrClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intr_clr::W`](W) writer structure"]
impl crate::Writable for IntrClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_CLR to value 0"]
impl crate::Resettable for IntrClrSpec {}
