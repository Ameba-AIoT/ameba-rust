#[doc = "Register `REG_TXDBG_INFO` reader"]
pub type R = crate::R<RegTxdbgInfoSpec>;
#[doc = "Field `BIT_SW_WR_TXLLT_FLAG` reader - 1: indicate SW write TXLLT write 1 to clear"]
pub type BitSwWrTxlltFlagR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: indicate SW write TXLLT write 1 to clear"]
    #[inline(always)]
    pub fn bit_sw_wr_txllt_flag(&self) -> BitSwWrTxlltFlagR {
        BitSwWrTxlltFlagR::new((self.bits & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txdbg_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxdbgInfoSpec;
impl crate::RegisterSpec for RegTxdbgInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txdbg_info::R`](R) reader structure"]
impl crate::Readable for RegTxdbgInfoSpec {}
#[doc = "`reset()` method sets REG_TXDBG_INFO to value 0"]
impl crate::Resettable for RegTxdbgInfoSpec {}
