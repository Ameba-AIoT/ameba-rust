#[doc = "Register `REG_BT_Interrupt_Status_Register` reader"]
pub type R = crate::R<RegBtInterruptStatusRegisterSpec>;
#[doc = "Register `REG_BT_Interrupt_Status_Register` writer"]
pub type W = crate::W<RegBtInterruptStatusRegisterSpec>;
#[doc = "Field `BIT_ISOLATION_CHK_2` reader - 0: leave PTA 1: enter PTA"]
pub type BitIsolationChk2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_ISOLATION_CHK_2` writer - 0: leave PTA 1: enter PTA"]
pub type BitIsolationChk2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_BT_PROFILE_ISR` reader - When WLAN received the packet including that WLAN specific profile and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware."]
pub type BitBtProfileIsrR = crate::BitReader;
#[doc = "Field `BIT_BT_SLOT_CHANGE_ISR` reader - When WLAN received the packet including that BT had multi-slots, hardware can trigger an interrupt to notify firmware"]
pub type BitBtSlotChangeIsrR = crate::BitReader;
#[doc = "Field `BIT_BT_CHANNEL_ISR` reader - When WLAN received the packet including BT channel number, hardware can trigger an interrupt to notify firmware."]
pub type BitBtChannelIsrR = crate::BitReader;
#[doc = "Field `BIT_BT_POWER_ISR` reader - When WLAN received the packet including BT Power and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware"]
pub type BitBtPowerIsrR = crate::BitReader;
#[doc = "Field `BIT_WLAN_RPT_ISR` reader - When PTA sent the report packet successfully, hardware triggers an interrupt to notify firmware."]
pub type BitWlanRptIsrR = crate::BitReader;
#[doc = "Field `BIT_MAC_NULL_PKT_NOTIFY_ISR` reader - When MAC had sent NULL packet for BT and firmware also set nav_for_bt bit, hardware triggers an interrupt to notify firmware"]
pub type BitMacNullPktNotifyIsrR = crate::BitReader;
#[doc = "Field `BIT_BT_QUERY_ISR` reader - When WLAN received a query packet, hardware triggers an interrupt to notify firmware"]
pub type BitBtQueryIsrR = crate::BitReader;
#[doc = "Field `BIT_BT_HID_ISR` reader - When WLAN received the report packet including HID information, hardware will trigger an interrupt to notify firmware"]
pub type BitBtHidIsrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - 0: leave PTA 1: enter PTA"]
    #[inline(always)]
    pub fn bit_isolation_chk_2(&self) -> BitIsolationChk2R {
        BitIsolationChk2R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - When WLAN received the packet including that WLAN specific profile and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware."]
    #[inline(always)]
    pub fn bit_bt_profile_isr(&self) -> BitBtProfileIsrR {
        BitBtProfileIsrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When WLAN received the packet including that BT had multi-slots, hardware can trigger an interrupt to notify firmware"]
    #[inline(always)]
    pub fn bit_bt_slot_change_isr(&self) -> BitBtSlotChangeIsrR {
        BitBtSlotChangeIsrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When WLAN received the packet including BT channel number, hardware can trigger an interrupt to notify firmware."]
    #[inline(always)]
    pub fn bit_bt_channel_isr(&self) -> BitBtChannelIsrR {
        BitBtChannelIsrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When WLAN received the packet including BT Power and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware"]
    #[inline(always)]
    pub fn bit_bt_power_isr(&self) -> BitBtPowerIsrR {
        BitBtPowerIsrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When PTA sent the report packet successfully, hardware triggers an interrupt to notify firmware."]
    #[inline(always)]
    pub fn bit_wlan_rpt_isr(&self) -> BitWlanRptIsrR {
        BitWlanRptIsrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When MAC had sent NULL packet for BT and firmware also set nav_for_bt bit, hardware triggers an interrupt to notify firmware"]
    #[inline(always)]
    pub fn bit_mac_null_pkt_notify_isr(&self) -> BitMacNullPktNotifyIsrR {
        BitMacNullPktNotifyIsrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When WLAN received a query packet, hardware triggers an interrupt to notify firmware"]
    #[inline(always)]
    pub fn bit_bt_query_isr(&self) -> BitBtQueryIsrR {
        BitBtQueryIsrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When WLAN received the report packet including HID information, hardware will trigger an interrupt to notify firmware"]
    #[inline(always)]
    pub fn bit_bt_hid_isr(&self) -> BitBtHidIsrR {
        BitBtHidIsrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - 0: leave PTA 1: enter PTA"]
    #[inline(always)]
    pub fn bit_isolation_chk_2(
        &mut self,
    ) -> BitIsolationChk2W<'_, RegBtInterruptStatusRegisterSpec> {
        BitIsolationChk2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_interrupt_status_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_interrupt_status_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtInterruptStatusRegisterSpec;
impl crate::RegisterSpec for RegBtInterruptStatusRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_interrupt_status_register::R`](R) reader structure"]
impl crate::Readable for RegBtInterruptStatusRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_interrupt_status_register::W`](W) writer structure"]
impl crate::Writable for RegBtInterruptStatusRegisterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_Interrupt_Status_Register to value 0"]
impl crate::Resettable for RegBtInterruptStatusRegisterSpec {}
