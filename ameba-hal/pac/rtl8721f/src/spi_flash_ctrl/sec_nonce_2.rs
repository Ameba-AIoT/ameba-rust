#[doc = "Register `SEC_NONCE_2` reader"]
pub type R = crate::R<SecNonce2Spec>;
#[doc = "Register `SEC_NONCE_2` writer"]
pub type W = crate::W<SecNonce2Spec>;
#[doc = "Field `SEC_NONCE2` reader - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 2. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce2R = crate::FieldReader<u32>;
#[doc = "Field `SEC_NONCE2` writer - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 2. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 2. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce2(&self) -> SecNonce2R {
        SecNonce2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 2. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce2(&mut self) -> SecNonce2W<'_, SecNonce2Spec> {
        SecNonce2W::new(self, 0)
    }
}
#[doc = "SEC region 2 nonce register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_nonce_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_nonce_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecNonce2Spec;
impl crate::RegisterSpec for SecNonce2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_nonce_2::R`](R) reader structure"]
impl crate::Readable for SecNonce2Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_nonce_2::W`](W) writer structure"]
impl crate::Writable for SecNonce2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_NONCE_2 to value 0"]
impl crate::Resettable for SecNonce2Spec {}
