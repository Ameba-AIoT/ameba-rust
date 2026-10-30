#[doc = "Register `REG_KEYCAM2_ENTRY30` reader"]
pub type R = crate::R<RegKeycam2Entry30Spec>;
#[doc = "Register `REG_KEYCAM2_ENTRY30` writer"]
pub type W = crate::W<RegKeycam2Entry30Spec>;
#[doc = "Field `KEYCAM2_ENTRY30_KEY0` reader - key\\[31:0\\]"]
pub type Keycam2Entry30Key0R = crate::FieldReader<u32>;
#[doc = "Field `KEYCAM2_ENTRY30_KEY0` writer - key\\[31:0\\]"]
pub type Keycam2Entry30Key0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - key\\[31:0\\]"]
    #[inline(always)]
    pub fn keycam2_entry30_key0(&self) -> Keycam2Entry30Key0R {
        Keycam2Entry30Key0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - key\\[31:0\\]"]
    #[inline(always)]
    pub fn keycam2_entry30_key0(&mut self) -> Keycam2Entry30Key0W<'_, RegKeycam2Entry30Spec> {
        Keycam2Entry30Key0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam2_entry30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam2_entry30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam2Entry30Spec;
impl crate::RegisterSpec for RegKeycam2Entry30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam2_entry30::R`](R) reader structure"]
impl crate::Readable for RegKeycam2Entry30Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam2_entry30::W`](W) writer structure"]
impl crate::Writable for RegKeycam2Entry30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM2_ENTRY30 to value 0"]
impl crate::Resettable for RegKeycam2Entry30Spec {}
