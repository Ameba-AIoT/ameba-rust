#[doc = "Register `REG_PS_TIMER_D_H` reader"]
pub type R = crate::R<RegPsTimerDHSpec>;
#[doc = "Register `REG_PS_TIMER_D_H` writer"]
pub type W = crate::W<RegPsTimerDHSpec>;
#[doc = "Field `BIT_PS_TIMER_D_H` reader - When the TSF(that select by reg 0x1520 \\[16:17\\]) matches the content of this register, the PS_TIMER_D_INT interrupt is issued"]
pub type BitPsTimerDHR = crate::FieldReader<u32>;
#[doc = "Field `BIT_PS_TIMER_D_H` writer - When the TSF(that select by reg 0x1520 \\[16:17\\]) matches the content of this register, the PS_TIMER_D_INT interrupt is issued"]
pub type BitPsTimerDHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When the TSF(that select by reg 0x1520 \\[16:17\\]) matches the content of this register, the PS_TIMER_D_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_ps_timer_d_h(&self) -> BitPsTimerDHR {
        BitPsTimerDHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When the TSF(that select by reg 0x1520 \\[16:17\\]) matches the content of this register, the PS_TIMER_D_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_ps_timer_d_h(&mut self) -> BitPsTimerDHW<'_, RegPsTimerDHSpec> {
        BitPsTimerDHW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ps_timer_d_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ps_timer_d_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPsTimerDHSpec;
impl crate::RegisterSpec for RegPsTimerDHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ps_timer_d_h::R`](R) reader structure"]
impl crate::Readable for RegPsTimerDHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ps_timer_d_h::W`](W) writer structure"]
impl crate::Writable for RegPsTimerDHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PS_TIMER_D_H to value 0"]
impl crate::Resettable for RegPsTimerDHSpec {}
