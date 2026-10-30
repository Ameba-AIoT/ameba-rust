#[doc = "Register `SHA_Digest11` reader"]
pub type R = crate::R<ShaDigest11Spec>;
#[doc = "Register `SHA_Digest11` writer"]
pub type W = crate::W<ShaDigest11Spec>;
#[doc = "Field `SHA_Digest11` reader - Hash value 11. Reset value is SHA384 IV."]
pub type ShaDigest11R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest11` writer - Hash value 11. Reset value is SHA384 IV."]
pub type ShaDigest11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 11. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest11(&self) -> ShaDigest11R {
        ShaDigest11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 11. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest11(&mut self) -> ShaDigest11W<'_, ShaDigest11Spec> {
        ShaDigest11W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest11Spec;
impl crate::RegisterSpec for ShaDigest11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest11::R`](R) reader structure"]
impl crate::Readable for ShaDigest11Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest11::W`](W) writer structure"]
impl crate::Writable for ShaDigest11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest11 to value 0x6858_1511"]
impl crate::Resettable for ShaDigest11Spec {
    const RESET_VALUE: u32 = 0x6858_1511;
}
