#[doc = "Register `REG_KEY_S_IPSEC1_1` reader"]
pub type R = crate::R<RegKeySIpsec1_1Spec>;
#[doc = "Field `key_S_IPSEC1_1` reader - "]
pub type KeySIpsec1_1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_s_ipsec1_1(&self) -> KeySIpsec1_1R {
        KeySIpsec1_1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_s_ipsec1_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySIpsec1_1Spec;
impl crate::RegisterSpec for RegKeySIpsec1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_s_ipsec1_1::R`](R) reader structure"]
impl crate::Readable for RegKeySIpsec1_1Spec {}
#[doc = "`reset()` method sets REG_KEY_S_IPSEC1_1 to value 0xffff_ffff"]
impl crate::Resettable for RegKeySIpsec1_1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
