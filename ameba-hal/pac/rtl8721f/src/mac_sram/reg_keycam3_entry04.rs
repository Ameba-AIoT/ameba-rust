#[doc = "Register `REG_KEYCAM3_ENTRY04` reader"]
pub type R = crate::R<RegKeycam3Entry04Spec>;
#[doc = "Register `REG_KEYCAM3_ENTRY04` writer"]
pub type W = crate::W<RegKeycam3Entry04Spec>;
#[doc = "Field `KEYCAM3_ENTRY04_KEY1` reader - key\\[63:32\\]"]
pub type Keycam3Entry04Key1R = crate::FieldReader<u32>;
#[doc = "Field `KEYCAM3_ENTRY04_KEY1` writer - key\\[63:32\\]"]
pub type Keycam3Entry04Key1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - key\\[63:32\\]"]
    #[inline(always)]
    pub fn keycam3_entry04_key1(&self) -> Keycam3Entry04Key1R {
        Keycam3Entry04Key1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - key\\[63:32\\]"]
    #[inline(always)]
    pub fn keycam3_entry04_key1(&mut self) -> Keycam3Entry04Key1W<'_, RegKeycam3Entry04Spec> {
        Keycam3Entry04Key1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam3_entry04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam3_entry04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam3Entry04Spec;
impl crate::RegisterSpec for RegKeycam3Entry04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam3_entry04::R`](R) reader structure"]
impl crate::Readable for RegKeycam3Entry04Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam3_entry04::W`](W) writer structure"]
impl crate::Writable for RegKeycam3Entry04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM3_ENTRY04 to value 0"]
impl crate::Resettable for RegKeycam3Entry04Spec {}
