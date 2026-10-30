#[doc = "Register `SHA_Digest3` reader"]
pub type R = crate::R<ShaDigest3Spec>;
#[doc = "Register `SHA_Digest3` writer"]
pub type W = crate::W<ShaDigest3Spec>;
#[doc = "Field `SHA_Digest3` reader - Hash value 3. Reset value is SHA384 IV."]
pub type ShaDigest3R = crate::FieldReader<u32>;
#[doc = "Field `SHA_Digest3` writer - Hash value 3. Reset value is SHA384 IV."]
pub type ShaDigest3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash value 3. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest3(&self) -> ShaDigest3R {
        ShaDigest3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash value 3. Reset value is SHA384 IV."]
    #[inline(always)]
    pub fn sha_digest3(&mut self) -> ShaDigest3W<'_, ShaDigest3Spec> {
        ShaDigest3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_digest3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_digest3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaDigest3Spec;
impl crate::RegisterSpec for ShaDigest3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_digest3::R`](R) reader structure"]
impl crate::Readable for ShaDigest3Spec {}
#[doc = "`write(|w| ..)` method takes [`sha_digest3::W`](W) writer structure"]
impl crate::Writable for ShaDigest3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Digest3 to value 0x367c_d507"]
impl crate::Resettable for ShaDigest3Spec {
    const RESET_VALUE: u32 = 0x367c_d507;
}
