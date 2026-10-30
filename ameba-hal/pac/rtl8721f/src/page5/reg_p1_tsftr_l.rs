#[doc = "Register `REG_P1_TSFTR_L` reader"]
pub type R = crate::R<RegP1TsftrLSpec>;
#[doc = "Register `REG_P1_TSFTR_L` writer"]
pub type W = crate::W<RegP1TsftrLSpec>;
#[doc = "Field `BIT_P1_TSF_TIMER_L` reader - low 32bits of port1's tsft"]
pub type BitP1TsfTimerLR = crate::FieldReader<u32>;
#[doc = "Field `BIT_P1_TSF_TIMER_L` writer - low 32bits of port1's tsft"]
pub type BitP1TsfTimerLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low 32bits of port1's tsft"]
    #[inline(always)]
    pub fn bit_p1_tsf_timer_l(&self) -> BitP1TsfTimerLR {
        BitP1TsfTimerLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low 32bits of port1's tsft"]
    #[inline(always)]
    pub fn bit_p1_tsf_timer_l(&mut self) -> BitP1TsfTimerLW<'_, RegP1TsftrLSpec> {
        BitP1TsfTimerLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p1_tsftr_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_p1_tsftr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP1TsftrLSpec;
impl crate::RegisterSpec for RegP1TsftrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p1_tsftr_l::R`](R) reader structure"]
impl crate::Readable for RegP1TsftrLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_p1_tsftr_l::W`](W) writer structure"]
impl crate::Writable for RegP1TsftrLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_P1_TSFTR_L to value 0x01"]
impl crate::Resettable for RegP1TsftrLSpec {
    const RESET_VALUE: u32 = 0x01;
}
