#[doc = "Register `REG_KEYCAM6_ENTRY09` reader"]
pub type R = crate::R<RegKeycam6Entry09Spec>;
#[doc = "Register `REG_KEYCAM6_ENTRY09` writer"]
pub type W = crate::W<RegKeycam6Entry09Spec>;
#[doc = "Field `KEYCAM6_ENTRY09_MFB` reader - "]
pub type Keycam6Entry09MfbR = crate::FieldReader;
#[doc = "Field `KEYCAM6_ENTRY09_MFB` writer - "]
pub type Keycam6Entry09MfbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn keycam6_entry09_mfb(&self) -> Keycam6Entry09MfbR {
        Keycam6Entry09MfbR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn keycam6_entry09_mfb(&mut self) -> Keycam6Entry09MfbW<'_, RegKeycam6Entry09Spec> {
        Keycam6Entry09MfbW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam6_entry09::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam6_entry09::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam6Entry09Spec;
impl crate::RegisterSpec for RegKeycam6Entry09Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam6_entry09::R`](R) reader structure"]
impl crate::Readable for RegKeycam6Entry09Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam6_entry09::W`](W) writer structure"]
impl crate::Writable for RegKeycam6Entry09Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM6_ENTRY09 to value 0"]
impl crate::Resettable for RegKeycam6Entry09Spec {}
