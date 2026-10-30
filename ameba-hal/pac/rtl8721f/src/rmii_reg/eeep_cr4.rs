#[doc = "Register `EEEP_CR4` reader"]
pub type R = crate::R<EeepCr4Spec>;
#[doc = "Register `EEEP_CR4` writer"]
pub type W = crate::W<EeepCr4Spec>;
#[doc = "Field `EEEP_Timer_PAUSE` reader - Timer value in EEEP function for T_pause_on. Unit:refer to EEEP_Timer_Unit. Default value is 16 us. EEPROM and register configurable."]
pub type EeepTimerPauseR = crate::FieldReader;
#[doc = "Field `EEEP_Timer_PAUSE` writer - Timer value in EEEP function for T_pause_on. Unit:refer to EEEP_Timer_Unit. Default value is 16 us. EEPROM and register configurable."]
pub type EeepTimerPauseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EEEP_Timer_TX_WAIT` reader - Timer value in EEEP function for T_tx_wait. Unit: microsecond. Default value is 16 us. EEPROM and register configurable."]
pub type EeepTimerTxWaitR = crate::FieldReader;
#[doc = "Field `EEEP_Timer_TX_WAIT` writer - Timer value in EEEP function for T_tx_wait. Unit: microsecond. Default value is 16 us. EEPROM and register configurable."]
pub type EeepTimerTxWaitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EEEP_Timer_RX_WAKE` reader - Timer value in EEEP function for T_rx_wake. Unit: microsecond. Default value is 32 us. EEPROM and register configurable."]
pub type EeepTimerRxWakeR = crate::FieldReader;
#[doc = "Field `EEEP_Timer_RX_WAKE` writer - Timer value in EEEP function for T_rx_wake. Unit: microsecond. Default value is 32 us. EEPROM and register configurable."]
pub type EeepTimerRxWakeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EEEP_Timer_RX_RATE` reader - Timer value in EEEP function for T_rx_rate. Unit: microsecond. Default value is 192 us. EEPROM and register configurable."]
pub type EeepTimerRxRateR = crate::FieldReader;
#[doc = "Field `EEEP_Timer_RX_RATE` writer - Timer value in EEEP function for T_rx_rate. Unit: microsecond. Default value is 192 us. EEPROM and register configurable."]
pub type EeepTimerRxRateW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer value in EEEP function for T_pause_on. Unit:refer to EEEP_Timer_Unit. Default value is 16 us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_pause(&self) -> EeepTimerPauseR {
        EeepTimerPauseR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timer value in EEEP function for T_tx_wait. Unit: microsecond. Default value is 16 us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_tx_wait(&self) -> EeepTimerTxWaitR {
        EeepTimerTxWaitR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Timer value in EEEP function for T_rx_wake. Unit: microsecond. Default value is 32 us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_rx_wake(&self) -> EeepTimerRxWakeR {
        EeepTimerRxWakeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Timer value in EEEP function for T_rx_rate. Unit: microsecond. Default value is 192 us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_rx_rate(&self) -> EeepTimerRxRateR {
        EeepTimerRxRateR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer value in EEEP function for T_pause_on. Unit:refer to EEEP_Timer_Unit. Default value is 16 us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_pause(&mut self) -> EeepTimerPauseW<'_, EeepCr4Spec> {
        EeepTimerPauseW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Timer value in EEEP function for T_tx_wait. Unit: microsecond. Default value is 16 us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_tx_wait(&mut self) -> EeepTimerTxWaitW<'_, EeepCr4Spec> {
        EeepTimerTxWaitW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Timer value in EEEP function for T_rx_wake. Unit: microsecond. Default value is 32 us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_rx_wake(&mut self) -> EeepTimerRxWakeW<'_, EeepCr4Spec> {
        EeepTimerRxWakeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Timer value in EEEP function for T_rx_rate. Unit: microsecond. Default value is 192 us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_rx_rate(&mut self) -> EeepTimerRxRateW<'_, EeepCr4Spec> {
        EeepTimerRxRateW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eeep_cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeep_cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeepCr4Spec;
impl crate::RegisterSpec for EeepCr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeep_cr4::R`](R) reader structure"]
impl crate::Readable for EeepCr4Spec {}
#[doc = "`write(|w| ..)` method takes [`eeep_cr4::W`](W) writer structure"]
impl crate::Writable for EeepCr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEEP_CR4 to value 0x0c02_0101"]
impl crate::Resettable for EeepCr4Spec {
    const RESET_VALUE: u32 = 0x0c02_0101;
}
