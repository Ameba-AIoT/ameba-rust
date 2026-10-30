#[doc = "Register `REG_KEY_NS_IPSEC1_6` reader"]
pub type R = crate::R<RegKeyNsIpsec1_6Spec>;
#[doc = "Field `key_NS_IPSEC1_6` reader - "]
pub type KeyNsIpsec1_6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_ns_ipsec1_6(&self) -> KeyNsIpsec1_6R {
        KeyNsIpsec1_6R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_ns_ipsec1_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyNsIpsec1_6Spec;
impl crate::RegisterSpec for RegKeyNsIpsec1_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_ns_ipsec1_6::R`](R) reader structure"]
impl crate::Readable for RegKeyNsIpsec1_6Spec {}
#[doc = "`reset()` method sets REG_KEY_NS_IPSEC1_6 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyNsIpsec1_6Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
