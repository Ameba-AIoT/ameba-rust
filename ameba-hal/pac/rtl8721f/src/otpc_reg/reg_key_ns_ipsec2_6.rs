#[doc = "Register `REG_KEY_NS_IPSEC2_6` reader"]
pub type R = crate::R<RegKeyNsIpsec2_6Spec>;
#[doc = "Field `key_NS_IPSEC2_6` reader - "]
pub type KeyNsIpsec2_6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_ns_ipsec2_6(&self) -> KeyNsIpsec2_6R {
        KeyNsIpsec2_6R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_ns_ipsec2_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyNsIpsec2_6Spec;
impl crate::RegisterSpec for RegKeyNsIpsec2_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_ns_ipsec2_6::R`](R) reader structure"]
impl crate::Readable for RegKeyNsIpsec2_6Spec {}
#[doc = "`reset()` method sets REG_KEY_NS_IPSEC2_6 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyNsIpsec2_6Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
