#[doc = "Register `Key5_1` reader"]
pub type R = crate::R<Key5_1Spec>;
#[doc = "Register `Key5_1` writer"]
pub type W = crate::W<Key5_1Spec>;
#[doc = "Field `key5_1` reader - Key5 register \\[63:32\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key5_1R = crate::FieldReader<u32>;
#[doc = "Field `key5_1` writer - Key5 register \\[63:32\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key5_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key5 register \\[63:32\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key5_1(&self) -> Key5_1R {
        Key5_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key5 register \\[63:32\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key5_1(&mut self) -> Key5_1W<'_, Key5_1Spec> {
        Key5_1W::new(self, 0)
    }
}
#[doc = "Key5 \\[63:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key5_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key5_1Spec;
impl crate::RegisterSpec for Key5_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key5_1::R`](R) reader structure"]
impl crate::Readable for Key5_1Spec {}
#[doc = "`write(|w| ..)` method takes [`key5_1::W`](W) writer structure"]
impl crate::Writable for Key5_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key5_1 to value 0"]
impl crate::Resettable for Key5_1Spec {}
