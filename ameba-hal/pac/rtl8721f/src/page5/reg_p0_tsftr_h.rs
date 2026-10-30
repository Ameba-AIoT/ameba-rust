#[doc = "Register `REG_P0_TSFTR_H` reader"]
pub type R = crate::R<RegP0TsftrHSpec>;
#[doc = "Register `REG_P0_TSFTR_H` writer"]
pub type W = crate::W<RegP0TsftrHSpec>;
#[doc = "Field `BIT_P0_TSF_TIMER_H` reader - high 32bits of port0's tsft"]
pub type BitP0TsfTimerHR = crate::FieldReader<u32>;
#[doc = "Field `BIT_P0_TSF_TIMER_H` writer - high 32bits of port0's tsft"]
pub type BitP0TsfTimerHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - high 32bits of port0's tsft"]
    #[inline(always)]
    pub fn bit_p0_tsf_timer_h(&self) -> BitP0TsfTimerHR {
        BitP0TsfTimerHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - high 32bits of port0's tsft"]
    #[inline(always)]
    pub fn bit_p0_tsf_timer_h(&mut self) -> BitP0TsfTimerHW<'_, RegP0TsftrHSpec> {
        BitP0TsfTimerHW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p0_tsftr_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_p0_tsftr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP0TsftrHSpec;
impl crate::RegisterSpec for RegP0TsftrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p0_tsftr_h::R`](R) reader structure"]
impl crate::Readable for RegP0TsftrHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_p0_tsftr_h::W`](W) writer structure"]
impl crate::Writable for RegP0TsftrHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_P0_TSFTR_H to value 0"]
impl crate::Resettable for RegP0TsftrHSpec {}
