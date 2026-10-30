#[doc = "Register `Key1_0` reader"]
pub type R = crate::R<Key1_0Spec>;
#[doc = "Register `Key1_0` writer"]
pub type W = crate::W<Key1_0Spec>;
#[doc = "Field `key1_0` reader - Key1 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key1_0R = crate::FieldReader<u32>;
#[doc = "Field `key1_0` writer - Key1 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key1_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key1 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key1_0(&self) -> Key1_0R {
        Key1_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key1 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key1_0(&mut self) -> Key1_0W<'_, Key1_0Spec> {
        Key1_0W::new(self, 0)
    }
}
#[doc = "Key1 \\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key1_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key1_0Spec;
impl crate::RegisterSpec for Key1_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key1_0::R`](R) reader structure"]
impl crate::Readable for Key1_0Spec {}
#[doc = "`write(|w| ..)` method takes [`key1_0::W`](W) writer structure"]
impl crate::Writable for Key1_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key1_0 to value 0"]
impl crate::Resettable for Key1_0Spec {}
