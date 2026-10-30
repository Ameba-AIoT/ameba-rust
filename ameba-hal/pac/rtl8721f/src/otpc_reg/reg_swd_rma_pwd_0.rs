#[doc = "Register `REG_SWD_RMA_PWD_0` reader"]
pub type R = crate::R<RegSwdRmaPwd0Spec>;
#[doc = "Field `key_RMA_PWD_0` reader - "]
pub type KeyRmaPwd0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rma_pwd_0(&self) -> KeyRmaPwd0R {
        KeyRmaPwd0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_swd_rma_pwd_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwdRmaPwd0Spec;
impl crate::RegisterSpec for RegSwdRmaPwd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_swd_rma_pwd_0::R`](R) reader structure"]
impl crate::Readable for RegSwdRmaPwd0Spec {}
#[doc = "`reset()` method sets REG_SWD_RMA_PWD_0 to value 0xffff_ffff"]
impl crate::Resettable for RegSwdRmaPwd0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
