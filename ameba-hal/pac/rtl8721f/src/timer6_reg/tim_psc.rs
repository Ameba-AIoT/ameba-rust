#[doc = "Register `TIM_PSC` reader"]
pub type R = crate::R<TimPscSpec>;
#[doc = "Register `TIM_PSC` writer"]
pub type W = crate::W<TimPscSpec>;
#[doc = "Field `PSC` reader - prescaler value. PSC contains the value to be loaded in the actual prescaler register at each UEV(including when the counter is cleared through UG bit of TIM_EGR register)."]
pub type PscR = crate::FieldReader<u16>;
#[doc = "Field `PSC` writer - prescaler value. PSC contains the value to be loaded in the actual prescaler register at each UEV(including when the counter is cleared through UG bit of TIM_EGR register)."]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - prescaler value. PSC contains the value to be loaded in the actual prescaler register at each UEV(including when the counter is cleared through UG bit of TIM_EGR register)."]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - prescaler value. PSC contains the value to be loaded in the actual prescaler register at each UEV(including when the counter is cleared through UG bit of TIM_EGR register)."]
    #[inline(always)]
    pub fn psc(&mut self) -> PscW<'_, TimPscSpec> {
        PscW::new(self, 0)
    }
}
#[doc = "TIM prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimPscSpec;
impl crate::RegisterSpec for TimPscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_psc::R`](R) reader structure"]
impl crate::Readable for TimPscSpec {}
#[doc = "`write(|w| ..)` method takes [`tim_psc::W`](W) writer structure"]
impl crate::Writable for TimPscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_PSC to value 0"]
impl crate::Resettable for TimPscSpec {}
