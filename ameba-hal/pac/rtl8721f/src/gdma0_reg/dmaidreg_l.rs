#[doc = "Register `DMAIDREG_L` reader"]
pub type R = crate::R<DmaidregLSpec>;
#[doc = "Field `DMAIDREG_L_DMA_ID` reader - Hardcoded RTK_DMAC Peripheral ID"]
pub type DmaidregLDmaIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hardcoded RTK_DMAC Peripheral ID"]
    #[inline(always)]
    pub fn dmaidreg_l_dma_id(&self) -> DmaidregLDmaIdR {
        DmaidregLDmaIdR::new(self.bits)
    }
}
#[doc = "DMA low-32 bits ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaidreg_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaidregLSpec;
impl crate::RegisterSpec for DmaidregLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaidreg_l::R`](R) reader structure"]
impl crate::Readable for DmaidregLSpec {}
#[doc = "`reset()` method sets DMAIDREG_L to value 0x2024_0513"]
impl crate::Resettable for DmaidregLSpec {
    const RESET_VALUE: u32 = 0x2024_0513;
}
