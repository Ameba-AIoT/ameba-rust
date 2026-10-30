#[doc = "Register `Key6_0` reader"]
pub type R = crate::R<Key6_0Spec>;
#[doc = "Register `Key6_0` writer"]
pub type W = crate::W<Key6_0Spec>;
#[doc = "Field `key6_0` reader - Key6 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key6_0R = crate::FieldReader<u32>;
#[doc = "Field `key6_0` writer - Key6 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key6_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key6 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key6_0(&self) -> Key6_0R {
        Key6_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key6 register \\[31:0\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key6_0(&mut self) -> Key6_0W<'_, Key6_0Spec> {
        Key6_0W::new(self, 0)
    }
}
#[doc = "Key6 \\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key6_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key6_0Spec;
impl crate::RegisterSpec for Key6_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key6_0::R`](R) reader structure"]
impl crate::Readable for Key6_0Spec {}
#[doc = "`write(|w| ..)` method takes [`key6_0::W`](W) writer structure"]
impl crate::Writable for Key6_0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key6_0 to value 0"]
impl crate::Resettable for Key6_0Spec {}
