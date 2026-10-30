#[doc = "Register `REG_FSISR_V1` reader"]
pub type R = crate::R<RegFsisrV1Spec>;
#[doc = "Field `BIT_FS_BTON_STS_UPDATE_INT` reader - BTON status update interrupt When 0x78\\[31\\] toggled, this interrupt is issue to indicate the status updated."]
pub type BitFsBtonStsUpdateIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PWMERR_INT_V1` reader - access page0 timeout error"]
pub type BitFsPwmerrIntV1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - BTON status update interrupt When 0x78\\[31\\] toggled, this interrupt is issue to indicate the status updated."]
    #[inline(always)]
    pub fn bit_fs_bton_sts_update_int(&self) -> BitFsBtonStsUpdateIntR {
        BitFsBtonStsUpdateIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - access page0 timeout error"]
    #[inline(always)]
    pub fn bit_fs_pwmerr_int_v1(&self) -> BitFsPwmerrIntV1R {
        BitFsPwmerrIntV1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fsisr_v1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFsisrV1Spec;
impl crate::RegisterSpec for RegFsisrV1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fsisr_v1::R`](R) reader structure"]
impl crate::Readable for RegFsisrV1Spec {}
#[doc = "`reset()` method sets REG_FSISR_V1 to value 0"]
impl crate::Resettable for RegFsisrV1Spec {}
