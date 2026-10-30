#[doc = "Register `REG_KEYCAM6_ENTRY21` reader"]
pub type R = crate::R<RegKeycam6Entry21Spec>;
#[doc = "Register `REG_KEYCAM6_ENTRY21` writer"]
pub type W = crate::W<RegKeycam6Entry21Spec>;
#[doc = "Field `KEYCAM6_ENTRY21_MFB` reader - "]
pub type Keycam6Entry21MfbR = crate::FieldReader;
#[doc = "Field `KEYCAM6_ENTRY21_MFB` writer - "]
pub type Keycam6Entry21MfbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn keycam6_entry21_mfb(&self) -> Keycam6Entry21MfbR {
        Keycam6Entry21MfbR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn keycam6_entry21_mfb(&mut self) -> Keycam6Entry21MfbW<'_, RegKeycam6Entry21Spec> {
        Keycam6Entry21MfbW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam6_entry21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam6_entry21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam6Entry21Spec;
impl crate::RegisterSpec for RegKeycam6Entry21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam6_entry21::R`](R) reader structure"]
impl crate::Readable for RegKeycam6Entry21Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam6_entry21::W`](W) writer structure"]
impl crate::Writable for RegKeycam6Entry21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM6_ENTRY21 to value 0"]
impl crate::Resettable for RegKeycam6Entry21Spec {}
