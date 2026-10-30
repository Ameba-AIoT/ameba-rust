#[doc = "Register `Key2_5` reader"]
pub type R = crate::R<Key2_5Spec>;
#[doc = "Register `Key2_5` writer"]
pub type W = crate::W<Key2_5Spec>;
#[doc = "Field `key2_5` reader - Key2 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key2_5R = crate::FieldReader<u32>;
#[doc = "Field `key2_5` writer - Key2 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key2_5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key2 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key2_5(&self) -> Key2_5R {
        Key2_5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key2 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key2_5(&mut self) -> Key2_5W<'_, Key2_5Spec> {
        Key2_5W::new(self, 0)
    }
}
#[doc = "Key2 \\[191:160\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key2_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key2_5Spec;
impl crate::RegisterSpec for Key2_5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key2_5::R`](R) reader structure"]
impl crate::Readable for Key2_5Spec {}
#[doc = "`write(|w| ..)` method takes [`key2_5::W`](W) writer structure"]
impl crate::Writable for Key2_5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key2_5 to value 0"]
impl crate::Resettable for Key2_5Spec {}
