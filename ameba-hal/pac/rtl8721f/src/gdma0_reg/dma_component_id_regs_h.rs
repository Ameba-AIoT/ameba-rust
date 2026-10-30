#[doc = "Register `DMA_COMPONENT_ID_REGS_H` reader"]
pub type R = crate::R<DmaComponentIdRegsHSpec>;
#[doc = "Field `RTL_GIT_VER` reader - It is obtained from the high 32 bits from git logs in git server."]
pub type RtlGitVerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - It is obtained from the high 32 bits from git logs in git server."]
    #[inline(always)]
    pub fn rtl_git_ver(&self) -> RtlGitVerR {
        RtlGitVerR::new(self.bits)
    }
}
#[doc = "It is obtained from the high 32 bits from git logs in git server.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_component_id_regs_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaComponentIdRegsHSpec;
impl crate::RegisterSpec for DmaComponentIdRegsHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_component_id_regs_h::R`](R) reader structure"]
impl crate::Readable for DmaComponentIdRegsHSpec {}
#[doc = "`reset()` method sets DMA_COMPONENT_ID_REGS_H to value 0x482f_5d83"]
impl crate::Resettable for DmaComponentIdRegsHSpec {
    const RESET_VALUE: u32 = 0x482f_5d83;
}
