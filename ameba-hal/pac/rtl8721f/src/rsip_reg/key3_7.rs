#[doc = "Register `Key3_7` reader"]
pub type R = crate::R<Key3_7Spec>;
#[doc = "Register `Key3_7` writer"]
pub type W = crate::W<Key3_7Spec>;
#[doc = "Field `key3_7` reader - Key3 register \\[255:224\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key3_7R = crate::FieldReader<u32>;
#[doc = "Field `key3_7` writer - Key3 register \\[255:224\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key3_7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key3 register \\[255:224\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key3_7(&self) -> Key3_7R {
        Key3_7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key3 register \\[255:224\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key3_7(&mut self) -> Key3_7W<'_, Key3_7Spec> {
        Key3_7W::new(self, 0)
    }
}
#[doc = "Key3 \\[255:224\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key3_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key3_7Spec;
impl crate::RegisterSpec for Key3_7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key3_7::R`](R) reader structure"]
impl crate::Readable for Key3_7Spec {}
#[doc = "`write(|w| ..)` method takes [`key3_7::W`](W) writer structure"]
impl crate::Writable for Key3_7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key3_7 to value 0"]
impl crate::Resettable for Key3_7Spec {}
