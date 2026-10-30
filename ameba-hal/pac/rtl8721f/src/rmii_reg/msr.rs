#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Register `MSR` writer"]
pub type W = crate::W<MsrSpec>;
#[doc = "Field `refclk_on` reader - 1: REF_CLK on 0: REF_CLK off"]
pub type RefclkOnR = crate::BitReader;
#[doc = "Field `refclk_on` writer - 1: REF_CLK on 0: REF_CLK off"]
pub type RefclkOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `refclk_phase` reader - 1: for Tx transaction, TXEN & TXD signals are changed on the falling edge of REF_CLK; for Rx transaction, RXD signals are sampled on the falling edge of REF_CLK 0: for Tx transaction, TXEN & TXD signals are changed on the rising edge of REF_CLK; for Rx transaction, RXD signals are sampled on the rising edge of REF_CLK"]
pub type RefclkPhaseR = crate::BitReader;
#[doc = "Field `refclk_phase` writer - 1: for Tx transaction, TXEN & TXD signals are changed on the falling edge of REF_CLK; for Rx transaction, RXD signals are sampled on the falling edge of REF_CLK 0: for Tx transaction, TXEN & TXD signals are changed on the rising edge of REF_CLK; for Rx transaction, RXD signals are sampled on the rising edge of REF_CLK"]
pub type RefclkPhaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac_md_mst` reader - 1: GMAC is used to be MDIO master. (When bit\\[13\\] is setting as '0') 0: GMAC is used to be MDIO slave. (When bit\\[13\\] is setting as '1')"]
pub type GmacMdMstR = crate::BitReader;
#[doc = "Field `reg_rmii2mii_en` reader - 1: transfer MII signals to RMII signals This function is only valid in MII mode. (default 0: for AmebaI//AmebaPro/AmebaPro2)"]
pub type RegRmii2miiEnR = crate::BitReader;
#[doc = "Field `reg_rmii2mii_en` writer - 1: transfer MII signals to RMII signals This function is only valid in MII mode. (default 0: for AmebaI//AmebaPro/AmebaPro2)"]
pub type RegRmii2miiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Eee_aby` reader - Indicates eee ability of PCS. Eee_aby\\[0\\]: eee ability in 100M Eee_aby\\[1\\]: eee ability in giga mode."]
pub type EeeAbyR = crate::FieldReader;
#[doc = "Field `force_spd_mode` reader - 1: gmac is in force speed mode. The real speed in force mode is set in MS_REG.FORCE_SPD. 0: gmac speed status is from md operation."]
pub type ForceSpdModeR = crate::BitReader;
#[doc = "Field `force_spd_mode` writer - 1: gmac is in force speed mode. The real speed in force mode is set in MS_REG.FORCE_SPD. 0: gmac speed status is from md operation."]
pub type ForceSpdModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rgmii_tx_encode_sts` reader - This field is valid only in phy mode. 0: Does not support rgmii in band status(link status, speed and duplex mode of the PHY) by encoding txd. 1: Supports rgmii in band status(link status, speed and duplex mode of the PHY) by encoding txd."]
pub type RgmiiTxEncodeStsR = crate::BitReader;
#[doc = "Field `rgmii_tx_encode_sts` writer - This field is valid only in phy mode. 0: Does not support rgmii in band status(link status, speed and duplex mode of the PHY) by encoding txd. 1: Supports rgmii in band status(link status, speed and duplex mode of the PHY) by encoding txd."]
pub type RgmiiTxEncodeStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rgmii_rx_decode_sts` reader - 0: Does not support rgmii in band status(link status, speed and duplex mode of the PHY) by decoding rxd. 1: Supports rgmii in band status(link status, speed and duplex mode of the PHY) by decoding rxd."]
pub type RgmiiRxDecodeStsR = crate::BitReader;
#[doc = "Field `rgmii_rx_decode_sts` writer - 0: Does not support rgmii in band status(link status, speed and duplex mode of the PHY) by decoding rxd. 1: Supports rgmii in band status(link status, speed and duplex mode of the PHY) by decoding rxd."]
pub type RgmiiRxDecodeStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac_phy_mode` reader - 1: in phy mode. (bit\\[6\\] will be read as '0') 0: not in phy mode. (bit\\[6\\] will be read as '1')"]
pub type GmacPhyModeR = crate::BitReader;
#[doc = "Field `gmac_phy_mode` writer - 1: in phy mode. (bit\\[6\\] will be read as '0') 0: not in phy mode. (bit\\[6\\] will be read as '1')"]
pub type GmacPhyModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac_sel_phyif_0` reader - 1: phy interface 0.works. 0: phy interface 1 works."]
pub type GmacSelPhyif0R = crate::BitReader;
#[doc = "Field `FORCE_SPD` reader - Force gmac in 10/100/GIGA mode. 2'b00: 100M 2'b01: 10M 2'b10: GIGA 2'b11: not force mode. These bits are Write only in RLE0390 (RL6166). The current speed status of gmac can be checked in MSR_reg.{speed1000, speed_10}. In force mode, SW can set gmac operates in , FD and Link ok(for example) and SW can not set gmac operates in , HD and Link ok. When FORCEDFULLDUP is b0, the duplex status is from MDIO auto-polling, therefore, it is not controlled by SW."]
pub type ForceSpdR = crate::FieldReader;
#[doc = "Field `FORCE_SPD` writer - Force gmac in 10/100/GIGA mode. 2'b00: 100M 2'b01: 10M 2'b10: GIGA 2'b11: not force mode. These bits are Write only in RLE0390 (RL6166). The current speed status of gmac can be checked in MSR_reg.{speed1000, speed_10}. In force mode, SW can set gmac operates in , FD and Link ok(for example) and SW can not set gmac operates in , HD and Link ok. When FORCEDFULLDUP is b0, the duplex status is from MDIO auto-polling, therefore, it is not controlled by SW."]
pub type ForceSpdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORCELINK` reader - force gmac in link ok mode. This bit is Write only in RLE0315. This bit is R/W in RLE0390 (RL6166). 1'b1: force gmac in linkok. 1'b0: link status is from MDIO auto-polling."]
pub type ForcelinkR = crate::BitReader;
#[doc = "Field `FORCELINK` writer - force gmac in link ok mode. This bit is Write only in RLE0315. This bit is R/W in RLE0390 (RL6166). 1'b1: force gmac in linkok. 1'b0: link status is from MDIO auto-polling."]
pub type ForcelinkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEDFULLDUP` reader - force gmac operates at full duplex mode. This bit is Write only in RLE0315. This bit is R/W in RLE0390 (formerly RL6166). 1'b1: force gmac in full duplex mode. 1'b0: duplex status is from MDIO auto-polling. Not means gmac is in half duplex mode."]
pub type ForcedfulldupR = crate::BitReader;
#[doc = "Field `FORCEDFULLDUP` writer - force gmac operates at full duplex mode. This bit is Write only in RLE0315. This bit is R/W in RLE0390 (formerly RL6166). 1'b1: force gmac in full duplex mode. 1'b0: duplex status is from MDIO auto-polling. Not means gmac is in half duplex mode."]
pub type ForcedfulldupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_MII` reader - indicates in mii mode."]
pub type SelMiiR = crate::BitReader;
#[doc = "Field `NWCOMPLETE` reader - Nway complete"]
pub type NwcompleteR = crate::BitReader;
#[doc = "Field `FULLDUPREG` reader - Indicates Full duplex mode from mdio results."]
pub type FulldupregR = crate::BitReader;
#[doc = "Field `SEL_RGMII` reader - gmac_sel_rgmii"]
pub type SelRgmiiR = crate::BitReader;
#[doc = "Field `SEL_RGMII` writer - gmac_sel_rgmii"]
pub type SelRgmiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPF` reader - Pause Flag: 1 = Ethernet module is in backoff state because a pause packet received. 0 = pause state is clear."]
pub type RxpfR = crate::BitReader;
#[doc = "Field `TXPF` reader - Tx Pause frame: 1 = Ethernet module has sent a pause packet. 0 = the Ethernet module has sent a timer done packet."]
pub type TxpfR = crate::BitReader;
#[doc = "Field `LINKB` reader - Inverse of Link status. 0 = Link OK. 1 = Link Fail."]
pub type LinkbR = crate::BitReader;
#[doc = "Field `SPEED` reader - {SPEED_1000, SPEED_10} 00: 100Mbps, 01: 10Mbps 10: 1000Mbps, 11: not allowed"]
pub type SpeedR = crate::FieldReader;
#[doc = "Field `TXFCE` reader - Tx Flow Control Enable: 1 = tx flow control enabled. 0 = tx flow control disabled. ACCEPT ERROR MUST NOT BE ENABLED"]
pub type TxfceR = crate::BitReader;
#[doc = "Field `TXFCE` writer - Tx Flow Control Enable: 1 = tx flow control enabled. 0 = tx flow control disabled. ACCEPT ERROR MUST NOT BE ENABLED"]
pub type TxfceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFCE` reader - RX Flow control Enable: The flow control is enabled in full-duplex mode only. Packets are dropped if buffer is exhausted. Default is 0. 1 = Rx Flow Control Enabled. 0 = Rx Flow Control Disabled."]
pub type RxfceR = crate::BitReader;
#[doc = "Field `RXFCE` writer - RX Flow control Enable: The flow control is enabled in full-duplex mode only. Packets are dropped if buffer is exhausted. Default is 0. 1 = Rx Flow Control Enabled. 0 = Rx Flow Control Disabled."]
pub type RxfceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Force_TRXFCE` reader - Force Tx/RX Flow Control: 1 = enabled Flow control in the absence of NWAY. 0 = disables Flow control in the absence of NWAY. Version effect: rl6166(ECO) and after."]
pub type ForceTrxfceR = crate::BitReader;
#[doc = "Field `Force_TRXFCE` writer - Force Tx/RX Flow Control: 1 = enabled Flow control in the absence of NWAY. 0 = disables Flow control in the absence of NWAY. Version effect: rl6166(ECO) and after."]
pub type ForceTrxfceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: REF_CLK on 0: REF_CLK off"]
    #[inline(always)]
    pub fn refclk_on(&self) -> RefclkOnR {
        RefclkOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: for Tx transaction, TXEN & TXD signals are changed on the falling edge of REF_CLK; for Rx transaction, RXD signals are sampled on the falling edge of REF_CLK 0: for Tx transaction, TXEN & TXD signals are changed on the rising edge of REF_CLK; for Rx transaction, RXD signals are sampled on the rising edge of REF_CLK"]
    #[inline(always)]
    pub fn refclk_phase(&self) -> RefclkPhaseR {
        RefclkPhaseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: GMAC is used to be MDIO master. (When bit\\[13\\] is setting as '0') 0: GMAC is used to be MDIO slave. (When bit\\[13\\] is setting as '1')"]
    #[inline(always)]
    pub fn gmac_md_mst(&self) -> GmacMdMstR {
        GmacMdMstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: transfer MII signals to RMII signals This function is only valid in MII mode. (default 0: for AmebaI//AmebaPro/AmebaPro2)"]
    #[inline(always)]
    pub fn reg_rmii2mii_en(&self) -> RegRmii2miiEnR {
        RegRmii2miiEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Indicates eee ability of PCS. Eee_aby\\[0\\]: eee ability in 100M Eee_aby\\[1\\]: eee ability in giga mode."]
    #[inline(always)]
    pub fn eee_aby(&self) -> EeeAbyR {
        EeeAbyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 1: gmac is in force speed mode. The real speed in force mode is set in MS_REG.FORCE_SPD. 0: gmac speed status is from md operation."]
    #[inline(always)]
    pub fn force_spd_mode(&self) -> ForceSpdModeR {
        ForceSpdModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This field is valid only in phy mode. 0: Does not support rgmii in band status(link status, speed and duplex mode of the PHY) by encoding txd. 1: Supports rgmii in band status(link status, speed and duplex mode of the PHY) by encoding txd."]
    #[inline(always)]
    pub fn rgmii_tx_encode_sts(&self) -> RgmiiTxEncodeStsR {
        RgmiiTxEncodeStsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 0: Does not support rgmii in band status(link status, speed and duplex mode of the PHY) by decoding rxd. 1: Supports rgmii in band status(link status, speed and duplex mode of the PHY) by decoding rxd."]
    #[inline(always)]
    pub fn rgmii_rx_decode_sts(&self) -> RgmiiRxDecodeStsR {
        RgmiiRxDecodeStsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: in phy mode. (bit\\[6\\] will be read as '0') 0: not in phy mode. (bit\\[6\\] will be read as '1')"]
    #[inline(always)]
    pub fn gmac_phy_mode(&self) -> GmacPhyModeR {
        GmacPhyModeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: phy interface 0.works. 0: phy interface 1 works."]
    #[inline(always)]
    pub fn gmac_sel_phyif_0(&self) -> GmacSelPhyif0R {
        GmacSelPhyif0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Force gmac in 10/100/GIGA mode. 2'b00: 100M 2'b01: 10M 2'b10: GIGA 2'b11: not force mode. These bits are Write only in RLE0390 (RL6166). The current speed status of gmac can be checked in MSR_reg.{speed1000, speed_10}. In force mode, SW can set gmac operates in , FD and Link ok(for example) and SW can not set gmac operates in , HD and Link ok. When FORCEDFULLDUP is b0, the duplex status is from MDIO auto-polling, therefore, it is not controlled by SW."]
    #[inline(always)]
    pub fn force_spd(&self) -> ForceSpdR {
        ForceSpdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - force gmac in link ok mode. This bit is Write only in RLE0315. This bit is R/W in RLE0390 (RL6166). 1'b1: force gmac in linkok. 1'b0: link status is from MDIO auto-polling."]
    #[inline(always)]
    pub fn forcelink(&self) -> ForcelinkR {
        ForcelinkR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - force gmac operates at full duplex mode. This bit is Write only in RLE0315. This bit is R/W in RLE0390 (formerly RL6166). 1'b1: force gmac in full duplex mode. 1'b0: duplex status is from MDIO auto-polling. Not means gmac is in half duplex mode."]
    #[inline(always)]
    pub fn forcedfulldup(&self) -> ForcedfulldupR {
        ForcedfulldupR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - indicates in mii mode."]
    #[inline(always)]
    pub fn sel_mii(&self) -> SelMiiR {
        SelMiiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Nway complete"]
    #[inline(always)]
    pub fn nwcomplete(&self) -> NwcompleteR {
        NwcompleteR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Indicates Full duplex mode from mdio results."]
    #[inline(always)]
    pub fn fulldupreg(&self) -> FulldupregR {
        FulldupregR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - gmac_sel_rgmii"]
    #[inline(always)]
    pub fn sel_rgmii(&self) -> SelRgmiiR {
        SelRgmiiR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Pause Flag: 1 = Ethernet module is in backoff state because a pause packet received. 0 = pause state is clear."]
    #[inline(always)]
    pub fn rxpf(&self) -> RxpfR {
        RxpfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Pause frame: 1 = Ethernet module has sent a pause packet. 0 = the Ethernet module has sent a timer done packet."]
    #[inline(always)]
    pub fn txpf(&self) -> TxpfR {
        TxpfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Inverse of Link status. 0 = Link OK. 1 = Link Fail."]
    #[inline(always)]
    pub fn linkb(&self) -> LinkbR {
        LinkbR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - {SPEED_1000, SPEED_10} 00: 100Mbps, 01: 10Mbps 10: 1000Mbps, 11: not allowed"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Tx Flow Control Enable: 1 = tx flow control enabled. 0 = tx flow control disabled. ACCEPT ERROR MUST NOT BE ENABLED"]
    #[inline(always)]
    pub fn txfce(&self) -> TxfceR {
        TxfceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RX Flow control Enable: The flow control is enabled in full-duplex mode only. Packets are dropped if buffer is exhausted. Default is 0. 1 = Rx Flow Control Enabled. 0 = Rx Flow Control Disabled."]
    #[inline(always)]
    pub fn rxfce(&self) -> RxfceR {
        RxfceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Force Tx/RX Flow Control: 1 = enabled Flow control in the absence of NWAY. 0 = disables Flow control in the absence of NWAY. Version effect: rl6166(ECO) and after."]
    #[inline(always)]
    pub fn force_trxfce(&self) -> ForceTrxfceR {
        ForceTrxfceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: REF_CLK on 0: REF_CLK off"]
    #[inline(always)]
    pub fn refclk_on(&mut self) -> RefclkOnW<'_, MsrSpec> {
        RefclkOnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: for Tx transaction, TXEN & TXD signals are changed on the falling edge of REF_CLK; for Rx transaction, RXD signals are sampled on the falling edge of REF_CLK 0: for Tx transaction, TXEN & TXD signals are changed on the rising edge of REF_CLK; for Rx transaction, RXD signals are sampled on the rising edge of REF_CLK"]
    #[inline(always)]
    pub fn refclk_phase(&mut self) -> RefclkPhaseW<'_, MsrSpec> {
        RefclkPhaseW::new(self, 1)
    }
    #[doc = "Bit 7 - 1: transfer MII signals to RMII signals This function is only valid in MII mode. (default 0: for AmebaI//AmebaPro/AmebaPro2)"]
    #[inline(always)]
    pub fn reg_rmii2mii_en(&mut self) -> RegRmii2miiEnW<'_, MsrSpec> {
        RegRmii2miiEnW::new(self, 7)
    }
    #[doc = "Bit 10 - 1: gmac is in force speed mode. The real speed in force mode is set in MS_REG.FORCE_SPD. 0: gmac speed status is from md operation."]
    #[inline(always)]
    pub fn force_spd_mode(&mut self) -> ForceSpdModeW<'_, MsrSpec> {
        ForceSpdModeW::new(self, 10)
    }
    #[doc = "Bit 11 - This field is valid only in phy mode. 0: Does not support rgmii in band status(link status, speed and duplex mode of the PHY) by encoding txd. 1: Supports rgmii in band status(link status, speed and duplex mode of the PHY) by encoding txd."]
    #[inline(always)]
    pub fn rgmii_tx_encode_sts(&mut self) -> RgmiiTxEncodeStsW<'_, MsrSpec> {
        RgmiiTxEncodeStsW::new(self, 11)
    }
    #[doc = "Bit 12 - 0: Does not support rgmii in band status(link status, speed and duplex mode of the PHY) by decoding rxd. 1: Supports rgmii in band status(link status, speed and duplex mode of the PHY) by decoding rxd."]
    #[inline(always)]
    pub fn rgmii_rx_decode_sts(&mut self) -> RgmiiRxDecodeStsW<'_, MsrSpec> {
        RgmiiRxDecodeStsW::new(self, 12)
    }
    #[doc = "Bit 13 - 1: in phy mode. (bit\\[6\\] will be read as '0') 0: not in phy mode. (bit\\[6\\] will be read as '1')"]
    #[inline(always)]
    pub fn gmac_phy_mode(&mut self) -> GmacPhyModeW<'_, MsrSpec> {
        GmacPhyModeW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Force gmac in 10/100/GIGA mode. 2'b00: 100M 2'b01: 10M 2'b10: GIGA 2'b11: not force mode. These bits are Write only in RLE0390 (RL6166). The current speed status of gmac can be checked in MSR_reg.{speed1000, speed_10}. In force mode, SW can set gmac operates in , FD and Link ok(for example) and SW can not set gmac operates in , HD and Link ok. When FORCEDFULLDUP is b0, the duplex status is from MDIO auto-polling, therefore, it is not controlled by SW."]
    #[inline(always)]
    pub fn force_spd(&mut self) -> ForceSpdW<'_, MsrSpec> {
        ForceSpdW::new(self, 16)
    }
    #[doc = "Bit 18 - force gmac in link ok mode. This bit is Write only in RLE0315. This bit is R/W in RLE0390 (RL6166). 1'b1: force gmac in linkok. 1'b0: link status is from MDIO auto-polling."]
    #[inline(always)]
    pub fn forcelink(&mut self) -> ForcelinkW<'_, MsrSpec> {
        ForcelinkW::new(self, 18)
    }
    #[doc = "Bit 19 - force gmac operates at full duplex mode. This bit is Write only in RLE0315. This bit is R/W in RLE0390 (formerly RL6166). 1'b1: force gmac in full duplex mode. 1'b0: duplex status is from MDIO auto-polling. Not means gmac is in half duplex mode."]
    #[inline(always)]
    pub fn forcedfulldup(&mut self) -> ForcedfulldupW<'_, MsrSpec> {
        ForcedfulldupW::new(self, 19)
    }
    #[doc = "Bit 23 - gmac_sel_rgmii"]
    #[inline(always)]
    pub fn sel_rgmii(&mut self) -> SelRgmiiW<'_, MsrSpec> {
        SelRgmiiW::new(self, 23)
    }
    #[doc = "Bit 29 - Tx Flow Control Enable: 1 = tx flow control enabled. 0 = tx flow control disabled. ACCEPT ERROR MUST NOT BE ENABLED"]
    #[inline(always)]
    pub fn txfce(&mut self) -> TxfceW<'_, MsrSpec> {
        TxfceW::new(self, 29)
    }
    #[doc = "Bit 30 - RX Flow control Enable: The flow control is enabled in full-duplex mode only. Packets are dropped if buffer is exhausted. Default is 0. 1 = Rx Flow Control Enabled. 0 = Rx Flow Control Disabled."]
    #[inline(always)]
    pub fn rxfce(&mut self) -> RxfceW<'_, MsrSpec> {
        RxfceW::new(self, 30)
    }
    #[doc = "Bit 31 - Force Tx/RX Flow Control: 1 = enabled Flow control in the absence of NWAY. 0 = disables Flow control in the absence of NWAY. Version effect: rl6166(ECO) and after."]
    #[inline(always)]
    pub fn force_trxfce(&mut self) -> ForceTrxfceW<'_, MsrSpec> {
        ForceTrxfceW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`write(|w| ..)` method takes [`msr::W`](W) writer structure"]
impl crate::Writable for MsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSR to value 0x0003_8041"]
impl crate::Resettable for MsrSpec {
    const RESET_VALUE: u32 = 0x0003_8041;
}
