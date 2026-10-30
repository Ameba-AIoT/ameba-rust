#[doc = "Register `REG_KEY_S_IPSEC2_2` reader"]
pub type R = crate::R<RegKeySIpsec2_2Spec>;
#[doc = "Field `key_S_IPSEC2_2` reader - "]
pub type KeySIpsec2_2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key_s_ipsec2_2(&self) -> KeySIpsec2_2R {
        KeySIpsec2_2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_s_ipsec2_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySIpsec2_2Spec;
impl crate::RegisterSpec for RegKeySIpsec2_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_s_ipsec2_2::R`](R) reader structure"]
impl crate::Readable for RegKeySIpsec2_2Spec {}
#[doc = "`reset()` method sets REG_KEY_S_IPSEC2_2 to value 0xffff_ffff"]
impl crate::Resettable for RegKeySIpsec2_2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
