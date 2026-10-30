#[doc = "Register `REG_KEY_NS_IPSEC2_3` reader"]
pub type R = crate::R<RegKeyNsIpsec2_3Spec>;
#[doc = "Field `key_NS_IPSEC2_3` reader - "]
pub type KeyNsIpsec2_3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_ns_ipsec2_3(&self) -> KeyNsIpsec2_3R {
        KeyNsIpsec2_3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_ns_ipsec2_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeyNsIpsec2_3Spec;
impl crate::RegisterSpec for RegKeyNsIpsec2_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_ns_ipsec2_3::R`](R) reader structure"]
impl crate::Readable for RegKeyNsIpsec2_3Spec {}
#[doc = "`reset()` method sets REG_KEY_NS_IPSEC2_3 to value 0xffff_ffff"]
impl crate::Resettable for RegKeyNsIpsec2_3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
