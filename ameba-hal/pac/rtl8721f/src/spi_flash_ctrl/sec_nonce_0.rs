#[doc = "Register `SEC_NONCE_0` reader"]
pub type R = crate::R<SecNonce0Spec>;
#[doc = "Register `SEC_NONCE_0` writer"]
pub type W = crate::W<SecNonce0Spec>;
#[doc = "Field `SEC_NONCE0` reader - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 0. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce0R = crate::FieldReader<u32>;
#[doc = "Field `SEC_NONCE0` writer - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 0. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 0. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce0(&self) -> SecNonce0R {
        SecNonce0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 0. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce0(&mut self) -> SecNonce0W<'_, SecNonce0Spec> {
        SecNonce0W::new(self, 0)
    }
}
#[doc = "SEC region 0 nonce register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_nonce_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_nonce_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecNonce0Spec;
impl crate::RegisterSpec for SecNonce0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_nonce_0::R`](R) reader structure"]
impl crate::Readable for SecNonce0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_nonce_0::W`](W) writer structure"]
impl crate::Writable for SecNonce0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_NONCE_0 to value 0"]
impl crate::Resettable for SecNonce0Spec {}
