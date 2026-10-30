#[doc = "Register `REG_KEYCAM4_ENTRY19` reader"]
pub type R = crate::R<RegKeycam4Entry19Spec>;
#[doc = "Register `REG_KEYCAM4_ENTRY19` writer"]
pub type W = crate::W<RegKeycam4Entry19Spec>;
#[doc = "Field `KEYCAM4_ENTRY19_KEY2` reader - key\\[95:64\\]"]
pub type Keycam4Entry19Key2R = crate::FieldReader<u32>;
#[doc = "Field `KEYCAM4_ENTRY19_KEY2` writer - key\\[95:64\\]"]
pub type Keycam4Entry19Key2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - key\\[95:64\\]"]
    #[inline(always)]
    pub fn keycam4_entry19_key2(&self) -> Keycam4Entry19Key2R {
        Keycam4Entry19Key2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - key\\[95:64\\]"]
    #[inline(always)]
    pub fn keycam4_entry19_key2(&mut self) -> Keycam4Entry19Key2W<'_, RegKeycam4Entry19Spec> {
        Keycam4Entry19Key2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam4_entry19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam4_entry19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam4Entry19Spec;
impl crate::RegisterSpec for RegKeycam4Entry19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam4_entry19::R`](R) reader structure"]
impl crate::Readable for RegKeycam4Entry19Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam4_entry19::W`](W) writer structure"]
impl crate::Writable for RegKeycam4Entry19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM4_ENTRY19 to value 0"]
impl crate::Resettable for RegKeycam4Entry19Spec {}
