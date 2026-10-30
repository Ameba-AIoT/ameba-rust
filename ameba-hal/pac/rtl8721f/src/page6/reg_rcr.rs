#[doc = "Register `REG_RCR` reader"]
pub type R = crate::R<RegRcrSpec>;
#[doc = "Register `REG_RCR` writer"]
pub type W = crate::W<RegRcrSpec>;
#[doc = "Field `BIT_AAP` reader - Accept Destination Address packets. This bit determines whether packets with a destination address will be accepted or rejected. 1: Accept 0: Reject"]
pub type BitAapR = crate::BitReader;
#[doc = "Field `BIT_AAP` writer - Accept Destination Address packets. This bit determines whether packets with a destination address will be accepted or rejected. 1: Accept 0: Reject"]
pub type BitAapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APM` reader - Accept Physical Match packets. This bit determines whether physical match packets will be accepted or rejected. 1: Accept 0: Reject"]
pub type BitApmR = crate::BitReader;
#[doc = "Field `BIT_APM` writer - Accept Physical Match packets. This bit determines whether physical match packets will be accepted or rejected. 1: Accept 0: Reject"]
pub type BitApmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_AM` reader - Accept Multicast packets. This bit determines whether multicast packets will be accepted or rejected. 1: Accept 0: Reject"]
pub type BitAmR = crate::BitReader;
#[doc = "Field `BIT_AM` writer - Accept Multicast packets. This bit determines whether multicast packets will be accepted or rejected. 1: Accept 0: Reject"]
pub type BitAmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_AB` reader - Accept Broadcast packets. This bit determines whether broadcast packets will be accepted or rejected. 1: Accept 0: Reject"]
pub type BitAbR = crate::BitReader;
#[doc = "Field `BIT_AB` writer - Accept Broadcast packets. This bit determines whether broadcast packets will be accepted or rejected. 1: Accept 0: Reject"]
pub type BitAbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ADD3` reader - Accept Address 3 Match Packets. Set this bit to 1 to accept broadcast/multicast data type frames that Address 3 matching RTL8190\"s MAC address. This bit is valid only when NETYPE (bit 3-2, MSR) is set to Link OK on an Infrastructure network."]
pub type BitAdd3R = crate::BitReader;
#[doc = "Field `BIT_ADD3` writer - Accept Address 3 Match Packets. Set this bit to 1 to accept broadcast/multicast data type frames that Address 3 matching RTL8190\"s MAC address. This bit is valid only when NETYPE (bit 3-2, MSR) is set to Link OK on an Infrastructure network."]
pub type BitAdd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APWRMGT` reader - Accept Power Management Packet. This bit determines whether the RTL8192D will accept or reject packets with the power management bit set. 1: Accept 0: Reject"]
pub type BitApwrmgtR = crate::BitReader;
#[doc = "Field `BIT_APWRMGT` writer - Accept Power Management Packet. This bit determines whether the RTL8192D will accept or reject packets with the power management bit set. 1: Accept 0: Reject"]
pub type BitApwrmgtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CBSSID_DATA` reader - Check BSSID_DATA, To DS, From DS Match Packet. When set to 1, MAC will check the Rx data type frame\"s BSSID, To DS, and From DS fields, to determine if it is set to Link OK in an Infrastructure or Ad hoc network. Note this setting is independent of nettype. Driver must set this bit in ad hoc mode and client mode."]
pub type BitCbssidDataR = crate::BitReader;
#[doc = "Field `BIT_CBSSID_DATA` writer - Check BSSID_DATA, To DS, From DS Match Packet. When set to 1, MAC will check the Rx data type frame\"s BSSID, To DS, and From DS fields, to determine if it is set to Link OK in an Infrastructure or Ad hoc network. Note this setting is independent of nettype. Driver must set this bit in ad hoc mode and client mode."]
pub type BitCbssidDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CBSSID_MGNT` reader - Check BSSID_BCN When set to 1, MAC will check the BSSID of RX beacon or Probe response to determine if this packet belongs to this BSSID."]
pub type BitCbssidMgntR = crate::BitReader;
#[doc = "Field `BIT_CBSSID_MGNT` writer - Check BSSID_BCN When set to 1, MAC will check the BSSID of RX beacon or Probe response to determine if this packet belongs to this BSSID."]
pub type BitCbssidMgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ACRC32` reader - Accept CRC32 Error packets. When set to 1, packets with CRC32 errors will be accepted. When set to 0, packets with CRC32 errors will be rejected. 1: Accept 0: Reject"]
pub type BitAcrc32R = crate::BitReader;
#[doc = "Field `BIT_ACRC32` writer - Accept CRC32 Error packets. When set to 1, packets with CRC32 errors will be accepted. When set to 0, packets with CRC32 errors will be rejected. 1: Accept 0: Reject"]
pub type BitAcrc32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_AICV` reader - Accept Integrity Check Value Error packets. This bit determines whether packets with ICV errors will be accepted or rejected. 1: Accept 0: Reject"]
pub type BitAicvR = crate::BitReader;
#[doc = "Field `BIT_AICV` writer - Accept Integrity Check Value Error packets. This bit determines whether packets with ICV errors will be accepted or rejected. 1: Accept 0: Reject"]
pub type BitAicvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DISDECNMYPKT` reader - This bit determines whether hw need to do decryption. 1: If A1 match or broadcast, do decryption. 0: Do decryption."]
pub type BitDisdecnmypktR = crate::BitReader;
#[doc = "Field `BIT_DISDECNMYPKT` writer - This bit determines whether hw need to do decryption. 1: If A1 match or broadcast, do decryption. 0: Do decryption."]
pub type BitDisdecnmypktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CHKTA_MGNT` reader - Check Transmitter_BCN When set to 1, MAC will check the Transmitter Address (A2) of RX beacon or Probe response to determine if this packet belongs to this Transmitter Address (Address 2) . Both in Infrastructure mode and Ad-hoc mode."]
pub type BitChktaMgntR = crate::BitReader;
#[doc = "Field `BIT_CHKTA_MGNT` writer - Check Transmitter_BCN When set to 1, MAC will check the Transmitter Address (A2) of RX beacon or Probe response to determine if this packet belongs to this Transmitter Address (Address 2) . Both in Infrastructure mode and Ad-hoc mode."]
pub type BitChktaMgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ACK_CBSSID` reader - The field effects behavior of response frame,except TB. 0:Only A1 match to send response frame. 1:A1 match and satisfy BSSID condition will send response frame."]
pub type BitAckCbssidR = crate::BitReader;
#[doc = "Field `BIT_ACK_CBSSID` writer - The field effects behavior of response frame,except TB. 0:Only A1 match to send response frame. 1:A1 match and satisfy BSSID condition will send response frame."]
pub type BitAckCbssidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CHK_PREVTXA2` reader - When set,enable check rxa1 with txa2."]
pub type BitChkPrevtxa2R = crate::BitReader;
#[doc = "Field `BIT_CHK_PREVTXA2` writer - When set,enable check rxa1 with txa2."]
pub type BitChkPrevtxa2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HTCBFMC` reader - 1: HTC -> MFC 0: MFC-> HTC"]
pub type BitHtcbfmcR = crate::BitReader;
#[doc = "Field `BIT_HTCBFMC` writer - 1: HTC -> MFC 0: MFC-> HTC"]
pub type BitHtcbfmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXSK_PERPKT` reader - Executing key search per MPDU"]
pub type BitRxskPerpktR = crate::BitReader;
#[doc = "Field `BIT_RXSK_PERPKT` writer - Executing key search per MPDU"]
pub type BitRxskPerpktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_UCMDINT_EN` reader - Unicast data packet with FC.MD = 1 interrupt enable. When this bit is set, this bit is auto clear by MAC. As long as a data type unicast packet is received, this bit will be set."]
pub type BitUcmdintEnR = crate::BitReader;
#[doc = "Field `BIT_UCMDINT_EN` writer - Unicast data packet with FC.MD = 1 interrupt enable. When this bit is set, this bit is auto clear by MAC. As long as a data type unicast packet is received, this bit will be set."]
pub type BitUcmdintEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BCMDINT_EN` reader - Broadcast data packet interrupt enable. When this bit is set, this bit is auto clear by MAC. As long as a data type broadcast packet is received, this bit will be set."]
pub type BitBcmdintEnR = crate::BitReader;
#[doc = "Field `BIT_BCMDINT_EN` writer - Broadcast data packet interrupt enable. When this bit is set, this bit is auto clear by MAC. As long as a data type broadcast packet is received, this bit will be set."]
pub type BitBcmdintEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TIMPSR_EN` reader - When set, enable RX Beacon TIM Parser"]
pub type BitTimpsrEnR = crate::BitReader;
#[doc = "Field `BIT_TIMPSR_EN` writer - When set, enable RX Beacon TIM Parser"]
pub type BitTimpsrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DISGCLK` reader - When set,disable macrx clock gating control"]
pub type BitDisgclkR = crate::BitReader;
#[doc = "Field `BIT_DISGCLK` writer - When set,disable macrx clock gating control"]
pub type BitDisgclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PKTCTL_DLKEN` reader - When set,while rx path dead lock occurs, reset rx path"]
pub type BitPktctlDlkenR = crate::BitReader;
#[doc = "Field `BIT_PKTCTL_DLKEN` writer - When set,while rx path dead lock occurs, reset rx path"]
pub type BitPktctlDlkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DISCHKPPDLLEN` reader - When set,do not check PPDU while the PPDU length is smaller than 14 byte."]
pub type BitDischkppdllenR = crate::BitReader;
#[doc = "Field `BIT_DISCHKPPDLLEN` writer - When set,do not check PPDU while the PPDU length is smaller than 14 byte."]
pub type BitDischkppdllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MFBEN` reader - Enable immediate MCS Feedback function 1: Enable. When Rx packet with MRQ = 1\"b1, then search KEYCAM to find sender\"s MCS Feedback function and send response 0: Disable"]
pub type BitMfbenR = crate::BitReader;
#[doc = "Field `BIT_MFBEN` writer - Enable immediate MCS Feedback function 1: Enable. When Rx packet with MRQ = 1\"b1, then search KEYCAM to find sender\"s MCS Feedback function and send response 0: Disable"]
pub type BitMfbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LSIGEN` reader - Enable LSIG TXOP Protection function. 1: Enable. Search KEYCAM for each rx packet to check if LSIGEN bit is set. If LSIGEN is set, then sender support LSIG TXOP protection and we need send response with LSIG TXOP protection 0: Disable"]
pub type BitLsigenR = crate::BitReader;
#[doc = "Field `BIT_LSIGEN` writer - Enable LSIG TXOP Protection function. 1: Enable. Search KEYCAM for each rx packet to check if LSIGEN bit is set. If LSIGEN is set, then sender support LSIG TXOP protection and we need send response with LSIG TXOP protection 0: Disable"]
pub type BitLsigenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ENMBID` reader - Enable Multiple Bssid. 1: Enable. Only response ACK to the packets whose DID(A1) matching to the addresses in the MBSSID CAM Entries. 0: Disable."]
pub type BitEnmbidR = crate::BitReader;
#[doc = "Field `BIT_ENMBID` writer - Enable Multiple Bssid. 1: Enable. Only response ACK to the packets whose DID(A1) matching to the addresses in the MBSSID CAM Entries. 0: Disable."]
pub type BitEnmbidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TCPOFLD_EN` reader - When set,Enable TCP checksum offload"]
pub type BitTcpofldEnR = crate::BitReader;
#[doc = "Field `BIT_TCPOFLD_EN` writer - When set,Enable TCP checksum offload"]
pub type BitTcpofldEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_VHT_DACK` reader - This bit to control response type for vht single mpdu data packet. 1: ACK as response 0: BA as response"]
pub type BitVhtDackR = crate::BitReader;
#[doc = "Field `BIT_VHT_DACK` writer - This bit to control response type for vht single mpdu data packet. 1: ACK as response 0: BA as response"]
pub type BitVhtDackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APP_BASSN` reader - Append SSN of previous TXBA Enable When this bit is set, then SSN of previous TXBA is appended as after original RXDESC as the 4-th DW of RXDESC."]
pub type BitAppBassnR = crate::BitReader;
#[doc = "Field `BIT_APP_BASSN` writer - Append SSN of previous TXBA Enable When this bit is set, then SSN of previous TXBA is appended as after original RXDESC as the 4-th DW of RXDESC."]
pub type BitAppBassnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APP_PHYSTS` reader - Append RXFF0 PHY Status Enable When this bit is set, then PHY Status is appended before RX packet in RXFF. When this bit is set, the RX_DRV_INFO_SZ shall be set to include PHY Status report."]
pub type BitAppPhystsR = crate::BitReader;
#[doc = "Field `BIT_APP_PHYSTS` writer - Append RXFF0 PHY Status Enable When this bit is set, then PHY Status is appended before RX packet in RXFF. When this bit is set, the RX_DRV_INFO_SZ shall be set to include PHY Status report."]
pub type BitAppPhystsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APP_ICV` reader - When this bit is set to 1, MACRX will retain the ICV at the bottom of the packet. Otherwise, MIC will be removed and reported the status in RXDESC."]
pub type BitAppIcvR = crate::BitReader;
#[doc = "Field `BIT_APP_ICV` writer - When this bit is set to 1, MACRX will retain the ICV at the bottom of the packet. Otherwise, MIC will be removed and reported the status in RXDESC."]
pub type BitAppIcvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APP_MIC` reader - When this bit is set to 1, MACRX will retain the MIC at the bottom of the packet. Otherwise, MIC will be removed and reported the status in RXDESC. When a MPDU is encrypted with TKIP without MIC, MACRX will ignore this option, that is, MIC must be remained in back of the plain text."]
pub type BitAppMicR = crate::BitReader;
#[doc = "Field `BIT_APP_MIC` writer - When this bit is set to 1, MACRX will retain the MIC at the bottom of the packet. Otherwise, MIC will be removed and reported the status in RXDESC. When a MPDU is encrypted with TKIP without MIC, MACRX will ignore this option, that is, MIC must be remained in back of the plain text."]
pub type BitAppMicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_APP_FCS` reader - When set, wmac RX will append FCS after payload."]
pub type BitAppFcsR = crate::BitReader;
#[doc = "Field `BIT_APP_FCS` writer - When set, wmac RX will append FCS after payload."]
pub type BitAppFcsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Accept Destination Address packets. This bit determines whether packets with a destination address will be accepted or rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_aap(&self) -> BitAapR {
        BitAapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Accept Physical Match packets. This bit determines whether physical match packets will be accepted or rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_apm(&self) -> BitApmR {
        BitApmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Accept Multicast packets. This bit determines whether multicast packets will be accepted or rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_am(&self) -> BitAmR {
        BitAmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Accept Broadcast packets. This bit determines whether broadcast packets will be accepted or rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_ab(&self) -> BitAbR {
        BitAbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Accept Address 3 Match Packets. Set this bit to 1 to accept broadcast/multicast data type frames that Address 3 matching RTL8190\"s MAC address. This bit is valid only when NETYPE (bit 3-2, MSR) is set to Link OK on an Infrastructure network."]
    #[inline(always)]
    pub fn bit_add3(&self) -> BitAdd3R {
        BitAdd3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Accept Power Management Packet. This bit determines whether the RTL8192D will accept or reject packets with the power management bit set. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_apwrmgt(&self) -> BitApwrmgtR {
        BitApwrmgtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Check BSSID_DATA, To DS, From DS Match Packet. When set to 1, MAC will check the Rx data type frame\"s BSSID, To DS, and From DS fields, to determine if it is set to Link OK in an Infrastructure or Ad hoc network. Note this setting is independent of nettype. Driver must set this bit in ad hoc mode and client mode."]
    #[inline(always)]
    pub fn bit_cbssid_data(&self) -> BitCbssidDataR {
        BitCbssidDataR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Check BSSID_BCN When set to 1, MAC will check the BSSID of RX beacon or Probe response to determine if this packet belongs to this BSSID."]
    #[inline(always)]
    pub fn bit_cbssid_mgnt(&self) -> BitCbssidMgntR {
        BitCbssidMgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Accept CRC32 Error packets. When set to 1, packets with CRC32 errors will be accepted. When set to 0, packets with CRC32 errors will be rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_acrc32(&self) -> BitAcrc32R {
        BitAcrc32R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Accept Integrity Check Value Error packets. This bit determines whether packets with ICV errors will be accepted or rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_aicv(&self) -> BitAicvR {
        BitAicvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit determines whether hw need to do decryption. 1: If A1 match or broadcast, do decryption. 0: Do decryption."]
    #[inline(always)]
    pub fn bit_disdecnmypkt(&self) -> BitDisdecnmypktR {
        BitDisdecnmypktR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Check Transmitter_BCN When set to 1, MAC will check the Transmitter Address (A2) of RX beacon or Probe response to determine if this packet belongs to this Transmitter Address (Address 2) . Both in Infrastructure mode and Ad-hoc mode."]
    #[inline(always)]
    pub fn bit_chkta_mgnt(&self) -> BitChktaMgntR {
        BitChktaMgntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The field effects behavior of response frame,except TB. 0:Only A1 match to send response frame. 1:A1 match and satisfy BSSID condition will send response frame."]
    #[inline(always)]
    pub fn bit_ack_cbssid(&self) -> BitAckCbssidR {
        BitAckCbssidR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When set,enable check rxa1 with txa2."]
    #[inline(always)]
    pub fn bit_chk_prevtxa2(&self) -> BitChkPrevtxa2R {
        BitChkPrevtxa2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: HTC -> MFC 0: MFC-> HTC"]
    #[inline(always)]
    pub fn bit_htcbfmc(&self) -> BitHtcbfmcR {
        BitHtcbfmcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Executing key search per MPDU"]
    #[inline(always)]
    pub fn bit_rxsk_perpkt(&self) -> BitRxskPerpktR {
        BitRxskPerpktR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Unicast data packet with FC.MD = 1 interrupt enable. When this bit is set, this bit is auto clear by MAC. As long as a data type unicast packet is received, this bit will be set."]
    #[inline(always)]
    pub fn bit_ucmdint_en(&self) -> BitUcmdintEnR {
        BitUcmdintEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Broadcast data packet interrupt enable. When this bit is set, this bit is auto clear by MAC. As long as a data type broadcast packet is received, this bit will be set."]
    #[inline(always)]
    pub fn bit_bcmdint_en(&self) -> BitBcmdintEnR {
        BitBcmdintEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When set, enable RX Beacon TIM Parser"]
    #[inline(always)]
    pub fn bit_timpsr_en(&self) -> BitTimpsrEnR {
        BitTimpsrEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When set,disable macrx clock gating control"]
    #[inline(always)]
    pub fn bit_disgclk(&self) -> BitDisgclkR {
        BitDisgclkR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When set,while rx path dead lock occurs, reset rx path"]
    #[inline(always)]
    pub fn bit_pktctl_dlken(&self) -> BitPktctlDlkenR {
        BitPktctlDlkenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When set,do not check PPDU while the PPDU length is smaller than 14 byte."]
    #[inline(always)]
    pub fn bit_dischkppdllen(&self) -> BitDischkppdllenR {
        BitDischkppdllenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable immediate MCS Feedback function 1: Enable. When Rx packet with MRQ = 1\"b1, then search KEYCAM to find sender\"s MCS Feedback function and send response 0: Disable"]
    #[inline(always)]
    pub fn bit_mfben(&self) -> BitMfbenR {
        BitMfbenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable LSIG TXOP Protection function. 1: Enable. Search KEYCAM for each rx packet to check if LSIGEN bit is set. If LSIGEN is set, then sender support LSIG TXOP protection and we need send response with LSIG TXOP protection 0: Disable"]
    #[inline(always)]
    pub fn bit_lsigen(&self) -> BitLsigenR {
        BitLsigenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Multiple Bssid. 1: Enable. Only response ACK to the packets whose DID(A1) matching to the addresses in the MBSSID CAM Entries. 0: Disable."]
    #[inline(always)]
    pub fn bit_enmbid(&self) -> BitEnmbidR {
        BitEnmbidR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When set,Enable TCP checksum offload"]
    #[inline(always)]
    pub fn bit_tcpofld_en(&self) -> BitTcpofldEnR {
        BitTcpofldEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This bit to control response type for vht single mpdu data packet. 1: ACK as response 0: BA as response"]
    #[inline(always)]
    pub fn bit_vht_dack(&self) -> BitVhtDackR {
        BitVhtDackR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Append SSN of previous TXBA Enable When this bit is set, then SSN of previous TXBA is appended as after original RXDESC as the 4-th DW of RXDESC."]
    #[inline(always)]
    pub fn bit_app_bassn(&self) -> BitAppBassnR {
        BitAppBassnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Append RXFF0 PHY Status Enable When this bit is set, then PHY Status is appended before RX packet in RXFF. When this bit is set, the RX_DRV_INFO_SZ shall be set to include PHY Status report."]
    #[inline(always)]
    pub fn bit_app_physts(&self) -> BitAppPhystsR {
        BitAppPhystsR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When this bit is set to 1, MACRX will retain the ICV at the bottom of the packet. Otherwise, MIC will be removed and reported the status in RXDESC."]
    #[inline(always)]
    pub fn bit_app_icv(&self) -> BitAppIcvR {
        BitAppIcvR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When this bit is set to 1, MACRX will retain the MIC at the bottom of the packet. Otherwise, MIC will be removed and reported the status in RXDESC. When a MPDU is encrypted with TKIP without MIC, MACRX will ignore this option, that is, MIC must be remained in back of the plain text."]
    #[inline(always)]
    pub fn bit_app_mic(&self) -> BitAppMicR {
        BitAppMicR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set, wmac RX will append FCS after payload."]
    #[inline(always)]
    pub fn bit_app_fcs(&self) -> BitAppFcsR {
        BitAppFcsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Accept Destination Address packets. This bit determines whether packets with a destination address will be accepted or rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_aap(&mut self) -> BitAapW<'_, RegRcrSpec> {
        BitAapW::new(self, 0)
    }
    #[doc = "Bit 1 - Accept Physical Match packets. This bit determines whether physical match packets will be accepted or rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_apm(&mut self) -> BitApmW<'_, RegRcrSpec> {
        BitApmW::new(self, 1)
    }
    #[doc = "Bit 2 - Accept Multicast packets. This bit determines whether multicast packets will be accepted or rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_am(&mut self) -> BitAmW<'_, RegRcrSpec> {
        BitAmW::new(self, 2)
    }
    #[doc = "Bit 3 - Accept Broadcast packets. This bit determines whether broadcast packets will be accepted or rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_ab(&mut self) -> BitAbW<'_, RegRcrSpec> {
        BitAbW::new(self, 3)
    }
    #[doc = "Bit 4 - Accept Address 3 Match Packets. Set this bit to 1 to accept broadcast/multicast data type frames that Address 3 matching RTL8190\"s MAC address. This bit is valid only when NETYPE (bit 3-2, MSR) is set to Link OK on an Infrastructure network."]
    #[inline(always)]
    pub fn bit_add3(&mut self) -> BitAdd3W<'_, RegRcrSpec> {
        BitAdd3W::new(self, 4)
    }
    #[doc = "Bit 5 - Accept Power Management Packet. This bit determines whether the RTL8192D will accept or reject packets with the power management bit set. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_apwrmgt(&mut self) -> BitApwrmgtW<'_, RegRcrSpec> {
        BitApwrmgtW::new(self, 5)
    }
    #[doc = "Bit 6 - Check BSSID_DATA, To DS, From DS Match Packet. When set to 1, MAC will check the Rx data type frame\"s BSSID, To DS, and From DS fields, to determine if it is set to Link OK in an Infrastructure or Ad hoc network. Note this setting is independent of nettype. Driver must set this bit in ad hoc mode and client mode."]
    #[inline(always)]
    pub fn bit_cbssid_data(&mut self) -> BitCbssidDataW<'_, RegRcrSpec> {
        BitCbssidDataW::new(self, 6)
    }
    #[doc = "Bit 7 - Check BSSID_BCN When set to 1, MAC will check the BSSID of RX beacon or Probe response to determine if this packet belongs to this BSSID."]
    #[inline(always)]
    pub fn bit_cbssid_mgnt(&mut self) -> BitCbssidMgntW<'_, RegRcrSpec> {
        BitCbssidMgntW::new(self, 7)
    }
    #[doc = "Bit 8 - Accept CRC32 Error packets. When set to 1, packets with CRC32 errors will be accepted. When set to 0, packets with CRC32 errors will be rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_acrc32(&mut self) -> BitAcrc32W<'_, RegRcrSpec> {
        BitAcrc32W::new(self, 8)
    }
    #[doc = "Bit 9 - Accept Integrity Check Value Error packets. This bit determines whether packets with ICV errors will be accepted or rejected. 1: Accept 0: Reject"]
    #[inline(always)]
    pub fn bit_aicv(&mut self) -> BitAicvW<'_, RegRcrSpec> {
        BitAicvW::new(self, 9)
    }
    #[doc = "Bit 10 - This bit determines whether hw need to do decryption. 1: If A1 match or broadcast, do decryption. 0: Do decryption."]
    #[inline(always)]
    pub fn bit_disdecnmypkt(&mut self) -> BitDisdecnmypktW<'_, RegRcrSpec> {
        BitDisdecnmypktW::new(self, 10)
    }
    #[doc = "Bit 11 - Check Transmitter_BCN When set to 1, MAC will check the Transmitter Address (A2) of RX beacon or Probe response to determine if this packet belongs to this Transmitter Address (Address 2) . Both in Infrastructure mode and Ad-hoc mode."]
    #[inline(always)]
    pub fn bit_chkta_mgnt(&mut self) -> BitChktaMgntW<'_, RegRcrSpec> {
        BitChktaMgntW::new(self, 11)
    }
    #[doc = "Bit 12 - The field effects behavior of response frame,except TB. 0:Only A1 match to send response frame. 1:A1 match and satisfy BSSID condition will send response frame."]
    #[inline(always)]
    pub fn bit_ack_cbssid(&mut self) -> BitAckCbssidW<'_, RegRcrSpec> {
        BitAckCbssidW::new(self, 12)
    }
    #[doc = "Bit 13 - When set,enable check rxa1 with txa2."]
    #[inline(always)]
    pub fn bit_chk_prevtxa2(&mut self) -> BitChkPrevtxa2W<'_, RegRcrSpec> {
        BitChkPrevtxa2W::new(self, 13)
    }
    #[doc = "Bit 14 - 1: HTC -> MFC 0: MFC-> HTC"]
    #[inline(always)]
    pub fn bit_htcbfmc(&mut self) -> BitHtcbfmcW<'_, RegRcrSpec> {
        BitHtcbfmcW::new(self, 14)
    }
    #[doc = "Bit 15 - Executing key search per MPDU"]
    #[inline(always)]
    pub fn bit_rxsk_perpkt(&mut self) -> BitRxskPerpktW<'_, RegRcrSpec> {
        BitRxskPerpktW::new(self, 15)
    }
    #[doc = "Bit 16 - Unicast data packet with FC.MD = 1 interrupt enable. When this bit is set, this bit is auto clear by MAC. As long as a data type unicast packet is received, this bit will be set."]
    #[inline(always)]
    pub fn bit_ucmdint_en(&mut self) -> BitUcmdintEnW<'_, RegRcrSpec> {
        BitUcmdintEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Broadcast data packet interrupt enable. When this bit is set, this bit is auto clear by MAC. As long as a data type broadcast packet is received, this bit will be set."]
    #[inline(always)]
    pub fn bit_bcmdint_en(&mut self) -> BitBcmdintEnW<'_, RegRcrSpec> {
        BitBcmdintEnW::new(self, 17)
    }
    #[doc = "Bit 18 - When set, enable RX Beacon TIM Parser"]
    #[inline(always)]
    pub fn bit_timpsr_en(&mut self) -> BitTimpsrEnW<'_, RegRcrSpec> {
        BitTimpsrEnW::new(self, 18)
    }
    #[doc = "Bit 19 - When set,disable macrx clock gating control"]
    #[inline(always)]
    pub fn bit_disgclk(&mut self) -> BitDisgclkW<'_, RegRcrSpec> {
        BitDisgclkW::new(self, 19)
    }
    #[doc = "Bit 20 - When set,while rx path dead lock occurs, reset rx path"]
    #[inline(always)]
    pub fn bit_pktctl_dlken(&mut self) -> BitPktctlDlkenW<'_, RegRcrSpec> {
        BitPktctlDlkenW::new(self, 20)
    }
    #[doc = "Bit 21 - When set,do not check PPDU while the PPDU length is smaller than 14 byte."]
    #[inline(always)]
    pub fn bit_dischkppdllen(&mut self) -> BitDischkppdllenW<'_, RegRcrSpec> {
        BitDischkppdllenW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable immediate MCS Feedback function 1: Enable. When Rx packet with MRQ = 1\"b1, then search KEYCAM to find sender\"s MCS Feedback function and send response 0: Disable"]
    #[inline(always)]
    pub fn bit_mfben(&mut self) -> BitMfbenW<'_, RegRcrSpec> {
        BitMfbenW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable LSIG TXOP Protection function. 1: Enable. Search KEYCAM for each rx packet to check if LSIGEN bit is set. If LSIGEN is set, then sender support LSIG TXOP protection and we need send response with LSIG TXOP protection 0: Disable"]
    #[inline(always)]
    pub fn bit_lsigen(&mut self) -> BitLsigenW<'_, RegRcrSpec> {
        BitLsigenW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Multiple Bssid. 1: Enable. Only response ACK to the packets whose DID(A1) matching to the addresses in the MBSSID CAM Entries. 0: Disable."]
    #[inline(always)]
    pub fn bit_enmbid(&mut self) -> BitEnmbidW<'_, RegRcrSpec> {
        BitEnmbidW::new(self, 24)
    }
    #[doc = "Bit 25 - When set,Enable TCP checksum offload"]
    #[inline(always)]
    pub fn bit_tcpofld_en(&mut self) -> BitTcpofldEnW<'_, RegRcrSpec> {
        BitTcpofldEnW::new(self, 25)
    }
    #[doc = "Bit 26 - This bit to control response type for vht single mpdu data packet. 1: ACK as response 0: BA as response"]
    #[inline(always)]
    pub fn bit_vht_dack(&mut self) -> BitVhtDackW<'_, RegRcrSpec> {
        BitVhtDackW::new(self, 26)
    }
    #[doc = "Bit 27 - Append SSN of previous TXBA Enable When this bit is set, then SSN of previous TXBA is appended as after original RXDESC as the 4-th DW of RXDESC."]
    #[inline(always)]
    pub fn bit_app_bassn(&mut self) -> BitAppBassnW<'_, RegRcrSpec> {
        BitAppBassnW::new(self, 27)
    }
    #[doc = "Bit 28 - Append RXFF0 PHY Status Enable When this bit is set, then PHY Status is appended before RX packet in RXFF. When this bit is set, the RX_DRV_INFO_SZ shall be set to include PHY Status report."]
    #[inline(always)]
    pub fn bit_app_physts(&mut self) -> BitAppPhystsW<'_, RegRcrSpec> {
        BitAppPhystsW::new(self, 28)
    }
    #[doc = "Bit 29 - When this bit is set to 1, MACRX will retain the ICV at the bottom of the packet. Otherwise, MIC will be removed and reported the status in RXDESC."]
    #[inline(always)]
    pub fn bit_app_icv(&mut self) -> BitAppIcvW<'_, RegRcrSpec> {
        BitAppIcvW::new(self, 29)
    }
    #[doc = "Bit 30 - When this bit is set to 1, MACRX will retain the MIC at the bottom of the packet. Otherwise, MIC will be removed and reported the status in RXDESC. When a MPDU is encrypted with TKIP without MIC, MACRX will ignore this option, that is, MIC must be remained in back of the plain text."]
    #[inline(always)]
    pub fn bit_app_mic(&mut self) -> BitAppMicW<'_, RegRcrSpec> {
        BitAppMicW::new(self, 30)
    }
    #[doc = "Bit 31 - When set, wmac RX will append FCS after payload."]
    #[inline(always)]
    pub fn bit_app_fcs(&mut self) -> BitAppFcsW<'_, RegRcrSpec> {
        BitAppFcsW::new(self, 31)
    }
}
#[doc = "Receive Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRcrSpec;
impl crate::RegisterSpec for RegRcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rcr::R`](R) reader structure"]
impl crate::Readable for RegRcrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rcr::W`](W) writer structure"]
impl crate::Writable for RegRcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RCR to value 0x0400_0000"]
impl crate::Resettable for RegRcrSpec {
    const RESET_VALUE: u32 = 0x0400_0000;
}
