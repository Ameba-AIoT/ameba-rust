#[doc = "Register `SEC_NONCE_3` reader"]
pub type R = crate::R<SecNonce3Spec>;
#[doc = "Register `SEC_NONCE_3` writer"]
pub type W = crate::W<SecNonce3Spec>;
#[doc = "Field `SEC_NONCE3` reader - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 3. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce3R = crate::FieldReader<u32>;
#[doc = "Field `SEC_NONCE3` writer - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 3. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 3. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce3(&self) -> SecNonce3R {
        SecNonce3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 3. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce3(&mut self) -> SecNonce3W<'_, SecNonce3Spec> {
        SecNonce3W::new(self, 0)
    }
}
#[doc = "SEC region 3 nonce register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_nonce_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_nonce_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecNonce3Spec;
impl crate::RegisterSpec for SecNonce3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_nonce_3::R`](R) reader structure"]
impl crate::Readable for SecNonce3Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_nonce_3::W`](W) writer structure"]
impl crate::Writable for SecNonce3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_NONCE_3 to value 0"]
impl crate::Resettable for SecNonce3Spec {}
