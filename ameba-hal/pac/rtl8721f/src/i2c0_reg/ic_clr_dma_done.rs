#[doc = "Register `IC_CLR_DMA_DONE` reader"]
pub type R = crate::R<IcClrDmaDoneSpec>;
#[doc = "Field `IC_CLR_RAW_INTR_STAT` reader - Read this register to clear the DMA_DONE interrupt (bit 15) of IC_RAW_INTR_STAT register."]
pub type IcClrRawIntrStatR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read this register to clear the DMA_DONE interrupt (bit 15) of IC_RAW_INTR_STAT register."]
    #[inline(always)]
    pub fn ic_clr_raw_intr_stat(&self) -> IcClrRawIntrStatR {
        IcClrRawIntrStatR::new((self.bits & 1) != 0)
    }
}
#[doc = "Clear DMA DONE Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_clr_dma_done::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcClrDmaDoneSpec;
impl crate::RegisterSpec for IcClrDmaDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_clr_dma_done::R`](R) reader structure"]
impl crate::Readable for IcClrDmaDoneSpec {}
#[doc = "`reset()` method sets IC_CLR_DMA_DONE to value 0"]
impl crate::Resettable for IcClrDmaDoneSpec {}
