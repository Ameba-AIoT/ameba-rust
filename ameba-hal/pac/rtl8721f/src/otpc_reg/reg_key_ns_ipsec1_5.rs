#[doc = "Register `REG_KEY_NS_IPSEC1_5` reader"]
pub type R = crate::R<RegKeyNsIpsec1_5Spec>;
#[doc = "Field `key_NS_IPSEC1_5` reader - "]
pub type KeyNsIpsec1_5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_ns_ipsec1_5(&self) -> KeyNsIpsec1_5R {
        KeyNsIpsec1_5R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_ns_ipsec1_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyNsIpsec1_5Spec;
impl crate::RegisterSpec for RegKeyNsIpsec1_5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_ns_ipsec1_5::R`](R) reader structure"]
impl crate::Readable for RegKeyNsIpsec1_5Spec {}
#[doc = "`reset()` method sets REG_KEY_NS_IPSEC1_5 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyNsIpsec1_5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
