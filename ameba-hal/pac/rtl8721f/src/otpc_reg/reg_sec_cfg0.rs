#[doc = "Register `REG_SEC_CFG0` reader"]
pub type R = crate::R<RegSecCfg0Spec>;
#[doc = "Field `swd_id` reader - "]
pub type SwdIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn swd_id(&self) -> SwdIdR {
        SwdIdR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sec_cfg0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSecCfg0Spec;
impl crate::RegisterSpec for RegSecCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sec_cfg0::R`](R) reader structure"]
impl crate::Readable for RegSecCfg0Spec {}
#[doc = "`reset()` method sets REG_SEC_CFG0 to value 0xffff_ffff"]
impl crate::Resettable for RegSecCfg0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
