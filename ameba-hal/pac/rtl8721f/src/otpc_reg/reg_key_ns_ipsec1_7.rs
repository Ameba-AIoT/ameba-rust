#[doc = "Register `REG_KEY_NS_IPSEC1_7` reader"]
pub type R = crate::R<RegKeyNsIpsec1_7Spec>;
#[doc = "Field `key_NS_IPSEC1_7` reader - "]
pub type KeyNsIpsec1_7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_ns_ipsec1_7(&self) -> KeyNsIpsec1_7R {
        KeyNsIpsec1_7R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_ns_ipsec1_7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyNsIpsec1_7Spec;
impl crate::RegisterSpec for RegKeyNsIpsec1_7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_ns_ipsec1_7::R`](R) reader structure"]
impl crate::Readable for RegKeyNsIpsec1_7Spec {}
#[doc = "`reset()` method sets REG_KEY_NS_IPSEC1_7 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyNsIpsec1_7Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
