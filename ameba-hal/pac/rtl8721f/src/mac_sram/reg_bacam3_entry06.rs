#[doc = "Register `REG_BACAM3_ENTRY06` reader"]
pub type R = crate::R<RegBacam3Entry06Spec>;
#[doc = "Register `REG_BACAM3_ENTRY06` writer"]
pub type W = crate::W<RegBacam3Entry06Spec>;
#[doc = "Field `BACAM3_ENTRY06_BITMAP1` reader - "]
pub type Bacam3Entry06Bitmap1R = crate::FieldReader<u32>;
#[doc = "Field `BACAM3_ENTRY06_BITMAP1` writer - "]
pub type Bacam3Entry06Bitmap1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam3_entry06_bitmap1(&self) -> Bacam3Entry06Bitmap1R {
        Bacam3Entry06Bitmap1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam3_entry06_bitmap1(&mut self) -> Bacam3Entry06Bitmap1W<'_, RegBacam3Entry06Spec> {
        Bacam3Entry06Bitmap1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bacam3_entry06::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bacam3_entry06::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBacam3Entry06Spec;
impl crate::RegisterSpec for RegBacam3Entry06Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bacam3_entry06::R`](R) reader structure"]
impl crate::Readable for RegBacam3Entry06Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bacam3_entry06::W`](W) writer structure"]
impl crate::Writable for RegBacam3Entry06Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BACAM3_ENTRY06 to value 0"]
impl crate::Resettable for RegBacam3Entry06Spec {}
