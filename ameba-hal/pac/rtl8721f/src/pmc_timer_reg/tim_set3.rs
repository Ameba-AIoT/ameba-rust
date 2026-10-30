#[doc = "Register `TIM_SET3` reader"]
pub type R = crate::R<TimSet3Spec>;
#[doc = "Register `TIM_SET3` writer"]
pub type W = crate::W<TimSet3Spec>;
#[doc = "Field `tim_set3` reader - Set a countdown value. This value come into force when TIM_EN register bit is enabled. counter will count down from this value immediately."]
pub type TimSet3R = crate::FieldReader<u32>;
#[doc = "Field `tim_set3` writer - Set a countdown value. This value come into force when TIM_EN register bit is enabled. counter will count down from this value immediately."]
pub type TimSet3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set a countdown value. This value come into force when TIM_EN register bit is enabled. counter will count down from this value immediately."]
    #[inline(always)]
    pub fn tim_set3(&self) -> TimSet3R {
        TimSet3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set a countdown value. This value come into force when TIM_EN register bit is enabled. counter will count down from this value immediately."]
    #[inline(always)]
    pub fn tim_set3(&mut self) -> TimSet3W<'_, TimSet3Spec> {
        TimSet3W::new(self, 0)
    }
}
#[doc = "TIM set register3\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_set3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_set3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimSet3Spec;
impl crate::RegisterSpec for TimSet3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_set3::R`](R) reader structure"]
impl crate::Readable for TimSet3Spec {}
#[doc = "`write(|w| ..)` method takes [`tim_set3::W`](W) writer structure"]
impl crate::Writable for TimSet3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_SET3 to value 0"]
impl crate::Resettable for TimSet3Spec {}
