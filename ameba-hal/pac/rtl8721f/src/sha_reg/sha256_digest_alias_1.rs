#[doc = "Register `SHA256_digest_alias_1` reader"]
pub type R = crate::R<Sha256DigestAlias1Spec>;
#[doc = "Register `SHA256_digest_alias_1` writer"]
pub type W = crate::W<Sha256DigestAlias1Spec>;
#[doc = "Field `SHA256_Digest_alias_1` reader - SHA256 Hash value 1. Reset value is SHA384 IV."]
pub type Sha256DigestAlias1R = crate::FieldReader<u32>;
#[doc = "Field `SHA256_Digest_alias_1` writer - SHA256 Hash value 1. Reset value is SHA384 IV."]
pub type Sha256DigestAlias1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA256 Hash value 1. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha256_digest_alias_1(&self) -> Sha256DigestAlias1R {
        Sha256DigestAlias1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA256 Hash value 1. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha256_digest_alias_1(&mut self) -> Sha256DigestAlias1W<'_, Sha256DigestAlias1Spec> {
        Sha256DigestAlias1W::new(self, 0)
    }
}
#[doc = "SHA256 Hash value register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sha256_digest_alias_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256_digest_alias_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sha256DigestAlias1Spec;
impl crate::RegisterSpec for Sha256DigestAlias1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha256_digest_alias_1::R`](R) reader structure"]
impl crate::Readable for Sha256DigestAlias1Spec {}
#[doc = "`write(|w| ..)` method takes [`sha256_digest_alias_1::W`](W) writer structure"]
impl crate::Writable for Sha256DigestAlias1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA256_digest_alias_1 to value 0x629a_292a"]
impl crate::Resettable for Sha256DigestAlias1Spec {
    const RESET_VALUE: u32 = 0x629a_292a;
}
