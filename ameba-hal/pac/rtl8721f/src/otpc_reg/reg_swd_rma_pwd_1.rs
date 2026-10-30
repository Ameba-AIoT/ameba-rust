#[doc = "Register `REG_SWD_RMA_PWD_1` reader"]
pub type R = crate::R<RegSwdRmaPwd1Spec>;
#[doc = "Field `key_RMA_PWD_1` reader - "]
pub type KeyRmaPwd1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rma_pwd_1(&self) -> KeyRmaPwd1R {
        KeyRmaPwd1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_swd_rma_pwd_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwdRmaPwd1Spec;
impl crate::RegisterSpec for RegSwdRmaPwd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_swd_rma_pwd_1::R`](R) reader structure"]
impl crate::Readable for RegSwdRmaPwd1Spec {}
#[doc = "`reset()` method sets REG_SWD_RMA_PWD_1 to value 0xffff_ffff"]
impl crate::Resettable for RegSwdRmaPwd1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
