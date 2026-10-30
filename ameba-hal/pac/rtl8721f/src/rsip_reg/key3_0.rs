#[doc = "Register `Key3_0` reader"]
pub type R = crate::R<Key3_0Spec>;
#[doc = "Register `Key3_0` writer"]
pub type W = crate::W<Key3_0Spec>;
#[doc = "Field `key3_0` reader - Key3 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key3_0R = crate::FieldReader<u32>;
#[doc = "Field `key3_0` writer - Key3 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key3_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key3 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key3_0(&self) -> Key3_0R {
        Key3_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key3 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key3_0(&mut self) -> Key3_0W<'_, Key3_0Spec> {
        Key3_0W::new(self, 0)
    }
}
#[doc = "Key3 \\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key3_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key3_0Spec;
impl crate::RegisterSpec for Key3_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key3_0::R`](R) reader structure"]
impl crate::Readable for Key3_0Spec {}
#[doc = "`write(|w| ..)` method takes [`key3_0::W`](W) writer structure"]
impl crate::Writable for Key3_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key3_0 to value 0"]
impl crate::Resettable for Key3_0Spec {}
