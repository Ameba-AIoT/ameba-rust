#[doc = "Register `SHA_Digest1` reader"]
pub type R = crate::R<ShaDigest1Spec>;
#[doc = "Register `SHA_Digest1` writer"]
pub type W = crate::W<ShaDigest1Spec>;
#[doc = "Field `SHA_Digest1` reader - Hash value 1. Reset value is SHA384 IV."]
pub type ShaDigest1R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest1` writer - Hash value 1. Reset value is SHA384 IV."]
pub type ShaDigest1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 1. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest1(&self) -> ShaDigest1R {
        ShaDigest1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 1. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest1(&mut self) -> ShaDigest1W<'_, ShaDigest1Spec> {
        ShaDigest1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest1Spec;
impl crate::RegisterSpec for ShaDigest1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest1::R`](R) reader structure"]
impl crate::Readable for ShaDigest1Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest1::W`](W) writer structure"]
impl crate::Writable for ShaDigest1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest1 to value 0xc105_9ed8"]
impl crate::Resettable for ShaDigest1Spec {
    const RESET_VALUE: u32 = 0xc105_9ed8;
}
