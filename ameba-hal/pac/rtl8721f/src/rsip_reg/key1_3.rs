#[doc = "Register `Key1_3` reader"]
pub type R = crate::R<Key1_3Spec>;
#[doc = "Register `Key1_3` writer"]
pub type W = crate::W<Key1_3Spec>;
#[doc = "Field `key1_3` reader - Key1 register \\[127:96\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key1_3R = crate::FieldReader<u32>;
#[doc = "Field `key1_3` writer - Key1 register \\[127:96\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key1_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key1 register \\[127:96\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key1_3(&self) -> Key1_3R {
        Key1_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key1 register \\[127:96\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key1_3(&mut self) -> Key1_3W<'_, Key1_3Spec> {
        Key1_3W::new(self, 0)
    }
}
#[doc = "Key1 \\[127:96\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key1_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key1_3Spec;
impl crate::RegisterSpec for Key1_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key1_3::R`](R) reader structure"]
impl crate::Readable for Key1_3Spec {}
#[doc = "`write(|w| ..)` method takes [`key1_3::W`](W) writer structure"]
impl crate::Writable for Key1_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key1_3 to value 0"]
impl crate::Resettable for Key1_3Spec {}
