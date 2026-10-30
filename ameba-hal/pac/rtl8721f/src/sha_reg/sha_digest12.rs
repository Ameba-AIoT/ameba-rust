#[doc = "Register `SHA_Digest12` reader"]
pub type R = crate::R<ShaDigest12Spec>;
#[doc = "Register `SHA_Digest12` writer"]
pub type W = crate::W<ShaDigest12Spec>;
#[doc = "Field `SHA_Digest12` reader - Hash value 12. Reset value is SHA384 IV."]
pub type ShaDigest12R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest12` writer - Hash value 12. Reset value is SHA384 IV."]
pub type ShaDigest12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 12. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest12(&self) -> ShaDigest12R {
        ShaDigest12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 12. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest12(&mut self) -> ShaDigest12W<'_, ShaDigest12Spec> {
        ShaDigest12W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest12Spec;
impl crate::RegisterSpec for ShaDigest12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest12::R`](R) reader structure"]
impl crate::Readable for ShaDigest12Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest12::W`](W) writer structure"]
impl crate::Writable for ShaDigest12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest12 to value 0xdb0c_2e0d"]
impl crate::Resettable for ShaDigest12Spec {
    const RESET_VALUE: u32 = 0xdb0c_2e0d;
}
