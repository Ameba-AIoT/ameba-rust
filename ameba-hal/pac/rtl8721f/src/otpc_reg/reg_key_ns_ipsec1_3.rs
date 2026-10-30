#[doc = "Register `REG_KEY_NS_IPSEC1_3` reader"]
pub type R = crate::R<RegKeyNsIpsec1_3Spec>;
#[doc = "Field `key_NS_IPSEC1_3` reader - "]
pub type KeyNsIpsec1_3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_ns_ipsec1_3(&self) -> KeyNsIpsec1_3R {
        KeyNsIpsec1_3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_ns_ipsec1_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyNsIpsec1_3Spec;
impl crate::RegisterSpec for RegKeyNsIpsec1_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_ns_ipsec1_3::R`](R) reader structure"]
impl crate::Readable for RegKeyNsIpsec1_3Spec {}
#[doc = "`reset()` method sets REG_KEY_NS_IPSEC1_3 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyNsIpsec1_3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
