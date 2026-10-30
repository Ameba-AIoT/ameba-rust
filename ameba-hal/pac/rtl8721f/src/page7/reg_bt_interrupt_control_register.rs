#[doc = "Register `REG_BT_Interrupt_Control_Register` reader"]
pub type R = crate::R<RegBtInterruptControlRegisterSpec>;
#[doc = "Register `REG_BT_Interrupt_Control_Register` writer"]
pub type W = crate::W<RegBtInterruptControlRegisterSpec>;
#[doc = "Field `BIT_BT_DATA` reader - When WLAN received the packet BT sent, hardware will place data to this register in addition to CMD_ID 0x0 ~ 0x3."]
pub type BitBtDataR = crate::FieldReader;
#[doc = "Field `BIT_TO_BT_CMDID` reader - Command id: 0x80~FF"]
pub type BitToBtCmdidR = crate::FieldReader;
#[doc = "Field `BIT_TO_BT_CMDID` writer - Command id: 0x80~FF"]
pub type BitToBtCmdidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_WLAN_RPT_DATA` reader - Data contents, When the report packet had sent, hardware will clear this register."]
pub type BitWlanRptDataR = crate::FieldReader;
#[doc = "Field `BIT_WLAN_RPT_DATA` writer - Data contents, When the report packet had sent, hardware will clear this register."]
pub type BitWlanRptDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EN_BT_PROFILE_OR_HID` reader - Enable two interrupt 1) When WLAN received the packet including that WLAN specific profile and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware. 2) When WLAN received the report packet including HID information, hardware will trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnBtProfileOrHidR = crate::BitReader;
#[doc = "Field `BIT_EN_BT_PROFILE_OR_HID` writer - Enable two interrupt 1) When WLAN received the packet including that WLAN specific profile and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware. 2) When WLAN received the report packet including HID information, hardware will trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnBtProfileOrHidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BT_SLOT_CHANGE` reader - When WLAN received the packet including that BT had multi-slots, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnBtSlotChangeR = crate::BitReader;
#[doc = "Field `BIT_EN_BT_SLOT_CHANGE` writer - When WLAN received the packet including that BT had multi-slots, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnBtSlotChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BT_CHANNEL` reader - When WLAN received the packet including BT channel number, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnBtChannelR = crate::BitReader;
#[doc = "Field `BIT_EN_BT_CHANNEL` writer - When WLAN received the packet including BT channel number, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnBtChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BT_POWER` reader - When WLAN received the packet including BT Power and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnBtPowerR = crate::BitReader;
#[doc = "Field `BIT_EN_BT_POWER` writer - When WLAN received the packet including BT Power and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnBtPowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BT_STSTUS_RPT` reader - When WLAN receive cmd_id \"0\", hardware trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnBtStstusRptR = crate::BitReader;
#[doc = "Field `BIT_EN_BT_STSTUS_RPT` writer - When WLAN receive cmd_id \"0\", hardware trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnBtStstusRptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_MAC_NULL_PKT_NOTIFY` reader - When MAC had sent NULL packet for BT and firmware also set nav_for_bt bit, hardware triggers an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnMacNullPktNotifyR = crate::BitReader;
#[doc = "Field `BIT_EN_MAC_NULL_PKT_NOTIFY` writer - When MAC had sent NULL packet for BT and firmware also set nav_for_bt bit, hardware triggers an interrupt to notify firmware. 0: Disable 1: Enable"]
pub type BitEnMacNullPktNotifyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_WLAN_RPT_AND_BT_QUERY` reader - Enable two interrupt: 1) PTA sent the report packet successfully 2) PTA sent a query packet 0: Disable 1: Enable"]
pub type BitEnWlanRptAndBtQueryR = crate::BitReader;
#[doc = "Field `BIT_EN_WLAN_RPT_AND_BT_QUERY` writer - Enable two interrupt: 1) PTA sent the report packet successfully 2) PTA sent a query packet 0: Disable 1: Enable"]
pub type BitEnWlanRptAndBtQueryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLAN_RPT_NOTIFY` reader - When firmware had prepared packet to send to BT, it need to set this bit to notify hardware that it can transmit. 0: There isn't a valid packet in register\" WLAN_RPT_ PKT\". 1: There is a valid packet in register\" WLAN_RPT_ PKT\"."]
pub type BitWlanRptNotifyR = crate::BitReader;
#[doc = "Field `BIT_WLAN_RPT_NOTIFY` writer - When firmware had prepared packet to send to BT, it need to set this bit to notify hardware that it can transmit. 0: There isn't a valid packet in register\" WLAN_RPT_ PKT\". 1: There is a valid packet in register\" WLAN_RPT_ PKT\"."]
pub type BitWlanRptNotifyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - When WLAN received the packet BT sent, hardware will place data to this register in addition to CMD_ID 0x0 ~ 0x3."]
    #[inline(always)]
    pub fn bit_bt_data(&self) -> BitBtDataR {
        BitBtDataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command id: 0x80~FF"]
    #[inline(always)]
    pub fn bit_to_bt_cmdid(&self) -> BitToBtCmdidR {
        BitToBtCmdidR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data contents, When the report packet had sent, hardware will clear this register."]
    #[inline(always)]
    pub fn bit_wlan_rpt_data(&self) -> BitWlanRptDataR {
        BitWlanRptDataR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable two interrupt 1) When WLAN received the packet including that WLAN specific profile and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware. 2) When WLAN received the report packet including HID information, hardware will trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_bt_profile_or_hid(&self) -> BitEnBtProfileOrHidR {
        BitEnBtProfileOrHidR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When WLAN received the packet including that BT had multi-slots, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_bt_slot_change(&self) -> BitEnBtSlotChangeR {
        BitEnBtSlotChangeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When WLAN received the packet including BT channel number, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_bt_channel(&self) -> BitEnBtChannelR {
        BitEnBtChannelR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When WLAN received the packet including BT Power and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_bt_power(&self) -> BitEnBtPowerR {
        BitEnBtPowerR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When WLAN receive cmd_id \"0\", hardware trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_bt_ststus_rpt(&self) -> BitEnBtStstusRptR {
        BitEnBtStstusRptR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When MAC had sent NULL packet for BT and firmware also set nav_for_bt bit, hardware triggers an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_mac_null_pkt_notify(&self) -> BitEnMacNullPktNotifyR {
        BitEnMacNullPktNotifyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable two interrupt: 1) PTA sent the report packet successfully 2) PTA sent a query packet 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_wlan_rpt_and_bt_query(&self) -> BitEnWlanRptAndBtQueryR {
        BitEnWlanRptAndBtQueryR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When firmware had prepared packet to send to BT, it need to set this bit to notify hardware that it can transmit. 0: There isn't a valid packet in register\" WLAN_RPT_ PKT\". 1: There is a valid packet in register\" WLAN_RPT_ PKT\"."]
    #[inline(always)]
    pub fn bit_wlan_rpt_notify(&self) -> BitWlanRptNotifyR {
        BitWlanRptNotifyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - Command id: 0x80~FF"]
    #[inline(always)]
    pub fn bit_to_bt_cmdid(&mut self) -> BitToBtCmdidW<'_, RegBtInterruptControlRegisterSpec> {
        BitToBtCmdidW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data contents, When the report packet had sent, hardware will clear this register."]
    #[inline(always)]
    pub fn bit_wlan_rpt_data(&mut self) -> BitWlanRptDataW<'_, RegBtInterruptControlRegisterSpec> {
        BitWlanRptDataW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable two interrupt 1) When WLAN received the packet including that WLAN specific profile and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware. 2) When WLAN received the report packet including HID information, hardware will trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_bt_profile_or_hid(
        &mut self,
    ) -> BitEnBtProfileOrHidW<'_, RegBtInterruptControlRegisterSpec> {
        BitEnBtProfileOrHidW::new(self, 24)
    }
    #[doc = "Bit 25 - When WLAN received the packet including that BT had multi-slots, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_bt_slot_change(
        &mut self,
    ) -> BitEnBtSlotChangeW<'_, RegBtInterruptControlRegisterSpec> {
        BitEnBtSlotChangeW::new(self, 25)
    }
    #[doc = "Bit 26 - When WLAN received the packet including BT channel number, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_bt_channel(&mut self) -> BitEnBtChannelW<'_, RegBtInterruptControlRegisterSpec> {
        BitEnBtChannelW::new(self, 26)
    }
    #[doc = "Bit 27 - When WLAN received the packet including BT Power and check register \"BT_STATUS_RPT\", if data contents aren't the same, hardware can trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_bt_power(&mut self) -> BitEnBtPowerW<'_, RegBtInterruptControlRegisterSpec> {
        BitEnBtPowerW::new(self, 27)
    }
    #[doc = "Bit 28 - When WLAN receive cmd_id \"0\", hardware trigger an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_bt_ststus_rpt(
        &mut self,
    ) -> BitEnBtStstusRptW<'_, RegBtInterruptControlRegisterSpec> {
        BitEnBtStstusRptW::new(self, 28)
    }
    #[doc = "Bit 29 - When MAC had sent NULL packet for BT and firmware also set nav_for_bt bit, hardware triggers an interrupt to notify firmware. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_mac_null_pkt_notify(
        &mut self,
    ) -> BitEnMacNullPktNotifyW<'_, RegBtInterruptControlRegisterSpec> {
        BitEnMacNullPktNotifyW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable two interrupt: 1) PTA sent the report packet successfully 2) PTA sent a query packet 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn bit_en_wlan_rpt_and_bt_query(
        &mut self,
    ) -> BitEnWlanRptAndBtQueryW<'_, RegBtInterruptControlRegisterSpec> {
        BitEnWlanRptAndBtQueryW::new(self, 30)
    }
    #[doc = "Bit 31 - When firmware had prepared packet to send to BT, it need to set this bit to notify hardware that it can transmit. 0: There isn't a valid packet in register\" WLAN_RPT_ PKT\". 1: There is a valid packet in register\" WLAN_RPT_ PKT\"."]
    #[inline(always)]
    pub fn bit_wlan_rpt_notify(
        &mut self,
    ) -> BitWlanRptNotifyW<'_, RegBtInterruptControlRegisterSpec> {
        BitWlanRptNotifyW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_interrupt_control_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_interrupt_control_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtInterruptControlRegisterSpec;
impl crate::RegisterSpec for RegBtInterruptControlRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_interrupt_control_register::R`](R) reader structure"]
impl crate::Readable for RegBtInterruptControlRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_interrupt_control_register::W`](W) writer structure"]
impl crate::Writable for RegBtInterruptControlRegisterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_Interrupt_Control_Register to value 0"]
impl crate::Resettable for RegBtInterruptControlRegisterSpec {}
