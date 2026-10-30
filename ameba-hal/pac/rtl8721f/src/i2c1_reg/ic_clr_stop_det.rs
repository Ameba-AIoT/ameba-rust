#[doc = "Register `IC_CLR_STOP_DET` reader"]
pub type R = crate::R<IcClrStopDetSpec>;
#[doc = "Field `CLR_STOP_DET` reader - ead this register to clear the STOP_DET interrupt (bit 9) of IC_RAW_INTR_STAT register."]
pub type ClrStopDetR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ead this register to clear the STOP_DET interrupt (bit 9) of IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn clr_stop_det(&self) -> ClrStopDetR {
        ClrStopDetR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear STOP_DET Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_stop_det::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrStopDetSpec;
impl crate::RegisterSpec for IcClrStopDetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_stop_det::R`](R) reader structure"]
impl crate::Readable for IcClrStopDetSpec {}
#[doc = "`reset()` method sets IC_CLR_STOP_DET to value 0"]
impl crate::Resettable for IcClrStopDetSpec {}
