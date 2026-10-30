#[doc = "Register `REG_P1_TSFTR_H` reader"]
pub type R = crate::R<RegP1TsftrHSpec>;
#[doc = "Register `REG_P1_TSFTR_H` writer"]
pub type W = crate::W<RegP1TsftrHSpec>;
#[doc = "Field `BIT_P1_TSF_TIMER_H` reader - high 32bits of port1's tsft"]
pub type BitP1TsfTimerHR = crate::FieldReader<u32>;
#[doc = "Field `BIT_P1_TSF_TIMER_H` writer - high 32bits of port1's tsft"]
pub type BitP1TsfTimerHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - high 32bits of port1's tsft"]
    #[inline(always)]
    pub fn bit_p1_tsf_timer_h(&self) -> BitP1TsfTimerHR {
        BitP1TsfTimerHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - high 32bits of port1's tsft"]
    #[inline(always)]
    pub fn bit_p1_tsf_timer_h(&mut self) -> BitP1TsfTimerHW<'_, RegP1TsftrHSpec> {
        BitP1TsfTimerHW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p1_tsftr_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_p1_tsftr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP1TsftrHSpec;
impl crate::RegisterSpec for RegP1TsftrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p1_tsftr_h::R`](R) reader structure"]
impl crate::Readable for RegP1TsftrHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_p1_tsftr_h::W`](W) writer structure"]
impl crate::Writable for RegP1TsftrHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_P1_TSFTR_H to value 0x01"]
impl crate::Resettable for RegP1TsftrHSpec {
    const RESET_VALUE: u32 = 0x01;
}
