#[doc = "Register `Key2_7` reader"]
pub type R = crate::R<Key2_7Spec>;
#[doc = "Register `Key2_7` writer"]
pub type W = crate::W<Key2_7Spec>;
#[doc = "Field `key2_7` reader - Key2 register \\[255:224\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key2_7R = crate::FieldReader<u32>;
#[doc = "Field `key2_7` writer - Key2 register \\[255:224\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key2_7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key2 register \\[255:224\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key2_7(&self) -> Key2_7R {
        Key2_7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key2 register \\[255:224\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key2_7(&mut self) -> Key2_7W<'_, Key2_7Spec> {
        Key2_7W::new(self, 0)
    }
}
#[doc = "Key2 \\[255:224\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key2_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key2_7Spec;
impl crate::RegisterSpec for Key2_7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key2_7::R`](R) reader structure"]
impl crate::Readable for Key2_7Spec {}
#[doc = "`write(|w| ..)` method takes [`key2_7::W`](W) writer structure"]
impl crate::Writable for Key2_7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key2_7 to value 0"]
impl crate::Resettable for Key2_7Spec {}
