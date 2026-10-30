#[doc = "Register `REG_SWD_RMA_PWD_2` reader"]
pub type R = crate::R<RegSwdRmaPwd2Spec>;
#[doc = "Field `key_RMA_PWD_2` reader - "]
pub type KeyRmaPwd2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_rma_pwd_2(&self) -> KeyRmaPwd2R {
        KeyRmaPwd2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_swd_rma_pwd_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwdRmaPwd2Spec;
impl crate::RegisterSpec for RegSwdRmaPwd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_swd_rma_pwd_2::R`](R) reader structure"]
impl crate::Readable for RegSwdRmaPwd2Spec {}
#[doc = "`reset()` method sets REG_SWD_RMA_PWD_2 to value 0xffff_ffff"]
impl crate::Resettable for RegSwdRmaPwd2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
