#[doc = "Register `SHA_Digest8` reader"]
pub type R = crate::R<ShaDigest8Spec>;
#[doc = "Register `SHA_Digest8` writer"]
pub type W = crate::W<ShaDigest8Spec>;
#[doc = "Field `SHA_Digest8` reader - Hash value 8. Reset value is SHA384 IV."]
pub type ShaDigest8R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest8` writer - Hash value 8. Reset value is SHA384 IV."]
pub type ShaDigest8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 8. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest8(&self) -> ShaDigest8R {
        ShaDigest8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 8. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest8(&mut self) -> ShaDigest8W<'_, ShaDigest8Spec> {
        ShaDigest8W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest8Spec;
impl crate::RegisterSpec for ShaDigest8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest8::R`](R) reader structure"]
impl crate::Readable for ShaDigest8Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest8::W`](W) writer structure"]
impl crate::Writable for ShaDigest8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest8 to value 0x6733_2667"]
impl crate::Resettable for ShaDigest8Spec {
    const RESET_VALUE: u32 = 0x6733_2667;
}
