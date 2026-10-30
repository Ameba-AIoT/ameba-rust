#[doc = "Register `REG_BACAM0_ENTRY13` reader"]
pub type R = crate::R<RegBacam0Entry13Spec>;
#[doc = "Register `REG_BACAM0_ENTRY13` writer"]
pub type W = crate::W<RegBacam0Entry13Spec>;
#[doc = "Field `BACAM0_ENTRY13_INFO_TA0` reader - "]
pub type Bacam0Entry13InfoTa0R = crate::FieldReader<u32>;
#[doc = "Field `BACAM0_ENTRY13_INFO_TA0` writer - "]
pub type Bacam0Entry13InfoTa0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam0_entry13_info_ta0(&self) -> Bacam0Entry13InfoTa0R {
        Bacam0Entry13InfoTa0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam0_entry13_info_ta0(&mut self) -> Bacam0Entry13InfoTa0W<'_, RegBacam0Entry13Spec> {
        Bacam0Entry13InfoTa0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bacam0_entry13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bacam0_entry13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBacam0Entry13Spec;
impl crate::RegisterSpec for RegBacam0Entry13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bacam0_entry13::R`](R) reader structure"]
impl crate::Readable for RegBacam0Entry13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bacam0_entry13::W`](W) writer structure"]
impl crate::Writable for RegBacam0Entry13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BACAM0_ENTRY13 to value 0"]
impl crate::Resettable for RegBacam0Entry13Spec {}
