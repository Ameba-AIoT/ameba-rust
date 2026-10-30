#[doc = "Register `IC_CLR_INTR` reader"]
pub type R = crate::R<IcClrIntrSpec>;
#[doc = "Field `CLR_INRT` reader - Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts."]
pub type ClrInrtR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts."]
    #[inline(always)]
    pub fn clr_inrt(&self) -> ClrInrtR {
        ClrInrtR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear Combined and Individual Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_intr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrIntrSpec;
impl crate::RegisterSpec for IcClrIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_intr::R`](R) reader structure"]
impl crate::Readable for IcClrIntrSpec {}
#[doc = "`reset()` method sets IC_CLR_INTR to value 0"]
impl crate::Resettable for IcClrIntrSpec {}
