#[doc = "Register `TIM_com_en` reader"]
pub type R = crate::R<TimComEnSpec>;
#[doc = "Register `TIM_com_en` writer"]
pub type W = crate::W<TimComEnSpec>;
#[doc = "Field `com_en` reader - enable compare option.The set cnt value takes effect only when it is greater than the actual cnt value 0:disable 1:enable \\[0\\] : control tim0 set ...... \\[3\\]: control tim3 set"]
pub type ComEnR = crate::FieldReader;
#[doc = "Field `com_en` writer - enable compare option.The set cnt value takes effect only when it is greater than the actual cnt value 0:disable 1:enable \\[0\\] : control tim0 set ...... \\[3\\]: control tim3 set"]
pub type ComEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - enable compare option.The set cnt value takes effect only when it is greater than the actual cnt value 0:disable 1:enable \\[0\\] : control tim0 set ...... \\[3\\]: control tim3 set"]
    #[inline(always)]
    pub fn com_en(&self) -> ComEnR {
        ComEnR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - enable compare option.The set cnt value takes effect only when it is greater than the actual cnt value 0:disable 1:enable \\[0\\] : control tim0 set ...... \\[3\\]: control tim3 set"]
    #[inline(always)]
    pub fn com_en(&mut self) -> ComEnW<'_, TimComEnSpec> {
        ComEnW::new(self, 0)
    }
}
#[doc = "TIM compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_com_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_com_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimComEnSpec;
impl crate::RegisterSpec for TimComEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_com_en::R`](R) reader structure"]
impl crate::Readable for TimComEnSpec {}
#[doc = "`write(|w| ..)` method takes [`tim_com_en::W`](W) writer structure"]
impl crate::Writable for TimComEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_com_en to value 0x0f"]
impl crate::Resettable for TimComEnSpec {
    const RESET_VALUE: u32 = 0x0f;
}
