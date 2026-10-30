#[doc = "Register `REG_KEY_S_IPSEC1_4` reader"]
pub type R = crate::R<RegKeySIpsec1_4Spec>;
#[doc = "Field `key_S_IPSEC1_4` reader - "]
pub type KeySIpsec1_4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_s_ipsec1_4(&self) -> KeySIpsec1_4R {
        KeySIpsec1_4R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_s_ipsec1_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySIpsec1_4Spec;
impl crate::RegisterSpec for RegKeySIpsec1_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_s_ipsec1_4::R`](R) reader structure"]
impl crate::Readable for RegKeySIpsec1_4Spec {}
#[doc = "`reset()` method sets REG_KEY_S_IPSEC1_4 to value 0xffff_ffff"]
impl crate::Resettable for RegKeySIpsec1_4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
