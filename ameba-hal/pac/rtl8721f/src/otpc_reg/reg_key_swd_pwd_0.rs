#[doc = "Register `REG_KEY_SWD_PWD_0` reader"]
pub type R = crate::R<RegKeySwdPwd0Spec>;
#[doc = "Field `key_SWD_PWD_0` reader - "]
pub type KeySwdPwd0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_swd_pwd_0(&self) -> KeySwdPwd0R {
        KeySwdPwd0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_swd_pwd_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySwdPwd0Spec;
impl crate::RegisterSpec for RegKeySwdPwd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_swd_pwd_0::R`](R) reader structure"]
impl crate::Readable for RegKeySwdPwd0Spec {}
#[doc = "`reset()` method sets REG_KEY_SWD_PWD_0 to value 0xffff_ffff"]
impl crate::Resettable for RegKeySwdPwd0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
