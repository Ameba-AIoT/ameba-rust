#[doc = "Register `CTRLR0` reader"]
pub type R = crate::R<Ctrlr0Spec>;
#[doc = "Register `CTRLR0` writer"]
pub type W = crate::W<Ctrlr0Spec>;
#[doc = "Field `DFS` reader - Data Frame Size. Selects the data frame length. When the data frame size is programmed to be less than 16 bits, the receive data are automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You must right-justify transmit data before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the DFS decode, refer to the following description: 0000/0001/0010: Reserved (undefined operation) 0011: 4-bit serial data transfer 0100: 5-bit serial data transfer 0101: 6-bit serial data transfer 0110: 7-bit serial data transfer 0111: 8-bit serial data transfer 1000: 9-bit serial data transfer 1001: 10-bit serial data transfer 1010: 11-bit serial data transfer 1011: 12-bit serial data transfer 1100: 13-bit serial data transfer 1101: 14-bit serial data transfer 1110: 15-bit serial data transfer 1111: 16-bit serial data transfer"]
pub type DfsR = crate::FieldReader;
#[doc = "Field `DFS` writer - Data Frame Size. Selects the data frame length. When the data frame size is programmed to be less than 16 bits, the receive data are automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You must right-justify transmit data before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the DFS decode, refer to the following description: 0000/0001/0010: Reserved (undefined operation) 0011: 4-bit serial data transfer 0100: 5-bit serial data transfer 0101: 6-bit serial data transfer 0110: 7-bit serial data transfer 0111: 8-bit serial data transfer 1000: 9-bit serial data transfer 1001: 10-bit serial data transfer 1010: 11-bit serial data transfer 1011: 12-bit serial data transfer 1100: 13-bit serial data transfer 1101: 14-bit serial data transfer 1110: 15-bit serial data transfer 1111: 16-bit serial data transfer"]
pub type DfsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FRF` reader - Frame Format. Selects which serial protocol transfers the data. 00: Motorola SPI Other: Reserved"]
pub type FrfR = crate::FieldReader;
#[doc = "Field `FRF` writer - Frame Format. Selects which serial protocol transfers the data. 00: Motorola SPI Other: Reserved"]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCPH` reader - Serial Clock Phase. Valid when the frame format (FRF) is set to Motorola SPI. The serial clock phase selects the relationship of the serial clock with the slave select signal. When SCPH = 0, data are captured on the first edge of the serial clock. When SCPH = 1, the serial clock starts toggling one cycle after the slave select line is activated, and data are captured on the second edge of the serial clock. 0: Serial clock toggles in middle of the first data bit 1: Serial clock toggles at start of the first data bit"]
pub type ScphR = crate::BitReader;
#[doc = "Field `SCPH` writer - Serial Clock Phase. Valid when the frame format (FRF) is set to Motorola SPI. The serial clock phase selects the relationship of the serial clock with the slave select signal. When SCPH = 0, data are captured on the first edge of the serial clock. When SCPH = 1, the serial clock starts toggling one cycle after the slave select line is activated, and data are captured on the second edge of the serial clock. 0: Serial clock toggles in middle of the first data bit 1: Serial clock toggles at start of the first data bit"]
pub type ScphW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCPOL` reader - Serial Clock Polarity. Valid when the frame format (FRF) is set to Motorola SPI. Used to select the polarity of the inactive serial clock, which is held inactive when the SPI master is not actively transferring data on the serial bus. 0: Inactive state of serial clock is low 1: Inactive state of serial clock is high"]
pub type ScpolR = crate::BitReader;
#[doc = "Field `SCPOL` writer - Serial Clock Polarity. Valid when the frame format (FRF) is set to Motorola SPI. Used to select the polarity of the inactive serial clock, which is held inactive when the SPI master is not actively transferring data on the serial bus. 0: Inactive state of serial clock is low 1: Inactive state of serial clock is high"]
pub type ScpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOD` reader - Transfer Mode. Relevant only when the SPI is configured as a serial-master device. Selects the mode of transfer for serial communication. This field does not affect the transfer duplicity. Only indicates whether the receive or transmit data are valid. In transmit-only mode, data received from the external device is not valid and is not stored in the receive FIFO memory; it is overwritten on the next transfer. In receive-only mode, transmitted data are not valid. After the first write to the transmit FIFO, the same word is retransmitted for the duration of the transfer. In transmit-and-receive mode, both transmit and receive data are valid. The transfer continues until the transmit FIFO is empty. Data received from the external device are stored into the receive FIFO memory, where it can be accessed by the host processor. 00: Transmit & Receive 01: Transmit Only 10: Receive Only"]
pub type TmodR = crate::FieldReader;
#[doc = "Field `TMOD` writer - Transfer Mode. Relevant only when the SPI is configured as a serial-master device. Selects the mode of transfer for serial communication. This field does not affect the transfer duplicity. Only indicates whether the receive or transmit data are valid. In transmit-only mode, data received from the external device is not valid and is not stored in the receive FIFO memory; it is overwritten on the next transfer. In receive-only mode, transmitted data are not valid. After the first write to the transmit FIFO, the same word is retransmitted for the duration of the transfer. In transmit-and-receive mode, both transmit and receive data are valid. The transfer continues until the transmit FIFO is empty. Data received from the external device are stored into the receive FIFO memory, where it can be accessed by the host processor. 00: Transmit & Receive 01: Transmit Only 10: Receive Only"]
pub type TmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLV_OE` reader - Slave Output Enable. Relevant only when the SPI is configured as a serial-slave device. When configured as a serial master, this bit field has no functionality. This bit enables or disables the setting of the ssi_oe_n output from the SPI serial slave. When SLV_OE = 1, the ssi_oe_n output can never be active. When the ssi_oe_n output controls the tri-state buffer on the txd output from the slave, a high impedance state is always present on the slave txd output when SLV_OE = 1. This is useful when the master transmits in broadcast mode (master transmits data to all slave devices). Only one slave may respond with data on the master rxd line. This bit is enabled after reset and must be disabled by software (when broadcast mode is used), if you do not want this device to respond with data. 0: Slave txd is enabled 1: Slave txd is disabled"]
pub type SlvOeR = crate::BitReader;
#[doc = "Field `SLV_OE` writer - Slave Output Enable. Relevant only when the SPI is configured as a serial-slave device. When configured as a serial master, this bit field has no functionality. This bit enables or disables the setting of the ssi_oe_n output from the SPI serial slave. When SLV_OE = 1, the ssi_oe_n output can never be active. When the ssi_oe_n output controls the tri-state buffer on the txd output from the slave, a high impedance state is always present on the slave txd output when SLV_OE = 1. This is useful when the master transmits in broadcast mode (master transmits data to all slave devices). Only one slave may respond with data on the master rxd line. This bit is enabled after reset and must be disabled by software (when broadcast mode is used), if you do not want this device to respond with data. 0: Slave txd is enabled 1: Slave txd is disabled"]
pub type SlvOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSF_EN` reader - CS Force Control Enable. Relevant only when the SPI is configured as a serial-master device. This bit field enable SPI to force CS signal keep low until the amount of data bits (DFS * NDF) had been sent/received. SPI master will force the CS signal low even when the TX FIFO is empty during transmission. 0: CS Force Control is disabled. 1: CS Force Control is enabled."]
pub type CsfEnR = crate::BitReader;
#[doc = "Field `CSF_EN` writer - CS Force Control Enable. Relevant only when the SPI is configured as a serial-master device. This bit field enable SPI to force CS signal keep low until the amount of data bits (DFS * NDF) had been sent/received. SPI master will force the CS signal low even when the TX FIFO is empty during transmission. 0: CS Force Control is disabled. 1: CS Force Control is enabled."]
pub type CsfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBYTESWAP` reader - 0: Order of transmit byte doesn't swap 1: Order of transmit byte does swap"]
pub type TxbyteswapR = crate::BitReader;
#[doc = "Field `TXBYTESWAP` writer - 0: Order of transmit byte doesn't swap 1: Order of transmit byte does swap"]
pub type TxbyteswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBITSWAP` reader - 0: Order of transmit bit doesn't swap 1: Order of transmit bit does swap"]
pub type TxbitswapR = crate::BitReader;
#[doc = "Field `TXBITSWAP` writer - 0: Order of transmit bit doesn't swap 1: Order of transmit bit does swap"]
pub type TxbitswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBYTESWAP` reader - 0: Order of receive byte doesn't swap 1: Order of receive byte does swap"]
pub type RxbyteswapR = crate::BitReader;
#[doc = "Field `RXBYTESWAP` writer - 0: Order of receive byte doesn't swap 1: Order of receive byte does swap"]
pub type RxbyteswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBITSWAP` reader - 0: Order of receive bit doesn't swap 1: Order of receive bit does swap"]
pub type RxbitswapR = crate::BitReader;
#[doc = "Field `RXBITSWAP` writer - 0: Order of receive bit doesn't swap 1: Order of receive bit does swap"]
pub type RxbitswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS_T` reader - When SCPH is 0 (Relevant only when the SPI is configured as a serial-master device) 0: ss_n_out doesn't toggle between successive frames 1: ss_n_out does toggle between successive frames"]
pub type SsTR = crate::BitReader;
#[doc = "Field `SS_T` writer - When SCPH is 0 (Relevant only when the SPI is configured as a serial-master device) 0: ss_n_out doesn't toggle between successive frames 1: ss_n_out does toggle between successive frames"]
pub type SsTW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Data Frame Size. Selects the data frame length. When the data frame size is programmed to be less than 16 bits, the receive data are automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You must right-justify transmit data before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the DFS decode, refer to the following description: 0000/0001/0010: Reserved (undefined operation) 0011: 4-bit serial data transfer 0100: 5-bit serial data transfer 0101: 6-bit serial data transfer 0110: 7-bit serial data transfer 0111: 8-bit serial data transfer 1000: 9-bit serial data transfer 1001: 10-bit serial data transfer 1010: 11-bit serial data transfer 1011: 12-bit serial data transfer 1100: 13-bit serial data transfer 1101: 14-bit serial data transfer 1110: 15-bit serial data transfer 1111: 16-bit serial data transfer"]
    #[inline(always)]
    pub fn dfs(&self) -> DfsR {
        DfsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Frame Format. Selects which serial protocol transfers the data. 00: Motorola SPI Other: Reserved"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Serial Clock Phase. Valid when the frame format (FRF) is set to Motorola SPI. The serial clock phase selects the relationship of the serial clock with the slave select signal. When SCPH = 0, data are captured on the first edge of the serial clock. When SCPH = 1, the serial clock starts toggling one cycle after the slave select line is activated, and data are captured on the second edge of the serial clock. 0: Serial clock toggles in middle of the first data bit 1: Serial clock toggles at start of the first data bit"]
    #[inline(always)]
    pub fn scph(&self) -> ScphR {
        ScphR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Serial Clock Polarity. Valid when the frame format (FRF) is set to Motorola SPI. Used to select the polarity of the inactive serial clock, which is held inactive when the SPI master is not actively transferring data on the serial bus. 0: Inactive state of serial clock is low 1: Inactive state of serial clock is high"]
    #[inline(always)]
    pub fn scpol(&self) -> ScpolR {
        ScpolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Transfer Mode. Relevant only when the SPI is configured as a serial-master device. Selects the mode of transfer for serial communication. This field does not affect the transfer duplicity. Only indicates whether the receive or transmit data are valid. In transmit-only mode, data received from the external device is not valid and is not stored in the receive FIFO memory; it is overwritten on the next transfer. In receive-only mode, transmitted data are not valid. After the first write to the transmit FIFO, the same word is retransmitted for the duration of the transfer. In transmit-and-receive mode, both transmit and receive data are valid. The transfer continues until the transmit FIFO is empty. Data received from the external device are stored into the receive FIFO memory, where it can be accessed by the host processor. 00: Transmit & Receive 01: Transmit Only 10: Receive Only"]
    #[inline(always)]
    pub fn tmod(&self) -> TmodR {
        TmodR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Slave Output Enable. Relevant only when the SPI is configured as a serial-slave device. When configured as a serial master, this bit field has no functionality. This bit enables or disables the setting of the ssi_oe_n output from the SPI serial slave. When SLV_OE = 1, the ssi_oe_n output can never be active. When the ssi_oe_n output controls the tri-state buffer on the txd output from the slave, a high impedance state is always present on the slave txd output when SLV_OE = 1. This is useful when the master transmits in broadcast mode (master transmits data to all slave devices). Only one slave may respond with data on the master rxd line. This bit is enabled after reset and must be disabled by software (when broadcast mode is used), if you do not want this device to respond with data. 0: Slave txd is enabled 1: Slave txd is disabled"]
    #[inline(always)]
    pub fn slv_oe(&self) -> SlvOeR {
        SlvOeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - CS Force Control Enable. Relevant only when the SPI is configured as a serial-master device. This bit field enable SPI to force CS signal keep low until the amount of data bits (DFS * NDF) had been sent/received. SPI master will force the CS signal low even when the TX FIFO is empty during transmission. 0: CS Force Control is disabled. 1: CS Force Control is enabled."]
    #[inline(always)]
    pub fn csf_en(&self) -> CsfEnR {
        CsfEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - 0: Order of transmit byte doesn't swap 1: Order of transmit byte does swap"]
    #[inline(always)]
    pub fn txbyteswap(&self) -> TxbyteswapR {
        TxbyteswapR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 0: Order of transmit bit doesn't swap 1: Order of transmit bit does swap"]
    #[inline(always)]
    pub fn txbitswap(&self) -> TxbitswapR {
        TxbitswapR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 0: Order of receive byte doesn't swap 1: Order of receive byte does swap"]
    #[inline(always)]
    pub fn rxbyteswap(&self) -> RxbyteswapR {
        RxbyteswapR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 0: Order of receive bit doesn't swap 1: Order of receive bit does swap"]
    #[inline(always)]
    pub fn rxbitswap(&self) -> RxbitswapR {
        RxbitswapR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - When SCPH is 0 (Relevant only when the SPI is configured as a serial-master device) 0: ss_n_out doesn't toggle between successive frames 1: ss_n_out does toggle between successive frames"]
    #[inline(always)]
    pub fn ss_t(&self) -> SsTR {
        SsTR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Frame Size. Selects the data frame length. When the data frame size is programmed to be less than 16 bits, the receive data are automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You must right-justify transmit data before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the DFS decode, refer to the following description: 0000/0001/0010: Reserved (undefined operation) 0011: 4-bit serial data transfer 0100: 5-bit serial data transfer 0101: 6-bit serial data transfer 0110: 7-bit serial data transfer 0111: 8-bit serial data transfer 1000: 9-bit serial data transfer 1001: 10-bit serial data transfer 1010: 11-bit serial data transfer 1011: 12-bit serial data transfer 1100: 13-bit serial data transfer 1101: 14-bit serial data transfer 1110: 15-bit serial data transfer 1111: 16-bit serial data transfer"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DfsW<'_, Ctrlr0Spec> {
        DfsW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Frame Format. Selects which serial protocol transfers the data. 00: Motorola SPI Other: Reserved"]
    #[inline(always)]
    pub fn frf(&mut self) -> FrfW<'_, Ctrlr0Spec> {
        FrfW::new(self, 4)
    }
    #[doc = "Bit 6 - Serial Clock Phase. Valid when the frame format (FRF) is set to Motorola SPI. The serial clock phase selects the relationship of the serial clock with the slave select signal. When SCPH = 0, data are captured on the first edge of the serial clock. When SCPH = 1, the serial clock starts toggling one cycle after the slave select line is activated, and data are captured on the second edge of the serial clock. 0: Serial clock toggles in middle of the first data bit 1: Serial clock toggles at start of the first data bit"]
    #[inline(always)]
    pub fn scph(&mut self) -> ScphW<'_, Ctrlr0Spec> {
        ScphW::new(self, 6)
    }
    #[doc = "Bit 7 - Serial Clock Polarity. Valid when the frame format (FRF) is set to Motorola SPI. Used to select the polarity of the inactive serial clock, which is held inactive when the SPI master is not actively transferring data on the serial bus. 0: Inactive state of serial clock is low 1: Inactive state of serial clock is high"]
    #[inline(always)]
    pub fn scpol(&mut self) -> ScpolW<'_, Ctrlr0Spec> {
        ScpolW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Transfer Mode. Relevant only when the SPI is configured as a serial-master device. Selects the mode of transfer for serial communication. This field does not affect the transfer duplicity. Only indicates whether the receive or transmit data are valid. In transmit-only mode, data received from the external device is not valid and is not stored in the receive FIFO memory; it is overwritten on the next transfer. In receive-only mode, transmitted data are not valid. After the first write to the transmit FIFO, the same word is retransmitted for the duration of the transfer. In transmit-and-receive mode, both transmit and receive data are valid. The transfer continues until the transmit FIFO is empty. Data received from the external device are stored into the receive FIFO memory, where it can be accessed by the host processor. 00: Transmit & Receive 01: Transmit Only 10: Receive Only"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TmodW<'_, Ctrlr0Spec> {
        TmodW::new(self, 8)
    }
    #[doc = "Bit 10 - Slave Output Enable. Relevant only when the SPI is configured as a serial-slave device. When configured as a serial master, this bit field has no functionality. This bit enables or disables the setting of the ssi_oe_n output from the SPI serial slave. When SLV_OE = 1, the ssi_oe_n output can never be active. When the ssi_oe_n output controls the tri-state buffer on the txd output from the slave, a high impedance state is always present on the slave txd output when SLV_OE = 1. This is useful when the master transmits in broadcast mode (master transmits data to all slave devices). Only one slave may respond with data on the master rxd line. This bit is enabled after reset and must be disabled by software (when broadcast mode is used), if you do not want this device to respond with data. 0: Slave txd is enabled 1: Slave txd is disabled"]
    #[inline(always)]
    pub fn slv_oe(&mut self) -> SlvOeW<'_, Ctrlr0Spec> {
        SlvOeW::new(self, 10)
    }
    #[doc = "Bit 15 - CS Force Control Enable. Relevant only when the SPI is configured as a serial-master device. This bit field enable SPI to force CS signal keep low until the amount of data bits (DFS * NDF) had been sent/received. SPI master will force the CS signal low even when the TX FIFO is empty during transmission. 0: CS Force Control is disabled. 1: CS Force Control is enabled."]
    #[inline(always)]
    pub fn csf_en(&mut self) -> CsfEnW<'_, Ctrlr0Spec> {
        CsfEnW::new(self, 15)
    }
    #[doc = "Bit 21 - 0: Order of transmit byte doesn't swap 1: Order of transmit byte does swap"]
    #[inline(always)]
    pub fn txbyteswap(&mut self) -> TxbyteswapW<'_, Ctrlr0Spec> {
        TxbyteswapW::new(self, 21)
    }
    #[doc = "Bit 22 - 0: Order of transmit bit doesn't swap 1: Order of transmit bit does swap"]
    #[inline(always)]
    pub fn txbitswap(&mut self) -> TxbitswapW<'_, Ctrlr0Spec> {
        TxbitswapW::new(self, 22)
    }
    #[doc = "Bit 23 - 0: Order of receive byte doesn't swap 1: Order of receive byte does swap"]
    #[inline(always)]
    pub fn rxbyteswap(&mut self) -> RxbyteswapW<'_, Ctrlr0Spec> {
        RxbyteswapW::new(self, 23)
    }
    #[doc = "Bit 24 - 0: Order of receive bit doesn't swap 1: Order of receive bit does swap"]
    #[inline(always)]
    pub fn rxbitswap(&mut self) -> RxbitswapW<'_, Ctrlr0Spec> {
        RxbitswapW::new(self, 24)
    }
    #[doc = "Bit 31 - When SCPH is 0 (Relevant only when the SPI is configured as a serial-master device) 0: ss_n_out doesn't toggle between successive frames 1: ss_n_out does toggle between successive frames"]
    #[inline(always)]
    pub fn ss_t(&mut self) -> SsTW<'_, Ctrlr0Spec> {
        SsTW::new(self, 31)
    }
}
#[doc = "Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrlr0Spec;
impl crate::RegisterSpec for Ctrlr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlr0::R`](R) reader structure"]
impl crate::Readable for Ctrlr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrlr0::W`](W) writer structure"]
impl crate::Writable for Ctrlr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLR0 to value 0x09"]
impl crate::Resettable for Ctrlr0Spec {
    const RESET_VALUE: u32 = 0x09;
}
