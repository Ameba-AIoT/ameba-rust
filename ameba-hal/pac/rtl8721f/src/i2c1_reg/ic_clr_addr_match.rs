#[doc = "Register `IC_CLR_ADDR_MATCH` reader"]
pub type R = crate::R<IcClrAddrMatchSpec>;
#[doc = "Field `IC_CLR_ADDR_MATCH` reader - Read this register to clear the slave mode address match interrupt (bit 12&13) of IC_RAW_INTR_STAT register."]
pub type IcClrAddrMatchR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the slave mode address match interrupt (bit 12&13) of IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn ic_clr_addr_match(&self) -> IcClrAddrMatchR {
        IcClrAddrMatchR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear Slave Mode Address Match Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_addr_match::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrAddrMatchSpec;
impl crate::RegisterSpec for IcClrAddrMatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_addr_match::R`](R) reader structure"]
impl crate::Readable for IcClrAddrMatchSpec {}
#[doc = "`reset()` method sets IC_CLR_ADDR_MATCH to value 0"]
impl crate::Resettable for IcClrAddrMatchSpec {}
