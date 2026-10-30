#[doc = "Register `REG_RXDMA_DPR` reader"]
pub type R = crate::R<RegRxdmaDprSpec>;
#[doc = "Field `BIT_RDE_DBG` reader - rde debug port register."]
pub type BitRdeDbgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - rde debug port register."]
    #[inline(always)]
    pub fn bit_rde_dbg(&self) -> BitRdeDbgR {
        BitRdeDbgR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxdma_dpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxdmaDprSpec;
impl crate::RegisterSpec for RegRxdmaDprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxdma_dpr::R`](R) reader structure"]
impl crate::Readable for RegRxdmaDprSpec {}
#[doc = "`reset()` method sets REG_RXDMA_DPR to value 0"]
impl crate::Resettable for RegRxdmaDprSpec {}
