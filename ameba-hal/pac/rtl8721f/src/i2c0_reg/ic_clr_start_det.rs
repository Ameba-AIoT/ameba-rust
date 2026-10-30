#[doc = "Register `IC_CLR_START_DET` reader"]
pub type R = crate::R<IcClrStartDetSpec>;
#[doc = "Field `CLR_START_DET` reader - ead this register to clear the START_DET interrupt (bit 10) of IC_RAW_INTR_STAT register."]
pub type ClrStartDetR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ead this register to clear the START_DET interrupt (bit 10) of IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn clr_start_det(&self) -> ClrStartDetR {
        ClrStartDetR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear START_DET Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_start_det::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrStartDetSpec;
impl crate::RegisterSpec for IcClrStartDetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_start_det::R`](R) reader structure"]
impl crate::Readable for IcClrStartDetSpec {}
#[doc = "`reset()` method sets IC_CLR_START_DET to value 0"]
impl crate::Resettable for IcClrStartDetSpec {}
