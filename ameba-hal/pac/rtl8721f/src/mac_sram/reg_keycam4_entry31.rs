#[doc = "Register `REG_KEYCAM4_ENTRY31` reader"]
pub type R = crate::R<RegKeycam4Entry31Spec>;
#[doc = "Register `REG_KEYCAM4_ENTRY31` writer"]
pub type W = crate::W<RegKeycam4Entry31Spec>;
#[doc = "Field `KEYCAM4_ENTRY31_KEY2` reader - key\\[95:64\\]"]
pub type Keycam4Entry31Key2R = crate::FieldReader<u32>;
#[doc = "Field `KEYCAM4_ENTRY31_KEY2` writer - key\\[95:64\\]"]
pub type Keycam4Entry31Key2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - key\\[95:64\\]"]
    #[inline(always)]
    pub fn keycam4_entry31_key2(&self) -> Keycam4Entry31Key2R {
        Keycam4Entry31Key2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - key\\[95:64\\]"]
    #[inline(always)]
    pub fn keycam4_entry31_key2(&mut self) -> Keycam4Entry31Key2W<'_, RegKeycam4Entry31Spec> {
        Keycam4Entry31Key2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam4_entry31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam4_entry31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam4Entry31Spec;
impl crate::RegisterSpec for RegKeycam4Entry31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam4_entry31::R`](R) reader structure"]
impl crate::Readable for RegKeycam4Entry31Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam4_entry31::W`](W) writer structure"]
impl crate::Writable for RegKeycam4Entry31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM4_ENTRY31 to value 0"]
impl crate::Resettable for RegKeycam4Entry31Spec {}
