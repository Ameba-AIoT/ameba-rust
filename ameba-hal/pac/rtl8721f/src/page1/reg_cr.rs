#[doc = "Register `REG_CR` reader"]
pub type R = crate::R<RegCrSpec>;
#[doc = "Register `REG_CR` writer"]
pub type W = crate::W<RegCrSpec>;
#[doc = "Field `BIT_HCI_TXDMA_EN` reader - HCI to TXDMA Interface Enable"]
pub type BitHciTxdmaEnR = crate::BitReader;
#[doc = "Field `BIT_HCI_TXDMA_EN` writer - HCI to TXDMA Interface Enable"]
pub type BitHciTxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HCI_RXDMA_EN` reader - HCI to RXDMA Interface Enable"]
pub type BitHciRxdmaEnR = crate::BitReader;
#[doc = "Field `BIT_HCI_RXDMA_EN` writer - HCI to RXDMA Interface Enable"]
pub type BitHciRxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXDMA_EN` reader - TXDMA Function Enable"]
pub type BitTxdmaEnR = crate::BitReader;
#[doc = "Field `BIT_TXDMA_EN` writer - TXDMA Function Enable"]
pub type BitTxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDMA_EN` reader - RXDMA Function Enable"]
pub type BitRxdmaEnR = crate::BitReader;
#[doc = "Field `BIT_RXDMA_EN` writer - RXDMA Function Enable"]
pub type BitRxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PROTOCOL_EN` reader - protocol Block Function Enable.(FW2HW)"]
pub type BitProtocolEnR = crate::BitReader;
#[doc = "Field `BIT_PROTOCOL_EN` writer - protocol Block Function Enable.(FW2HW)"]
pub type BitProtocolEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SCHEDULE_EN` reader - Schedule Enable."]
pub type BitScheduleEnR = crate::BitReader;
#[doc = "Field `BIT_SCHEDULE_EN` writer - Schedule Enable."]
pub type BitScheduleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MACTXEN` reader - MAC Transmitter Enable. When set to 1, and the transmit state machine is idle, the transmit state machine becomes active. This bit will read back as 1 whenever the transmit state machine is active. After initial power-up, software must ensure that the transmitter has completely reset before setting this bit. 1: Enable 0: Disable Default: 0b."]
pub type BitMactxenR = crate::BitReader;
#[doc = "Field `BIT_MACTXEN` writer - MAC Transmitter Enable. When set to 1, and the transmit state machine is idle, the transmit state machine becomes active. This bit will read back as 1 whenever the transmit state machine is active. After initial power-up, software must ensure that the transmitter has completely reset before setting this bit. 1: Enable 0: Disable Default: 0b."]
pub type BitMactxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MACRXEN` reader - MAC Receiver Enable. When set to 1, and the receive state machine is idle, the receive machine becomes active. This bit will read back as 1 whenever the receive state machine is active. After initial power-up, software must ensure that the receiver has completely reset before setting this bit. 1: Enable 0: Disable Default: 0b."]
pub type BitMacrxenR = crate::BitReader;
#[doc = "Field `BIT_MACRXEN` writer - MAC Receiver Enable. When set to 1, and the receive state machine is idle, the receive machine becomes active. This bit will read back as 1 whenever the receive state machine is active. After initial power-up, software must ensure that the receiver has completely reset before setting this bit. 1: Enable 0: Disable Default: 0b."]
pub type BitMacrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ENSWBCN` reader - Enable SW TX beacon 0 : disable, beacon is DMAed from host memory by HW at BCNDMAINT 1 : enable, beacon is polled to DMA to TXBUF. In both case, as long as a beacon frame is set in BCN page, MAC will automatically sent beacon frame automatically."]
pub type BitEnswbcnR = crate::BitReader;
#[doc = "Field `BIT_ENSWBCN` writer - Enable SW TX beacon 0 : disable, beacon is DMAed from host memory by HW at BCNDMAINT 1 : enable, beacon is polled to DMA to TXBUF. In both case, as long as a beacon frame is set in BCN page, MAC will automatically sent beacon frame automatically."]
pub type BitEnswbcnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAC_SEC_EN` reader - Enable MAC security engine When this bit is clear, security engine is reset"]
pub type BitMacSecEnR = crate::BitReader;
#[doc = "Field `BIT_MAC_SEC_EN` writer - Enable MAC security engine When this bit is clear, security engine is reset"]
pub type BitMacSecEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_5` reader - RSVD"]
pub type Rsvd5R = crate::BitReader;
#[doc = "Field `RSVD_5` writer - RSVD"]
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_I2C_MAILBOX_EN` reader - Enable I2C mailbox"]
pub type BitI2cMailboxEnR = crate::BitReader;
#[doc = "Field `BIT_I2C_MAILBOX_EN` writer - Enable I2C mailbox"]
pub type BitI2cMailboxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_4` reader - RSVD"]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `RSVD_4` writer - RSVD"]
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_3` reader - RSVD"]
pub type Rsvd3R = crate::FieldReader;
#[doc = "Field `RSVD_3` writer - RSVD"]
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_NETYPE0` reader - Network Type for port 0 00: No link 01: Link in ad hoc network 10: Link in infrastructure network 11 : AP mode Default: 00b."]
pub type BitNetype0R = crate::FieldReader;
#[doc = "Field `BIT_NETYPE0` writer - Network Type for port 0 00: No link 01: Link in ad hoc network 10: Link in infrastructure network 11 : AP mode Default: 00b."]
pub type BitNetype0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_NETYPE1` reader - Network Type for Port 1 00: No link 01: Link in ad hoc network 10: Link in infrastructure network 11 : AP mode Default: 00b."]
pub type BitNetype1R = crate::FieldReader;
#[doc = "Field `BIT_NETYPE1` writer - Network Type for Port 1 00: No link 01: Link in ad hoc network 10: Link in infrastructure network 11 : AP mode Default: 00b."]
pub type BitNetype1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::FieldReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_DIGLBK` reader - "]
pub type BitDiglbkR = crate::BitReader;
#[doc = "Field `BIT_DIGLBK` writer - "]
pub type BitDiglbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NO_TXPHY` reader - "]
pub type BitNoTxphyR = crate::BitReader;
#[doc = "Field `BIT_NO_TXPHY` writer - "]
pub type BitNoTxphyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DMALBK` reader - "]
pub type BitDmalbkR = crate::BitReader;
#[doc = "Field `BIT_DMALBK` writer - "]
pub type BitDmalbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NO_TBUF` reader - "]
pub type BitNoTbufR = crate::BitReader;
#[doc = "Field `BIT_NO_TBUF` writer - "]
pub type BitNoTbufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DMACLBK` reader - 28:24\\] LBMODE \\[4\\]dual-mac lbk, \\[3\\] no buffer, 1: no delay, 0: delay; \\[2\\] dmalbk, \\[1\\] no_txphy, \\[0\\] diglbk. 00000: Normal 01011: Single MAC Direct loopback (no HW encryption/decryption) 00011: Single MAC Delay Loopback (with HW encryption/decryption)"]
pub type BitDmaclbkR = crate::BitReader;
#[doc = "Field `BIT_DMACLBK` writer - 28:24\\] LBMODE \\[4\\]dual-mac lbk, \\[3\\] no buffer, 1: no delay, 0: delay; \\[2\\] dmalbk, \\[1\\] no_txphy, \\[0\\] diglbk. 00000: Normal 01011: Single MAC Direct loopback (no HW encryption/decryption) 00011: Single MAC Delay Loopback (with HW encryption/decryption)"]
pub type BitDmaclbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - HCI to TXDMA Interface Enable"]
    #[inline(always)]
    pub fn bit_hci_txdma_en(&self) -> BitHciTxdmaEnR {
        BitHciTxdmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HCI to RXDMA Interface Enable"]
    #[inline(always)]
    pub fn bit_hci_rxdma_en(&self) -> BitHciRxdmaEnR {
        BitHciRxdmaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXDMA Function Enable"]
    #[inline(always)]
    pub fn bit_txdma_en(&self) -> BitTxdmaEnR {
        BitTxdmaEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXDMA Function Enable"]
    #[inline(always)]
    pub fn bit_rxdma_en(&self) -> BitRxdmaEnR {
        BitRxdmaEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - protocol Block Function Enable.(FW2HW)"]
    #[inline(always)]
    pub fn bit_protocol_en(&self) -> BitProtocolEnR {
        BitProtocolEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Schedule Enable."]
    #[inline(always)]
    pub fn bit_schedule_en(&self) -> BitScheduleEnR {
        BitScheduleEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MAC Transmitter Enable. When set to 1, and the transmit state machine is idle, the transmit state machine becomes active. This bit will read back as 1 whenever the transmit state machine is active. After initial power-up, software must ensure that the transmitter has completely reset before setting this bit. 1: Enable 0: Disable Default: 0b."]
    #[inline(always)]
    pub fn bit_mactxen(&self) -> BitMactxenR {
        BitMactxenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MAC Receiver Enable. When set to 1, and the receive state machine is idle, the receive machine becomes active. This bit will read back as 1 whenever the receive state machine is active. After initial power-up, software must ensure that the receiver has completely reset before setting this bit. 1: Enable 0: Disable Default: 0b."]
    #[inline(always)]
    pub fn bit_macrxen(&self) -> BitMacrxenR {
        BitMacrxenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable SW TX beacon 0 : disable, beacon is DMAed from host memory by HW at BCNDMAINT 1 : enable, beacon is polled to DMA to TXBUF. In both case, as long as a beacon frame is set in BCN page, MAC will automatically sent beacon frame automatically."]
    #[inline(always)]
    pub fn bit_enswbcn(&self) -> BitEnswbcnR {
        BitEnswbcnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable MAC security engine When this bit is clear, security engine is reset"]
    #[inline(always)]
    pub fn bit_mac_sec_en(&self) -> BitMacSecEnR {
        BitMacSecEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable I2C mailbox"]
    #[inline(always)]
    pub fn bit_i2c_mailbox_en(&self) -> BitI2cMailboxEnR {
        BitI2cMailboxEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RSVD"]
    #[inline(always)]
    pub fn rsvd_4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - RSVD"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Network Type for port 0 00: No link 01: Link in ad hoc network 10: Link in infrastructure network 11 : AP mode Default: 00b."]
    #[inline(always)]
    pub fn bit_netype0(&self) -> BitNetype0R {
        BitNetype0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Network Type for Port 1 00: No link 01: Link in ad hoc network 10: Link in infrastructure network 11 : AP mode Default: 00b."]
    #[inline(always)]
    pub fn bit_netype1(&self) -> BitNetype1R {
        BitNetype1R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn bit_diglbk(&self) -> BitDiglbkR {
        BitDiglbkR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn bit_no_txphy(&self) -> BitNoTxphyR {
        BitNoTxphyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bit_dmalbk(&self) -> BitDmalbkR {
        BitDmalbkR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bit_no_tbuf(&self) -> BitNoTbufR {
        BitNoTbufR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:24\\] LBMODE \\[4\\]dual-mac lbk, \\[3\\] no buffer, 1: no delay, 0: delay; \\[2\\] dmalbk, \\[1\\] no_txphy, \\[0\\] diglbk. 00000: Normal 01011: Single MAC Direct loopback (no HW encryption/decryption) 00011: Single MAC Delay Loopback (with HW encryption/decryption)"]
    #[inline(always)]
    pub fn bit_dmaclbk(&self) -> BitDmaclbkR {
        BitDmaclbkR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 30:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HCI to TXDMA Interface Enable"]
    #[inline(always)]
    pub fn bit_hci_txdma_en(&mut self) -> BitHciTxdmaEnW<'_, RegCrSpec> {
        BitHciTxdmaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - HCI to RXDMA Interface Enable"]
    #[inline(always)]
    pub fn bit_hci_rxdma_en(&mut self) -> BitHciRxdmaEnW<'_, RegCrSpec> {
        BitHciRxdmaEnW::new(self, 1)
    }
    #[doc = "Bit 2 - TXDMA Function Enable"]
    #[inline(always)]
    pub fn bit_txdma_en(&mut self) -> BitTxdmaEnW<'_, RegCrSpec> {
        BitTxdmaEnW::new(self, 2)
    }
    #[doc = "Bit 3 - RXDMA Function Enable"]
    #[inline(always)]
    pub fn bit_rxdma_en(&mut self) -> BitRxdmaEnW<'_, RegCrSpec> {
        BitRxdmaEnW::new(self, 3)
    }
    #[doc = "Bit 4 - protocol Block Function Enable.(FW2HW)"]
    #[inline(always)]
    pub fn bit_protocol_en(&mut self) -> BitProtocolEnW<'_, RegCrSpec> {
        BitProtocolEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Schedule Enable."]
    #[inline(always)]
    pub fn bit_schedule_en(&mut self) -> BitScheduleEnW<'_, RegCrSpec> {
        BitScheduleEnW::new(self, 5)
    }
    #[doc = "Bit 6 - MAC Transmitter Enable. When set to 1, and the transmit state machine is idle, the transmit state machine becomes active. This bit will read back as 1 whenever the transmit state machine is active. After initial power-up, software must ensure that the transmitter has completely reset before setting this bit. 1: Enable 0: Disable Default: 0b."]
    #[inline(always)]
    pub fn bit_mactxen(&mut self) -> BitMactxenW<'_, RegCrSpec> {
        BitMactxenW::new(self, 6)
    }
    #[doc = "Bit 7 - MAC Receiver Enable. When set to 1, and the receive state machine is idle, the receive machine becomes active. This bit will read back as 1 whenever the receive state machine is active. After initial power-up, software must ensure that the receiver has completely reset before setting this bit. 1: Enable 0: Disable Default: 0b."]
    #[inline(always)]
    pub fn bit_macrxen(&mut self) -> BitMacrxenW<'_, RegCrSpec> {
        BitMacrxenW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable SW TX beacon 0 : disable, beacon is DMAed from host memory by HW at BCNDMAINT 1 : enable, beacon is polled to DMA to TXBUF. In both case, as long as a beacon frame is set in BCN page, MAC will automatically sent beacon frame automatically."]
    #[inline(always)]
    pub fn bit_enswbcn(&mut self) -> BitEnswbcnW<'_, RegCrSpec> {
        BitEnswbcnW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable MAC security engine When this bit is clear, security engine is reset"]
    #[inline(always)]
    pub fn bit_mac_sec_en(&mut self) -> BitMacSecEnW<'_, RegCrSpec> {
        BitMacSecEnW::new(self, 9)
    }
    #[doc = "Bit 11 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&mut self) -> Rsvd5W<'_, RegCrSpec> {
        Rsvd5W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable I2C mailbox"]
    #[inline(always)]
    pub fn bit_i2c_mailbox_en(&mut self) -> BitI2cMailboxEnW<'_, RegCrSpec> {
        BitI2cMailboxEnW::new(self, 12)
    }
    #[doc = "Bit 13 - RSVD"]
    #[inline(always)]
    pub fn rsvd_4(&mut self) -> Rsvd4W<'_, RegCrSpec> {
        Rsvd4W::new(self, 13)
    }
    #[doc = "Bits 14:15 - RSVD"]
    #[inline(always)]
    pub fn rsvd_3(&mut self) -> Rsvd3W<'_, RegCrSpec> {
        Rsvd3W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Network Type for port 0 00: No link 01: Link in ad hoc network 10: Link in infrastructure network 11 : AP mode Default: 00b."]
    #[inline(always)]
    pub fn bit_netype0(&mut self) -> BitNetype0W<'_, RegCrSpec> {
        BitNetype0W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Network Type for Port 1 00: No link 01: Link in ad hoc network 10: Link in infrastructure network 11 : AP mode Default: 00b."]
    #[inline(always)]
    pub fn bit_netype1(&mut self) -> BitNetype1W<'_, RegCrSpec> {
        BitNetype1W::new(self, 18)
    }
    #[doc = "Bits 20:23 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegCrSpec> {
        Rsvd2W::new(self, 20)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn bit_diglbk(&mut self) -> BitDiglbkW<'_, RegCrSpec> {
        BitDiglbkW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn bit_no_txphy(&mut self) -> BitNoTxphyW<'_, RegCrSpec> {
        BitNoTxphyW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bit_dmalbk(&mut self) -> BitDmalbkW<'_, RegCrSpec> {
        BitDmalbkW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bit_no_tbuf(&mut self) -> BitNoTbufW<'_, RegCrSpec> {
        BitNoTbufW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:24\\] LBMODE \\[4\\]dual-mac lbk, \\[3\\] no buffer, 1: no delay, 0: delay; \\[2\\] dmalbk, \\[1\\] no_txphy, \\[0\\] diglbk. 00000: Normal 01011: Single MAC Direct loopback (no HW encryption/decryption) 00011: Single MAC Delay Loopback (with HW encryption/decryption)"]
    #[inline(always)]
    pub fn bit_dmaclbk(&mut self) -> BitDmaclbkW<'_, RegCrSpec> {
        BitDmaclbkW::new(self, 28)
    }
    #[doc = "Bits 30:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegCrSpec> {
        RsvdW::new(self, 30)
    }
}
#[doc = "MAC TOP Function enable\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCrSpec;
impl crate::RegisterSpec for RegCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cr::R`](R) reader structure"]
impl crate::Readable for RegCrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cr::W`](W) writer structure"]
impl crate::Writable for RegCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CR to value 0"]
impl crate::Resettable for RegCrSpec {}
