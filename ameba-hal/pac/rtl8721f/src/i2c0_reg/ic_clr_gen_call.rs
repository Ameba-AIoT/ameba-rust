#[doc = "Register `IC_CLR_GEN_CALL` reader"]
pub type R = crate::R<IcClrGenCallSpec>;
#[doc = "Field `CLR_GEN_CALL` reader - ead this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register."]
pub type ClrGenCallR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ead this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn clr_gen_call(&self) -> ClrGenCallR {
        ClrGenCallR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear GEN_ALL Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_gen_call::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrGenCallSpec;
impl crate::RegisterSpec for IcClrGenCallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_gen_call::R`](R) reader structure"]
impl crate::Readable for IcClrGenCallSpec {}
#[doc = "`reset()` method sets IC_CLR_GEN_CALL to value 0"]
impl crate::Resettable for IcClrGenCallSpec {}
