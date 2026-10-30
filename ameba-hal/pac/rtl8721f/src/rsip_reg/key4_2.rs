#[doc = "Register `Key4_2` reader"]
pub type R = crate::R<Key4_2Spec>;
#[doc = "Register `Key4_2` writer"]
pub type W = crate::W<Key4_2Spec>;
#[doc = "Field `key4_2` reader - Key4 register \\[95:64\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key4_2R = crate::FieldReader<u32>;
#[doc = "Field `key4_2` writer - Key4 register \\[95:64\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key4_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key4 register \\[95:64\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key4_2(&self) -> Key4_2R {
        Key4_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key4 register \\[95:64\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key4_2(&mut self) -> Key4_2W<'_, Key4_2Spec> {
        Key4_2W::new(self, 0)
    }
}
#[doc = "Key4 \\[95:64\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key4_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key4_2Spec;
impl crate::RegisterSpec for Key4_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key4_2::R`](R) reader structure"]
impl crate::Readable for Key4_2Spec {}
#[doc = "`write(|w| ..)` method takes [`key4_2::W`](W) writer structure"]
impl crate::Writable for Key4_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key4_2 to value 0"]
impl crate::Resettable for Key4_2Spec {}
