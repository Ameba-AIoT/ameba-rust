#[doc = "Register `REG_PS_TIMER_B_H` reader"]
pub type R = crate::R<RegPsTimerBHSpec>;
#[doc = "Register `REG_PS_TIMER_B_H` writer"]
pub type W = crate::W<RegPsTimerBHSpec>;
#[doc = "Field `BIT_PS_TIMER_B_H` reader - When the TSF(that select by reg 0x1520 \\[29:28\\]) matches the content of this register, the PS_TIMER_B_INT interrupt is issued"]
pub type BitPsTimerBHR = crate::FieldReader<u32>;
#[doc = "Field `BIT_PS_TIMER_B_H` writer - When the TSF(that select by reg 0x1520 \\[29:28\\]) matches the content of this register, the PS_TIMER_B_INT interrupt is issued"]
pub type BitPsTimerBHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When the TSF(that select by reg 0x1520 \\[29:28\\]) matches the content of this register, the PS_TIMER_B_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_ps_timer_b_h(&self) -> BitPsTimerBHR {
        BitPsTimerBHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When the TSF(that select by reg 0x1520 \\[29:28\\]) matches the content of this register, the PS_TIMER_B_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_ps_timer_b_h(&mut self) -> BitPsTimerBHW<'_, RegPsTimerBHSpec> {
        BitPsTimerBHW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ps_timer_b_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ps_timer_b_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPsTimerBHSpec;
impl crate::RegisterSpec for RegPsTimerBHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ps_timer_b_h::R`](R) reader structure"]
impl crate::Readable for RegPsTimerBHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ps_timer_b_h::W`](W) writer structure"]
impl crate::Writable for RegPsTimerBHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PS_TIMER_B_H to value 0"]
impl crate::Resettable for RegPsTimerBHSpec {}
