#[doc = "Register `SEC_NONCE_5` reader"]
pub type R = crate::R<SecNonce5Spec>;
#[doc = "Register `SEC_NONCE_5` writer"]
pub type W = crate::W<SecNonce5Spec>;
#[doc = "Field `SEC_NONCE5` reader - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 5. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce5R = crate::FieldReader<u32>;
#[doc = "Field `SEC_NONCE5` writer - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 5. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 5. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce5(&self) -> SecNonce5R {
        SecNonce5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 5. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce5(&mut self) -> SecNonce5W<'_, SecNonce5Spec> {
        SecNonce5W::new(self, 0)
    }
}
#[doc = "SEC region 5 nonce register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_nonce_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_nonce_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecNonce5Spec;
impl crate::RegisterSpec for SecNonce5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_nonce_5::R`](R) reader structure"]
impl crate::Readable for SecNonce5Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_nonce_5::W`](W) writer structure"]
impl crate::Writable for SecNonce5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_NONCE_5 to value 0"]
impl crate::Resettable for SecNonce5Spec {}
