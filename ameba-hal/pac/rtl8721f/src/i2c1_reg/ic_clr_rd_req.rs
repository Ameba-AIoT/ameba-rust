#[doc = "Register `IC_CLR_RD_REQ` reader"]
pub type R = crate::R<IcClrRdReqSpec>;
#[doc = "Field `CLR_RD_REQ` reader - Read this register to clear the RD_REQ interrupt (bit 5) of IC_RAW_INTR_STAT register."]
pub type ClrRdReqR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the RD_REQ interrupt (bit 5) of IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn clr_rd_req(&self) -> ClrRdReqR {
        ClrRdReqR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear RD_REQ Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_rd_req::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrRdReqSpec;
impl crate::RegisterSpec for IcClrRdReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_rd_req::R`](R) reader structure"]
impl crate::Readable for IcClrRdReqSpec {}
#[doc = "`reset()` method sets IC_CLR_RD_REQ to value 0"]
impl crate::Resettable for IcClrRdReqSpec {}
