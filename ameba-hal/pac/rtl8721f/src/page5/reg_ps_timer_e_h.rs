#[doc = "Register `REG_PS_TIMER_E_H` reader"]
pub type R = crate::R<RegPsTimerEHSpec>;
#[doc = "Register `REG_PS_TIMER_E_H` writer"]
pub type W = crate::W<RegPsTimerEHSpec>;
#[doc = "Field `BIT_PS_TIMER_E_H` reader - When the TSF (that select by reg 0x1520 \\[13:12\\]) matches the content of this register, the PS_TIMER_E_INT interrupt is issued"]
pub type BitPsTimerEHR = crate::FieldReader<u32>;
#[doc = "Field `BIT_PS_TIMER_E_H` writer - When the TSF (that select by reg 0x1520 \\[13:12\\]) matches the content of this register, the PS_TIMER_E_INT interrupt is issued"]
pub type BitPsTimerEHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When the TSF (that select by reg 0x1520 \\[13:12\\]) matches the content of this register, the PS_TIMER_E_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_ps_timer_e_h(&self) -> BitPsTimerEHR {
        BitPsTimerEHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When the TSF (that select by reg 0x1520 \\[13:12\\]) matches the content of this register, the PS_TIMER_E_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_ps_timer_e_h(&mut self) -> BitPsTimerEHW<'_, RegPsTimerEHSpec> {
        BitPsTimerEHW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ps_timer_e_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ps_timer_e_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPsTimerEHSpec;
impl crate::RegisterSpec for RegPsTimerEHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ps_timer_e_h::R`](R) reader structure"]
impl crate::Readable for RegPsTimerEHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ps_timer_e_h::W`](W) writer structure"]
impl crate::Writable for RegPsTimerEHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PS_TIMER_E_H to value 0"]
impl crate::Resettable for RegPsTimerEHSpec {}
