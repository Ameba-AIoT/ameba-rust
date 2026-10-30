#[doc = "Register `Key5_5` reader"]
pub type R = crate::R<Key5_5Spec>;
#[doc = "Register `Key5_5` writer"]
pub type W = crate::W<Key5_5Spec>;
#[doc = "Field `key5_5` reader - Key5 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key5_5R = crate::FieldReader<u32>;
#[doc = "Field `key5_5` writer - Key5 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key5_5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key5 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key5_5(&self) -> Key5_5R {
        Key5_5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key5 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key5_5(&mut self) -> Key5_5W<'_, Key5_5Spec> {
        Key5_5W::new(self, 0)
    }
}
#[doc = "Key5 \\[191:160\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key5_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key5_5Spec;
impl crate::RegisterSpec for Key5_5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key5_5::R`](R) reader structure"]
impl crate::Readable for Key5_5Spec {}
#[doc = "`write(|w| ..)` method takes [`key5_5::W`](W) writer structure"]
impl crate::Writable for Key5_5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key5_5 to value 0"]
impl crate::Resettable for Key5_5Spec {}
