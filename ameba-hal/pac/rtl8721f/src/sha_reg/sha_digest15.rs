#[doc = "Register `SHA_Digest15` reader"]
pub type R = crate::R<ShaDigest15Spec>;
#[doc = "Register `SHA_Digest15` writer"]
pub type W = crate::W<ShaDigest15Spec>;
#[doc = "Field `SHA_Digest15` reader - Hash value 15. Reset value is SHA384 IV."]
pub type ShaDigest15R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest15` writer - Hash value 15. Reset value is SHA384 IV."]
pub type ShaDigest15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 15. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest15(&self) -> ShaDigest15R {
        ShaDigest15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 15. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest15(&mut self) -> ShaDigest15W<'_, ShaDigest15Spec> {
        ShaDigest15W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest15Spec;
impl crate::RegisterSpec for ShaDigest15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest15::R`](R) reader structure"]
impl crate::Readable for ShaDigest15Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest15::W`](W) writer structure"]
impl crate::Writable for ShaDigest15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest15 to value 0xbefa_4fa4"]
impl crate::Resettable for ShaDigest15Spec {
    const RESET_VALUE: u32 = 0xbefa_4fa4;
}
