#[doc = "Register `INTR_RAW` reader"]
pub type R = crate::R<IntrRawSpec>;
#[doc = "Field `INTR_RAW_ALL_OVER` reader - All frames over Raised after all frames completion."]
pub type IntrRawAllOverR = crate::BitReader;
#[doc = "Field `INTR_RAW_FR_OVER` reader - Frame over Raised after every frame completion."]
pub type IntrRawFrOverR = crate::BitReader;
#[doc = "Field `INTR_RAW_LOAD_OVER` reader - Register load over Raised after layers registers load to internal DMA."]
pub type IntrRawLoadOverR = crate::BitReader;
#[doc = "Field `INTR_RAW_LINE_OVER` reader - Line index over Raised after the transaction of last pixel of LINE_INDEX marked line has been completed."]
pub type IntrRawLineOverR = crate::BitReader;
#[doc = "Field `INTR_RAW_SUSP_INAC` reader - Master port inactive after being suspended Raised after the master port is inactive and PPE is suspended."]
pub type IntrRawSuspInacR = crate::BitReader;
#[doc = "Field `INTR_RAW_SECURE_ERR` reader - Secure error Raised after non-secure APB request access secure PPE."]
pub type IntrRawSecureErrR = crate::BitReader;
#[doc = "Field `INTR_RAW_SET_ERR` reader - Setting error (Reserved)"]
pub type IntrRawSetErrR = crate::BitReader;
#[doc = "Field `INTR_RAW_BUS_ERR` reader - "]
pub type IntrRawBusErrR = crate::BitReader;
#[doc = "Field `INTR_RAW_DIV0_ERR` reader - "]
pub type IntrRawDiv0ErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - All frames over Raised after all frames completion."]
    #[inline(always)]
    pub fn intr_raw_all_over(&self) -> IntrRawAllOverR {
        IntrRawAllOverR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame over Raised after every frame completion."]
    #[inline(always)]
    pub fn intr_raw_fr_over(&self) -> IntrRawFrOverR {
        IntrRawFrOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register load over Raised after layers registers load to internal DMA."]
    #[inline(always)]
    pub fn intr_raw_load_over(&self) -> IntrRawLoadOverR {
        IntrRawLoadOverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line index over Raised after the transaction of last pixel of LINE_INDEX marked line has been completed."]
    #[inline(always)]
    pub fn intr_raw_line_over(&self) -> IntrRawLineOverR {
        IntrRawLineOverR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master port inactive after being suspended Raised after the master port is inactive and PPE is suspended."]
    #[inline(always)]
    pub fn intr_raw_susp_inac(&self) -> IntrRawSuspInacR {
        IntrRawSuspInacR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Secure error Raised after non-secure APB request access secure PPE."]
    #[inline(always)]
    pub fn intr_raw_secure_err(&self) -> IntrRawSecureErrR {
        IntrRawSecureErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Setting error (Reserved)"]
    #[inline(always)]
    pub fn intr_raw_set_err(&self) -> IntrRawSetErrR {
        IntrRawSetErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn intr_raw_bus_err(&self) -> IntrRawBusErrR {
        IntrRawBusErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn intr_raw_div0_err(&self) -> IntrRawDiv0ErrR {
        IntrRawDiv0ErrR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrRawSpec;
impl crate::RegisterSpec for IntrRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_raw::R`](R) reader structure"]
impl crate::Readable for IntrRawSpec {}
#[doc = "`reset()` method sets INTR_RAW to value 0"]
impl crate::Resettable for IntrRawSpec {}
