#[doc = "Register `DMA_COMPONENT_ID_REGS_L` reader"]
pub type R = crate::R<DmaComponentIdRegsLSpec>;
#[doc = "Field `DMA_COMPONENT_ID` reader - DMAC ID (release_date_. Configure it in config form."]
pub type DmaComponentIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DMAC ID (release_date_. Configure it in config form."]
    #[inline(always)]
    pub fn dma_component_id(&self) -> DmaComponentIdR {
        DmaComponentIdR::new(self.bits)
    }
}
#[doc = "DMAC ID\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_component_id_regs_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaComponentIdRegsLSpec;
impl crate::RegisterSpec for DmaComponentIdRegsLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_component_id_regs_l::R`](R) reader structure"]
impl crate::Readable for DmaComponentIdRegsLSpec {}
#[doc = "`reset()` method sets DMA_COMPONENT_ID_REGS_L to value 0x2024_0513"]
impl crate::Resettable for DmaComponentIdRegsLSpec {
    const RESET_VALUE: u32 = 0x2024_0513;
}
