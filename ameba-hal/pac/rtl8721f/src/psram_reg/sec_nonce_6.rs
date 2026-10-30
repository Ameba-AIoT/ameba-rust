#[doc = "Register `SEC_NONCE_6` reader"]
pub type R = crate::R<SecNonce6Spec>;
#[doc = "Register `SEC_NONCE_6` writer"]
pub type W = crate::W<SecNonce6Spec>;
#[doc = "Field `SEC_NONCE6` reader - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 6. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce6R = crate::FieldReader<u32>;
#[doc = "Field `SEC_NONCE6` writer - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 6. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 6. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce6(&self) -> SecNonce6R {
        SecNonce6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 6. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce6(&mut self) -> SecNonce6W<'_, SecNonce6Spec> {
        SecNonce6W::new(self, 0)
    }
}
#[doc = "SEC region 6 nonce register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_nonce_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_nonce_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecNonce6Spec;
impl crate::RegisterSpec for SecNonce6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_nonce_6::R`](R) reader structure"]
impl crate::Readable for SecNonce6Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_nonce_6::W`](W) writer structure"]
impl crate::Writable for SecNonce6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_NONCE_6 to value 0"]
impl crate::Resettable for SecNonce6Spec {}
