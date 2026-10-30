#[doc = "Register `Key6_3` reader"]
pub type R = crate::R<Key6_3Spec>;
#[doc = "Register `Key6_3` writer"]
pub type W = crate::W<Key6_3Spec>;
#[doc = "Field `key6_3` reader - Key6 register \\[127:96\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key6_3R = crate::FieldReader<u32>;
#[doc = "Field `key6_3` writer - Key6 register \\[127:96\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key6_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key6 register \\[127:96\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key6_3(&self) -> Key6_3R {
        Key6_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key6 register \\[127:96\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key6_3(&mut self) -> Key6_3W<'_, Key6_3Spec> {
        Key6_3W::new(self, 0)
    }
}
#[doc = "Key6 \\[127:96\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key6_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key6_3Spec;
impl crate::RegisterSpec for Key6_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key6_3::R`](R) reader structure"]
impl crate::Readable for Key6_3Spec {}
#[doc = "`write(|w| ..)` method takes [`key6_3::W`](W) writer structure"]
impl crate::Writable for Key6_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key6_3 to value 0"]
impl crate::Resettable for Key6_3Spec {}
