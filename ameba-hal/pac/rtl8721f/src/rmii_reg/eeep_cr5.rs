#[doc = "Register `EEEP_CR5` reader"]
pub type R = crate::R<EeepCr5Spec>;
#[doc = "Register `EEEP_CR5` writer"]
pub type W = crate::W<EeepCr5Spec>;
#[doc = "Field `EEEP_Timer_RX_SLEEP` reader - value of Timer in EEEP function for T_rx_sleep. Unit: microsecond. Default value is 192 us. EEPROM and register configurable."]
pub type EeepTimerRxSleepR = crate::FieldReader;
#[doc = "Field `EEEP_Timer_RX_SLEEP` writer - value of Timer in EEEP function for T_rx_sleep. Unit: microsecond. Default value is 192 us. EEPROM and register configurable."]
pub type EeepTimerRxSleepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EEEP_Timer_Unit` reader - Set the unit for timer in EEEP function. 00: 1us; 01: 16us; 10: 128us; 11: 1024us. EEPROM and register configurable."]
pub type EeepTimerUnitR = crate::FieldReader;
#[doc = "Field `EEEP_Timer_Unit` writer - Set the unit for timer in EEEP function. 00: 1us; 01: 16us; 10: 128us; 11: 1024us. EEPROM and register configurable."]
pub type EeepTimerUnitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EEEP_TX_RATE` reader - "]
pub type EeepTxRateR = crate::FieldReader;
#[doc = "Field `EEEP_TX_RATE` writer - "]
pub type EeepTxRateW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - value of Timer in EEEP function for T_rx_sleep. Unit: microsecond. Default value is 192 us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_rx_sleep(&self) -> EeepTimerRxSleepR {
        EeepTimerRxSleepR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Set the unit for timer in EEEP function. 00: 1us; 01: 16us; 10: 128us; 11: 1024us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_unit(&self) -> EeepTimerUnitR {
        EeepTimerUnitR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn eeep_tx_rate(&self) -> EeepTxRateR {
        EeepTxRateR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - value of Timer in EEEP function for T_rx_sleep. Unit: microsecond. Default value is 192 us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_rx_sleep(&mut self) -> EeepTimerRxSleepW<'_, EeepCr5Spec> {
        EeepTimerRxSleepW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Set the unit for timer in EEEP function. 00: 1us; 01: 16us; 10: 128us; 11: 1024us. EEPROM and register configurable."]
    #[inline(always)]
    pub fn eeep_timer_unit(&mut self) -> EeepTimerUnitW<'_, EeepCr5Spec> {
        EeepTimerUnitW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn eeep_tx_rate(&mut self) -> EeepTxRateW<'_, EeepCr5Spec> {
        EeepTxRateW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eeep_cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeep_cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeepCr5Spec;
impl crate::RegisterSpec for EeepCr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeep_cr5::R`](R) reader structure"]
impl crate::Readable for EeepCr5Spec {}
#[doc = "`write(|w| ..)` method takes [`eeep_cr5::W`](W) writer structure"]
impl crate::Writable for EeepCr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEEP_CR5 to value 0x010c"]
impl crate::Resettable for EeepCr5Spec {
    const RESET_VALUE: u32 = 0x010c;
}
