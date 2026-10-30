#[doc = "Register `SHA_Digest2` reader"]
pub type R = crate::R<ShaDigest2Spec>;
#[doc = "Register `SHA_Digest2` writer"]
pub type W = crate::W<ShaDigest2Spec>;
#[doc = "Field `SHA_Digest2` reader - Hash value 2. Reset value is SHA384 IV."]
pub type ShaDigest2R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest2` writer - Hash value 2. Reset value is SHA384 IV."]
pub type ShaDigest2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 2. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest2(&self) -> ShaDigest2R {
        ShaDigest2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 2. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest2(&mut self) -> ShaDigest2W<'_, ShaDigest2Spec> {
        ShaDigest2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest2Spec;
impl crate::RegisterSpec for ShaDigest2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest2::R`](R) reader structure"]
impl crate::Readable for ShaDigest2Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest2::W`](W) writer structure"]
impl crate::Writable for ShaDigest2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest2 to value 0x629a_292a"]
impl crate::Resettable for ShaDigest2Spec {
    const RESET_VALUE: u32 = 0x629a_292a;
}
