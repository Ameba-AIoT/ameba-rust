#[doc = "Register `EEEP_CR3` reader"]
pub type R = crate::R<EeepCr3Spec>;
#[doc = "Register `EEEP_CR3` writer"]
pub type W = crate::W<EeepCr3Spec>;
#[doc = "Field `EEEP_RX_THR` reader - Set the RX threshold in EEEP function. Unit: Byte; EEEP_RX_THR=Round \\[(T_rx_rate*Rate) / 8 \\]. If you set T_rx_rate=192us,Rate=10Mbps, so set EEEP_TX_THR= 0xF0 (240 bytes). EEPROM and register configurable. //HYH???"]
pub type EeepRxThrR = crate::FieldReader<u16>;
#[doc = "Field `EEEP_RX_THR` writer - Set the RX threshold in EEEP function. Unit: Byte; EEEP_RX_THR=Round \\[(T_rx_rate*Rate) / 8 \\]. If you set T_rx_rate=192us,Rate=10Mbps, so set EEEP_TX_THR= 0xF0 (240 bytes). EEPROM and register configurable. //HYH???"]
pub type EeepRxThrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EEEP_TX_THR` reader - Set the TX threshold in EEEP function. Unit: Byte; EEEP_TX_THR=Round \\[(T_rx_rate*Rate) / 8 \\]. If you set T_rx_rate=192us,Rate=10Mbps, so set EEEP_TX_THR= 0xF0 (240 bytes). EEPROM and register configurable.//HYH???"]
pub type EeepTxThrR = crate::FieldReader<u16>;
#[doc = "Field `EEEP_TX_THR` writer - Set the TX threshold in EEEP function. Unit: Byte; EEEP_TX_THR=Round \\[(T_rx_rate*Rate) / 8 \\]. If you set T_rx_rate=192us,Rate=10Mbps, so set EEEP_TX_THR= 0xF0 (240 bytes). EEPROM and register configurable.//HYH???"]
pub type EeepTxThrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Set the RX threshold in EEEP function. Unit: Byte; EEEP_RX_THR=Round \\[(T_rx_rate*Rate) / 8 \\]. If you set T_rx_rate=192us,Rate=10Mbps, so set EEEP_TX_THR= 0xF0 (240 bytes). EEPROM and register configurable. //HYH???"]
    #[inline(always)]
    pub fn eeep_rx_thr(&self) -> EeepRxThrR {
        EeepRxThrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Set the TX threshold in EEEP function. Unit: Byte; EEEP_TX_THR=Round \\[(T_rx_rate*Rate) / 8 \\]. If you set T_rx_rate=192us,Rate=10Mbps, so set EEEP_TX_THR= 0xF0 (240 bytes). EEPROM and register configurable.//HYH???"]
    #[inline(always)]
    pub fn eeep_tx_thr(&self) -> EeepTxThrR {
        EeepTxThrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set the RX threshold in EEEP function. Unit: Byte; EEEP_RX_THR=Round \\[(T_rx_rate*Rate) / 8 \\]. If you set T_rx_rate=192us,Rate=10Mbps, so set EEEP_TX_THR= 0xF0 (240 bytes). EEPROM and register configurable. //HYH???"]
    #[inline(always)]
    pub fn eeep_rx_thr(&mut self) -> EeepRxThrW<'_, EeepCr3Spec> {
        EeepRxThrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Set the TX threshold in EEEP function. Unit: Byte; EEEP_TX_THR=Round \\[(T_rx_rate*Rate) / 8 \\]. If you set T_rx_rate=192us,Rate=10Mbps, so set EEEP_TX_THR= 0xF0 (240 bytes). EEPROM and register configurable.//HYH???"]
    #[inline(always)]
    pub fn eeep_tx_thr(&mut self) -> EeepTxThrW<'_, EeepCr3Spec> {
        EeepTxThrW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eeep_cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeep_cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeepCr3Spec;
impl crate::RegisterSpec for EeepCr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeep_cr3::R`](R) reader structure"]
impl crate::Readable for EeepCr3Spec {}
#[doc = "`write(|w| ..)` method takes [`eeep_cr3::W`](W) writer structure"]
impl crate::Writable for EeepCr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEEP_CR3 to value 0x00f0_00f0"]
impl crate::Resettable for EeepCr3Spec {
    const RESET_VALUE: u32 = 0x00f0_00f0;
}
