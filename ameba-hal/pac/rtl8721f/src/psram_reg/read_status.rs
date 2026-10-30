#[doc = "Register `READ_STATUS` reader"]
pub type R = crate::R<ReadStatusSpec>;
#[doc = "Register `READ_STATUS` writer"]
pub type W = crate::W<ReadStatusSpec>;
#[doc = "Field `RD_ST_CMD_1st_BYTE` reader - Indicates SPI Flash command value of 1st byte of read status."]
pub type RdStCmd1stByteR = crate::FieldReader;
#[doc = "Field `RD_ST_CMD_1st_BYTE` writer - Indicates SPI Flash command value of 1st byte of read status."]
pub type RdStCmd1stByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD_ST_CMD_2nd_BYTE` reader - Indicates SPI Flash command value of 2nd byte of read status. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
pub type RdStCmd2ndByteR = crate::FieldReader;
#[doc = "Field `RD_ST_CMD_2nd_BYTE` writer - Indicates SPI Flash command value of 2nd byte of read status. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
pub type RdStCmd2ndByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD_ST_CMD_3rd_BYTE` reader - Indicates SPI Flash command value of 3rd byte of read status. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
pub type RdStCmd3rdByteR = crate::FieldReader;
#[doc = "Field `RD_ST_CMD_3rd_BYTE` writer - Indicates SPI Flash command value of 3rd byte of read status. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
pub type RdStCmd3rdByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ST_CMD_DDR_EN` reader - Indicates the DDR mode in ST_CMD_CH. Available only if configuration of DDR_EN is defined."]
pub type StCmdDdrEnR = crate::BitReader;
#[doc = "Field `ST_CMD_DDR_EN` writer - Indicates the DDR mode in ST_CMD_CH. Available only if configuration of DDR_EN is defined."]
pub type StCmdDdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST_CMD_CH` reader - Indicates channel number of read status command in transmitting and receiving data. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
pub type StCmdChR = crate::FieldReader;
#[doc = "Field `ST_CMD_CH` writer - Indicates channel number of read status command in transmitting and receiving data. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
pub type StCmdChW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ST_CMD_LEN` reader - Indicates the number of bytes in read status command. ST_CMD_LEN can be 1~3. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
pub type StCmdLenR = crate::FieldReader;
#[doc = "Field `ST_CMD_LEN` writer - Indicates the number of bytes in read status command. ST_CMD_LEN can be 1~3. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
pub type StCmdLenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTERVAL_ODD` reader - Indicates SPI Flash read status with 2-Byte status but checking only odd byte or even byte. 0: even byte (2, 4, 6, ...) 1: odd byte (1, 3, 5, ...) Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
pub type IntervalOddR = crate::BitReader;
#[doc = "Field `INTERVAL_ODD` writer - Indicates SPI Flash read status with 2-Byte status but checking only odd byte or even byte. 0: even byte (2, 4, 6, ...) 1: odd byte (1, 3, 5, ...) Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
pub type IntervalOddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERVAL_EN` reader - Set to enable INTERVAL_ODD. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
pub type IntervalEnR = crate::BitReader;
#[doc = "Field `INTERVAL_EN` writer - Set to enable INTERVAL_ODD. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
pub type IntervalEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of 1st byte of read status."]
    #[inline(always)]
    pub fn rd_st_cmd_1st_byte(&self) -> RdStCmd1stByteR {
        RdStCmd1stByteR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates SPI Flash command value of 2nd byte of read status. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
    #[inline(always)]
    pub fn rd_st_cmd_2nd_byte(&self) -> RdStCmd2ndByteR {
        RdStCmd2ndByteR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Indicates SPI Flash command value of 3rd byte of read status. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
    #[inline(always)]
    pub fn rd_st_cmd_3rd_byte(&self) -> RdStCmd3rdByteR {
        RdStCmd3rdByteR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 25 - Indicates the DDR mode in ST_CMD_CH. Available only if configuration of DDR_EN is defined."]
    #[inline(always)]
    pub fn st_cmd_ddr_en(&self) -> StCmdDdrEnR {
        StCmdDdrEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Indicates channel number of read status command in transmitting and receiving data. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
    #[inline(always)]
    pub fn st_cmd_ch(&self) -> StCmdChR {
        StCmdChR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Indicates the number of bytes in read status command. ST_CMD_LEN can be 1~3. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
    #[inline(always)]
    pub fn st_cmd_len(&self) -> StCmdLenR {
        StCmdLenR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Indicates SPI Flash read status with 2-Byte status but checking only odd byte or even byte. 0: even byte (2, 4, 6, ...) 1: odd byte (1, 3, 5, ...) Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
    #[inline(always)]
    pub fn interval_odd(&self) -> IntervalOddR {
        IntervalOddR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set to enable INTERVAL_ODD. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
    #[inline(always)]
    pub fn interval_en(&self) -> IntervalEnR {
        IntervalEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of 1st byte of read status."]
    #[inline(always)]
    pub fn rd_st_cmd_1st_byte(&mut self) -> RdStCmd1stByteW<'_, ReadStatusSpec> {
        RdStCmd1stByteW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates SPI Flash command value of 2nd byte of read status. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
    #[inline(always)]
    pub fn rd_st_cmd_2nd_byte(&mut self) -> RdStCmd2ndByteW<'_, ReadStatusSpec> {
        RdStCmd2ndByteW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Indicates SPI Flash command value of 3rd byte of read status. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
    #[inline(always)]
    pub fn rd_st_cmd_3rd_byte(&mut self) -> RdStCmd3rdByteW<'_, ReadStatusSpec> {
        RdStCmd3rdByteW::new(self, 16)
    }
    #[doc = "Bit 25 - Indicates the DDR mode in ST_CMD_CH. Available only if configuration of DDR_EN is defined."]
    #[inline(always)]
    pub fn st_cmd_ddr_en(&mut self) -> StCmdDdrEnW<'_, ReadStatusSpec> {
        StCmdDdrEnW::new(self, 25)
    }
    #[doc = "Bits 26:27 - Indicates channel number of read status command in transmitting and receiving data. 0: single channel 1: dual channels 2: quad channels 3: octal channel"]
    #[inline(always)]
    pub fn st_cmd_ch(&mut self) -> StCmdChW<'_, ReadStatusSpec> {
        StCmdChW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Indicates the number of bytes in read status command. ST_CMD_LEN can be 1~3. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
    #[inline(always)]
    pub fn st_cmd_len(&mut self) -> StCmdLenW<'_, ReadStatusSpec> {
        StCmdLenW::new(self, 28)
    }
    #[doc = "Bit 30 - Indicates SPI Flash read status with 2-Byte status but checking only odd byte or even byte. 0: even byte (2, 4, 6, ...) 1: odd byte (1, 3, 5, ...) Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
    #[inline(always)]
    pub fn interval_odd(&mut self) -> IntervalOddW<'_, ReadStatusSpec> {
        IntervalOddW::new(self, 30)
    }
    #[doc = "Bit 31 - Set to enable INTERVAL_ODD. Available only if configuration of SPIC_LITE is not defined (SPIC Full ver)."]
    #[inline(always)]
    pub fn interval_en(&mut self) -> IntervalEnW<'_, ReadStatusSpec> {
        IntervalEnW::new(self, 31)
    }
}
#[doc = "Read Status Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`read_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadStatusSpec;
impl crate::RegisterSpec for ReadStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_status::R`](R) reader structure"]
impl crate::Readable for ReadStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`read_status::W`](W) writer structure"]
impl crate::Writable for ReadStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READ_STATUS to value 0x1000_0505"]
impl crate::Resettable for ReadStatusSpec {
    const RESET_VALUE: u32 = 0x1000_0505;
}
