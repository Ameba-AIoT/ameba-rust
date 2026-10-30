#[doc = "Register `REG_KEY_S_IPSEC2_6` reader"]
pub type R = crate::R<RegKeySIpsec2_6Spec>;
#[doc = "Field `key_S_IPSEC2_6` reader - "]
pub type KeySIpsec2_6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_s_ipsec2_6(&self) -> KeySIpsec2_6R {
        KeySIpsec2_6R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_s_ipsec2_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySIpsec2_6Spec;
impl crate::RegisterSpec for RegKeySIpsec2_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_s_ipsec2_6::R`](R) reader structure"]
impl crate::Readable for RegKeySIpsec2_6Spec {}
#[doc = "`reset()` method sets REG_KEY_S_IPSEC2_6 to value 0xffff_ffff"]
impl crate::Resettable for RegKeySIpsec2_6Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
