#[doc = "Register `SHA_Digest4` reader"]
pub type R = crate::R<ShaDigest4Spec>;
#[doc = "Register `SHA_Digest4` writer"]
pub type W = crate::W<ShaDigest4Spec>;
#[doc = "Field `SHA_Digest4` reader - Hash value 4. Reset value is SHA384 IV."]
pub type ShaDigest4R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest4` writer - Hash value 4. Reset value is SHA384 IV."]
pub type ShaDigest4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 4. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest4(&self) -> ShaDigest4R {
        ShaDigest4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 4. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest4(&mut self) -> ShaDigest4W<'_, ShaDigest4Spec> {
        ShaDigest4W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest4Spec;
impl crate::RegisterSpec for ShaDigest4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest4::R`](R) reader structure"]
impl crate::Readable for ShaDigest4Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest4::W`](W) writer structure"]
impl crate::Writable for ShaDigest4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest4 to value 0x9159_015a"]
impl crate::Resettable for ShaDigest4Spec {
    const RESET_VALUE: u32 = 0x9159_015a;
}
