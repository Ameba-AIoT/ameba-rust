#[doc = "Register `EEE_CR2` reader"]
pub type R = crate::R<EeeCr2Spec>;
#[doc = "Register `EEE_CR2` writer"]
pub type W = crate::W<EeeCr2Spec>;
#[doc = "Field `EEE_Timer_Tp_Giga` reader - Timer value in EEE function for TX decision maker T_pause. Unit: EEE_Timer_Unit_Giga_1. Default value is 10 us."]
pub type EeeTimerTpGigaR = crate::FieldReader;
#[doc = "Field `EEE_Timer_Tp_Giga` writer - Timer value in EEE function for TX decision maker T_pause. Unit: EEE_Timer_Unit_Giga_1. Default value is 10 us."]
pub type EeeTimerTpGigaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EEE_Timer_Td_Giga` reader - Timer value in EEE function for QoS based LPI decision maker T_tx_delay. Unit: EEE_Timer_Unit_Giga_2*4. Default value is 200 ms."]
pub type EeeTimerTdGigaR = crate::FieldReader;
#[doc = "Field `EEE_Timer_Td_Giga` writer - Timer value in EEE function for QoS based LPI decision maker T_tx_delay. Unit: EEE_Timer_Unit_Giga_2*4. Default value is 200 ms."]
pub type EeeTimerTdGigaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EEE_Timer_Tr_Giga` reader - Timer value in EEE function for T_tx_rate. Unit: EEE_Timer_Unit_Giga_1. Default value is 10 us."]
pub type EeeTimerTrGigaR = crate::FieldReader;
#[doc = "Field `EEE_Timer_Tr_Giga` writer - Timer value in EEE function for T_tx_rate. Unit: EEE_Timer_Unit_Giga_1. Default value is 10 us."]
pub type EeeTimerTrGigaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EEE_Timer_Tw_Giga` reader - Timer value in EEE function for T_wakeup. Value from 0~256. Unit: EEE_Timer_Unit_Giga_1. Default value is 16 us."]
pub type EeeTimerTwGigaR = crate::FieldReader;
#[doc = "Field `EEE_Timer_Tw_Giga` writer - Timer value in EEE function for T_wakeup. Value from 0~256. Unit: EEE_Timer_Unit_Giga_1. Default value is 16 us."]
pub type EeeTimerTwGigaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer value in EEE function for TX decision maker T_pause. Unit: EEE_Timer_Unit_Giga_1. Default value is 10 us."]
    #[inline(always)]
    pub fn eee_timer_tp_giga(&self) -> EeeTimerTpGigaR {
        EeeTimerTpGigaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timer value in EEE function for QoS based LPI decision maker T_tx_delay. Unit: EEE_Timer_Unit_Giga_2*4. Default value is 200 ms."]
    #[inline(always)]
    pub fn eee_timer_td_giga(&self) -> EeeTimerTdGigaR {
        EeeTimerTdGigaR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Timer value in EEE function for T_tx_rate. Unit: EEE_Timer_Unit_Giga_1. Default value is 10 us."]
    #[inline(always)]
    pub fn eee_timer_tr_giga(&self) -> EeeTimerTrGigaR {
        EeeTimerTrGigaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Timer value in EEE function for T_wakeup. Value from 0~256. Unit: EEE_Timer_Unit_Giga_1. Default value is 16 us."]
    #[inline(always)]
    pub fn eee_timer_tw_giga(&self) -> EeeTimerTwGigaR {
        EeeTimerTwGigaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer value in EEE function for TX decision maker T_pause. Unit: EEE_Timer_Unit_Giga_1. Default value is 10 us."]
    #[inline(always)]
    pub fn eee_timer_tp_giga(&mut self) -> EeeTimerTpGigaW<'_, EeeCr2Spec> {
        EeeTimerTpGigaW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Timer value in EEE function for QoS based LPI decision maker T_tx_delay. Unit: EEE_Timer_Unit_Giga_2*4. Default value is 200 ms."]
    #[inline(always)]
    pub fn eee_timer_td_giga(&mut self) -> EeeTimerTdGigaW<'_, EeeCr2Spec> {
        EeeTimerTdGigaW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Timer value in EEE function for T_tx_rate. Unit: EEE_Timer_Unit_Giga_1. Default value is 10 us."]
    #[inline(always)]
    pub fn eee_timer_tr_giga(&mut self) -> EeeTimerTrGigaW<'_, EeeCr2Spec> {
        EeeTimerTrGigaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Timer value in EEE function for T_wakeup. Value from 0~256. Unit: EEE_Timer_Unit_Giga_1. Default value is 16 us."]
    #[inline(always)]
    pub fn eee_timer_tw_giga(&mut self) -> EeeTimerTwGigaW<'_, EeeCr2Spec> {
        EeeTimerTwGigaW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eee_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eee_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeeCr2Spec;
impl crate::RegisterSpec for EeeCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eee_cr2::R`](R) reader structure"]
impl crate::Readable for EeeCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`eee_cr2::W`](W) writer structure"]
impl crate::Writable for EeeCr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEE_CR2 to value 0x100a_c40a"]
impl crate::Resettable for EeeCr2Spec {
    const RESET_VALUE: u32 = 0x100a_c40a;
}
