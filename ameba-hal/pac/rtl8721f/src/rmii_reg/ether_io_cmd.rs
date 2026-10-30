#[doc = "Register `ETHER_IO_CMD` reader"]
pub type R = crate::R<EtherIoCmdSpec>;
#[doc = "Register `ETHER_IO_CMD` writer"]
pub type W = crate::W<EtherIoCmdSpec>;
#[doc = "Field `TxFN1st` writer - 1st Priority DMA-Ethernet Transmit enable. 1: Enable. 0: Disable."]
pub type TxFn1stW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFN2nd` writer - 2nd Priority DMA-Ethernet Transmit enable. 1: Enable. 0: Disable."]
pub type TxFn2ndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFN3rd` writer - 3rd Priority DMA-Ethernet Transmit enable. 1: Enable. 0: Disable."]
pub type TxFn3rdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFN4th` writer - 4th Priority DMA-Ethernet Transmit enable. 1: Enable. 0: Disable."]
pub type TxFn4thW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - MII Tx Enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - MII Tx Enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - MII Rx Enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - MII Rx Enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_timer_sel` reader - RXPktTimer, TXPktTimer Unit. (TU) speed TU 00: 10M 25.6uS 00: 100M 2.56uS 00: 1000M 256nS 01: 10M 102.4uS 01: 100M 10.24uS 01: 1000M 1.024uS 10: 10M 409.6uS 10: 100M 40.96uS 10: 1000M 4.096uS 11: 10M 1.6mS 11: 100M 0.16mS 11: 1000M 16uS"]
pub type RegIntTimerSelR = crate::FieldReader;
#[doc = "Field `reg_int_timer_sel` writer - RXPktTimer, TXPktTimer Unit. (TU) speed TU 00: 10M 25.6uS 00: 100M 2.56uS 00: 1000M 256nS 01: 10M 102.4uS 01: 100M 10.24uS 01: 1000M 1.024uS 10: 10M 409.6uS 10: 100M 40.96uS 10: 1000M 4.096uS 11: 10M 1.6mS 11: 100M 0.16mS 11: 1000M 16uS"]
pub type RegIntTimerSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RxIntMitigation_2_0` reader - This sets the number of packets received before RxOK interrupt is triggered. 0000- 1 pkt 0001- 4 pkts 0010- 8 pkts 0011- 12pkts 0100- 16 pkts 0101- 20 pkts 0110- 24 pkts 0111- 28 pkts ..."]
pub type RxIntMitigation2_0R = crate::FieldReader;
#[doc = "Field `RxIntMitigation_2_0` writer - This sets the number of packets received before RxOK interrupt is triggered. 0000- 1 pkt 0001- 4 pkts 0010- 8 pkts 0011- 12pkts 0100- 16 pkts 0101- 20 pkts 0110- 24 pkts 0111- 28 pkts ..."]
pub type RxIntMitigation2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXFTH` reader - Rx Threshold: Specifies the threshold level in the Rx FIFO to begin the transmission. When the byte count of the data in the Rx FIFO reaches this level, (or the FIFO contains at least one complete packet or the end of a packet) the Ethernet module will transmit this packet. 00: 1024B 01: 128B 10: 256B 11: 512B The rx threshold should be larger than maximum dma burst size. The combination of max_dma_sel and rxfth are listed as followed: max_dma_sel rxfth b00(64B) b00(1024B), b01 (128B), b10(256B), b11(512B). b01(128B) b00(1024B), b10(256B), b11(512B). b10(256B) b00(1024B), b11(512B), In rle0516: SW should not set rxfth to 0b00 due to the rx fifo size is only 512B. The recommend value of rle0516 is 0b11 for performance concerns. 00 = no rx threshold. The Ethernet Module begins the transfer of data after having received a whole packet in the FIFO. 01 = 32 bytes 10 = 64 bytes 11 = 128 bytes max_dma_sel rxfth b0(64B) b10(64B), b11(128B). b1(128B) b11(128B)."]
pub type RxfthR = crate::FieldReader;
#[doc = "Field `RXFTH` writer - Rx Threshold: Specifies the threshold level in the Rx FIFO to begin the transmission. When the byte count of the data in the Rx FIFO reaches this level, (or the FIFO contains at least one complete packet or the end of a packet) the Ethernet module will transmit this packet. 00: 1024B 01: 128B 10: 256B 11: 512B The rx threshold should be larger than maximum dma burst size. The combination of max_dma_sel and rxfth are listed as followed: max_dma_sel rxfth b00(64B) b00(1024B), b01 (128B), b10(256B), b11(512B). b01(128B) b00(1024B), b10(256B), b11(512B). b10(256B) b00(1024B), b11(512B), In rle0516: SW should not set rxfth to 0b00 due to the rx fifo size is only 512B. The recommend value of rle0516 is 0b11 for performance concerns. 00 = no rx threshold. The Ethernet Module begins the transfer of data after having received a whole packet in the FIFO. 01 = 32 bytes 10 = 64 bytes 11 = 128 bytes max_dma_sel rxfth b0(64B) b10(64B), b11(128B). b1(128B) b11(128B)."]
pub type RxfthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXPktTimer_2_0` reader - Timer to trigger RxOK interrupt before receipt of RxIntMitigation pkts. 0000 - no timer set 0001 ~ 1111 : the timer interval defining a multiple of TU ex: 011 = timer interval set to 3 x4 =12TU This only applies to packets of size larger than 128 bytes. Once RxOK is asserted the timer mechanism is reinitialized. For ingress pkt which is short than 128B, RxOK interrupt asserts after DMA completes."]
pub type RxpktTimer2_0R = crate::FieldReader;
#[doc = "Field `RXPktTimer_2_0` writer - Timer to trigger RxOK interrupt before receipt of RxIntMitigation pkts. 0000 - no timer set 0001 ~ 1111 : the timer interval defining a multiple of TU ex: 011 = timer interval set to 3 x4 =12TU This only applies to packets of size larger than 128 bytes. Once RxOK is asserted the timer mechanism is reinitialized. For ingress pkt which is short than 128B, RxOK interrupt asserts after DMA completes."]
pub type RxpktTimer2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TxIntMitigation_2_0` reader - This sets the number of packets received before TxOK interrupt is triggered. 0000- 1 pkt 0001- 4 pkts 0010- 8 pkts 0011- 12 pkts 0100- 16 pkts 0101- 20 pkts 0110- 24 pkts 0111- 28 pkts ..."]
pub type TxIntMitigation2_0R = crate::FieldReader;
#[doc = "Field `TxIntMitigation_2_0` writer - This sets the number of packets received before TxOK interrupt is triggered. 0000- 1 pkt 0001- 4 pkts 0010- 8 pkts 0011- 12 pkts 0100- 16 pkts 0101- 20 pkts 0110- 24 pkts 0111- 28 pkts ..."]
pub type TxIntMitigation2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tsh` reader - Tx Threshold: Specifies the threshold level in the Tx FIFO to begin the transmission. When the byte count of the data in the Tx FIFO reaches this level, (or the FIFO contains at least one complete packet or the end of a packet) the Ethernet module will transmit this packet. 00:128B. 01:256B. 10:512B. 11:1024B. For poor lxbus bandwidth, tsh should be set b11. In rle0516, this field should be setted 0 or 1 (128B)."]
pub type TshR = crate::FieldReader;
#[doc = "Field `tsh` writer - Tx Threshold: Specifies the threshold level in the Tx FIFO to begin the transmission. When the byte count of the data in the Tx FIFO reaches this level, (or the FIFO contains at least one complete packet or the end of a packet) the Ethernet module will transmit this packet. 00:128B. 01:256B. 10:512B. 11:1024B. For poor lxbus bandwidth, tsh should be set b11. In rle0516, this field should be setted 0 or 1 (128B)."]
pub type TshW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RxIntMitigation_3` reader - see Ethernet_IO_CMD.bit10-8"]
pub type RxIntMitigation3R = crate::BitReader;
#[doc = "Field `RxIntMitigation_3` writer - see Ethernet_IO_CMD.bit10-8"]
pub type RxIntMitigation3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPktTimer_3` reader - see Ethernet_IO_CMD.bit15-13"]
pub type RxpktTimer3R = crate::BitReader;
#[doc = "Field `RXPktTimer_3` writer - see Ethernet_IO_CMD.bit15-13"]
pub type RxpktTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxIntMitigation_3` reader - see Ethernet_IO_CMD.bit18-16"]
pub type TxIntMitigation3R = crate::BitReader;
#[doc = "Field `TxIntMitigation_3` writer - see Ethernet_IO_CMD.bit18-16"]
pub type TxIntMitigation3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPktTimer` reader - Timer to trigger TxOK interrupt before receipt of TxIntMitigation pkts. 0000 - no timer set 0001 ~ 1111 : the timer interval defining a multiple of TU ex: 0011 = timer interval set to 3 x4 =12TU Once TxOK is asserted the timer mechanism is reinitialized."]
pub type TxpktTimerR = crate::FieldReader;
#[doc = "Field `TXPktTimer` writer - Timer to trigger TxOK interrupt before receipt of TxIntMitigation pkts. 0000 - no timer set 0001 ~ 1111 : the timer interval defining a multiple of TU ex: 0011 = timer interval set to 3 x4 =12TU Once TxOK is asserted the timer mechanism is reinitialized."]
pub type TxpktTimerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `en_early_tx` reader - default 0, 0: disable, 1: enable. Disable early tx by GAMC while tx command descriptor.IPCS, UDPCS or TCPCS are set to high. For multiple tx descriptor application, this field should be 0. For low lxbus throughput environment, this field should be 0. For slow clk_gmac application, this field should be 0."]
pub type EnEarlyTxR = crate::BitReader;
#[doc = "Field `en_early_tx` writer - default 0, 0: disable, 1: enable. Disable early tx by GAMC while tx command descriptor.IPCS, UDPCS or TCPCS are set to high. For multiple tx descriptor application, this field should be 0. For low lxbus throughput environment, this field should be 0. For slow clk_gmac application, this field should be 0."]
pub type EnEarlyTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `max_dma_sel_1` reader - see Ethernet_IO_CMD.bit31 AmebaI/AmebaPro/AmebaPro2 only support DMA burst length = 16DW"]
pub type MaxDmaSel1R = crate::BitReader;
#[doc = "Field `max_dma_sel_1` writer - see Ethernet_IO_CMD.bit31 AmebaI/AmebaPro/AmebaPro2 only support DMA burst length = 16DW"]
pub type MaxDmaSel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ShortDesFormat` reader - Short descriptor format. Set 1 tx/rx descriptor will use old format - 4x32bit each item, set 0 use new format to support sram mapping AmebaI/AmebaPro/AmebaPro2 only support old format (1b'1)"]
pub type ShortDesFormatR = crate::BitReader;
#[doc = "Field `ShortDesFormat` writer - Short descriptor format. Set 1 tx/rx descriptor will use old format - 4x32bit each item, set 0 use new format to support sram mapping AmebaI/AmebaPro/AmebaPro2 only support old format (1b'1)"]
pub type ShortDesFormatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `max_dma_sel_0` reader - Select the dma burst size on bus(memory controller should assert continuous btrdy). 00:16 DW(1DW=4B) 01:32 DW 10:64 DW 11:128DW. AmebaI/AmebaPro/AmebaPro2 only support DMA burst length = 16DW"]
pub type MaxDmaSel0R = crate::BitReader;
#[doc = "Field `max_dma_sel_0` writer - Select the dma burst size on bus(memory controller should assert continuous btrdy). 00:16 DW(1DW=4B) 01:32 DW 10:64 DW 11:128DW. AmebaI/AmebaPro/AmebaPro2 only support DMA burst length = 16DW"]
pub type MaxDmaSel0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - MII Tx Enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MII Rx Enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - RXPktTimer, TXPktTimer Unit. (TU) speed TU 00: 10M 25.6uS 00: 100M 2.56uS 00: 1000M 256nS 01: 10M 102.4uS 01: 100M 10.24uS 01: 1000M 1.024uS 10: 10M 409.6uS 10: 100M 40.96uS 10: 1000M 4.096uS 11: 10M 1.6mS 11: 100M 0.16mS 11: 1000M 16uS"]
    #[inline(always)]
    pub fn reg_int_timer_sel(&self) -> RegIntTimerSelR {
        RegIntTimerSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - This sets the number of packets received before RxOK interrupt is triggered. 0000- 1 pkt 0001- 4 pkts 0010- 8 pkts 0011- 12pkts 0100- 16 pkts 0101- 20 pkts 0110- 24 pkts 0111- 28 pkts ..."]
    #[inline(always)]
    pub fn rx_int_mitigation_2_0(&self) -> RxIntMitigation2_0R {
        RxIntMitigation2_0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Rx Threshold: Specifies the threshold level in the Rx FIFO to begin the transmission. When the byte count of the data in the Rx FIFO reaches this level, (or the FIFO contains at least one complete packet or the end of a packet) the Ethernet module will transmit this packet. 00: 1024B 01: 128B 10: 256B 11: 512B The rx threshold should be larger than maximum dma burst size. The combination of max_dma_sel and rxfth are listed as followed: max_dma_sel rxfth b00(64B) b00(1024B), b01 (128B), b10(256B), b11(512B). b01(128B) b00(1024B), b10(256B), b11(512B). b10(256B) b00(1024B), b11(512B), In rle0516: SW should not set rxfth to 0b00 due to the rx fifo size is only 512B. The recommend value of rle0516 is 0b11 for performance concerns. 00 = no rx threshold. The Ethernet Module begins the transfer of data after having received a whole packet in the FIFO. 01 = 32 bytes 10 = 64 bytes 11 = 128 bytes max_dma_sel rxfth b0(64B) b10(64B), b11(128B). b1(128B) b11(128B)."]
    #[inline(always)]
    pub fn rxfth(&self) -> RxfthR {
        RxfthR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - Timer to trigger RxOK interrupt before receipt of RxIntMitigation pkts. 0000 - no timer set 0001 ~ 1111 : the timer interval defining a multiple of TU ex: 011 = timer interval set to 3 x4 =12TU This only applies to packets of size larger than 128 bytes. Once RxOK is asserted the timer mechanism is reinitialized. For ingress pkt which is short than 128B, RxOK interrupt asserts after DMA completes."]
    #[inline(always)]
    pub fn rxpkt_timer_2_0(&self) -> RxpktTimer2_0R {
        RxpktTimer2_0R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - This sets the number of packets received before TxOK interrupt is triggered. 0000- 1 pkt 0001- 4 pkts 0010- 8 pkts 0011- 12 pkts 0100- 16 pkts 0101- 20 pkts 0110- 24 pkts 0111- 28 pkts ..."]
    #[inline(always)]
    pub fn tx_int_mitigation_2_0(&self) -> TxIntMitigation2_0R {
        TxIntMitigation2_0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - Tx Threshold: Specifies the threshold level in the Tx FIFO to begin the transmission. When the byte count of the data in the Tx FIFO reaches this level, (or the FIFO contains at least one complete packet or the end of a packet) the Ethernet module will transmit this packet. 00:128B. 01:256B. 10:512B. 11:1024B. For poor lxbus bandwidth, tsh should be set b11. In rle0516, this field should be setted 0 or 1 (128B)."]
    #[inline(always)]
    pub fn tsh(&self) -> TshR {
        TshR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - see Ethernet_IO_CMD.bit10-8"]
    #[inline(always)]
    pub fn rx_int_mitigation_3(&self) -> RxIntMitigation3R {
        RxIntMitigation3R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - see Ethernet_IO_CMD.bit15-13"]
    #[inline(always)]
    pub fn rxpkt_timer_3(&self) -> RxpktTimer3R {
        RxpktTimer3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - see Ethernet_IO_CMD.bit18-16"]
    #[inline(always)]
    pub fn tx_int_mitigation_3(&self) -> TxIntMitigation3R {
        TxIntMitigation3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Timer to trigger TxOK interrupt before receipt of TxIntMitigation pkts. 0000 - no timer set 0001 ~ 1111 : the timer interval defining a multiple of TU ex: 0011 = timer interval set to 3 x4 =12TU Once TxOK is asserted the timer mechanism is reinitialized."]
    #[inline(always)]
    pub fn txpkt_timer(&self) -> TxpktTimerR {
        TxpktTimerR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - default 0, 0: disable, 1: enable. Disable early tx by GAMC while tx command descriptor.IPCS, UDPCS or TCPCS are set to high. For multiple tx descriptor application, this field should be 0. For low lxbus throughput environment, this field should be 0. For slow clk_gmac application, this field should be 0."]
    #[inline(always)]
    pub fn en_early_tx(&self) -> EnEarlyTxR {
        EnEarlyTxR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - see Ethernet_IO_CMD.bit31 AmebaI/AmebaPro/AmebaPro2 only support DMA burst length = 16DW"]
    #[inline(always)]
    pub fn max_dma_sel_1(&self) -> MaxDmaSel1R {
        MaxDmaSel1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Short descriptor format. Set 1 tx/rx descriptor will use old format - 4x32bit each item, set 0 use new format to support sram mapping AmebaI/AmebaPro/AmebaPro2 only support old format (1b'1)"]
    #[inline(always)]
    pub fn short_des_format(&self) -> ShortDesFormatR {
        ShortDesFormatR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Select the dma burst size on bus(memory controller should assert continuous btrdy). 00:16 DW(1DW=4B) 01:32 DW 10:64 DW 11:128DW. AmebaI/AmebaPro/AmebaPro2 only support DMA burst length = 16DW"]
    #[inline(always)]
    pub fn max_dma_sel_0(&self) -> MaxDmaSel0R {
        MaxDmaSel0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Priority DMA-Ethernet Transmit enable. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn tx_fn1st(&mut self) -> TxFn1stW<'_, EtherIoCmdSpec> {
        TxFn1stW::new(self, 0)
    }
    #[doc = "Bit 1 - 2nd Priority DMA-Ethernet Transmit enable. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn tx_fn2nd(&mut self) -> TxFn2ndW<'_, EtherIoCmdSpec> {
        TxFn2ndW::new(self, 1)
    }
    #[doc = "Bit 2 - 3rd Priority DMA-Ethernet Transmit enable. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn tx_fn3rd(&mut self) -> TxFn3rdW<'_, EtherIoCmdSpec> {
        TxFn3rdW::new(self, 2)
    }
    #[doc = "Bit 3 - 4th Priority DMA-Ethernet Transmit enable. 1: Enable. 0: Disable."]
    #[inline(always)]
    pub fn tx_fn4th(&mut self) -> TxFn4thW<'_, EtherIoCmdSpec> {
        TxFn4thW::new(self, 3)
    }
    #[doc = "Bit 4 - MII Tx Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<'_, EtherIoCmdSpec> {
        TeW::new(self, 4)
    }
    #[doc = "Bit 5 - MII Rx Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<'_, EtherIoCmdSpec> {
        ReW::new(self, 5)
    }
    #[doc = "Bits 6:7 - RXPktTimer, TXPktTimer Unit. (TU) speed TU 00: 10M 25.6uS 00: 100M 2.56uS 00: 1000M 256nS 01: 10M 102.4uS 01: 100M 10.24uS 01: 1000M 1.024uS 10: 10M 409.6uS 10: 100M 40.96uS 10: 1000M 4.096uS 11: 10M 1.6mS 11: 100M 0.16mS 11: 1000M 16uS"]
    #[inline(always)]
    pub fn reg_int_timer_sel(&mut self) -> RegIntTimerSelW<'_, EtherIoCmdSpec> {
        RegIntTimerSelW::new(self, 6)
    }
    #[doc = "Bits 8:10 - This sets the number of packets received before RxOK interrupt is triggered. 0000- 1 pkt 0001- 4 pkts 0010- 8 pkts 0011- 12pkts 0100- 16 pkts 0101- 20 pkts 0110- 24 pkts 0111- 28 pkts ..."]
    #[inline(always)]
    pub fn rx_int_mitigation_2_0(&mut self) -> RxIntMitigation2_0W<'_, EtherIoCmdSpec> {
        RxIntMitigation2_0W::new(self, 8)
    }
    #[doc = "Bits 11:12 - Rx Threshold: Specifies the threshold level in the Rx FIFO to begin the transmission. When the byte count of the data in the Rx FIFO reaches this level, (or the FIFO contains at least one complete packet or the end of a packet) the Ethernet module will transmit this packet. 00: 1024B 01: 128B 10: 256B 11: 512B The rx threshold should be larger than maximum dma burst size. The combination of max_dma_sel and rxfth are listed as followed: max_dma_sel rxfth b00(64B) b00(1024B), b01 (128B), b10(256B), b11(512B). b01(128B) b00(1024B), b10(256B), b11(512B). b10(256B) b00(1024B), b11(512B), In rle0516: SW should not set rxfth to 0b00 due to the rx fifo size is only 512B. The recommend value of rle0516 is 0b11 for performance concerns. 00 = no rx threshold. The Ethernet Module begins the transfer of data after having received a whole packet in the FIFO. 01 = 32 bytes 10 = 64 bytes 11 = 128 bytes max_dma_sel rxfth b0(64B) b10(64B), b11(128B). b1(128B) b11(128B)."]
    #[inline(always)]
    pub fn rxfth(&mut self) -> RxfthW<'_, EtherIoCmdSpec> {
        RxfthW::new(self, 11)
    }
    #[doc = "Bits 13:15 - Timer to trigger RxOK interrupt before receipt of RxIntMitigation pkts. 0000 - no timer set 0001 ~ 1111 : the timer interval defining a multiple of TU ex: 011 = timer interval set to 3 x4 =12TU This only applies to packets of size larger than 128 bytes. Once RxOK is asserted the timer mechanism is reinitialized. For ingress pkt which is short than 128B, RxOK interrupt asserts after DMA completes."]
    #[inline(always)]
    pub fn rxpkt_timer_2_0(&mut self) -> RxpktTimer2_0W<'_, EtherIoCmdSpec> {
        RxpktTimer2_0W::new(self, 13)
    }
    #[doc = "Bits 16:18 - This sets the number of packets received before TxOK interrupt is triggered. 0000- 1 pkt 0001- 4 pkts 0010- 8 pkts 0011- 12 pkts 0100- 16 pkts 0101- 20 pkts 0110- 24 pkts 0111- 28 pkts ..."]
    #[inline(always)]
    pub fn tx_int_mitigation_2_0(&mut self) -> TxIntMitigation2_0W<'_, EtherIoCmdSpec> {
        TxIntMitigation2_0W::new(self, 16)
    }
    #[doc = "Bits 19:20 - Tx Threshold: Specifies the threshold level in the Tx FIFO to begin the transmission. When the byte count of the data in the Tx FIFO reaches this level, (or the FIFO contains at least one complete packet or the end of a packet) the Ethernet module will transmit this packet. 00:128B. 01:256B. 10:512B. 11:1024B. For poor lxbus bandwidth, tsh should be set b11. In rle0516, this field should be setted 0 or 1 (128B)."]
    #[inline(always)]
    pub fn tsh(&mut self) -> TshW<'_, EtherIoCmdSpec> {
        TshW::new(self, 19)
    }
    #[doc = "Bit 21 - see Ethernet_IO_CMD.bit10-8"]
    #[inline(always)]
    pub fn rx_int_mitigation_3(&mut self) -> RxIntMitigation3W<'_, EtherIoCmdSpec> {
        RxIntMitigation3W::new(self, 21)
    }
    #[doc = "Bit 22 - see Ethernet_IO_CMD.bit15-13"]
    #[inline(always)]
    pub fn rxpkt_timer_3(&mut self) -> RxpktTimer3W<'_, EtherIoCmdSpec> {
        RxpktTimer3W::new(self, 22)
    }
    #[doc = "Bit 23 - see Ethernet_IO_CMD.bit18-16"]
    #[inline(always)]
    pub fn tx_int_mitigation_3(&mut self) -> TxIntMitigation3W<'_, EtherIoCmdSpec> {
        TxIntMitigation3W::new(self, 23)
    }
    #[doc = "Bits 24:27 - Timer to trigger TxOK interrupt before receipt of TxIntMitigation pkts. 0000 - no timer set 0001 ~ 1111 : the timer interval defining a multiple of TU ex: 0011 = timer interval set to 3 x4 =12TU Once TxOK is asserted the timer mechanism is reinitialized."]
    #[inline(always)]
    pub fn txpkt_timer(&mut self) -> TxpktTimerW<'_, EtherIoCmdSpec> {
        TxpktTimerW::new(self, 24)
    }
    #[doc = "Bit 28 - default 0, 0: disable, 1: enable. Disable early tx by GAMC while tx command descriptor.IPCS, UDPCS or TCPCS are set to high. For multiple tx descriptor application, this field should be 0. For low lxbus throughput environment, this field should be 0. For slow clk_gmac application, this field should be 0."]
    #[inline(always)]
    pub fn en_early_tx(&mut self) -> EnEarlyTxW<'_, EtherIoCmdSpec> {
        EnEarlyTxW::new(self, 28)
    }
    #[doc = "Bit 29 - see Ethernet_IO_CMD.bit31 AmebaI/AmebaPro/AmebaPro2 only support DMA burst length = 16DW"]
    #[inline(always)]
    pub fn max_dma_sel_1(&mut self) -> MaxDmaSel1W<'_, EtherIoCmdSpec> {
        MaxDmaSel1W::new(self, 29)
    }
    #[doc = "Bit 30 - Short descriptor format. Set 1 tx/rx descriptor will use old format - 4x32bit each item, set 0 use new format to support sram mapping AmebaI/AmebaPro/AmebaPro2 only support old format (1b'1)"]
    #[inline(always)]
    pub fn short_des_format(&mut self) -> ShortDesFormatW<'_, EtherIoCmdSpec> {
        ShortDesFormatW::new(self, 30)
    }
    #[doc = "Bit 31 - Select the dma burst size on bus(memory controller should assert continuous btrdy). 00:16 DW(1DW=4B) 01:32 DW 10:64 DW 11:128DW. AmebaI/AmebaPro/AmebaPro2 only support DMA burst length = 16DW"]
    #[inline(always)]
    pub fn max_dma_sel_0(&mut self) -> MaxDmaSel0W<'_, EtherIoCmdSpec> {
        MaxDmaSel0W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ether_io_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ether_io_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtherIoCmdSpec;
impl crate::RegisterSpec for EtherIoCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ether_io_cmd::R`](R) reader structure"]
impl crate::Readable for EtherIoCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`ether_io_cmd::W`](W) writer structure"]
impl crate::Writable for EtherIoCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETHER_IO_CMD to value 0"]
impl crate::Resettable for EtherIoCmdSpec {}
