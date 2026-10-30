#[doc = "Register `REG_PS_TIMER_E_L` reader"]
pub type R = crate::R<RegPsTimerELSpec>;
#[doc = "Register `REG_PS_TIMER_E_L` writer"]
pub type W = crate::W<RegPsTimerELSpec>;
#[doc = "Field `BIT_PS_TIMER_E_L` reader - When the TSF (that select by reg 0x1520 \\[13:12\\]) matches the content of this register, the PS_TIMER_E_INT interrupt is issued"]
pub type BitPsTimerELR = crate::FieldReader<u32>;
#[doc = "Field `BIT_PS_TIMER_E_L` writer - When the TSF (that select by reg 0x1520 \\[13:12\\]) matches the content of this register, the PS_TIMER_E_INT interrupt is issued"]
pub type BitPsTimerELW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When the TSF (that select by reg 0x1520 \\[13:12\\]) matches the content of this register, the PS_TIMER_E_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_ps_timer_e_l(&self) -> BitPsTimerELR {
        BitPsTimerELR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When the TSF (that select by reg 0x1520 \\[13:12\\]) matches the content of this register, the PS_TIMER_E_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_ps_timer_e_l(&mut self) -> BitPsTimerELW<'_, RegPsTimerELSpec> {
        BitPsTimerELW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ps_timer_e_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ps_timer_e_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPsTimerELSpec;
impl crate::RegisterSpec for RegPsTimerELSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ps_timer_e_l::R`](R) reader structure"]
impl crate::Readable for RegPsTimerELSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ps_timer_e_l::W`](W) writer structure"]
impl crate::Writable for RegPsTimerELSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PS_TIMER_E_L to value 0"]
impl crate::Resettable for RegPsTimerELSpec {}
