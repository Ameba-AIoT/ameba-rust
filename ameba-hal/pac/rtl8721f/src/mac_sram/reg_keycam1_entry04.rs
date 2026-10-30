#[doc = "Register `REG_KEYCAM1_ENTRY04` reader"]
pub type R = crate::R<RegKeycam1Entry04Spec>;
#[doc = "Register `REG_KEYCAM1_ENTRY04` writer"]
pub type W = crate::W<RegKeycam1Entry04Spec>;
#[doc = "Field `KEYCAM1_ENTRY04_ADDR1` reader - address\\[47:16\\]"]
pub type Keycam1Entry04Addr1R = crate::FieldReader<u32>;
#[doc = "Field `KEYCAM1_ENTRY04_ADDR1` writer - address\\[47:16\\]"]
pub type Keycam1Entry04Addr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - address\\[47:16\\]"]
    #[inline(always)]
    pub fn keycam1_entry04_addr1(&self) -> Keycam1Entry04Addr1R {
        Keycam1Entry04Addr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - address\\[47:16\\]"]
    #[inline(always)]
    pub fn keycam1_entry04_addr1(&mut self) -> Keycam1Entry04Addr1W<'_, RegKeycam1Entry04Spec> {
        Keycam1Entry04Addr1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam1_entry04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam1_entry04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam1Entry04Spec;
impl crate::RegisterSpec for RegKeycam1Entry04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam1_entry04::R`](R) reader structure"]
impl crate::Readable for RegKeycam1Entry04Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam1_entry04::W`](W) writer structure"]
impl crate::Writable for RegKeycam1Entry04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM1_ENTRY04 to value 0"]
impl crate::Resettable for RegKeycam1Entry04Spec {}
