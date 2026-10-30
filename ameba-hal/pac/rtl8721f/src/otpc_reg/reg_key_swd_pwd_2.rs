#[doc = "Register `REG_KEY_SWD_PWD_2` reader"]
pub type R = crate::R<RegKeySwdPwd2Spec>;
#[doc = "Field `key_SWD_PWD_2` reader - "]
pub type KeySwdPwd2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_swd_pwd_2(&self) -> KeySwdPwd2R {
        KeySwdPwd2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_swd_pwd_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySwdPwd2Spec;
impl crate::RegisterSpec for RegKeySwdPwd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_swd_pwd_2::R`](R) reader structure"]
impl crate::Readable for RegKeySwdPwd2Spec {}
#[doc = "`reset()` method sets REG_KEY_SWD_PWD_2 to value 0xffff_ffff"]
impl crate::Resettable for RegKeySwdPwd2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
