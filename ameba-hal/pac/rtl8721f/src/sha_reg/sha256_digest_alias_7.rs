#[doc = "Register `SHA256_digest_alias_7` reader"]
pub type R = crate::R<Sha256DigestAlias7Spec>;
#[doc = "Register `SHA256_digest_alias_7` writer"]
pub type W = crate::W<Sha256DigestAlias7Spec>;
#[doc = "Field `SHA256_Digest_alias_7` reader - SHA256 Hash value 7. Reset value is SHA384 IV."]
pub type Sha256DigestAlias7R = crate::FieldReader<u32>;
#[doc = "Field `SHA256_Digest_alias_7` writer - SHA256 Hash value 7. Reset value is SHA384 IV."]
pub type Sha256DigestAlias7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA256 Hash value 7. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha256_digest_alias_7(&self) -> Sha256DigestAlias7R {
        Sha256DigestAlias7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA256 Hash value 7. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha256_digest_alias_7(&mut self) -> Sha256DigestAlias7W<'_, Sha256DigestAlias7Spec> {
        Sha256DigestAlias7W::new(self, 0)
    }
}
#[doc = "SHA256 Hash value register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`sha256_digest_alias_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256_digest_alias_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sha256DigestAlias7Spec;
impl crate::RegisterSpec for Sha256DigestAlias7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha256_digest_alias_7::R`](R) reader structure"]
impl crate::Readable for Sha256DigestAlias7Spec {}
#[doc = "`write(|w| ..)` method takes [`sha256_digest_alias_7::W`](W) writer structure"]
impl crate::Writable for Sha256DigestAlias7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA256_digest_alias_7 to value 0x47b5_481d"]
impl crate::Resettable for Sha256DigestAlias7Spec {
    const RESET_VALUE: u32 = 0x47b5_481d;
}
