#[doc = "Register `IC_DATA_CMD` reader"]
pub type R = crate::R<IcDataCmdSpec>;
#[doc = "Register `IC_DATA_CMD` writer"]
pub type W = crate::W<IcDataCmdSpec>;
#[doc = "Field `IC_DATA` reader - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, these bits are ignored by the I2C. However, when you read this register, these bits return the value of data received on the I2C interface."]
pub type IcDataR = crate::FieldReader;
#[doc = "Field `IC_DATA` writer - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, these bits are ignored by the I2C. However, when you read this register, these bits return the value of data received on the I2C interface."]
pub type IcDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMD_RW` writer - This bit controls whether a read or write is performed. This bit does not control the direction when the I2C acts as a slave. It controls only the direction when it acts as a master. * 1: Read * 0: Write"]
pub type CmdRwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_STOP` writer - This bit controls whether a STOP is issued after the byte is sent or received. * 1: STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. * 0: STOP is not issued after this byte."]
pub type CmdStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_RESTART` writer - This bit controls whether a RESTART is issued after the byte is sent or received. * 1: a RESTART is issued after the data is sent/received (according to the value of CMD_RW), regardless of whether or not the transfer direction is changing from the previous command."]
pub type CmdRestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NULL_DATA` writer - *1 : null data transfer"]
pub type NullDataW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, these bits are ignored by the I2C. However, when you read this register, these bits return the value of data received on the I2C interface."]
    #[inline(always)]
    pub fn ic_data(&self) -> IcDataR {
        IcDataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, these bits are ignored by the I2C. However, when you read this register, these bits return the value of data received on the I2C interface."]
    #[inline(always)]
    pub fn ic_data(&mut self) -> IcDataW<'_, IcDataCmdSpec> {
        IcDataW::new(self, 0)
    }
    #[doc = "Bit 8 - This bit controls whether a read or write is performed. This bit does not control the direction when the I2C acts as a slave. It controls only the direction when it acts as a master. * 1: Read * 0: Write"]
    #[inline(always)]
    pub fn cmd_rw(&mut self) -> CmdRwW<'_, IcDataCmdSpec> {
        CmdRwW::new(self, 8)
    }
    #[doc = "Bit 9 - This bit controls whether a STOP is issued after the byte is sent or received. * 1: STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. * 0: STOP is not issued after this byte."]
    #[inline(always)]
    pub fn cmd_stop(&mut self) -> CmdStopW<'_, IcDataCmdSpec> {
        CmdStopW::new(self, 9)
    }
    #[doc = "Bit 10 - This bit controls whether a RESTART is issued after the byte is sent or received. * 1: a RESTART is issued after the data is sent/received (according to the value of CMD_RW), regardless of whether or not the transfer direction is changing from the previous command."]
    #[inline(always)]
    pub fn cmd_restart(&mut self) -> CmdRestartW<'_, IcDataCmdSpec> {
        CmdRestartW::new(self, 10)
    }
    #[doc = "Bit 11 - *1 : null data transfer"]
    #[inline(always)]
    pub fn null_data(&mut self) -> NullDataW<'_, IcDataCmdSpec> {
        NullDataW::new(self, 11)
    }
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_data_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_data_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcDataCmdSpec;
impl crate::RegisterSpec for IcDataCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_data_cmd::R`](R) reader structure"]
impl crate::Readable for IcDataCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_data_cmd::W`](W) writer structure"]
impl crate::Writable for IcDataCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_DATA_CMD to value 0"]
impl crate::Resettable for IcDataCmdSpec {}
