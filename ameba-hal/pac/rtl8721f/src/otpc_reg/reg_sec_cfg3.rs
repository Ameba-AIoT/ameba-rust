#[doc = "Register `REG_SEC_CFG3` reader"]
pub type R = crate::R<RegSecCfg3Spec>;
#[doc = "Field `RMA_SWD_PWD_R_Protection_EN` reader - "]
pub type RmaSwdPwdRProtectionEnR = crate::BitReader;
#[doc = "Field `RMA_SWD_PWD_W_Forbidden_EN` reader - "]
pub type RmaSwdPwdWForbiddenEnR = crate::BitReader;
#[doc = "Field `RMA_PK_W_Forbidden_EN` reader - "]
pub type RmaPkWForbiddenEnR = crate::BitReader;
#[doc = "Field `otp_rsvd0` reader - "]
pub type OtpRsvd0R = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rma_swd_pwd_r_protection_en(&self) -> RmaSwdPwdRProtectionEnR {
        RmaSwdPwdRProtectionEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rma_swd_pwd_w_forbidden_en(&self) -> RmaSwdPwdWForbiddenEnR {
        RmaSwdPwdWForbiddenEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rma_pk_w_forbidden_en(&self) -> RmaPkWForbiddenEnR {
        RmaPkWForbiddenEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn otp_rsvd0(&self) -> OtpRsvd0R {
        OtpRsvd0R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sec_cfg3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSecCfg3Spec;
impl crate::RegisterSpec for RegSecCfg3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_sec_cfg3::R`](R) reader structure"]
impl crate::Readable for RegSecCfg3Spec {}
#[doc = "`reset()` method sets REG_SEC_CFG3 to value 0xff"]
impl crate::Resettable for RegSecCfg3Spec {
    const RESET_VALUE: u16 = 0xff;
}
