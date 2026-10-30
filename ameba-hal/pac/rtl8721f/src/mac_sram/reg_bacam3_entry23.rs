#[doc = "Register `REG_BACAM3_ENTRY23` reader"]
pub type R = crate::R<RegBacam3Entry23Spec>;
#[doc = "Register `REG_BACAM3_ENTRY23` writer"]
pub type W = crate::W<RegBacam3Entry23Spec>;
#[doc = "Field `BACAM3_ENTRY23_BITMAP1` reader - "]
pub type Bacam3Entry23Bitmap1R = crate::FieldReader<u32>;
#[doc = "Field `BACAM3_ENTRY23_BITMAP1` writer - "]
pub type Bacam3Entry23Bitmap1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam3_entry23_bitmap1(&self) -> Bacam3Entry23Bitmap1R {
        Bacam3Entry23Bitmap1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam3_entry23_bitmap1(&mut self) -> Bacam3Entry23Bitmap1W<'_, RegBacam3Entry23Spec> {
        Bacam3Entry23Bitmap1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bacam3_entry23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bacam3_entry23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBacam3Entry23Spec;
impl crate::RegisterSpec for RegBacam3Entry23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bacam3_entry23::R`](R) reader structure"]
impl crate::Readable for RegBacam3Entry23Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bacam3_entry23::W`](W) writer structure"]
impl crate::Writable for RegBacam3Entry23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BACAM3_ENTRY23 to value 0"]
impl crate::Resettable for RegBacam3Entry23Spec {}
