#[doc = "Register `REG_RXPSF_CTRL` reader"]
pub type R = crate::R<RegRxpsfCtrlSpec>;
#[doc = "Register `REG_RXPSF_CTRL` writer"]
pub type W = crate::W<RegRxpsfCtrlSpec>;
#[doc = "Field `BIT_RXPSF_ERRTHR` reader - FCS error packet threshold 3'd0: 1 packet 3'd1: 4 packets 3'd2: 8 packets 3'd3: 12 packets 3'd4: 16 packets 3'd5: 20 packets 3'd6: 24 packets 3'd7: 28 packets"]
pub type BitRxpsfErrthrR = crate::FieldReader;
#[doc = "Field `BIT_RXPSF_ERRTHR` writer - FCS error packet threshold 3'd0: 1 packet 3'd1: 4 packets 3'd2: 8 packets 3'd3: 12 packets 3'd4: 16 packets 3'd5: 20 packets 3'd6: 24 packets 3'd7: 28 packets"]
pub type BitRxpsfErrthrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_RXPSF_ALL_ERRCHKEN` reader - This bit is set to 1 to enable MAC to inform BB enter power saving mode as rx FCS error packets number in a PPDU is larger than the FCS error packet threshold defined by BBPSF_ERRTHR. This function is independent from rate, type and IDs."]
pub type BitRxpsfAllErrchkenR = crate::BitReader;
#[doc = "Field `BIT_RXPSF_ALL_ERRCHKEN` writer - This bit is set to 1 to enable MAC to inform BB enter power saving mode as rx FCS error packets number in a PPDU is larger than the FCS error packet threshold defined by BBPSF_ERRTHR. This function is independent from rate, type and IDs."]
pub type BitRxpsfAllErrchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXPSF_CONT_ERRCHKEN` reader - This bit is set to 1 to enable MAC to inform BB enter power saving mode as continuous rx FCS error packets number in a PPDU is large than the FCS error packet threshold defined by BBPSF_ERRTHR. This function is independent from rate, type and IDs."]
pub type BitRxpsfContErrchkenR = crate::BitReader;
#[doc = "Field `BIT_RXPSF_CONT_ERRCHKEN` writer - This bit is set to 1 to enable MAC to inform BB enter power saving mode as continuous rx FCS error packets number in a PPDU is large than the FCS error packet threshold defined by BBPSF_ERRTHR. This function is independent from rate, type and IDs."]
pub type BitRxpsfContErrchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXPSF_MHCHKEN` reader - This bit is set to 1 to enable MAC to inform BB enter power saving mode as rx unmatch my MACID unicast or unmatch my BSSID Broadcast/Mutlicast packets (including aggrigation and non-aggrigation)"]
pub type BitRxpsfMhchkenR = crate::BitReader;
#[doc = "Field `BIT_RXPSF_MHCHKEN` writer - This bit is set to 1 to enable MAC to inform BB enter power saving mode as rx unmatch my MACID unicast or unmatch my BSSID Broadcast/Mutlicast packets (including aggrigation and non-aggrigation)"]
pub type BitRxpsfMhchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXPSF_CCKRST` reader - This bit is set to 1 to reset MAC Rx before sending CCK packet triggered invalid_pkt pulse(0 for debug)"]
pub type BitRxpsfCckrstR = crate::BitReader;
#[doc = "Field `BIT_RXPSF_CCKRST` writer - This bit is set to 1 to reset MAC Rx before sending CCK packet triggered invalid_pkt pulse(0 for debug)"]
pub type BitRxpsfCckrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXPSF_OFDMRST` reader - This bit is set to 1 to reset MAC Rx before sending OFDM (include OFDM, HT, VHT) packet triggered invalid_pkt pulse (0 for debug)"]
pub type BitRxpsfOfdmrstR = crate::BitReader;
#[doc = "Field `BIT_RXPSF_OFDMRST` writer - This bit is set to 1 to reset MAC Rx before sending OFDM (include OFDM, HT, VHT) packet triggered invalid_pkt pulse (0 for debug)"]
pub type BitRxpsfOfdmrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXPSF_CCKCHKEN` reader - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = CCK"]
pub type BitRxpsfCckchkenR = crate::BitReader;
#[doc = "Field `BIT_RXPSF_CCKCHKEN` writer - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = CCK"]
pub type BitRxpsfCckchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXPSF_OFDMCHKEN` reader - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = OFDM"]
pub type BitRxpsfOfdmchkenR = crate::BitReader;
#[doc = "Field `BIT_RXPSF_OFDMCHKEN` writer - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = OFDM"]
pub type BitRxpsfOfdmchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXPSF_HTCHKEN` reader - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = HT"]
pub type BitRxpsfHtchkenR = crate::BitReader;
#[doc = "Field `BIT_RXPSF_HTCHKEN` writer - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = HT"]
pub type BitRxpsfHtchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXPSF_VHTCHKEN` reader - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = VHT"]
pub type BitRxpsfVhtchkenR = crate::BitReader;
#[doc = "Field `BIT_RXPSF_VHTCHKEN` writer - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = VHT"]
pub type BitRxpsfVhtchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXPSF_CTRLEN` reader - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet Type=01."]
pub type BitRxpsfCtrlenR = crate::BitReader;
#[doc = "Field `BIT_RXPSF_CTRLEN` writer - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet Type=01."]
pub type BitRxpsfCtrlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXPSF_PKTLENTHR` reader - Packet length threshold 3'd0: 16 Bytes 3'd1: 32 Bytes 3'd2: 64 Bytes 3'd3: 128 Bytes 3'd4: 256 Bytes 3'd5: 512 Bytes 3'd6: 1024 Bytes 3'd7: 2048 Bytes Only the packet which packet length is longer than this threshold will trigger an invalid_pkt pulse."]
pub type BitRxpsfPktlenthrR = crate::FieldReader;
#[doc = "Field `BIT_RXPSF_PKTLENTHR` writer - Packet length threshold 3'd0: 16 Bytes 3'd1: 32 Bytes 3'd2: 64 Bytes 3'd3: 128 Bytes 3'd4: 256 Bytes 3'd5: 512 Bytes 3'd6: 1024 Bytes 3'd7: 2048 Bytes Only the packet which packet length is longer than this threshold will trigger an invalid_pkt pulse."]
pub type BitRxpsfPktlenthrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_RXGCK_OFDMCCA_EN` reader - This bit is set to 1 to let OFDMCCA be one of the signals that can wakeup MAC from Rx_idle"]
pub type BitRxgckOfdmccaEnR = crate::BitReader;
#[doc = "Field `BIT_RXGCK_OFDMCCA_EN` writer - This bit is set to 1 to let OFDMCCA be one of the signals that can wakeup MAC from Rx_idle"]
pub type BitRxgckOfdmccaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXGCK_ENTRY_DELAY` reader - MAC_RX wait some time to gate clock after making the decision (about gating clock). Unit: T (MAC clock cycle) 0: 0 1: 2 2: 4 3: 8 4: 16 5: 32 6: 64 7: 128"]
pub type BitRxgckEntryDelayR = crate::FieldReader;
#[doc = "Field `BIT_RXGCK_ENTRY_DELAY` writer - MAC_RX wait some time to gate clock after making the decision (about gating clock). Unit: T (MAC clock cycle) 0: 0 1: 2 2: 4 3: 8 4: 16 5: 32 6: 64 7: 128"]
pub type BitRxgckEntryDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_RXGCK_CCK_FIFOTHR` reader - CCK rate RXGCK fifo threshold If CCKCCA = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 8, may cause error at some situation. 3: 15 (number of RDRDY before receiving MAC header), may cause error at some situation."]
pub type BitRxgckCckFifothrR = crate::FieldReader;
#[doc = "Field `BIT_RXGCK_CCK_FIFOTHR` writer - CCK rate RXGCK fifo threshold If CCKCCA = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 8, may cause error at some situation. 3: 15 (number of RDRDY before receiving MAC header), may cause error at some situation."]
pub type BitRxgckCckFifothrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RXGCK_OFDM_FIFOTHR` reader - OFDM rate RXGCK fifo threshold If OFDM_VBON = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (number of RDRDY before receiving MAC header = minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 2, may cause error at some situation. 3: 3 (minimum RDRDY number within one packet), may cause error at some situation."]
pub type BitRxgckOfdmFifothrR = crate::FieldReader;
#[doc = "Field `BIT_RXGCK_OFDM_FIFOTHR` writer - OFDM rate RXGCK fifo threshold If OFDM_VBON = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (number of RDRDY before receiving MAC header = minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 2, may cause error at some situation. 3: 3 (minimum RDRDY number within one packet), may cause error at some situation."]
pub type BitRxgckOfdmFifothrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RXGCK_HT_FIFOTHR` reader - HT rate RXGCK fifo threshold If RXHT = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 2 (number of RDRDY before receiving MAC header - 1), may cause error at some situation. 3: 3 (number of RDRDY before receiving MAC header), may cause error at some situation."]
pub type BitRxgckHtFifothrR = crate::FieldReader;
#[doc = "Field `BIT_RXGCK_HT_FIFOTHR` writer - HT rate RXGCK fifo threshold If RXHT = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 2 (number of RDRDY before receiving MAC header - 1), may cause error at some situation. 3: 3 (number of RDRDY before receiving MAC header), may cause error at some situation."]
pub type BitRxgckHtFifothrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RXGCK_VHT_FIFOTHR` reader - VHT rate RXGCK fifo threshold If RXVHT = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 3 (number of RDRDY before receiving MAC header - 1), may cause error at some situation. 3: 4 (number of RDRDY before receiving MAC header), may cause error at some situation."]
pub type BitRxgckVhtFifothrR = crate::FieldReader;
#[doc = "Field `BIT_RXGCK_VHT_FIFOTHR` writer - VHT rate RXGCK fifo threshold If RXVHT = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 3 (number of RDRDY before receiving MAC header - 1), may cause error at some situation. 3: 4 (number of RDRDY before receiving MAC header), may cause error at some situation."]
pub type BitRxgckVhtFifothrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RXGCK_FIFOTHR_EN` reader - 1: enable RXGCK fifo threshold 0: disable RXGCK fifo threshold (ignore fifo threshold) Should be 0 if MAC loopback mode"]
pub type BitRxgckFifothrEnR = crate::BitReader;
#[doc = "Field `BIT_RXGCK_FIFOTHR_EN` writer - 1: enable RXGCK fifo threshold 0: disable RXGCK fifo threshold (ignore fifo threshold) Should be 0 if MAC loopback mode"]
pub type BitRxgckFifothrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - FCS error packet threshold 3'd0: 1 packet 3'd1: 4 packets 3'd2: 8 packets 3'd3: 12 packets 3'd4: 16 packets 3'd5: 20 packets 3'd6: 24 packets 3'd7: 28 packets"]
    #[inline(always)]
    pub fn bit_rxpsf_errthr(&self) -> BitRxpsfErrthrR {
        BitRxpsfErrthrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This bit is set to 1 to enable MAC to inform BB enter power saving mode as rx FCS error packets number in a PPDU is larger than the FCS error packet threshold defined by BBPSF_ERRTHR. This function is independent from rate, type and IDs."]
    #[inline(always)]
    pub fn bit_rxpsf_all_errchken(&self) -> BitRxpsfAllErrchkenR {
        BitRxpsfAllErrchkenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is set to 1 to enable MAC to inform BB enter power saving mode as continuous rx FCS error packets number in a PPDU is large than the FCS error packet threshold defined by BBPSF_ERRTHR. This function is independent from rate, type and IDs."]
    #[inline(always)]
    pub fn bit_rxpsf_cont_errchken(&self) -> BitRxpsfContErrchkenR {
        BitRxpsfContErrchkenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set to 1 to enable MAC to inform BB enter power saving mode as rx unmatch my MACID unicast or unmatch my BSSID Broadcast/Mutlicast packets (including aggrigation and non-aggrigation)"]
    #[inline(always)]
    pub fn bit_rxpsf_mhchken(&self) -> BitRxpsfMhchkenR {
        BitRxpsfMhchkenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is set to 1 to reset MAC Rx before sending CCK packet triggered invalid_pkt pulse(0 for debug)"]
    #[inline(always)]
    pub fn bit_rxpsf_cckrst(&self) -> BitRxpsfCckrstR {
        BitRxpsfCckrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is set to 1 to reset MAC Rx before sending OFDM (include OFDM, HT, VHT) packet triggered invalid_pkt pulse (0 for debug)"]
    #[inline(always)]
    pub fn bit_rxpsf_ofdmrst(&self) -> BitRxpsfOfdmrstR {
        BitRxpsfOfdmrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = CCK"]
    #[inline(always)]
    pub fn bit_rxpsf_cckchken(&self) -> BitRxpsfCckchkenR {
        BitRxpsfCckchkenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = OFDM"]
    #[inline(always)]
    pub fn bit_rxpsf_ofdmchken(&self) -> BitRxpsfOfdmchkenR {
        BitRxpsfOfdmchkenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = HT"]
    #[inline(always)]
    pub fn bit_rxpsf_htchken(&self) -> BitRxpsfHtchkenR {
        BitRxpsfHtchkenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = VHT"]
    #[inline(always)]
    pub fn bit_rxpsf_vhtchken(&self) -> BitRxpsfVhtchkenR {
        BitRxpsfVhtchkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet Type=01."]
    #[inline(always)]
    pub fn bit_rxpsf_ctrlen(&self) -> BitRxpsfCtrlenR {
        BitRxpsfCtrlenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Packet length threshold 3'd0: 16 Bytes 3'd1: 32 Bytes 3'd2: 64 Bytes 3'd3: 128 Bytes 3'd4: 256 Bytes 3'd5: 512 Bytes 3'd6: 1024 Bytes 3'd7: 2048 Bytes Only the packet which packet length is longer than this threshold will trigger an invalid_pkt pulse."]
    #[inline(always)]
    pub fn bit_rxpsf_pktlenthr(&self) -> BitRxpsfPktlenthrR {
        BitRxpsfPktlenthrR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - This bit is set to 1 to let OFDMCCA be one of the signals that can wakeup MAC from Rx_idle"]
    #[inline(always)]
    pub fn bit_rxgck_ofdmcca_en(&self) -> BitRxgckOfdmccaEnR {
        BitRxgckOfdmccaEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - MAC_RX wait some time to gate clock after making the decision (about gating clock). Unit: T (MAC clock cycle) 0: 0 1: 2 2: 4 3: 8 4: 16 5: 32 6: 64 7: 128"]
    #[inline(always)]
    pub fn bit_rxgck_entry_delay(&self) -> BitRxgckEntryDelayR {
        BitRxgckEntryDelayR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21 - CCK rate RXGCK fifo threshold If CCKCCA = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 8, may cause error at some situation. 3: 15 (number of RDRDY before receiving MAC header), may cause error at some situation."]
    #[inline(always)]
    pub fn bit_rxgck_cck_fifothr(&self) -> BitRxgckCckFifothrR {
        BitRxgckCckFifothrR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - OFDM rate RXGCK fifo threshold If OFDM_VBON = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (number of RDRDY before receiving MAC header = minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 2, may cause error at some situation. 3: 3 (minimum RDRDY number within one packet), may cause error at some situation."]
    #[inline(always)]
    pub fn bit_rxgck_ofdm_fifothr(&self) -> BitRxgckOfdmFifothrR {
        BitRxgckOfdmFifothrR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - HT rate RXGCK fifo threshold If RXHT = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 2 (number of RDRDY before receiving MAC header - 1), may cause error at some situation. 3: 3 (number of RDRDY before receiving MAC header), may cause error at some situation."]
    #[inline(always)]
    pub fn bit_rxgck_ht_fifothr(&self) -> BitRxgckHtFifothrR {
        BitRxgckHtFifothrR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - VHT rate RXGCK fifo threshold If RXVHT = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 3 (number of RDRDY before receiving MAC header - 1), may cause error at some situation. 3: 4 (number of RDRDY before receiving MAC header), may cause error at some situation."]
    #[inline(always)]
    pub fn bit_rxgck_vht_fifothr(&self) -> BitRxgckVhtFifothrR {
        BitRxgckVhtFifothrR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - 1: enable RXGCK fifo threshold 0: disable RXGCK fifo threshold (ignore fifo threshold) Should be 0 if MAC loopback mode"]
    #[inline(always)]
    pub fn bit_rxgck_fifothr_en(&self) -> BitRxgckFifothrEnR {
        BitRxgckFifothrEnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FCS error packet threshold 3'd0: 1 packet 3'd1: 4 packets 3'd2: 8 packets 3'd3: 12 packets 3'd4: 16 packets 3'd5: 20 packets 3'd6: 24 packets 3'd7: 28 packets"]
    #[inline(always)]
    pub fn bit_rxpsf_errthr(&mut self) -> BitRxpsfErrthrW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfErrthrW::new(self, 0)
    }
    #[doc = "Bit 3 - This bit is set to 1 to enable MAC to inform BB enter power saving mode as rx FCS error packets number in a PPDU is larger than the FCS error packet threshold defined by BBPSF_ERRTHR. This function is independent from rate, type and IDs."]
    #[inline(always)]
    pub fn bit_rxpsf_all_errchken(&mut self) -> BitRxpsfAllErrchkenW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfAllErrchkenW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is set to 1 to enable MAC to inform BB enter power saving mode as continuous rx FCS error packets number in a PPDU is large than the FCS error packet threshold defined by BBPSF_ERRTHR. This function is independent from rate, type and IDs."]
    #[inline(always)]
    pub fn bit_rxpsf_cont_errchken(&mut self) -> BitRxpsfContErrchkenW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfContErrchkenW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is set to 1 to enable MAC to inform BB enter power saving mode as rx unmatch my MACID unicast or unmatch my BSSID Broadcast/Mutlicast packets (including aggrigation and non-aggrigation)"]
    #[inline(always)]
    pub fn bit_rxpsf_mhchken(&mut self) -> BitRxpsfMhchkenW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfMhchkenW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is set to 1 to reset MAC Rx before sending CCK packet triggered invalid_pkt pulse(0 for debug)"]
    #[inline(always)]
    pub fn bit_rxpsf_cckrst(&mut self) -> BitRxpsfCckrstW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfCckrstW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit is set to 1 to reset MAC Rx before sending OFDM (include OFDM, HT, VHT) packet triggered invalid_pkt pulse (0 for debug)"]
    #[inline(always)]
    pub fn bit_rxpsf_ofdmrst(&mut self) -> BitRxpsfOfdmrstW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfOfdmrstW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = CCK"]
    #[inline(always)]
    pub fn bit_rxpsf_cckchken(&mut self) -> BitRxpsfCckchkenW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfCckchkenW::new(self, 8)
    }
    #[doc = "Bit 9 - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = OFDM"]
    #[inline(always)]
    pub fn bit_rxpsf_ofdmchken(&mut self) -> BitRxpsfOfdmchkenW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfOfdmchkenW::new(self, 9)
    }
    #[doc = "Bit 10 - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = HT"]
    #[inline(always)]
    pub fn bit_rxpsf_htchken(&mut self) -> BitRxpsfHtchkenW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfHtchkenW::new(self, 10)
    }
    #[doc = "Bit 11 - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet rate = VHT"]
    #[inline(always)]
    pub fn bit_rxpsf_vhtchken(&mut self) -> BitRxpsfVhtchkenW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfVhtchkenW::new(self, 11)
    }
    #[doc = "Bit 12 - This bit is set to 1 to enable MAC to consider trigging invalid_pkt as rx packet Type=01."]
    #[inline(always)]
    pub fn bit_rxpsf_ctrlen(&mut self) -> BitRxpsfCtrlenW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfCtrlenW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Packet length threshold 3'd0: 16 Bytes 3'd1: 32 Bytes 3'd2: 64 Bytes 3'd3: 128 Bytes 3'd4: 256 Bytes 3'd5: 512 Bytes 3'd6: 1024 Bytes 3'd7: 2048 Bytes Only the packet which packet length is longer than this threshold will trigger an invalid_pkt pulse."]
    #[inline(always)]
    pub fn bit_rxpsf_pktlenthr(&mut self) -> BitRxpsfPktlenthrW<'_, RegRxpsfCtrlSpec> {
        BitRxpsfPktlenthrW::new(self, 13)
    }
    #[doc = "Bit 16 - This bit is set to 1 to let OFDMCCA be one of the signals that can wakeup MAC from Rx_idle"]
    #[inline(always)]
    pub fn bit_rxgck_ofdmcca_en(&mut self) -> BitRxgckOfdmccaEnW<'_, RegRxpsfCtrlSpec> {
        BitRxgckOfdmccaEnW::new(self, 16)
    }
    #[doc = "Bits 17:19 - MAC_RX wait some time to gate clock after making the decision (about gating clock). Unit: T (MAC clock cycle) 0: 0 1: 2 2: 4 3: 8 4: 16 5: 32 6: 64 7: 128"]
    #[inline(always)]
    pub fn bit_rxgck_entry_delay(&mut self) -> BitRxgckEntryDelayW<'_, RegRxpsfCtrlSpec> {
        BitRxgckEntryDelayW::new(self, 17)
    }
    #[doc = "Bits 20:21 - CCK rate RXGCK fifo threshold If CCKCCA = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 8, may cause error at some situation. 3: 15 (number of RDRDY before receiving MAC header), may cause error at some situation."]
    #[inline(always)]
    pub fn bit_rxgck_cck_fifothr(&mut self) -> BitRxgckCckFifothrW<'_, RegRxpsfCtrlSpec> {
        BitRxgckCckFifothrW::new(self, 20)
    }
    #[doc = "Bits 22:23 - OFDM rate RXGCK fifo threshold If OFDM_VBON = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (number of RDRDY before receiving MAC header = minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 2, may cause error at some situation. 3: 3 (minimum RDRDY number within one packet), may cause error at some situation."]
    #[inline(always)]
    pub fn bit_rxgck_ofdm_fifothr(&mut self) -> BitRxgckOfdmFifothrW<'_, RegRxpsfCtrlSpec> {
        BitRxgckOfdmFifothrW::new(self, 22)
    }
    #[doc = "Bits 24:25 - HT rate RXGCK fifo threshold If RXHT = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 2 (number of RDRDY before receiving MAC header - 1), may cause error at some situation. 3: 3 (number of RDRDY before receiving MAC header), may cause error at some situation."]
    #[inline(always)]
    pub fn bit_rxgck_ht_fifothr(&mut self) -> BitRxgckHtFifothrW<'_, RegRxpsfCtrlSpec> {
        BitRxgckHtFifothrW::new(self, 24)
    }
    #[doc = "Bits 26:27 - VHT rate RXGCK fifo threshold If RXVHT = 1 and RXGCK fifo write pointer >= this threshold, MAC_RX clock will stop gating. 0: 0 1: 1 (minimum RDRDY threshold for avoiding false alarm), can work at all situation. 2: 3 (number of RDRDY before receiving MAC header - 1), may cause error at some situation. 3: 4 (number of RDRDY before receiving MAC header), may cause error at some situation."]
    #[inline(always)]
    pub fn bit_rxgck_vht_fifothr(&mut self) -> BitRxgckVhtFifothrW<'_, RegRxpsfCtrlSpec> {
        BitRxgckVhtFifothrW::new(self, 26)
    }
    #[doc = "Bit 28 - 1: enable RXGCK fifo threshold 0: disable RXGCK fifo threshold (ignore fifo threshold) Should be 0 if MAC loopback mode"]
    #[inline(always)]
    pub fn bit_rxgck_fifothr_en(&mut self) -> BitRxgckFifothrEnW<'_, RegRxpsfCtrlSpec> {
        BitRxgckFifothrEnW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxpsf_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxpsf_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxpsfCtrlSpec;
impl crate::RegisterSpec for RegRxpsfCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxpsf_ctrl::R`](R) reader structure"]
impl crate::Readable for RegRxpsfCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxpsf_ctrl::W`](W) writer structure"]
impl crate::Writable for RegRxpsfCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXPSF_CTRL to value 0x0004_00c0"]
impl crate::Resettable for RegRxpsfCtrlSpec {
    const RESET_VALUE: u32 = 0x0004_00c0;
}
