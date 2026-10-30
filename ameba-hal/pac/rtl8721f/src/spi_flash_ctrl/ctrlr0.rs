#[doc = "Register `CTRLR0` reader"]
pub type R = crate::R<Ctrlr0Spec>;
#[doc = "Register `CTRLR0` writer"]
pub type W = crate::W<Ctrlr0Spec>;
#[doc = "Field `SIPOL` reader - While SIPOL_EN = 1 : SPI_CS is inactive state (SPI_CS ==1): spi_sin\\[3:0\\] = SIPOL\\[3:0\\] SPI_CS is active(SPI_CS ==0): spi_sin\\[3:1\\] = SIPOL\\[3:1\\] (single channel) spi_sin\\[3:2\\] = SIPOL\\[3:2\\] (dual channel) (SIPOL\\[4\\]: reserved for spi_in\\[7:4\\])"]
pub type SipolR = crate::FieldReader;
#[doc = "Field `SIPOL` writer - While SIPOL_EN = 1 : SPI_CS is inactive state (SPI_CS ==1): spi_sin\\[3:0\\] = SIPOL\\[3:0\\] SPI_CS is active(SPI_CS ==0): spi_sin\\[3:1\\] = SIPOL\\[3:1\\] (single channel) spi_sin\\[3:2\\] = SIPOL\\[3:2\\] (dual channel) (SIPOL\\[4\\]: reserved for spi_in\\[7:4\\])"]
pub type SipolW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SIPOL_EN` reader - Set to enable SIPOL"]
pub type SipolEnR = crate::BitReader;
#[doc = "Field `SIPOL_EN` writer - Set to enable SIPOL"]
pub type SipolEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCPH` reader - Indicates serial clock phase. The serial clock phase selects the relationship the serial clock with the slave select signal. 0: serial clock toggles in middle of first data bit 1: serial clock toggles at start of first data bit"]
pub type ScphR = crate::BitReader;
#[doc = "Field `SCPH` writer - Indicates serial clock phase. The serial clock phase selects the relationship the serial clock with the slave select signal. 0: serial clock toggles in middle of first data bit 1: serial clock toggles at start of first data bit"]
pub type ScphW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCPOL` reader - Indicates serial clock polarity. It is used to select the polarity of the inactive serial clock. 0: inactive state of serial clock is low 1: inactive state of serial clock is high"]
pub type ScpolR = crate::BitReader;
#[doc = "Field `SCPOL` writer - Indicates serial clock polarity. It is used to select the polarity of the inactive serial clock. 0: inactive state of serial clock is low 1: inactive state of serial clock is high"]
pub type ScpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOD` reader - Indicates transfer mode. 2'b00: transmit mode Others (or 2'b11): receive mode"]
pub type TmodR = crate::FieldReader;
#[doc = "Field `TMOD` writer - Indicates transfer mode. 2'b00: transmit mode Others (or 2'b11): receive mode"]
pub type TmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GCLK_DIS` reader - Set to disable gated clock of icg cell"]
pub type GclkDisR = crate::BitReader;
#[doc = "Field `GCLK_DIS` writer - Set to disable gated clock of icg cell"]
pub type GclkDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR_EN` reader - Indicates the DDR mode in CMD_CH/DATA_CH/ADDR_CH. CTRLR0\\[15\\]: CMD_CH (always 2-Byte CMD type) CTRLR0\\[14\\]: DATA_CH CTRLR0\\[13\\]: ADDR_CH 0: Disable 1: Enable Available only if configuration of DDR_EN is defined."]
pub type DdrEnR = crate::FieldReader;
#[doc = "Field `DDR_EN` writer - Indicates the DDR mode in CMD_CH/DATA_CH/ADDR_CH. CTRLR0\\[15\\]: CMD_CH (always 2-Byte CMD type) CTRLR0\\[14\\]: DATA_CH CTRLR0\\[13\\]: ADDR_CH 0: Disable 1: Enable Available only if configuration of DDR_EN is defined."]
pub type DdrEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADDR_CH` reader - Indicates channel number of address phase after command phase. Addr phase is used to send address or data. Addr phase is between one-byte command and a data phase. The number of bytes is determined by the ADDR_LENGTH. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
pub type AddrChR = crate::FieldReader;
#[doc = "Field `ADDR_CH` writer - Indicates channel number of address phase after command phase. Addr phase is used to send address or data. Addr phase is between one-byte command and a data phase. The number of bytes is determined by the ADDR_LENGTH. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
pub type AddrChW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATA_CH` reader - Indicates channel number of data phase in transmitting or receiving data. Data phase is used to send data after address phase. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
pub type DataChR = crate::FieldReader;
#[doc = "Field `DATA_CH` writer - Indicates channel number of data phase in transmitting or receiving data. Data phase is used to send data after address phase. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
pub type DataChW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMD_CH` reader - Indicates channel number of command phase in transmitting or receiving data. Command phase is usually used to send SPI command. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
pub type CmdChR = crate::FieldReader;
#[doc = "Field `CMD_CH` writer - Indicates channel number of command phase in transmitting or receiving data. Command phase is usually used to send SPI command. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
pub type CmdChW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FAST_RD` reader - Indicates to use fast read command in user mode. If setting to 1, SPIC would use FBAUDR to generate spi_sclk."]
pub type FastRdR = crate::BitReader;
#[doc = "Field `FAST_RD` writer - Indicates to use fast read command in user mode. If setting to 1, SPIC would use FBAUDR to generate spi_sclk."]
pub type FastRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_MTIMES` reader - Indicates the check times. If Flash is always busy in auto-check times, it causes ACEIR interrupt. The delay time (cycles) is related to bus_clk. The cycles of the parameter = (CK_MTIMES << (11+ATCK_BIT_EXTEND).). User should ensure the timeout > Write time of Flash The timeout time(ns) = 2*BAUDR*(1/SPIC_freq)(8/CH) (CK_MTIMES << (11+ATCK_BIT_EXTEND).) Example 1: Flash Program time= 1ms, BAUDR=1, CH=1 (SPI) and (1/SPIC_freq) = 10ns. ATCK_MTIMES should be to set 4. Example 2: SPIC_freq = 200MHz, BAUDR = 1, CH =4 (QSPI), and ATCK_BIT_EXTEND = 2, the maximum auto_check time should be = 2*1*(1/200MHz)(8/4) (5'b11111<<13) = 2*1*(1/200MHz)(8/4) (18'b11_1110_0000_0000_0000) = 2*1*(1/200MHz)(8/4) (18'h3_e000) = 2*1*(1/200MHz)(8/4) (253952) = 5079040 ns = 5.07 ms Suggestion: If ACEIR interrupt occurs, the user should check the flash status by the software itself. Note: Before SVN 8369 Version, CK_MTIMES is fixed to shift 10bit. The formula becomes 2*BAUDR*(1/SPIC_freq)(8/CH) (CK_MTIMES << (10))"]
pub type CkMtimesR = crate::FieldReader;
#[doc = "Field `CK_MTIMES` writer - Indicates the check times. If Flash is always busy in auto-check times, it causes ACEIR interrupt. The delay time (cycles) is related to bus_clk. The cycles of the parameter = (CK_MTIMES << (11+ATCK_BIT_EXTEND).). User should ensure the timeout > Write time of Flash The timeout time(ns) = 2*BAUDR*(1/SPIC_freq)(8/CH) (CK_MTIMES << (11+ATCK_BIT_EXTEND).) Example 1: Flash Program time= 1ms, BAUDR=1, CH=1 (SPI) and (1/SPIC_freq) = 10ns. ATCK_MTIMES should be to set 4. Example 2: SPIC_freq = 200MHz, BAUDR = 1, CH =4 (QSPI), and ATCK_BIT_EXTEND = 2, the maximum auto_check time should be = 2*1*(1/200MHz)(8/4) (5'b11111<<13) = 2*1*(1/200MHz)(8/4) (18'b11_1110_0000_0000_0000) = 2*1*(1/200MHz)(8/4) (18'h3_e000) = 2*1*(1/200MHz)(8/4) (253952) = 5079040 ns = 5.07 ms Suggestion: If ACEIR interrupt occurs, the user should check the flash status by the software itself. Note: Before SVN 8369 Version, CK_MTIMES is fixed to shift 10bit. The formula becomes 2*BAUDR*(1/SPIC_freq)(8/CH) (CK_MTIMES << (10))"]
pub type CkMtimesW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ECC_DATA_EN` reader - Indicate the data path should go through ECC in NAND user mode read write. ECC mode. User should program 0x170 and 0x174. The read/write data flow will go through ECC module. Normal mode. The read/write data flow will bypass ECC module. Exists: TAG >= 2.0.11. Available only if configuration of NAND_EN and ECC_EN are defined."]
pub type EccDataEnR = crate::BitReader;
#[doc = "Field `ECC_DATA_EN` writer - Indicate the data path should go through ECC in NAND user mode read write. ECC mode. User should program 0x170 and 0x174. The read/write data flow will go through ECC module. Normal mode. The read/write data flow will bypass ECC module. Exists: TAG >= 2.0.11. Available only if configuration of NAND_EN and ECC_EN are defined."]
pub type EccDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UAR` reader - User mode bit auto reset. Enable to auto reset USER_MODE to 0 after current user mode transaction is over (SPIC_EN reset to 0)."]
pub type UarR = crate::BitReader;
#[doc = "Field `UAR` writer - User mode bit auto reset. Enable to auto reset USER_MODE to 0 after current user mode transaction is over (SPIC_EN reset to 0)."]
pub type UarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USER_MODE` reader - User mode bit. Enable to enter user mode. Disable to enter auto mode. It can not be changed while SPIC is busy (28 \\[0\\] BUSY). 1: User mode. User can push data to FIFO and set SSIENR to 1. SPIC won't accept auto read/write command. 0: Auto mode. User can't push data to FIFO and set SSIENR to 1. SPIC will accept auto read/write command."]
pub type UserModeR = crate::BitReader;
#[doc = "Field `USER_MODE` writer - User mode bit. Enable to enter user mode. Disable to enter auto mode. It can not be changed while SPIC is busy (28 \\[0\\] BUSY). 1: User mode. User can push data to FIFO and set SSIENR to 1. SPIC won't accept auto read/write command. 0: Auto mode. User can't push data to FIFO and set SSIENR to 1. SPIC will accept auto read/write command."]
pub type UserModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - While SIPOL_EN = 1 : SPI_CS is inactive state (SPI_CS ==1): spi_sin\\[3:0\\] = SIPOL\\[3:0\\] SPI_CS is active(SPI_CS ==0): spi_sin\\[3:1\\] = SIPOL\\[3:1\\] (single channel) spi_sin\\[3:2\\] = SIPOL\\[3:2\\] (dual channel) (SIPOL\\[4\\]: reserved for spi_in\\[7:4\\])"]
    #[inline(always)]
    pub fn sipol(&self) -> SipolR {
        SipolR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Set to enable SIPOL"]
    #[inline(always)]
    pub fn sipol_en(&self) -> SipolEnR {
        SipolEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates serial clock phase. The serial clock phase selects the relationship the serial clock with the slave select signal. 0: serial clock toggles in middle of first data bit 1: serial clock toggles at start of first data bit"]
    #[inline(always)]
    pub fn scph(&self) -> ScphR {
        ScphR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates serial clock polarity. It is used to select the polarity of the inactive serial clock. 0: inactive state of serial clock is low 1: inactive state of serial clock is high"]
    #[inline(always)]
    pub fn scpol(&self) -> ScpolR {
        ScpolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Indicates transfer mode. 2'b00: transmit mode Others (or 2'b11): receive mode"]
    #[inline(always)]
    pub fn tmod(&self) -> TmodR {
        TmodR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Set to disable gated clock of icg cell"]
    #[inline(always)]
    pub fn gclk_dis(&self) -> GclkDisR {
        GclkDisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Indicates the DDR mode in CMD_CH/DATA_CH/ADDR_CH. CTRLR0\\[15\\]: CMD_CH (always 2-Byte CMD type) CTRLR0\\[14\\]: DATA_CH CTRLR0\\[13\\]: ADDR_CH 0: Disable 1: Enable Available only if configuration of DDR_EN is defined."]
    #[inline(always)]
    pub fn ddr_en(&self) -> DdrEnR {
        DdrEnR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Indicates channel number of address phase after command phase. Addr phase is used to send address or data. Addr phase is between one-byte command and a data phase. The number of bytes is determined by the ADDR_LENGTH. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
    #[inline(always)]
    pub fn addr_ch(&self) -> AddrChR {
        AddrChR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Indicates channel number of data phase in transmitting or receiving data. Data phase is used to send data after address phase. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
    #[inline(always)]
    pub fn data_ch(&self) -> DataChR {
        DataChR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Indicates channel number of command phase in transmitting or receiving data. Command phase is usually used to send SPI command. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
    #[inline(always)]
    pub fn cmd_ch(&self) -> CmdChR {
        CmdChR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Indicates to use fast read command in user mode. If setting to 1, SPIC would use FBAUDR to generate spi_sclk."]
    #[inline(always)]
    pub fn fast_rd(&self) -> FastRdR {
        FastRdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27 - Indicates the check times. If Flash is always busy in auto-check times, it causes ACEIR interrupt. The delay time (cycles) is related to bus_clk. The cycles of the parameter = (CK_MTIMES << (11+ATCK_BIT_EXTEND).). User should ensure the timeout > Write time of Flash The timeout time(ns) = 2*BAUDR*(1/SPIC_freq)(8/CH) (CK_MTIMES << (11+ATCK_BIT_EXTEND).) Example 1: Flash Program time= 1ms, BAUDR=1, CH=1 (SPI) and (1/SPIC_freq) = 10ns. ATCK_MTIMES should be to set 4. Example 2: SPIC_freq = 200MHz, BAUDR = 1, CH =4 (QSPI), and ATCK_BIT_EXTEND = 2, the maximum auto_check time should be = 2*1*(1/200MHz)(8/4) (5'b11111<<13) = 2*1*(1/200MHz)(8/4) (18'b11_1110_0000_0000_0000) = 2*1*(1/200MHz)(8/4) (18'h3_e000) = 2*1*(1/200MHz)(8/4) (253952) = 5079040 ns = 5.07 ms Suggestion: If ACEIR interrupt occurs, the user should check the flash status by the software itself. Note: Before SVN 8369 Version, CK_MTIMES is fixed to shift 10bit. The formula becomes 2*BAUDR*(1/SPIC_freq)(8/CH) (CK_MTIMES << (10))"]
    #[inline(always)]
    pub fn ck_mtimes(&self) -> CkMtimesR {
        CkMtimesR::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - Indicate the data path should go through ECC in NAND user mode read write. ECC mode. User should program 0x170 and 0x174. The read/write data flow will go through ECC module. Normal mode. The read/write data flow will bypass ECC module. Exists: TAG >= 2.0.11. Available only if configuration of NAND_EN and ECC_EN are defined."]
    #[inline(always)]
    pub fn ecc_data_en(&self) -> EccDataEnR {
        EccDataEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - User mode bit auto reset. Enable to auto reset USER_MODE to 0 after current user mode transaction is over (SPIC_EN reset to 0)."]
    #[inline(always)]
    pub fn uar(&self) -> UarR {
        UarR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - User mode bit. Enable to enter user mode. Disable to enter auto mode. It can not be changed while SPIC is busy (28 \\[0\\] BUSY). 1: User mode. User can push data to FIFO and set SSIENR to 1. SPIC won't accept auto read/write command. 0: Auto mode. User can't push data to FIFO and set SSIENR to 1. SPIC will accept auto read/write command."]
    #[inline(always)]
    pub fn user_mode(&self) -> UserModeR {
        UserModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - While SIPOL_EN = 1 : SPI_CS is inactive state (SPI_CS ==1): spi_sin\\[3:0\\] = SIPOL\\[3:0\\] SPI_CS is active(SPI_CS ==0): spi_sin\\[3:1\\] = SIPOL\\[3:1\\] (single channel) spi_sin\\[3:2\\] = SIPOL\\[3:2\\] (dual channel) (SIPOL\\[4\\]: reserved for spi_in\\[7:4\\])"]
    #[inline(always)]
    pub fn sipol(&mut self) -> SipolW<'_, Ctrlr0Spec> {
        SipolW::new(self, 0)
    }
    #[doc = "Bit 5 - Set to enable SIPOL"]
    #[inline(always)]
    pub fn sipol_en(&mut self) -> SipolEnW<'_, Ctrlr0Spec> {
        SipolEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates serial clock phase. The serial clock phase selects the relationship the serial clock with the slave select signal. 0: serial clock toggles in middle of first data bit 1: serial clock toggles at start of first data bit"]
    #[inline(always)]
    pub fn scph(&mut self) -> ScphW<'_, Ctrlr0Spec> {
        ScphW::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates serial clock polarity. It is used to select the polarity of the inactive serial clock. 0: inactive state of serial clock is low 1: inactive state of serial clock is high"]
    #[inline(always)]
    pub fn scpol(&mut self) -> ScpolW<'_, Ctrlr0Spec> {
        ScpolW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Indicates transfer mode. 2'b00: transmit mode Others (or 2'b11): receive mode"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TmodW<'_, Ctrlr0Spec> {
        TmodW::new(self, 8)
    }
    #[doc = "Bit 10 - Set to disable gated clock of icg cell"]
    #[inline(always)]
    pub fn gclk_dis(&mut self) -> GclkDisW<'_, Ctrlr0Spec> {
        GclkDisW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Indicates the DDR mode in CMD_CH/DATA_CH/ADDR_CH. CTRLR0\\[15\\]: CMD_CH (always 2-Byte CMD type) CTRLR0\\[14\\]: DATA_CH CTRLR0\\[13\\]: ADDR_CH 0: Disable 1: Enable Available only if configuration of DDR_EN is defined."]
    #[inline(always)]
    pub fn ddr_en(&mut self) -> DdrEnW<'_, Ctrlr0Spec> {
        DdrEnW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Indicates channel number of address phase after command phase. Addr phase is used to send address or data. Addr phase is between one-byte command and a data phase. The number of bytes is determined by the ADDR_LENGTH. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
    #[inline(always)]
    pub fn addr_ch(&mut self) -> AddrChW<'_, Ctrlr0Spec> {
        AddrChW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Indicates channel number of data phase in transmitting or receiving data. Data phase is used to send data after address phase. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
    #[inline(always)]
    pub fn data_ch(&mut self) -> DataChW<'_, Ctrlr0Spec> {
        DataChW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Indicates channel number of command phase in transmitting or receiving data. Command phase is usually used to send SPI command. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
    #[inline(always)]
    pub fn cmd_ch(&mut self) -> CmdChW<'_, Ctrlr0Spec> {
        CmdChW::new(self, 20)
    }
    #[doc = "Bit 22 - Indicates to use fast read command in user mode. If setting to 1, SPIC would use FBAUDR to generate spi_sclk."]
    #[inline(always)]
    pub fn fast_rd(&mut self) -> FastRdW<'_, Ctrlr0Spec> {
        FastRdW::new(self, 22)
    }
    #[doc = "Bits 23:27 - Indicates the check times. If Flash is always busy in auto-check times, it causes ACEIR interrupt. The delay time (cycles) is related to bus_clk. The cycles of the parameter = (CK_MTIMES << (11+ATCK_BIT_EXTEND).). User should ensure the timeout > Write time of Flash The timeout time(ns) = 2*BAUDR*(1/SPIC_freq)(8/CH) (CK_MTIMES << (11+ATCK_BIT_EXTEND).) Example 1: Flash Program time= 1ms, BAUDR=1, CH=1 (SPI) and (1/SPIC_freq) = 10ns. ATCK_MTIMES should be to set 4. Example 2: SPIC_freq = 200MHz, BAUDR = 1, CH =4 (QSPI), and ATCK_BIT_EXTEND = 2, the maximum auto_check time should be = 2*1*(1/200MHz)(8/4) (5'b11111<<13) = 2*1*(1/200MHz)(8/4) (18'b11_1110_0000_0000_0000) = 2*1*(1/200MHz)(8/4) (18'h3_e000) = 2*1*(1/200MHz)(8/4) (253952) = 5079040 ns = 5.07 ms Suggestion: If ACEIR interrupt occurs, the user should check the flash status by the software itself. Note: Before SVN 8369 Version, CK_MTIMES is fixed to shift 10bit. The formula becomes 2*BAUDR*(1/SPIC_freq)(8/CH) (CK_MTIMES << (10))"]
    #[inline(always)]
    pub fn ck_mtimes(&mut self) -> CkMtimesW<'_, Ctrlr0Spec> {
        CkMtimesW::new(self, 23)
    }
    #[doc = "Bit 28 - Indicate the data path should go through ECC in NAND user mode read write. ECC mode. User should program 0x170 and 0x174. The read/write data flow will go through ECC module. Normal mode. The read/write data flow will bypass ECC module. Exists: TAG >= 2.0.11. Available only if configuration of NAND_EN and ECC_EN are defined."]
    #[inline(always)]
    pub fn ecc_data_en(&mut self) -> EccDataEnW<'_, Ctrlr0Spec> {
        EccDataEnW::new(self, 28)
    }
    #[doc = "Bit 30 - User mode bit auto reset. Enable to auto reset USER_MODE to 0 after current user mode transaction is over (SPIC_EN reset to 0)."]
    #[inline(always)]
    pub fn uar(&mut self) -> UarW<'_, Ctrlr0Spec> {
        UarW::new(self, 30)
    }
    #[doc = "Bit 31 - User mode bit. Enable to enter user mode. Disable to enter auto mode. It can not be changed while SPIC is busy (28 \\[0\\] BUSY). 1: User mode. User can push data to FIFO and set SSIENR to 1. SPIC won't accept auto read/write command. 0: Auto mode. User can't push data to FIFO and set SSIENR to 1. SPIC will accept auto read/write command."]
    #[inline(always)]
    pub fn user_mode(&mut self) -> UserModeW<'_, Ctrlr0Spec> {
        UserModeW::new(self, 31)
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
#[doc = "`reset()` method sets CTRLR0 to value 0x0100_0000"]
impl crate::Resettable for Ctrlr0Spec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
