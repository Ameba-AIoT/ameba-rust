#[doc = "Register `REG_PS_Timer_Early` reader"]
pub type R = crate::R<RegPsTimerEarlySpec>;
#[doc = "Register `REG_PS_Timer_Early` writer"]
pub type W = crate::W<RegPsTimerEarlySpec>;
#[doc = "Field `BIT_PS_Timer_A_Early` reader - It is used to wake up early at PS_Timer_A_Early Time. Time unit is 32us."]
pub type BitPsTimerAEarlyR = crate::FieldReader;
#[doc = "Field `BIT_PS_Timer_A_Early` writer - It is used to wake up early at PS_Timer_A_Early Time. Time unit is 32us."]
pub type BitPsTimerAEarlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PS_Timer_B_Early` reader - It is used to wake up early at PS_Timer_B_Early Time. Time unit is 32us."]
pub type BitPsTimerBEarlyR = crate::FieldReader;
#[doc = "Field `BIT_PS_Timer_B_Early` writer - It is used to wake up early at PS_Timer_B_Early Time. Time unit is 32us."]
pub type BitPsTimerBEarlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PS_Timer_C_Early` reader - It is used to wake up early at PS_Timer_C_Early Time. Time unit is 32us."]
pub type BitPsTimerCEarlyR = crate::FieldReader;
#[doc = "Field `BIT_PS_Timer_C_Early` writer - It is used to wake up early at PS_Timer_C_Early Time. Time unit is 32us."]
pub type BitPsTimerCEarlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PS_Timer_D_Early` reader - It is used to wake up early at PS_Timer_D_Early Time. Time unit is 32us."]
pub type BitPsTimerDEarlyR = crate::FieldReader;
#[doc = "Field `BIT_PS_Timer_D_Early` writer - It is used to wake up early at PS_Timer_D_Early Time. Time unit is 32us."]
pub type BitPsTimerDEarlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - It is used to wake up early at PS_Timer_A_Early Time. Time unit is 32us."]
    #[inline(always)]
    pub fn bit_ps_timer_a_early(&self) -> BitPsTimerAEarlyR {
        BitPsTimerAEarlyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - It is used to wake up early at PS_Timer_B_Early Time. Time unit is 32us."]
    #[inline(always)]
    pub fn bit_ps_timer_b_early(&self) -> BitPsTimerBEarlyR {
        BitPsTimerBEarlyR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - It is used to wake up early at PS_Timer_C_Early Time. Time unit is 32us."]
    #[inline(always)]
    pub fn bit_ps_timer_c_early(&self) -> BitPsTimerCEarlyR {
        BitPsTimerCEarlyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - It is used to wake up early at PS_Timer_D_Early Time. Time unit is 32us."]
    #[inline(always)]
    pub fn bit_ps_timer_d_early(&self) -> BitPsTimerDEarlyR {
        BitPsTimerDEarlyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - It is used to wake up early at PS_Timer_A_Early Time. Time unit is 32us."]
    #[inline(always)]
    pub fn bit_ps_timer_a_early(&mut self) -> BitPsTimerAEarlyW<'_, RegPsTimerEarlySpec> {
        BitPsTimerAEarlyW::new(self, 0)
    }
    #[doc = "Bits 8:15 - It is used to wake up early at PS_Timer_B_Early Time. Time unit is 32us."]
    #[inline(always)]
    pub fn bit_ps_timer_b_early(&mut self) -> BitPsTimerBEarlyW<'_, RegPsTimerEarlySpec> {
        BitPsTimerBEarlyW::new(self, 8)
    }
    #[doc = "Bits 16:23 - It is used to wake up early at PS_Timer_C_Early Time. Time unit is 32us."]
    #[inline(always)]
    pub fn bit_ps_timer_c_early(&mut self) -> BitPsTimerCEarlyW<'_, RegPsTimerEarlySpec> {
        BitPsTimerCEarlyW::new(self, 16)
    }
    #[doc = "Bits 24:31 - It is used to wake up early at PS_Timer_D_Early Time. Time unit is 32us."]
    #[inline(always)]
    pub fn bit_ps_timer_d_early(&mut self) -> BitPsTimerDEarlyW<'_, RegPsTimerEarlySpec> {
        BitPsTimerDEarlyW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ps_timer_early::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ps_timer_early::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPsTimerEarlySpec;
impl crate::RegisterSpec for RegPsTimerEarlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ps_timer_early::R`](R) reader structure"]
impl crate::Readable for RegPsTimerEarlySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ps_timer_early::W`](W) writer structure"]
impl crate::Writable for RegPsTimerEarlySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PS_Timer_Early to value 0"]
impl crate::Resettable for RegPsTimerEarlySpec {}
