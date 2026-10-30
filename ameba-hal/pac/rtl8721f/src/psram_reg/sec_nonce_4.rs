#[doc = "Register `SEC_NONCE_4` reader"]
pub type R = crate::R<SecNonce4Spec>;
#[doc = "Register `SEC_NONCE_4` writer"]
pub type W = crate::W<SecNonce4Spec>;
#[doc = "Field `SEC_NONCE4` reader - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 4. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce4R = crate::FieldReader<u32>;
#[doc = "Field `SEC_NONCE4` writer - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 4. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
pub type SecNonce4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 4. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce4(&self) -> SecNonce4R {
        SecNonce4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Only present if SEC_CONF_MODE > 0. Seed of scrambler mode 4. This register value cannot be 0, otherwise the encrypted data will be equal to the original data. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn sec_nonce4(&mut self) -> SecNonce4W<'_, SecNonce4Spec> {
        SecNonce4W::new(self, 0)
    }
}
#[doc = "SEC region 4 nonce register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_nonce_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_nonce_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecNonce4Spec;
impl crate::RegisterSpec for SecNonce4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_nonce_4::R`](R) reader structure"]
impl crate::Readable for SecNonce4Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_nonce_4::W`](W) writer structure"]
impl crate::Writable for SecNonce4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_NONCE_4 to value 0"]
impl crate::Resettable for SecNonce4Spec {}
