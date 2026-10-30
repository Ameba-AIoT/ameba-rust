#[doc = "Register `REG_CPUMGQ_PARAMETER` reader"]
pub type R = crate::R<RegCpumgqParameterSpec>;
#[doc = "Register `REG_CPUMGQ_PARAMETER` writer"]
pub type W = crate::W<RegCpumgqParameterSpec>;
#[doc = "Field `BIT_AIFS` reader - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch"]
pub type BitAifsR = crate::FieldReader;
#[doc = "Field `BIT_AIFS` writer - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch"]
pub type BitAifsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_CW` reader - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
pub type BitCwR = crate::FieldReader;
#[doc = "Field `BIT_CW` writer - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
pub type BitCwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PS_Timer_E_Early` reader - It is used to wake up early at PS_Timer_E_Early Time. Time unit is 32us."]
pub type BitPsTimerEEarlyR = crate::FieldReader;
#[doc = "Field `BIT_PS_Timer_E_Early` writer - It is used to wake up early at PS_Timer_E_Early Time. Time unit is 32us."]
pub type BitPsTimerEEarlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch"]
    #[inline(always)]
    pub fn bit_aifs(&self) -> BitAifsR {
        BitAifsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
    #[inline(always)]
    pub fn bit_cw(&self) -> BitCwR {
        BitCwR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - It is used to wake up early at PS_Timer_E_Early Time. Time unit is 32us."]
    #[inline(always)]
    pub fn bit_ps_timer_e_early(&self) -> BitPsTimerEEarlyR {
        BitPsTimerEEarlyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Arbitrary Inter frame space In unit of microsecond. AIFS indicates the time a non-AP QSTA should defer before either invoking a backoff or starting a transmission Default: BK = 49h; BE = 25h; VI = 1Ch; VO = 1Ch"]
    #[inline(always)]
    pub fn bit_aifs(&mut self) -> BitAifsW<'_, RegCpumgqParameterSpec> {
        BitAifsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - ECWmin and ECWmax encode the values of Cwmin and Cwmax respectively in an exponent form. Ex: Cwmax = 2^(ECWmax)-1 Default: BK = Ah/5h; BE = Ah/5h; VI = 5h/4h; VO = 4h/3h."]
    #[inline(always)]
    pub fn bit_cw(&mut self) -> BitCwW<'_, RegCpumgqParameterSpec> {
        BitCwW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegCpumgqParameterSpec> {
        DummyW::new(self, 16)
    }
    #[doc = "Bits 24:31 - It is used to wake up early at PS_Timer_E_Early Time. Time unit is 32us."]
    #[inline(always)]
    pub fn bit_ps_timer_e_early(&mut self) -> BitPsTimerEEarlyW<'_, RegCpumgqParameterSpec> {
        BitPsTimerEEarlyW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cpumgq_parameter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cpumgq_parameter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCpumgqParameterSpec;
impl crate::RegisterSpec for RegCpumgqParameterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cpumgq_parameter::R`](R) reader structure"]
impl crate::Readable for RegCpumgqParameterSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cpumgq_parameter::W`](W) writer structure"]
impl crate::Writable for RegCpumgqParameterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CPUMGQ_PARAMETER to value 0x3224"]
impl crate::Resettable for RegCpumgqParameterSpec {
    const RESET_VALUE: u32 = 0x3224;
}
