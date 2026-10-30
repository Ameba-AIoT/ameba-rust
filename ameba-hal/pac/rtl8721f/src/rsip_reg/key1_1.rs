#[doc = "Register `Key1_1` reader"]
pub type R = crate::R<Key1_1Spec>;
#[doc = "Register `Key1_1` writer"]
pub type W = crate::W<Key1_1Spec>;
#[doc = "Field `key1_1` reader - Key1 register \\[63:32\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key1_1R = crate::FieldReader<u32>;
#[doc = "Field `key1_1` writer - Key1 register \\[63:32\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key1_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key1 register \\[63:32\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key1_1(&self) -> Key1_1R {
        Key1_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key1 register \\[63:32\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key1_1(&mut self) -> Key1_1W<'_, Key1_1Spec> {
        Key1_1W::new(self, 0)
    }
}
#[doc = "Key1 \\[63:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key1_1Spec;
impl crate::RegisterSpec for Key1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key1_1::R`](R) reader structure"]
impl crate::Readable for Key1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`key1_1::W`](W) writer structure"]
impl crate::Writable for Key1_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key1_1 to value 0"]
impl crate::Resettable for Key1_1Spec {}
