#[doc = "Register `REG_KEYCAM5_ENTRY02` reader"]
pub type R = crate::R<RegKeycam5Entry02Spec>;
#[doc = "Register `REG_KEYCAM5_ENTRY02` writer"]
pub type W = crate::W<RegKeycam5Entry02Spec>;
#[doc = "Field `KEYCAM5_ENTRY02_KEY3` reader - key\\[127:96\\]"]
pub type Keycam5Entry02Key3R = crate::FieldReader<u32>;
#[doc = "Field `KEYCAM5_ENTRY02_KEY3` writer - key\\[127:96\\]"]
pub type Keycam5Entry02Key3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - key\\[127:96\\]"]
    #[inline(always)]
    pub fn keycam5_entry02_key3(&self) -> Keycam5Entry02Key3R {
        Keycam5Entry02Key3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - key\\[127:96\\]"]
    #[inline(always)]
    pub fn keycam5_entry02_key3(&mut self) -> Keycam5Entry02Key3W<'_, RegKeycam5Entry02Spec> {
        Keycam5Entry02Key3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam5_entry02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam5_entry02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam5Entry02Spec;
impl crate::RegisterSpec for RegKeycam5Entry02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam5_entry02::R`](R) reader structure"]
impl crate::Readable for RegKeycam5Entry02Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam5_entry02::W`](W) writer structure"]
impl crate::Writable for RegKeycam5Entry02Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM5_ENTRY02 to value 0"]
impl crate::Resettable for RegKeycam5Entry02Spec {}
