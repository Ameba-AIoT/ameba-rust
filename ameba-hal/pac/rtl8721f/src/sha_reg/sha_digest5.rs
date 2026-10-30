#[doc = "Register `SHA_Digest5` reader"]
pub type R = crate::R<ShaDigest5Spec>;
#[doc = "Register `SHA_Digest5` writer"]
pub type W = crate::W<ShaDigest5Spec>;
#[doc = "Field `SHA_Digest5` reader - Hash value 5. Reset value is SHA384 IV."]
pub type ShaDigest5R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest5` writer - Hash value 5. Reset value is SHA384 IV."]
pub type ShaDigest5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 5. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest5(&self) -> ShaDigest5R {
        ShaDigest5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 5. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest5(&mut self) -> ShaDigest5W<'_, ShaDigest5Spec> {
        ShaDigest5W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest5Spec;
impl crate::RegisterSpec for ShaDigest5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest5::R`](R) reader structure"]
impl crate::Readable for ShaDigest5Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest5::W`](W) writer structure"]
impl crate::Writable for ShaDigest5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest5 to value 0x3070_dd17"]
impl crate::Resettable for ShaDigest5Spec {
    const RESET_VALUE: u32 = 0x3070_dd17;
}
