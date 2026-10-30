#[doc = "Register `REG_WMAC_RTX_SWAES_RD_3` reader"]
pub type R = crate::R<RegWmacRtxSwaesRd3Spec>;
#[doc = "Field `wmac_swaes_rd_2` reader - "]
pub type WmacSwaesRd2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wmac_swaes_rd_2(&self) -> WmacSwaesRd2R {
        WmacSwaesRd2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_rtx_swaes_rd_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacRtxSwaesRd3Spec;
impl crate::RegisterSpec for RegWmacRtxSwaesRd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_rtx_swaes_rd_3::R`](R) reader structure"]
impl crate::Readable for RegWmacRtxSwaesRd3Spec {}
#[doc = "`reset()` method sets REG_WMAC_RTX_SWAES_RD_3 to value 0"]
impl crate::Resettable for RegWmacRtxSwaesRd3Spec {}
