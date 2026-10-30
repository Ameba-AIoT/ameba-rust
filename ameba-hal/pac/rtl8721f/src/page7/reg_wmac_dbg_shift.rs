#[doc = "Register `REG_WMAC_DBG_SHIFT` reader"]
pub type R = crate::R<RegWmacDbgShiftSpec>;
#[doc = "Field `BIT_MACTX_CMD_SHIFT` reader - "]
pub type BitMactxCmdShiftR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bit_mactx_cmd_shift(&self) -> BitMactxCmdShiftR {
        BitMactxCmdShiftR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_dbg_shift::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacDbgShiftSpec;
impl crate::RegisterSpec for RegWmacDbgShiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_dbg_shift::R`](R) reader structure"]
impl crate::Readable for RegWmacDbgShiftSpec {}
#[doc = "`reset()` method sets REG_WMAC_DBG_SHIFT to value 0"]
impl crate::Resettable for RegWmacDbgShiftSpec {}
