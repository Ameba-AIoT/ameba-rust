#[doc = "Register `SEC_NONCE_1` reader"]
pub type R = crate::R<SecNonce1Spec>;
#[doc = "Register `SEC_NONCE_1` writer"]
pub type W = crate::W<SecNonce1Spec>;
#[doc = "Field `SEC_NONCE1` reader - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 1. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce1R = crate::FieldReader<u32>;
#[doc = "Field `SEC_NONCE1` writer - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 1. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 1. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce1(&self) -> SecNonce1R {
        SecNonce1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 1. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce1(&mut self) -> SecNonce1W<'_, SecNonce1Spec> {
        SecNonce1W::new(self, 0)
    }
}
#[doc = "SEC region 1 nonce register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_nonce_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_nonce_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecNonce1Spec;
impl crate::RegisterSpec for SecNonce1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_nonce_1::R`](R) reader structure"]
impl crate::Readable for SecNonce1Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_nonce_1::W`](W) writer structure"]
impl crate::Writable for SecNonce1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_NONCE_1 to value 0"]
impl crate::Resettable for SecNonce1Spec {}
