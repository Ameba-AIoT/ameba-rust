#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `INTR_MASK_ALL_OVER` reader - "]
pub type IntrMaskAllOverR = crate::BitReader;
#[doc = "Field `INTR_MASK_ALL_OVER` writer - "]
pub type IntrMaskAllOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_MASK_FR_OVER` reader - "]
pub type IntrMaskFrOverR = crate::BitReader;
#[doc = "Field `INTR_MASK_FR_OVER` writer - "]
pub type IntrMaskFrOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_MASK_LOAD_OVER` reader - "]
pub type IntrMaskLoadOverR = crate::BitReader;
#[doc = "Field `INTR_MASK_LOAD_OVER` writer - "]
pub type IntrMaskLoadOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_MASK_LINE_OVER` reader - "]
pub type IntrMaskLineOverR = crate::BitReader;
#[doc = "Field `INTR_MASK_LINE_OVER` writer - "]
pub type IntrMaskLineOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_MASK_SUSP_INAC` reader - "]
pub type IntrMaskSuspInacR = crate::BitReader;
#[doc = "Field `INTR_MASK_SUSP_INAC` writer - "]
pub type IntrMaskSuspInacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_MASK_SECURE_ERR` reader - "]
pub type IntrMaskSecureErrR = crate::BitReader;
#[doc = "Field `INTR_MASK_SECURE_ERR` writer - "]
pub type IntrMaskSecureErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_MASK_SET_ERR` reader - "]
pub type IntrMaskSetErrR = crate::BitReader;
#[doc = "Field `INTR_MASK_SET_ERR` writer - "]
pub type IntrMaskSetErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_MASK_BUS_ERR` reader - "]
pub type IntrMaskBusErrR = crate::BitReader;
#[doc = "Field `INTR_MASK_BUS_ERR` writer - "]
pub type IntrMaskBusErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_MASK_DIV0_ERR` reader - "]
pub type IntrMaskDiv0ErrR = crate::BitReader;
#[doc = "Field `INTR_MASK_DIV0_ERR` writer - "]
pub type IntrMaskDiv0ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn intr_mask_all_over(&self) -> IntrMaskAllOverR {
        IntrMaskAllOverR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn intr_mask_fr_over(&self) -> IntrMaskFrOverR {
        IntrMaskFrOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn intr_mask_load_over(&self) -> IntrMaskLoadOverR {
        IntrMaskLoadOverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn intr_mask_line_over(&self) -> IntrMaskLineOverR {
        IntrMaskLineOverR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn intr_mask_susp_inac(&self) -> IntrMaskSuspInacR {
        IntrMaskSuspInacR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn intr_mask_secure_err(&self) -> IntrMaskSecureErrR {
        IntrMaskSecureErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn intr_mask_set_err(&self) -> IntrMaskSetErrR {
        IntrMaskSetErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn intr_mask_bus_err(&self) -> IntrMaskBusErrR {
        IntrMaskBusErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn intr_mask_div0_err(&self) -> IntrMaskDiv0ErrR {
        IntrMaskDiv0ErrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn intr_mask_all_over(&mut self) -> IntrMaskAllOverW<'_, IntrMaskSpec> {
        IntrMaskAllOverW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn intr_mask_fr_over(&mut self) -> IntrMaskFrOverW<'_, IntrMaskSpec> {
        IntrMaskFrOverW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn intr_mask_load_over(&mut self) -> IntrMaskLoadOverW<'_, IntrMaskSpec> {
        IntrMaskLoadOverW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn intr_mask_line_over(&mut self) -> IntrMaskLineOverW<'_, IntrMaskSpec> {
        IntrMaskLineOverW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn intr_mask_susp_inac(&mut self) -> IntrMaskSuspInacW<'_, IntrMaskSpec> {
        IntrMaskSuspInacW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn intr_mask_secure_err(&mut self) -> IntrMaskSecureErrW<'_, IntrMaskSpec> {
        IntrMaskSecureErrW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn intr_mask_set_err(&mut self) -> IntrMaskSetErrW<'_, IntrMaskSpec> {
        IntrMaskSetErrW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn intr_mask_bus_err(&mut self) -> IntrMaskBusErrW<'_, IntrMaskSpec> {
        IntrMaskBusErrW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn intr_mask_div0_err(&mut self) -> IntrMaskDiv0ErrW<'_, IntrMaskSpec> {
        IntrMaskDiv0ErrW::new(self, 8)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskSpec;
impl crate::RegisterSpec for IntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for IntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for IntrMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for IntrMaskSpec {}
