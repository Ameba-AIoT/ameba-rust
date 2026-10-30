#[doc = "Register `REG_PS_TIMER_B_L` reader"]
pub type R = crate::R<RegPsTimerBLSpec>;
#[doc = "Register `REG_PS_TIMER_B_L` writer"]
pub type W = crate::W<RegPsTimerBLSpec>;
#[doc = "Field `BIT_PS_TIMER_B_L` reader - When the TSF (that select by reg 0x1520 \\[29:28\\]) matches the content of this register, the PS_TIMER_B_INT interrupt is issued"]
pub type BitPsTimerBLR = crate::FieldReader<u32>;
#[doc = "Field `BIT_PS_TIMER_B_L` writer - When the TSF (that select by reg 0x1520 \\[29:28\\]) matches the content of this register, the PS_TIMER_B_INT interrupt is issued"]
pub type BitPsTimerBLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When the TSF (that select by reg 0x1520 \\[29:28\\]) matches the content of this register, the PS_TIMER_B_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_ps_timer_b_l(&self) -> BitPsTimerBLR {
        BitPsTimerBLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When the TSF (that select by reg 0x1520 \\[29:28\\]) matches the content of this register, the PS_TIMER_B_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_ps_timer_b_l(&mut self) -> BitPsTimerBLW<'_, RegPsTimerBLSpec> {
        BitPsTimerBLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ps_timer_b_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ps_timer_b_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPsTimerBLSpec;
impl crate::RegisterSpec for RegPsTimerBLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ps_timer_b_l::R`](R) reader structure"]
impl crate::Readable for RegPsTimerBLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ps_timer_b_l::W`](W) writer structure"]
impl crate::Writable for RegPsTimerBLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PS_TIMER_B_L to value 0"]
impl crate::Resettable for RegPsTimerBLSpec {}
