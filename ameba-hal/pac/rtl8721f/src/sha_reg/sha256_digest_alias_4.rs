#[doc = "Register `SHA256_digest_alias_4` reader"]
pub type R = crate::R<Sha256DigestAlias4Spec>;
#[doc = "Register `SHA256_digest_alias_4` writer"]
pub type W = crate::W<Sha256DigestAlias4Spec>;
#[doc = "Field `SHA256_Digest_alias_4` reader - SHA256 Hash value 4. Reset value is SHA384 IV."]
pub type Sha256DigestAlias4R = crate::FieldReader<u32>;
#[doc = "Field `SHA256_Digest_alias_4` writer - SHA256 Hash value 4. Reset value is SHA384 IV."]
pub type Sha256DigestAlias4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA256 Hash value 4. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha256_digest_alias_4(&self) -> Sha256DigestAlias4R {
        Sha256DigestAlias4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA256 Hash value 4. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha256_digest_alias_4(&mut self) -> Sha256DigestAlias4W<'_, Sha256DigestAlias4Spec> {
        Sha256DigestAlias4W::new(self, 0)
    }
}
#[doc = "SHA256 Hash value register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sha256_digest_alias_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256_digest_alias_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sha256DigestAlias4Spec;
impl crate::RegisterSpec for Sha256DigestAlias4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha256_digest_alias_4::R`](R) reader structure"]
impl crate::Readable for Sha256DigestAlias4Spec {}
#[doc = "`write(|w| ..)` method takes [`sha256_digest_alias_4::W`](W) writer structure"]
impl crate::Writable for Sha256DigestAlias4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA256_digest_alias_4 to value 0x6733_2667"]
impl crate::Resettable for Sha256DigestAlias4Spec {
    const RESET_VALUE: u32 = 0x6733_2667;
}
