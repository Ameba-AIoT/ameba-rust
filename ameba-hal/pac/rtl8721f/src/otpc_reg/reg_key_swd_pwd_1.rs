#[doc = "Register `REG_KEY_SWD_PWD_1` reader"]
pub type R = crate::R<RegKeySwdPwd1Spec>;
#[doc = "Field `key_SWD_PWD_1` reader - "]
pub type KeySwdPwd1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_swd_pwd_1(&self) -> KeySwdPwd1R {
        KeySwdPwd1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_swd_pwd_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySwdPwd1Spec;
impl crate::RegisterSpec for RegKeySwdPwd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_swd_pwd_1::R`](R) reader structure"]
impl crate::Readable for RegKeySwdPwd1Spec {}
#[doc = "`reset()` method sets REG_KEY_SWD_PWD_1 to value 0xffff_ffff"]
impl crate::Resettable for RegKeySwdPwd1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
