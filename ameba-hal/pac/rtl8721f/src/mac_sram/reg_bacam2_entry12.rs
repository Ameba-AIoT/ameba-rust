#[doc = "Register `REG_BACAM2_ENTRY12` reader"]
pub type R = crate::R<RegBacam2Entry12Spec>;
#[doc = "Register `REG_BACAM2_ENTRY12` writer"]
pub type W = crate::W<RegBacam2Entry12Spec>;
#[doc = "Field `BACAM2_ENTRY12_BITMAP0` reader - "]
pub type Bacam2Entry12Bitmap0R = crate::FieldReader<u32>;
#[doc = "Field `BACAM2_ENTRY12_BITMAP0` writer - "]
pub type Bacam2Entry12Bitmap0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam2_entry12_bitmap0(&self) -> Bacam2Entry12Bitmap0R {
        Bacam2Entry12Bitmap0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam2_entry12_bitmap0(&mut self) -> Bacam2Entry12Bitmap0W<'_, RegBacam2Entry12Spec> {
        Bacam2Entry12Bitmap0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bacam2_entry12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bacam2_entry12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBacam2Entry12Spec;
impl crate::RegisterSpec for RegBacam2Entry12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bacam2_entry12::R`](R) reader structure"]
impl crate::Readable for RegBacam2Entry12Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bacam2_entry12::W`](W) writer structure"]
impl crate::Writable for RegBacam2Entry12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BACAM2_ENTRY12 to value 0"]
impl crate::Resettable for RegBacam2Entry12Spec {}
