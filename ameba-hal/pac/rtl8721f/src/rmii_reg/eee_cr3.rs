#[doc = "Register `EEE_CR3` reader"]
pub type R = crate::R<EeeCr3Spec>;
#[doc = "Register `EEE_CR3` writer"]
pub type W = crate::W<EeeCr3Spec>;
#[doc = "Field `EEE_Timer_Tp_100` reader - Timer value in EEE function for TX decision maker T_pause. Unit: EEE_Timer_Unit_100_1. Default value is 10 us."]
pub type EeeTimerTp100R = crate::FieldReader;
#[doc = "Field `EEE_Timer_Tp_100` writer - Timer value in EEE function for TX decision maker T_pause. Unit: EEE_Timer_Unit_100_1. Default value is 10 us."]
pub type EeeTimerTp100W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EEE_Timer_Td_100` reader - Timer value in EEE function for QoS based LPI decision maker T_tx_delay. Unit: EEE_Timer_Unit_100_2*4. Default value is 1s."]
pub type EeeTimerTd100R = crate::FieldReader;
#[doc = "Field `EEE_Timer_Td_100` writer - Timer value in EEE function for QoS based LPI decision maker T_tx_delay. Unit: EEE_Timer_Unit_100_2*4. Default value is 1s."]
pub type EeeTimerTd100W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EEE_Timer_Tr_100` reader - Timer value in EEE function for T_tx_rate. Unit: EEE_Timer_Unit_100_1. Default value is 10 us."]
pub type EeeTimerTr100R = crate::FieldReader;
#[doc = "Field `EEE_Timer_Tr_100` writer - Timer value in EEE function for T_tx_rate. Unit: EEE_Timer_Unit_100_1. Default value is 10 us."]
pub type EeeTimerTr100W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EEE_Timer_Tw_100` reader - Timer value in EEE function for T_wakeup. Value from 0~256. Unit: EEE_Timer_Unit_100_1 Default value is 30 us."]
pub type EeeTimerTw100R = crate::FieldReader;
#[doc = "Field `EEE_Timer_Tw_100` writer - Timer value in EEE function for T_wakeup. Value from 0~256. Unit: EEE_Timer_Unit_100_1 Default value is 30 us."]
pub type EeeTimerTw100W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer value in EEE function for TX decision maker T_pause. Unit: EEE_Timer_Unit_100_1. Default value is 10 us."]
    #[inline(always)]
    pub fn eee_timer_tp_100(&self) -> EeeTimerTp100R {
        EeeTimerTp100R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timer value in EEE function for QoS based LPI decision maker T_tx_delay. Unit: EEE_Timer_Unit_100_2*4. Default value is 1s."]
    #[inline(always)]
    pub fn eee_timer_td_100(&self) -> EeeTimerTd100R {
        EeeTimerTd100R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Timer value in EEE function for T_tx_rate. Unit: EEE_Timer_Unit_100_1. Default value is 10 us."]
    #[inline(always)]
    pub fn eee_timer_tr_100(&self) -> EeeTimerTr100R {
        EeeTimerTr100R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Timer value in EEE function for T_wakeup. Value from 0~256. Unit: EEE_Timer_Unit_100_1 Default value is 30 us."]
    #[inline(always)]
    pub fn eee_timer_tw_100(&self) -> EeeTimerTw100R {
        EeeTimerTw100R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer value in EEE function for TX decision maker T_pause. Unit: EEE_Timer_Unit_100_1. Default value is 10 us."]
    #[inline(always)]
    pub fn eee_timer_tp_100(&mut self) -> EeeTimerTp100W<'_, EeeCr3Spec> {
        EeeTimerTp100W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Timer value in EEE function for QoS based LPI decision maker T_tx_delay. Unit: EEE_Timer_Unit_100_2*4. Default value is 1s."]
    #[inline(always)]
    pub fn eee_timer_td_100(&mut self) -> EeeTimerTd100W<'_, EeeCr3Spec> {
        EeeTimerTd100W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Timer value in EEE function for T_tx_rate. Unit: EEE_Timer_Unit_100_1. Default value is 10 us."]
    #[inline(always)]
    pub fn eee_timer_tr_100(&mut self) -> EeeTimerTr100W<'_, EeeCr3Spec> {
        EeeTimerTr100W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Timer value in EEE function for T_wakeup. Value from 0~256. Unit: EEE_Timer_Unit_100_1 Default value is 30 us."]
    #[inline(always)]
    pub fn eee_timer_tw_100(&mut self) -> EeeTimerTw100W<'_, EeeCr3Spec> {
        EeeTimerTw100W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eee_cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eee_cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeeCr3Spec;
impl crate::RegisterSpec for EeeCr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eee_cr3::R`](R) reader structure"]
impl crate::Readable for EeeCr3Spec {}
#[doc = "`write(|w| ..)` method takes [`eee_cr3::W`](W) writer structure"]
impl crate::Writable for EeeCr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEE_CR3 to value 0x1e0a_f40a"]
impl crate::Resettable for EeeCr3Spec {
    const RESET_VALUE: u32 = 0x1e0a_f40a;
}
