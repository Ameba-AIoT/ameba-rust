#[doc = "Register `REG_SWD_RMA_PWD_3` reader"]
pub type R = crate::R<RegSwdRmaPwd3Spec>;
#[doc = "Field `key_RMA_PWD_3` reader - "]
pub type KeyRmaPwd3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rma_pwd_3(&self) -> KeyRmaPwd3R {
        KeyRmaPwd3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_swd_rma_pwd_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwdRmaPwd3Spec;
impl crate::RegisterSpec for RegSwdRmaPwd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_swd_rma_pwd_3::R`](R) reader structure"]
impl crate::Readable for RegSwdRmaPwd3Spec {}
#[doc = "`reset()` method sets REG_SWD_RMA_PWD_3 to value 0xffff_ffff"]
impl crate::Resettable for RegSwdRmaPwd3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
