#[doc = "Register `SHA_Digest13` reader"]
pub type R = crate::R<ShaDigest13Spec>;
#[doc = "Register `SHA_Digest13` writer"]
pub type W = crate::W<ShaDigest13Spec>;
#[doc = "Field `SHA_Digest13` reader - Hash value 13. Reset value is SHA384 IV."]
pub type ShaDigest13R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest13` writer - Hash value 13. Reset value is SHA384 IV."]
pub type ShaDigest13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 13. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest13(&self) -> ShaDigest13R {
        ShaDigest13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 13. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest13(&mut self) -> ShaDigest13W<'_, ShaDigest13Spec> {
        ShaDigest13W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest13Spec;
impl crate::RegisterSpec for ShaDigest13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest13::R`](R) reader structure"]
impl crate::Readable for ShaDigest13Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest13::W`](W) writer structure"]
impl crate::Writable for ShaDigest13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest13 to value 0x64f9_8fa7"]
impl crate::Resettable for ShaDigest13Spec {
    const RESET_VALUE: u32 = 0x64f9_8fa7;
}
