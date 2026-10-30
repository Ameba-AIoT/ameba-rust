#[doc = "Register `TIM_SET2` reader"]
pub type R = crate::R<TimSet2Spec>;
#[doc = "Register `TIM_SET2` writer"]
pub type W = crate::W<TimSet2Spec>;
#[doc = "Field `tim_set2` reader - Set a countdown value. This value come into force when TIM_EN register bit is enabled. counter will count down from this value immediately."]
pub type TimSet2R = crate::FieldReader<u32>;
#[doc = "Field `tim_set2` writer - Set a countdown value. This value come into force when TIM_EN register bit is enabled. counter will count down from this value immediately."]
pub type TimSet2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set a countdown value. This value come into force when TIM_EN register bit is enabled. counter will count down from this value immediately."]
    #[inline(always)]
    pub fn tim_set2(&self) -> TimSet2R {
        TimSet2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set a countdown value. This value come into force when TIM_EN register bit is enabled. counter will count down from this value immediately."]
    #[inline(always)]
    pub fn tim_set2(&mut self) -> TimSet2W<'_, TimSet2Spec> {
        TimSet2W::new(self, 0)
    }
}
#[doc = "TIM set register2\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_set2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_set2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimSet2Spec;
impl crate::RegisterSpec for TimSet2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_set2::R`](R) reader structure"]
impl crate::Readable for TimSet2Spec {}
#[doc = "`write(|w| ..)` method takes [`tim_set2::W`](W) writer structure"]
impl crate::Writable for TimSet2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_SET2 to value 0"]
impl crate::Resettable for TimSet2Spec {}
