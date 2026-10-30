#[doc = "Register `TIM_ARR` reader"]
pub type R = crate::R<TimArrSpec>;
#[doc = "Register `TIM_ARR` writer"]
pub type W = crate::W<TimArrSpec>;
#[doc = "Field `ARR` reader - the value to be loaded in the actual auto-reload register. It can be preloaded by setting ARPE bit in TIM_CR."]
pub type ArrR = crate::FieldReader<u16>;
#[doc = "Field `ARR` writer - the value to be loaded in the actual auto-reload register. It can be preloaded by setting ARPE bit in TIM_CR."]
pub type ArrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the value to be loaded in the actual auto-reload register. It can be preloaded by setting ARPE bit in TIM_CR."]
    #[inline(always)]
    pub fn arr(&self) -> ArrR {
        ArrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the value to be loaded in the actual auto-reload register. It can be preloaded by setting ARPE bit in TIM_CR."]
    #[inline(always)]
    pub fn arr(&mut self) -> ArrW<'_, TimArrSpec> {
        ArrW::new(self, 0)
    }
}
#[doc = "TIM auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimArrSpec;
impl crate::RegisterSpec for TimArrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_arr::R`](R) reader structure"]
impl crate::Readable for TimArrSpec {}
#[doc = "`write(|w| ..)` method takes [`tim_arr::W`](W) writer structure"]
impl crate::Writable for TimArrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_ARR to value 0xffff"]
impl crate::Resettable for TimArrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
