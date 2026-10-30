#[doc = "Register `REG_WMAC_RTX_SWAES_RD_1` reader"]
pub type R = crate::R<RegWmacRtxSwaesRd1Spec>;
#[doc = "Field `wmac_swaes_rd_0` reader - "]
pub type WmacSwaesRd0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wmac_swaes_rd_0(&self) -> WmacSwaesRd0R {
        WmacSwaesRd0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_rtx_swaes_rd_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacRtxSwaesRd1Spec;
impl crate::RegisterSpec for RegWmacRtxSwaesRd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_rtx_swaes_rd_1::R`](R) reader structure"]
impl crate::Readable for RegWmacRtxSwaesRd1Spec {}
#[doc = "`reset()` method sets REG_WMAC_RTX_SWAES_RD_1 to value 0"]
impl crate::Resettable for RegWmacRtxSwaesRd1Spec {}
