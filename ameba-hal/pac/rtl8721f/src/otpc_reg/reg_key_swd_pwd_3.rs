#[doc = "Register `REG_KEY_SWD_PWD_3` reader"]
pub type R = crate::R<RegKeySwdPwd3Spec>;
#[doc = "Field `key_SWD_PWD_3` reader - "]
pub type KeySwdPwd3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_swd_pwd_3(&self) -> KeySwdPwd3R {
        KeySwdPwd3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_swd_pwd_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySwdPwd3Spec;
impl crate::RegisterSpec for RegKeySwdPwd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_swd_pwd_3::R`](R) reader structure"]
impl crate::Readable for RegKeySwdPwd3Spec {}
#[doc = "`reset()` method sets REG_KEY_SWD_PWD_3 to value 0xffff_ffff"]
impl crate::Resettable for RegKeySwdPwd3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
