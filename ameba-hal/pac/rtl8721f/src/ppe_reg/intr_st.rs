#[doc = "Register `INTR_ST` reader"]
pub type R = crate::R<IntrStSpec>;
#[doc = "Field `INTR_ST_ALL_OVER` reader - "]
pub type IntrStAllOverR = crate::BitReader;
#[doc = "Field `INTR_ST_FR_OVER` reader - "]
pub type IntrStFrOverR = crate::BitReader;
#[doc = "Field `INTR_ST_LOAD_OVER` reader - "]
pub type IntrStLoadOverR = crate::BitReader;
#[doc = "Field `INTR_ST_LINE_OVER` reader - "]
pub type IntrStLineOverR = crate::BitReader;
#[doc = "Field `INTR_ST_SUSP_INAC` reader - "]
pub type IntrStSuspInacR = crate::BitReader;
#[doc = "Field `INTR_ST_SECURE_ERR` reader - "]
pub type IntrStSecureErrR = crate::BitReader;
#[doc = "Field `INTR_ST_SET_ERR` reader - "]
pub type IntrStSetErrR = crate::BitReader;
#[doc = "Field `INTR_ST_BUS_ERR` reader - "]
pub type IntrStBusErrR = crate::BitReader;
#[doc = "Field `INTR_ST_DIV0_ERR` reader - "]
pub type IntrStDiv0ErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn intr_st_all_over(&self) -> IntrStAllOverR {
        IntrStAllOverR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn intr_st_fr_over(&self) -> IntrStFrOverR {
        IntrStFrOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn intr_st_load_over(&self) -> IntrStLoadOverR {
        IntrStLoadOverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn intr_st_line_over(&self) -> IntrStLineOverR {
        IntrStLineOverR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn intr_st_susp_inac(&self) -> IntrStSuspInacR {
        IntrStSuspInacR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn intr_st_secure_err(&self) -> IntrStSecureErrR {
        IntrStSecureErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn intr_st_set_err(&self) -> IntrStSetErrR {
        IntrStSetErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn intr_st_bus_err(&self) -> IntrStBusErrR {
        IntrStBusErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn intr_st_div0_err(&self) -> IntrStDiv0ErrR {
        IntrStDiv0ErrR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Interrupt status (raw & mask) register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrStSpec;
impl crate::RegisterSpec for IntrStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_st::R`](R) reader structure"]
impl crate::Readable for IntrStSpec {}
#[doc = "`reset()` method sets INTR_ST to value 0"]
impl crate::Resettable for IntrStSpec {}
