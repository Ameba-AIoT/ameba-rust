#[doc = "Register `Key7_6` reader"]
pub type R = crate::R<Key7_6Spec>;
#[doc = "Register `Key7_6` writer"]
pub type W = crate::W<Key7_6Spec>;
#[doc = "Field `key7_6` reader - Key7 register \\[223:192\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key7_6R = crate::FieldReader<u32>;
#[doc = "Field `key7_6` writer - Key7 register \\[223:192\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key7_6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key7 register \\[223:192\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key7_6(&self) -> Key7_6R {
        Key7_6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key7 register \\[223:192\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key7_6(&mut self) -> Key7_6W<'_, Key7_6Spec> {
        Key7_6W::new(self, 0)
    }
}
#[doc = "Key7 \\[223:192\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key7_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key7_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key7_6Spec;
impl crate::RegisterSpec for Key7_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key7_6::R`](R) reader structure"]
impl crate::Readable for Key7_6Spec {}
#[doc = "`write(|w| ..)` method takes [`key7_6::W`](W) writer structure"]
impl crate::Writable for Key7_6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key7_6 to value 0"]
impl crate::Resettable for Key7_6Spec {}
