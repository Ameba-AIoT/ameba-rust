#[doc = "Register `REG_BACAM3_ENTRY01` reader"]
pub type R = crate::R<RegBacam3Entry01Spec>;
#[doc = "Register `REG_BACAM3_ENTRY01` writer"]
pub type W = crate::W<RegBacam3Entry01Spec>;
#[doc = "Field `BACAM3_ENTRY01_BITMAP1` reader - "]
pub type Bacam3Entry01Bitmap1R = crate::FieldReader<u32>;
#[doc = "Field `BACAM3_ENTRY01_BITMAP1` writer - "]
pub type Bacam3Entry01Bitmap1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam3_entry01_bitmap1(&self) -> Bacam3Entry01Bitmap1R {
        Bacam3Entry01Bitmap1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam3_entry01_bitmap1(&mut self) -> Bacam3Entry01Bitmap1W<'_, RegBacam3Entry01Spec> {
        Bacam3Entry01Bitmap1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bacam3_entry01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bacam3_entry01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBacam3Entry01Spec;
impl crate::RegisterSpec for RegBacam3Entry01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bacam3_entry01::R`](R) reader structure"]
impl crate::Readable for RegBacam3Entry01Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bacam3_entry01::W`](W) writer structure"]
impl crate::Writable for RegBacam3Entry01Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BACAM3_ENTRY01 to value 0"]
impl crate::Resettable for RegBacam3Entry01Spec {}
