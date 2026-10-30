#[doc = "Register `SHA_Digest14` reader"]
pub type R = crate::R<ShaDigest14Spec>;
#[doc = "Register `SHA_Digest14` writer"]
pub type W = crate::W<ShaDigest14Spec>;
#[doc = "Field `SHA_Digest14` reader - Hash value 14. Reset value is SHA384 IV."]
pub type ShaDigest14R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest14` writer - Hash value 14. Reset value is SHA384 IV."]
pub type ShaDigest14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 14. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest14(&self) -> ShaDigest14R {
        ShaDigest14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 14. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest14(&mut self) -> ShaDigest14W<'_, ShaDigest14Spec> {
        ShaDigest14W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest14Spec;
impl crate::RegisterSpec for ShaDigest14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest14::R`](R) reader structure"]
impl crate::Readable for ShaDigest14Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest14::W`](W) writer structure"]
impl crate::Writable for ShaDigest14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest14 to value 0x47b5_481d"]
impl crate::Resettable for ShaDigest14Spec {
    const RESET_VALUE: u32 = 0x47b5_481d;
}
