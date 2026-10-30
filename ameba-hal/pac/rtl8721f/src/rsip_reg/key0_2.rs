#[doc = "Register `Key0_2` reader"]
pub type R = crate::R<Key0_2Spec>;
#[doc = "Register `Key0_2` writer"]
pub type W = crate::W<Key0_2Spec>;
#[doc = "Field `key0_2` reader - Key0 register \\[95:64\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key0_2R = crate::FieldReader<u32>;
#[doc = "Field `key0_2` writer - Key0 register \\[95:64\\] CTR of MIX, GCM, CTR mode using the key"]
pub type Key0_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key0 register \\[95:64\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key0_2(&self) -> Key0_2R {
        Key0_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key0 register \\[95:64\\] CTR of MIX, GCM, CTR mode using the key"]
    #[inline(always)]
    pub fn key0_2(&mut self) -> Key0_2W<'_, Key0_2Spec> {
        Key0_2W::new(self, 0)
    }
}
#[doc = "Key0 \\[95:64\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`key0_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key0_2Spec;
impl crate::RegisterSpec for Key0_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key0_2::R`](R) reader structure"]
impl crate::Readable for Key0_2Spec {}
#[doc = "`write(|w| ..)` method takes [`key0_2::W`](W) writer structure"]
impl crate::Writable for Key0_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Key0_2 to value 0"]
impl crate::Resettable for Key0_2Spec {}
