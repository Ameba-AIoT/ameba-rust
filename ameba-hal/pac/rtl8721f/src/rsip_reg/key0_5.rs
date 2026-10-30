#[doc = "Register `Key0_5` reader"]
pub type R = crate::R<Key0_5Spec>;
#[doc = "Register `Key0_5` writer"]
pub type W = crate::W<Key0_5Spec>;
#[doc = "Field `key0_5` reader - Key0 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key0_5R = crate::FieldReader<u32>;
#[doc = "Field `key0_5` writer - Key0 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key0_5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key0 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key0_5(&self) -> Key0_5R {
        Key0_5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key0 register \\[191:160\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key0_5(&mut self) -> Key0_5W<'_, Key0_5Spec> {
        Key0_5W::new(self, 0)
    }
}
#[doc = "Key0 \\[191:160\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key0_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key0_5Spec;
impl crate::RegisterSpec for Key0_5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key0_5::R`](R) reader structure"]
impl crate::Readable for Key0_5Spec {}
#[doc = "`write(|w| ..)` method takes [`key0_5::W`](W) writer structure"]
impl crate::Writable for Key0_5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key0_5 to value 0"]
impl crate::Resettable for Key0_5Spec {}
