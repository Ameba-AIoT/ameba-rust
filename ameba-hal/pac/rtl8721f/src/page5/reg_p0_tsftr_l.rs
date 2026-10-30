#[doc = "Register `REG_P0_TSFTR_L` reader"]
pub type R = crate::R<RegP0TsftrLSpec>;
#[doc = "Register `REG_P0_TSFTR_L` writer"]
pub type W = crate::W<RegP0TsftrLSpec>;
#[doc = "Field `BIT_P0_TSF_TIMER_L` reader - low 32bits of port0's tsft"]
pub type BitP0TsfTimerLR = crate::FieldReader<u32>;
#[doc = "Field `BIT_P0_TSF_TIMER_L` writer - low 32bits of port0's tsft"]
pub type BitP0TsfTimerLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low 32bits of port0's tsft"]
    #[inline(always)]
    pub fn bit_p0_tsf_timer_l(&self) -> BitP0TsfTimerLR {
        BitP0TsfTimerLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low 32bits of port0's tsft"]
    #[inline(always)]
    pub fn bit_p0_tsf_timer_l(&mut self) -> BitP0TsfTimerLW<'_, RegP0TsftrLSpec> {
        BitP0TsfTimerLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p0_tsftr_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_p0_tsftr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP0TsftrLSpec;
impl crate::RegisterSpec for RegP0TsftrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p0_tsftr_l::R`](R) reader structure"]
impl crate::Readable for RegP0TsftrLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_p0_tsftr_l::W`](W) writer structure"]
impl crate::Writable for RegP0TsftrLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_P0_TSFTR_L to value 0x01"]
impl crate::Resettable for RegP0TsftrLSpec {
    const RESET_VALUE: u32 = 0x01;
}
