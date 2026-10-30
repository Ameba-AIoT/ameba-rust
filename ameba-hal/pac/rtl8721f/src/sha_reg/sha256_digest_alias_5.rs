#[doc = "Register `SHA256_digest_alias_5` reader"]
pub type R = crate::R<Sha256DigestAlias5Spec>;
#[doc = "Register `SHA256_digest_alias_5` writer"]
pub type W = crate::W<Sha256DigestAlias5Spec>;
#[doc = "Field `SHA256_Digest_alias_5` reader - SHA256 Hash value 5. Reset value is SHA384 IV."]
pub type Sha256DigestAlias5R = crate::FieldReader<u32>;
#[doc = "Field `SHA256_Digest_alias_5` writer - SHA256 Hash value 5. Reset value is SHA384 IV."]
pub type Sha256DigestAlias5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA256 Hash value 5. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha256_digest_alias_5(&self) -> Sha256DigestAlias5R {
        Sha256DigestAlias5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA256 Hash value 5. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha256_digest_alias_5(&mut self) -> Sha256DigestAlias5W<'_, Sha256DigestAlias5Spec> {
        Sha256DigestAlias5W::new(self, 0)
    }
}
#[doc = "SHA256 Hash value register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`sha256_digest_alias_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256_digest_alias_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sha256DigestAlias5Spec;
impl crate::RegisterSpec for Sha256DigestAlias5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha256_digest_alias_5::R`](R) reader structure"]
impl crate::Readable for Sha256DigestAlias5Spec {}
#[doc = "`write(|w| ..)` method takes [`sha256_digest_alias_5::W`](W) writer structure"]
impl crate::Writable for Sha256DigestAlias5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA256_digest_alias_5 to value 0x8eb4_4a87"]
impl crate::Resettable for Sha256DigestAlias5Spec {
    const RESET_VALUE: u32 = 0x8eb4_4a87;
}
