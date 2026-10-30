#[doc = "Register `REG_WOW_NAN_CTRL` reader"]
pub type R = crate::R<RegWowNanCtrlSpec>;
#[doc = "Register `REG_WOW_NAN_CTRL` writer"]
pub type W = crate::W<RegWowNanCtrlSpec>;
#[doc = "Field `BIT_WFMSK` reader - WoW function on or off."]
pub type BitWfmskR = crate::BitReader;
#[doc = "Field `BIT_WFMSK` writer - WoW function on or off."]
pub type BitWfmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAGIC` reader - Magic Packet. This bit is valid when the PMEn bit of the PMR register is set. The RTL8192 will assert the PMEB signal to wakeup the operating system when a Magic Packet is received. Once the RTL8192 has been enabled for Magic Packet wakeup, it scans incoming packets addressed to the node for a data sequence. The correct sequence indicates to the controller that this is a Magic Packet frame. A Magic Packet frame must also meet the basic requirements: Frame Control + Duration/ID + Destination address + Address 2 + Address 3 + Sequence Control + data + CRC. The destination address may be the node ID of the receiving station or a multicast address, including the broadcast address. The specific sequence consists of 16 duplications of 6 byte ID registers, with no breaks or interrupts. This sequence can be located anywhere within the packet, but must be preceded by a synchronization stream, 6 bytes of FFh. The device will also accept a multicast address, as long as the 16 duplications of the IEEE address match the address of the ID registers. If the Node ID is 11h 22h 33h 44h 55h 66h, then the magic frame\"s format is as follows: Frame Control + Duration/ID + Destination address + Address 2 + Address 3 + Sequence Control + MISC + FF FF FF FF FF FF + MISC + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + MISC + CRC."]
pub type BitMagicR = crate::BitReader;
#[doc = "Field `BIT_MAGIC` writer - Magic Packet. This bit is valid when the PMEn bit of the PMR register is set. The RTL8192 will assert the PMEB signal to wakeup the operating system when a Magic Packet is received. Once the RTL8192 has been enabled for Magic Packet wakeup, it scans incoming packets addressed to the node for a data sequence. The correct sequence indicates to the controller that this is a Magic Packet frame. A Magic Packet frame must also meet the basic requirements: Frame Control + Duration/ID + Destination address + Address 2 + Address 3 + Sequence Control + data + CRC. The destination address may be the node ID of the receiving station or a multicast address, including the broadcast address. The specific sequence consists of 16 duplications of 6 byte ID registers, with no breaks or interrupts. This sequence can be located anywhere within the packet, but must be preceded by a synchronization stream, 6 bytes of FFh. The device will also accept a multicast address, as long as the 16 duplications of the IEEE address match the address of the ID registers. If the Node ID is 11h 22h 33h 44h 55h 66h, then the magic frame\"s format is as follows: Frame Control + Duration/ID + Destination address + Address 2 + Address 3 + Sequence Control + MISC + FF FF FF FF FF FF + MISC + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + MISC + CRC."]
pub type BitMagicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_UWF` reader - Unicast Packet. 1: Enable to check packet\"s DA match MAC address or not. 0: Disable. (default value)"]
pub type BitUwfR = crate::BitReader;
#[doc = "Field `BIT_UWF` writer - Unicast Packet. 1: Enable to check packet\"s DA match MAC address or not. 0: Disable. (default value)"]
pub type BitUwfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PSF_BSSIDSEL_L` reader - Bit 0 of Select to check the MACID of which port for the WMAC PSF NSARP function. 0: to check the MACID of port0; 1: to check the MACID of port1. 2: to check the MACID of port2. 3: to check the MACID of port3. 4: to check the MACID of port4."]
pub type BitPsfBssidselLR = crate::BitReader;
#[doc = "Field `BIT_PSF_BSSIDSEL_L` writer - Bit 0 of Select to check the MACID of which port for the WMAC PSF NSARP function. 0: to check the MACID of port0; 1: to check the MACID of port1. 2: to check the MACID of port2. 3: to check the MACID of port3. 4: to check the MACID of port4."]
pub type BitPsfBssidselLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WOWHCI` reader - This bit is set to allow wake host as wake on wlan event occurring"]
pub type BitWowhciR = crate::BitReader;
#[doc = "Field `BIT_WOWHCI` writer - This bit is set to allow wake host as wake on wlan event occurring"]
pub type BitWowhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PSF_BSSIDSEL_H` reader - Bit 1,2 of Select to check the MACID of which port for the WMAC PSF NSARP function. 0: to check the MACID of port0; 1: to check the MACID of port1. 2: to check the MACID of port2. 3: to check the MACID of port3. 4: to check the MACID of port4."]
pub type BitPsfBssidselHR = crate::FieldReader;
#[doc = "Field `BIT_PSF_BSSIDSEL_H` writer - Bit 1,2 of Select to check the MACID of which port for the WMAC PSF NSARP function. 0: to check the MACID of port0; 1: to check the MACID of port1. 2: to check the MACID of port2. 3: to check the MACID of port3. 4: to check the MACID of port4."]
pub type BitPsfBssidselHW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_NAN_RX_FILTER_EN` reader - When set to 1, the TSF address filter will enable. When set to 0, the TSF address filter will disable."]
pub type BitNanRxFilterEnR = crate::BitReader;
#[doc = "Field `BIT_NAN_RX_FILTER_EN` writer - When set to 1, the TSF address filter will enable. When set to 0, the TSF address filter will disable."]
pub type BitNanRxFilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NAN_RX_CHKBSSID` reader - Check BSSID_BCN for TSF When set to 1, MAC will check the BSSID (A3) of RX beacon or Probe to determine if this packet will sync TSF or not"]
pub type BitNanRxChkbssidR = crate::BitReader;
#[doc = "Field `BIT_NAN_RX_CHKBSSID` writer - Check BSSID_BCN for TSF When set to 1, MAC will check the BSSID (A3) of RX beacon or Probe to determine if this packet will sync TSF or not"]
pub type BitNanRxChkbssidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NAN_RX_CHKTA` reader - Enable bit for check TSF filter When set to 1, the TSF address filter will enable. When set to 0, the TSF address filter will disable."]
pub type BitNanRxChktaR = crate::BitReader;
#[doc = "Field `BIT_NAN_RX_CHKTA` writer - Enable bit for check TSF filter When set to 1, the TSF address filter will enable. When set to 0, the TSF address filter will disable."]
pub type BitNanRxChktaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDATAIN0` reader - It is set to 1 if NIC received data frame with correct BSSID and DA (including unicast and BMC)."]
pub type BitRxdatain0R = crate::BitReader;
#[doc = "Field `BIT_RXDATAIN1` reader - It is set to 1 if NIC received data frame with correct BSSID and DA (including unicast and BMC)."]
pub type BitRxdatain1R = crate::BitReader;
#[doc = "Field `BIT_RXDATAIN2` reader - It is set to 1 if NIC received data frame with correct BSSID and DA (including unicast and BMC)."]
pub type BitRxdatain2R = crate::BitReader;
#[doc = "Field `BIT_RXMGTIN0` reader - It is set to 1 if NIC received management frame with correct BSSID and DA (including unicast and BMC)."]
pub type BitRxmgtin0R = crate::BitReader;
#[doc = "Field `BIT_RXCTRLIN0` reader - It is set to 1 if NIC received control frame with correct DA."]
pub type BitRxctrlin0R = crate::BitReader;
#[doc = "Field `BIT_PORTSEL_PS_RX_INFO` reader - The selector for the source of PS RX information (0x692\\[4:0\\]). 0 for port 0, 1 for clint0, 2 for clint1, 3 for clint2, 4 for clint3."]
pub type BitPortselPsRxInfoR = crate::FieldReader;
#[doc = "Field `BIT_PORTSEL_PS_RX_INFO` writer - The selector for the source of PS RX information (0x692\\[4:0\\]). 0 for port 0, 1 for clint0, 2 for clint1, 3 for clint2, 4 for clint3."]
pub type BitPortselPsRxInfoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_WMMPS_UAPSD_TID0` reader - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID0"]
pub type BitWmmpsUapsdTid0R = crate::BitReader;
#[doc = "Field `BIT_WMMPS_UAPSD_TID0` writer - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID0"]
pub type BitWmmpsUapsdTid0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMMPS_UAPSD_TID1` reader - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID1"]
pub type BitWmmpsUapsdTid1R = crate::BitReader;
#[doc = "Field `BIT_WMMPS_UAPSD_TID1` writer - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID1"]
pub type BitWmmpsUapsdTid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMMPS_UAPSD_TID2` reader - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID2"]
pub type BitWmmpsUapsdTid2R = crate::BitReader;
#[doc = "Field `BIT_WMMPS_UAPSD_TID2` writer - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID2"]
pub type BitWmmpsUapsdTid2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMMPS_UAPSD_TID3` reader - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID3"]
pub type BitWmmpsUapsdTid3R = crate::BitReader;
#[doc = "Field `BIT_WMMPS_UAPSD_TID3` writer - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID3"]
pub type BitWmmpsUapsdTid3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMMPS_UAPSD_TID4` reader - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID4"]
pub type BitWmmpsUapsdTid4R = crate::BitReader;
#[doc = "Field `BIT_WMMPS_UAPSD_TID4` writer - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID4"]
pub type BitWmmpsUapsdTid4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMMPS_UAPSD_TID5` reader - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID5"]
pub type BitWmmpsUapsdTid5R = crate::BitReader;
#[doc = "Field `BIT_WMMPS_UAPSD_TID5` writer - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID5"]
pub type BitWmmpsUapsdTid5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMMPS_UAPSD_TID6` reader - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID6"]
pub type BitWmmpsUapsdTid6R = crate::BitReader;
#[doc = "Field `BIT_WMMPS_UAPSD_TID6` writer - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID6"]
pub type BitWmmpsUapsdTid6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMMPS_UAPSD_TID7` reader - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID7"]
pub type BitWmmpsUapsdTid7R = crate::BitReader;
#[doc = "Field `BIT_WMMPS_UAPSD_TID7` writer - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID7"]
pub type BitWmmpsUapsdTid7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - WoW function on or off."]
    #[inline(always)]
    pub fn bit_wfmsk(&self) -> BitWfmskR {
        BitWfmskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Magic Packet. This bit is valid when the PMEn bit of the PMR register is set. The RTL8192 will assert the PMEB signal to wakeup the operating system when a Magic Packet is received. Once the RTL8192 has been enabled for Magic Packet wakeup, it scans incoming packets addressed to the node for a data sequence. The correct sequence indicates to the controller that this is a Magic Packet frame. A Magic Packet frame must also meet the basic requirements: Frame Control + Duration/ID + Destination address + Address 2 + Address 3 + Sequence Control + data + CRC. The destination address may be the node ID of the receiving station or a multicast address, including the broadcast address. The specific sequence consists of 16 duplications of 6 byte ID registers, with no breaks or interrupts. This sequence can be located anywhere within the packet, but must be preceded by a synchronization stream, 6 bytes of FFh. The device will also accept a multicast address, as long as the 16 duplications of the IEEE address match the address of the ID registers. If the Node ID is 11h 22h 33h 44h 55h 66h, then the magic frame\"s format is as follows: Frame Control + Duration/ID + Destination address + Address 2 + Address 3 + Sequence Control + MISC + FF FF FF FF FF FF + MISC + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + MISC + CRC."]
    #[inline(always)]
    pub fn bit_magic(&self) -> BitMagicR {
        BitMagicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Packet. 1: Enable to check packet\"s DA match MAC address or not. 0: Disable. (default value)"]
    #[inline(always)]
    pub fn bit_uwf(&self) -> BitUwfR {
        BitUwfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit 0 of Select to check the MACID of which port for the WMAC PSF NSARP function. 0: to check the MACID of port0; 1: to check the MACID of port1. 2: to check the MACID of port2. 3: to check the MACID of port3. 4: to check the MACID of port4."]
    #[inline(always)]
    pub fn bit_psf_bssidsel_l(&self) -> BitPsfBssidselLR {
        BitPsfBssidselLR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set to allow wake host as wake on wlan event occurring"]
    #[inline(always)]
    pub fn bit_wowhci(&self) -> BitWowhciR {
        BitWowhciR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Bit 1,2 of Select to check the MACID of which port for the WMAC PSF NSARP function. 0: to check the MACID of port0; 1: to check the MACID of port1. 2: to check the MACID of port2. 3: to check the MACID of port3. 4: to check the MACID of port4."]
    #[inline(always)]
    pub fn bit_psf_bssidsel_h(&self) -> BitPsfBssidselHR {
        BitPsfBssidselHR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - When set to 1, the TSF address filter will enable. When set to 0, the TSF address filter will disable."]
    #[inline(always)]
    pub fn bit_nan_rx_filter_en(&self) -> BitNanRxFilterEnR {
        BitNanRxFilterEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Check BSSID_BCN for TSF When set to 1, MAC will check the BSSID (A3) of RX beacon or Probe to determine if this packet will sync TSF or not"]
    #[inline(always)]
    pub fn bit_nan_rx_chkbssid(&self) -> BitNanRxChkbssidR {
        BitNanRxChkbssidR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable bit for check TSF filter When set to 1, the TSF address filter will enable. When set to 0, the TSF address filter will disable."]
    #[inline(always)]
    pub fn bit_nan_rx_chkta(&self) -> BitNanRxChktaR {
        BitNanRxChktaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - It is set to 1 if NIC received data frame with correct BSSID and DA (including unicast and BMC)."]
    #[inline(always)]
    pub fn bit_rxdatain0(&self) -> BitRxdatain0R {
        BitRxdatain0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - It is set to 1 if NIC received data frame with correct BSSID and DA (including unicast and BMC)."]
    #[inline(always)]
    pub fn bit_rxdatain1(&self) -> BitRxdatain1R {
        BitRxdatain1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - It is set to 1 if NIC received data frame with correct BSSID and DA (including unicast and BMC)."]
    #[inline(always)]
    pub fn bit_rxdatain2(&self) -> BitRxdatain2R {
        BitRxdatain2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - It is set to 1 if NIC received management frame with correct BSSID and DA (including unicast and BMC)."]
    #[inline(always)]
    pub fn bit_rxmgtin0(&self) -> BitRxmgtin0R {
        BitRxmgtin0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - It is set to 1 if NIC received control frame with correct DA."]
    #[inline(always)]
    pub fn bit_rxctrlin0(&self) -> BitRxctrlin0R {
        BitRxctrlin0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - The selector for the source of PS RX information (0x692\\[4:0\\]). 0 for port 0, 1 for clint0, 2 for clint1, 3 for clint2, 4 for clint3."]
    #[inline(always)]
    pub fn bit_portsel_ps_rx_info(&self) -> BitPortselPsRxInfoR {
        BitPortselPsRxInfoR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID0"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid0(&self) -> BitWmmpsUapsdTid0R {
        BitWmmpsUapsdTid0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID1"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid1(&self) -> BitWmmpsUapsdTid1R {
        BitWmmpsUapsdTid1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID2"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid2(&self) -> BitWmmpsUapsdTid2R {
        BitWmmpsUapsdTid2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID3"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid3(&self) -> BitWmmpsUapsdTid3R {
        BitWmmpsUapsdTid3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID4"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid4(&self) -> BitWmmpsUapsdTid4R {
        BitWmmpsUapsdTid4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID5"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid5(&self) -> BitWmmpsUapsdTid5R {
        BitWmmpsUapsdTid5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID6"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid6(&self) -> BitWmmpsUapsdTid6R {
        BitWmmpsUapsdTid6R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID7"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid7(&self) -> BitWmmpsUapsdTid7R {
        BitWmmpsUapsdTid7R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - WoW function on or off."]
    #[inline(always)]
    pub fn bit_wfmsk(&mut self) -> BitWfmskW<'_, RegWowNanCtrlSpec> {
        BitWfmskW::new(self, 1)
    }
    #[doc = "Bit 2 - Magic Packet. This bit is valid when the PMEn bit of the PMR register is set. The RTL8192 will assert the PMEB signal to wakeup the operating system when a Magic Packet is received. Once the RTL8192 has been enabled for Magic Packet wakeup, it scans incoming packets addressed to the node for a data sequence. The correct sequence indicates to the controller that this is a Magic Packet frame. A Magic Packet frame must also meet the basic requirements: Frame Control + Duration/ID + Destination address + Address 2 + Address 3 + Sequence Control + data + CRC. The destination address may be the node ID of the receiving station or a multicast address, including the broadcast address. The specific sequence consists of 16 duplications of 6 byte ID registers, with no breaks or interrupts. This sequence can be located anywhere within the packet, but must be preceded by a synchronization stream, 6 bytes of FFh. The device will also accept a multicast address, as long as the 16 duplications of the IEEE address match the address of the ID registers. If the Node ID is 11h 22h 33h 44h 55h 66h, then the magic frame\"s format is as follows: Frame Control + Duration/ID + Destination address + Address 2 + Address 3 + Sequence Control + MISC + FF FF FF FF FF FF + MISC + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + 11 22 33 44 55 66 + MISC + CRC."]
    #[inline(always)]
    pub fn bit_magic(&mut self) -> BitMagicW<'_, RegWowNanCtrlSpec> {
        BitMagicW::new(self, 2)
    }
    #[doc = "Bit 3 - Unicast Packet. 1: Enable to check packet\"s DA match MAC address or not. 0: Disable. (default value)"]
    #[inline(always)]
    pub fn bit_uwf(&mut self) -> BitUwfW<'_, RegWowNanCtrlSpec> {
        BitUwfW::new(self, 3)
    }
    #[doc = "Bit 4 - Bit 0 of Select to check the MACID of which port for the WMAC PSF NSARP function. 0: to check the MACID of port0; 1: to check the MACID of port1. 2: to check the MACID of port2. 3: to check the MACID of port3. 4: to check the MACID of port4."]
    #[inline(always)]
    pub fn bit_psf_bssidsel_l(&mut self) -> BitPsfBssidselLW<'_, RegWowNanCtrlSpec> {
        BitPsfBssidselLW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is set to allow wake host as wake on wlan event occurring"]
    #[inline(always)]
    pub fn bit_wowhci(&mut self) -> BitWowhciW<'_, RegWowNanCtrlSpec> {
        BitWowhciW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Bit 1,2 of Select to check the MACID of which port for the WMAC PSF NSARP function. 0: to check the MACID of port0; 1: to check the MACID of port1. 2: to check the MACID of port2. 3: to check the MACID of port3. 4: to check the MACID of port4."]
    #[inline(always)]
    pub fn bit_psf_bssidsel_h(&mut self) -> BitPsfBssidselHW<'_, RegWowNanCtrlSpec> {
        BitPsfBssidselHW::new(self, 6)
    }
    #[doc = "Bit 8 - When set to 1, the TSF address filter will enable. When set to 0, the TSF address filter will disable."]
    #[inline(always)]
    pub fn bit_nan_rx_filter_en(&mut self) -> BitNanRxFilterEnW<'_, RegWowNanCtrlSpec> {
        BitNanRxFilterEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Check BSSID_BCN for TSF When set to 1, MAC will check the BSSID (A3) of RX beacon or Probe to determine if this packet will sync TSF or not"]
    #[inline(always)]
    pub fn bit_nan_rx_chkbssid(&mut self) -> BitNanRxChkbssidW<'_, RegWowNanCtrlSpec> {
        BitNanRxChkbssidW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable bit for check TSF filter When set to 1, the TSF address filter will enable. When set to 0, the TSF address filter will disable."]
    #[inline(always)]
    pub fn bit_nan_rx_chkta(&mut self) -> BitNanRxChktaW<'_, RegWowNanCtrlSpec> {
        BitNanRxChktaW::new(self, 10)
    }
    #[doc = "Bits 21:23 - The selector for the source of PS RX information (0x692\\[4:0\\]). 0 for port 0, 1 for clint0, 2 for clint1, 3 for clint2, 4 for clint3."]
    #[inline(always)]
    pub fn bit_portsel_ps_rx_info(&mut self) -> BitPortselPsRxInfoW<'_, RegWowNanCtrlSpec> {
        BitPortselPsRxInfoW::new(self, 21)
    }
    #[doc = "Bit 24 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID0"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid0(&mut self) -> BitWmmpsUapsdTid0W<'_, RegWowNanCtrlSpec> {
        BitWmmpsUapsdTid0W::new(self, 24)
    }
    #[doc = "Bit 25 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID1"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid1(&mut self) -> BitWmmpsUapsdTid1W<'_, RegWowNanCtrlSpec> {
        BitWmmpsUapsdTid1W::new(self, 25)
    }
    #[doc = "Bit 26 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID2"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid2(&mut self) -> BitWmmpsUapsdTid2W<'_, RegWowNanCtrlSpec> {
        BitWmmpsUapsdTid2W::new(self, 26)
    }
    #[doc = "Bit 27 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID3"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid3(&mut self) -> BitWmmpsUapsdTid3W<'_, RegWowNanCtrlSpec> {
        BitWmmpsUapsdTid3W::new(self, 27)
    }
    #[doc = "Bit 28 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID4"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid4(&mut self) -> BitWmmpsUapsdTid4W<'_, RegWowNanCtrlSpec> {
        BitWmmpsUapsdTid4W::new(self, 28)
    }
    #[doc = "Bit 29 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID5"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid5(&mut self) -> BitWmmpsUapsdTid5W<'_, RegWowNanCtrlSpec> {
        BitWmmpsUapsdTid5W::new(self, 29)
    }
    #[doc = "Bit 30 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID6"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid6(&mut self) -> BitWmmpsUapsdTid6W<'_, RegWowNanCtrlSpec> {
        BitWmmpsUapsdTid6W::new(self, 30)
    }
    #[doc = "Bit 31 - Disable interrupt 8051 when rx WMMPS UAPSD Qos data with TID7"]
    #[inline(always)]
    pub fn bit_wmmps_uapsd_tid7(&mut self) -> BitWmmpsUapsdTid7W<'_, RegWowNanCtrlSpec> {
        BitWmmpsUapsdTid7W::new(self, 31)
    }
}
#[doc = "Wake On WLAN and NAN Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wow_nan_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wow_nan_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWowNanCtrlSpec;
impl crate::RegisterSpec for RegWowNanCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wow_nan_ctrl::R`](R) reader structure"]
impl crate::Readable for RegWowNanCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wow_nan_ctrl::W`](W) writer structure"]
impl crate::Writable for RegWowNanCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WOW_NAN_CTRL to value 0"]
impl crate::Resettable for RegWowNanCtrlSpec {}
