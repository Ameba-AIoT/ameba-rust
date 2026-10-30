#[doc = "Register `SHA_Digest0` reader"]
pub type R = crate::R<ShaDigest0Spec>;
#[doc = "Register `SHA_Digest0` writer"]
pub type W = crate::W<ShaDigest0Spec>;
#[doc = "Field `SHA_Digest0` reader - Hash value 0. Reset value is SHA384 IV."]
pub type ShaDigest0R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest0` writer - Hash value 0. Reset value is SHA384 IV."]
pub type ShaDigest0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 0. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest0(&self) -> ShaDigest0R {
        ShaDigest0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 0. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest0(&mut self) -> ShaDigest0W<'_, ShaDigest0Spec> {
        ShaDigest0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest0Spec;
impl crate::RegisterSpec for ShaDigest0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest0::R`](R) reader structure"]
impl crate::Readable for ShaDigest0Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest0::W`](W) writer structure"]
impl crate::Writable for ShaDigest0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest0 to value 0xc105_9ed8"]
impl crate::Resettable for ShaDigest0Spec {
    const RESET_VALUE: u32 = 0xc105_9ed8;
}
