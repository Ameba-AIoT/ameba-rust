#[doc = "Register `EEE_CR1` reader"]
pub type R = crate::R<EeeCr1Spec>;
#[doc = "Register `EEE_CR1` writer"]
pub type W = crate::W<EeeCr1Spec>;
#[doc = "Field `EEE_Timer_Unit_100_2` reader - Set the unit for timer T_tx_delay in EEE function. 00: 8uS; 01: 64uS; 10: 512uS; 11: 1024uS."]
pub type EeeTimerUnit100_2R = crate::FieldReader;
#[doc = "Field `EEE_Timer_Unit_100_2` writer - Set the unit for timer T_tx_delay in EEE function. 00: 8uS; 01: 64uS; 10: 512uS; 11: 1024uS."]
pub type EeeTimerUnit100_2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EEE_Timer_Unit_100_1` reader - Set the unit for timer T_wakeup, T_tx_rate, and T_pause in EEE function. 00: 1uS; 01: 16uS; 10: 128uS; 11: 1024uS."]
pub type EeeTimerUnit100_1R = crate::FieldReader;
#[doc = "Field `EEE_Timer_Unit_100_1` writer - Set the unit for timer T_wakeup, T_tx_rate, and T_pause in EEE function. 00: 1uS; 01: 16uS; 10: 128uS; 11: 1024uS."]
pub type EeeTimerUnit100_1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EEE_Timer_Unit_Giga_2` reader - Set the unit for timer T_tx_delay in EEE function. 00: 8uS; 01: 64uS; 10: 256uS; 11: 1024uS."]
pub type EeeTimerUnitGiga2R = crate::FieldReader;
#[doc = "Field `EEE_Timer_Unit_Giga_2` writer - Set the unit for timer T_tx_delay in EEE function. 00: 8uS; 01: 64uS; 10: 256uS; 11: 1024uS."]
pub type EeeTimerUnitGiga2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EEE_Timer_Unit_Giga_1` reader - Set the unit for timer T_wakeup, T_tx_rate, and T_pause in EEE function. 00: 1uS; 01: 16 uS; 10: 128 uS; 11: 1024uS."]
pub type EeeTimerUnitGiga1R = crate::FieldReader;
#[doc = "Field `EEE_Timer_Unit_Giga_1` writer - Set the unit for timer T_wakeup, T_tx_rate, and T_pause in EEE function. 00: 1uS; 01: 16 uS; 10: 128 uS; 11: 1024uS."]
pub type EeeTimerUnitGiga1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EEE_TXTimer_EN` reader - Enable TX meter to count TX LPI status time"]
pub type EeeTxtimerEnR = crate::BitReader;
#[doc = "Field `EEE_TXTimer_EN` writer - Enable TX meter to count TX LPI status time"]
pub type EeeTxtimerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEE_RXTimer_EN` reader - Enable RX meter to count RX LPI status time"]
pub type EeeRxtimerEnR = crate::BitReader;
#[doc = "Field `EEE_RXTimer_EN` writer - Enable RX meter to count RX LPI status time"]
pub type EeeRxtimerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEE_Req_Set0` reader - Enable LPI TX request Rate decision maker for EEE. 1: enable; 0: disable."]
pub type EeeReqSet0R = crate::BitReader;
#[doc = "Field `EEE_Req_Set0` writer - Enable LPI TX request Rate decision maker for EEE. 1: enable; 0: disable."]
pub type EeeReqSet0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEE_Req_Set1` reader - Enable LPI TX request queue empty decision maker for EEE. 1: enable; 0: disable. For RLE0390(formerly RL6166), only for tx is single descriptor operation. For next version, this bit can support multi-descriptor operation."]
pub type EeeReqSet1R = crate::BitReader;
#[doc = "Field `EEE_Req_Set1` writer - Enable LPI TX request queue empty decision maker for EEE. 1: enable; 0: disable. For RLE0390(formerly RL6166), only for tx is single descriptor operation. For next version, this bit can support multi-descriptor operation."]
pub type EeeReqSet1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEE_Req_Set2` reader - Enable LPI TX request RX pause on frame decision maker for EEE. 1: enable; 0: disable."]
pub type EeeReqSet2R = crate::BitReader;
#[doc = "Field `EEE_Req_Set2` writer - Enable LPI TX request RX pause on frame decision maker for EEE. 1: enable; 0: disable."]
pub type EeeReqSet2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEE_Wake_Set0` reader - Enable LPI TX transmit wakeup decision maker for EEE. 1: enable; 0: disable."]
pub type EeeWakeSet0R = crate::BitReader;
#[doc = "Field `EEE_Wake_Set0` writer - Enable LPI TX transmit wakeup decision maker for EEE. 1: enable; 0: disable."]
pub type EeeWakeSet0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEE_Wake_Set1` reader - Enable LPI TX high priority pkt Wakeup decision maker for EEE. 1: enable; 0: disable. Gmac will leave lpi mode when EEE_CR1_reg.txq_th is met or one of the egress is larger than 512B."]
pub type EeeWakeSet1R = crate::BitReader;
#[doc = "Field `EEE_Wake_Set1` writer - Enable LPI TX high priority pkt Wakeup decision maker for EEE. 1: enable; 0: disable. Gmac will leave lpi mode when EEE_CR1_reg.txq_th is met or one of the egress is larger than 512B."]
pub type EeeWakeSet1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEE_PauseFlag` reader - Set indication for LPI request due to RX pause on frame. 1: indicate RX pause on frame; 0: no indicate."]
pub type EeePauseFlagR = crate::BitReader;
#[doc = "Field `EEE_RX_STS` reader - Indicate Port 0 EEE RX status 1: RX is in LPI mode. 0: RX is in normal mode."]
pub type EeeRxStsR = crate::BitReader;
#[doc = "Field `EEE_TX_STS` reader - Indicate EEE TX status 1: TX is in LPI mode. 0: TX is in normal mode."]
pub type EeeTxStsR = crate::BitReader;
#[doc = "Field `EEE_STS` reader - Indicate EEE status, both TX and RX. 1: Both TX and RX in LPI mode simultaneously. 0: not both TX and RX in LPI mode simultaneously"]
pub type EeeStsR = crate::BitReader;
#[doc = "Field `Eee_outq_lowq_over` reader - SW has many packets to be tranmitted, then set this bit to tell HW leave lpi state in condition2."]
pub type EeeOutqLowqOverR = crate::BitReader;
#[doc = "Field `Eee_outq_lowq_over` writer - SW has many packets to be tranmitted, then set this bit to tell HW leave lpi state in condition2."]
pub type EeeOutqLowqOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_EEE_RX` reader - Enable MAC RX EEE ability 0: Disable EEE ability for RX. 1: Enable EEE ability for RX. Application note : The following methods could be used to disable EEE function. a) n\\[1:0\\] = 00 -> completely disable EEE function, or b) n\\[3:2\\] = 00 -> disable /1000Mbps EEE capability (excludes 10Mbps )."]
pub type EnEeeRxR = crate::BitReader;
#[doc = "Field `EN_EEE_RX` writer - Enable MAC RX EEE ability 0: Disable EEE ability for RX. 1: Enable EEE ability for RX. Application note : The following methods could be used to disable EEE function. a) n\\[1:0\\] = 00 -> completely disable EEE function, or b) n\\[3:2\\] = 00 -> disable /1000Mbps EEE capability (excludes 10Mbps )."]
pub type EnEeeRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_EEE_TX` reader - Enable MAC TX EEE ability 0: Disable EEE ability for TX. 1: Enable EEE ability for TX."]
pub type EnEeeTxR = crate::BitReader;
#[doc = "Field `EN_EEE_TX` writer - Enable MAC TX EEE ability 0: Disable EEE ability for TX. 1: Enable EEE ability for TX."]
pub type EnEeeTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_EEE` reader - Set same as PHY setting, for LED."]
pub type EnEeeR = crate::BitReader;
#[doc = "Field `EN_EEE` writer - Set same as PHY setting, for LED."]
pub type EnEeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRC_EEE` reader - If EN_FRC_EEE=1, FRC_EEE_100 0: Disable MAC EEE ability for 100BASE-TX. 1: Enable MAC EEE ability for 100BASE-TX. If EN_FRC_EEE=0, FRC_EEE_100 means EEE advertisement register 7.60 bit\\[1\\]"]
pub type FrcEeeR = crate::BitReader;
#[doc = "Field `FRC_EEE` writer - If EN_FRC_EEE=1, FRC_EEE_100 0: Disable MAC EEE ability for 100BASE-TX. 1: Enable MAC EEE ability for 100BASE-TX. If EN_FRC_EEE=0, FRC_EEE_100 means EEE advertisement register 7.60 bit\\[1\\]"]
pub type FrcEeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRC_EEE_Giga` reader - If EN_FRC_EEE=1, FRC_EEE_Giga: 0: Disable MAC EEE ability for 1000BASE-T. 1: Enable MAC EEE ability for 1000BASE-T. If EN_FRC_EEE=0, FRC_EEE_Giga means EEE advertisement register 7.60 bit\\[2\\]"]
pub type FrcEeeGigaR = crate::BitReader;
#[doc = "Field `FRC_EEE_Giga` writer - If EN_FRC_EEE=1, FRC_EEE_Giga: 0: Disable MAC EEE ability for 1000BASE-T. 1: Enable MAC EEE ability for 1000BASE-T. If EN_FRC_EEE=0, FRC_EEE_Giga means EEE advertisement register 7.60 bit\\[2\\]"]
pub type FrcEeeGigaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_FRC_EEE` reader - Enable EEE force mode 0: Disable EEE FORCE Mode 1: Enable EEE FORCE Mode If EN_FRC_EEE=1, both FRC_EEE_Giga and FRC_EEE_100 are used to force MAC operation ability."]
pub type EnFrcEeeR = crate::BitReader;
#[doc = "Field `EN_FRC_EEE` writer - Enable EEE force mode 0: Disable EEE FORCE Mode 1: Enable EEE FORCE Mode If EN_FRC_EEE=1, both FRC_EEE_Giga and FRC_EEE_100 are used to force MAC operation ability."]
pub type EnFrcEeeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Set the unit for timer T_tx_delay in EEE function. 00: 8uS; 01: 64uS; 10: 512uS; 11: 1024uS."]
    #[inline(always)]
    pub fn eee_timer_unit_100_2(&self) -> EeeTimerUnit100_2R {
        EeeTimerUnit100_2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set the unit for timer T_wakeup, T_tx_rate, and T_pause in EEE function. 00: 1uS; 01: 16uS; 10: 128uS; 11: 1024uS."]
    #[inline(always)]
    pub fn eee_timer_unit_100_1(&self) -> EeeTimerUnit100_1R {
        EeeTimerUnit100_1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Set the unit for timer T_tx_delay in EEE function. 00: 8uS; 01: 64uS; 10: 256uS; 11: 1024uS."]
    #[inline(always)]
    pub fn eee_timer_unit_giga_2(&self) -> EeeTimerUnitGiga2R {
        EeeTimerUnitGiga2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Set the unit for timer T_wakeup, T_tx_rate, and T_pause in EEE function. 00: 1uS; 01: 16 uS; 10: 128 uS; 11: 1024uS."]
    #[inline(always)]
    pub fn eee_timer_unit_giga_1(&self) -> EeeTimerUnitGiga1R {
        EeeTimerUnitGiga1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable TX meter to count TX LPI status time"]
    #[inline(always)]
    pub fn eee_txtimer_en(&self) -> EeeTxtimerEnR {
        EeeTxtimerEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable RX meter to count RX LPI status time"]
    #[inline(always)]
    pub fn eee_rxtimer_en(&self) -> EeeRxtimerEnR {
        EeeRxtimerEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable LPI TX request Rate decision maker for EEE. 1: enable; 0: disable."]
    #[inline(always)]
    pub fn eee_req_set0(&self) -> EeeReqSet0R {
        EeeReqSet0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable LPI TX request queue empty decision maker for EEE. 1: enable; 0: disable. For RLE0390(formerly RL6166), only for tx is single descriptor operation. For next version, this bit can support multi-descriptor operation."]
    #[inline(always)]
    pub fn eee_req_set1(&self) -> EeeReqSet1R {
        EeeReqSet1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable LPI TX request RX pause on frame decision maker for EEE. 1: enable; 0: disable."]
    #[inline(always)]
    pub fn eee_req_set2(&self) -> EeeReqSet2R {
        EeeReqSet2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable LPI TX transmit wakeup decision maker for EEE. 1: enable; 0: disable."]
    #[inline(always)]
    pub fn eee_wake_set0(&self) -> EeeWakeSet0R {
        EeeWakeSet0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable LPI TX high priority pkt Wakeup decision maker for EEE. 1: enable; 0: disable. Gmac will leave lpi mode when EEE_CR1_reg.txq_th is met or one of the egress is larger than 512B."]
    #[inline(always)]
    pub fn eee_wake_set1(&self) -> EeeWakeSet1R {
        EeeWakeSet1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Set indication for LPI request due to RX pause on frame. 1: indicate RX pause on frame; 0: no indicate."]
    #[inline(always)]
    pub fn eee_pause_flag(&self) -> EeePauseFlagR {
        EeePauseFlagR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicate Port 0 EEE RX status 1: RX is in LPI mode. 0: RX is in normal mode."]
    #[inline(always)]
    pub fn eee_rx_sts(&self) -> EeeRxStsR {
        EeeRxStsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicate EEE TX status 1: TX is in LPI mode. 0: TX is in normal mode."]
    #[inline(always)]
    pub fn eee_tx_sts(&self) -> EeeTxStsR {
        EeeTxStsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicate EEE status, both TX and RX. 1: Both TX and RX in LPI mode simultaneously. 0: not both TX and RX in LPI mode simultaneously"]
    #[inline(always)]
    pub fn eee_sts(&self) -> EeeStsR {
        EeeStsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - SW has many packets to be tranmitted, then set this bit to tell HW leave lpi state in condition2."]
    #[inline(always)]
    pub fn eee_outq_lowq_over(&self) -> EeeOutqLowqOverR {
        EeeOutqLowqOverR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable MAC RX EEE ability 0: Disable EEE ability for RX. 1: Enable EEE ability for RX. Application note : The following methods could be used to disable EEE function. a) n\\[1:0\\] = 00 -> completely disable EEE function, or b) n\\[3:2\\] = 00 -> disable /1000Mbps EEE capability (excludes 10Mbps )."]
    #[inline(always)]
    pub fn en_eee_rx(&self) -> EnEeeRxR {
        EnEeeRxR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable MAC TX EEE ability 0: Disable EEE ability for TX. 1: Enable EEE ability for TX."]
    #[inline(always)]
    pub fn en_eee_tx(&self) -> EnEeeTxR {
        EnEeeTxR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set same as PHY setting, for LED."]
    #[inline(always)]
    pub fn en_eee(&self) -> EnEeeR {
        EnEeeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - If EN_FRC_EEE=1, FRC_EEE_100 0: Disable MAC EEE ability for 100BASE-TX. 1: Enable MAC EEE ability for 100BASE-TX. If EN_FRC_EEE=0, FRC_EEE_100 means EEE advertisement register 7.60 bit\\[1\\]"]
    #[inline(always)]
    pub fn frc_eee(&self) -> FrcEeeR {
        FrcEeeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If EN_FRC_EEE=1, FRC_EEE_Giga: 0: Disable MAC EEE ability for 1000BASE-T. 1: Enable MAC EEE ability for 1000BASE-T. If EN_FRC_EEE=0, FRC_EEE_Giga means EEE advertisement register 7.60 bit\\[2\\]"]
    #[inline(always)]
    pub fn frc_eee_giga(&self) -> FrcEeeGigaR {
        FrcEeeGigaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable EEE force mode 0: Disable EEE FORCE Mode 1: Enable EEE FORCE Mode If EN_FRC_EEE=1, both FRC_EEE_Giga and FRC_EEE_100 are used to force MAC operation ability."]
    #[inline(always)]
    pub fn en_frc_eee(&self) -> EnFrcEeeR {
        EnFrcEeeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set the unit for timer T_tx_delay in EEE function. 00: 8uS; 01: 64uS; 10: 512uS; 11: 1024uS."]
    #[inline(always)]
    pub fn eee_timer_unit_100_2(&mut self) -> EeeTimerUnit100_2W<'_, EeeCr1Spec> {
        EeeTimerUnit100_2W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Set the unit for timer T_wakeup, T_tx_rate, and T_pause in EEE function. 00: 1uS; 01: 16uS; 10: 128uS; 11: 1024uS."]
    #[inline(always)]
    pub fn eee_timer_unit_100_1(&mut self) -> EeeTimerUnit100_1W<'_, EeeCr1Spec> {
        EeeTimerUnit100_1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Set the unit for timer T_tx_delay in EEE function. 00: 8uS; 01: 64uS; 10: 256uS; 11: 1024uS."]
    #[inline(always)]
    pub fn eee_timer_unit_giga_2(&mut self) -> EeeTimerUnitGiga2W<'_, EeeCr1Spec> {
        EeeTimerUnitGiga2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Set the unit for timer T_wakeup, T_tx_rate, and T_pause in EEE function. 00: 1uS; 01: 16 uS; 10: 128 uS; 11: 1024uS."]
    #[inline(always)]
    pub fn eee_timer_unit_giga_1(&mut self) -> EeeTimerUnitGiga1W<'_, EeeCr1Spec> {
        EeeTimerUnitGiga1W::new(self, 6)
    }
    #[doc = "Bit 8 - Enable TX meter to count TX LPI status time"]
    #[inline(always)]
    pub fn eee_txtimer_en(&mut self) -> EeeTxtimerEnW<'_, EeeCr1Spec> {
        EeeTxtimerEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable RX meter to count RX LPI status time"]
    #[inline(always)]
    pub fn eee_rxtimer_en(&mut self) -> EeeRxtimerEnW<'_, EeeCr1Spec> {
        EeeRxtimerEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable LPI TX request Rate decision maker for EEE. 1: enable; 0: disable."]
    #[inline(always)]
    pub fn eee_req_set0(&mut self) -> EeeReqSet0W<'_, EeeCr1Spec> {
        EeeReqSet0W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable LPI TX request queue empty decision maker for EEE. 1: enable; 0: disable. For RLE0390(formerly RL6166), only for tx is single descriptor operation. For next version, this bit can support multi-descriptor operation."]
    #[inline(always)]
    pub fn eee_req_set1(&mut self) -> EeeReqSet1W<'_, EeeCr1Spec> {
        EeeReqSet1W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable LPI TX request RX pause on frame decision maker for EEE. 1: enable; 0: disable."]
    #[inline(always)]
    pub fn eee_req_set2(&mut self) -> EeeReqSet2W<'_, EeeCr1Spec> {
        EeeReqSet2W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable LPI TX transmit wakeup decision maker for EEE. 1: enable; 0: disable."]
    #[inline(always)]
    pub fn eee_wake_set0(&mut self) -> EeeWakeSet0W<'_, EeeCr1Spec> {
        EeeWakeSet0W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable LPI TX high priority pkt Wakeup decision maker for EEE. 1: enable; 0: disable. Gmac will leave lpi mode when EEE_CR1_reg.txq_th is met or one of the egress is larger than 512B."]
    #[inline(always)]
    pub fn eee_wake_set1(&mut self) -> EeeWakeSet1W<'_, EeeCr1Spec> {
        EeeWakeSet1W::new(self, 14)
    }
    #[doc = "Bit 23 - SW has many packets to be tranmitted, then set this bit to tell HW leave lpi state in condition2."]
    #[inline(always)]
    pub fn eee_outq_lowq_over(&mut self) -> EeeOutqLowqOverW<'_, EeeCr1Spec> {
        EeeOutqLowqOverW::new(self, 23)
    }
    #[doc = "Bit 26 - Enable MAC RX EEE ability 0: Disable EEE ability for RX. 1: Enable EEE ability for RX. Application note : The following methods could be used to disable EEE function. a) n\\[1:0\\] = 00 -> completely disable EEE function, or b) n\\[3:2\\] = 00 -> disable /1000Mbps EEE capability (excludes 10Mbps )."]
    #[inline(always)]
    pub fn en_eee_rx(&mut self) -> EnEeeRxW<'_, EeeCr1Spec> {
        EnEeeRxW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable MAC TX EEE ability 0: Disable EEE ability for TX. 1: Enable EEE ability for TX."]
    #[inline(always)]
    pub fn en_eee_tx(&mut self) -> EnEeeTxW<'_, EeeCr1Spec> {
        EnEeeTxW::new(self, 27)
    }
    #[doc = "Bit 28 - Set same as PHY setting, for LED."]
    #[inline(always)]
    pub fn en_eee(&mut self) -> EnEeeW<'_, EeeCr1Spec> {
        EnEeeW::new(self, 28)
    }
    #[doc = "Bit 29 - If EN_FRC_EEE=1, FRC_EEE_100 0: Disable MAC EEE ability for 100BASE-TX. 1: Enable MAC EEE ability for 100BASE-TX. If EN_FRC_EEE=0, FRC_EEE_100 means EEE advertisement register 7.60 bit\\[1\\]"]
    #[inline(always)]
    pub fn frc_eee(&mut self) -> FrcEeeW<'_, EeeCr1Spec> {
        FrcEeeW::new(self, 29)
    }
    #[doc = "Bit 30 - If EN_FRC_EEE=1, FRC_EEE_Giga: 0: Disable MAC EEE ability for 1000BASE-T. 1: Enable MAC EEE ability for 1000BASE-T. If EN_FRC_EEE=0, FRC_EEE_Giga means EEE advertisement register 7.60 bit\\[2\\]"]
    #[inline(always)]
    pub fn frc_eee_giga(&mut self) -> FrcEeeGigaW<'_, EeeCr1Spec> {
        FrcEeeGigaW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable EEE force mode 0: Disable EEE FORCE Mode 1: Enable EEE FORCE Mode If EN_FRC_EEE=1, both FRC_EEE_Giga and FRC_EEE_100 are used to force MAC operation ability."]
    #[inline(always)]
    pub fn en_frc_eee(&mut self) -> EnFrcEeeW<'_, EeeCr1Spec> {
        EnFrcEeeW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eee_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eee_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeeCr1Spec;
impl crate::RegisterSpec for EeeCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eee_cr1::R`](R) reader structure"]
impl crate::Readable for EeeCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`eee_cr1::W`](W) writer structure"]
impl crate::Writable for EeeCr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEE_CR1 to value 0"]
impl crate::Resettable for EeeCr1Spec {}
