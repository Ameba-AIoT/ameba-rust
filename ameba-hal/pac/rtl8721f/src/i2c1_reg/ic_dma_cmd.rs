#[doc = "Register `IC_DMA_CMD` reader"]
pub type R = crate::R<IcDmaCmdSpec>;
#[doc = "Register `IC_DMA_CMD` writer"]
pub type W = crate::W<IcDmaCmdSpec>;
#[doc = "Field `DMODE_ENABLE` writer - * 1: Set to enable dma mode, clear when transfer is done"]
pub type DmodeEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMODE_CMD` reader - This bit controls whether a read or write is performed in DMA mode. This bit does not control the direction when the I2C acts as a slave. It controls only the direction when it acts as a master. * 1: Read * 0: Write"]
pub type DmodeCmdR = crate::BitReader;
#[doc = "Field `DMODE_CMD` writer - This bit controls whether a read or write is performed in DMA mode. This bit does not control the direction when the I2C acts as a slave. It controls only the direction when it acts as a master. * 1: Read * 0: Write"]
pub type DmodeCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMODE_STOP` reader - This bit controls whether a STOP is issued after the byte is sent or received in DMA mode. * 1: STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. * 0: STOP is not issued after this byte."]
pub type DmodeStopR = crate::BitReader;
#[doc = "Field `DMODE_STOP` writer - This bit controls whether a STOP is issued after the byte is sent or received in DMA mode. * 1: STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. * 0: STOP is not issued after this byte."]
pub type DmodeStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMODE_RESTART` reader - This bit controls whether a RESTART is issued after the byte is sent or received in DMA mode. * 1: a RESTART is issued after the data is sent/received (according to the value of CMD_RW), regardless of whether or not the transfer direction is changing from the previous command."]
pub type DmodeRestartR = crate::BitReader;
#[doc = "Field `DMODE_RESTART` writer - This bit controls whether a RESTART is issued after the byte is sent or received in DMA mode. * 1: a RESTART is issued after the data is sent/received (according to the value of CMD_RW), regardless of whether or not the transfer direction is changing from the previous command."]
pub type DmodeRestartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - This bit controls whether a read or write is performed in DMA mode. This bit does not control the direction when the I2C acts as a slave. It controls only the direction when it acts as a master. * 1: Read * 0: Write"]
    #[inline(always)]
    pub fn dmode_cmd(&self) -> DmodeCmdR {
        DmodeCmdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit controls whether a STOP is issued after the byte is sent or received in DMA mode. * 1: STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. * 0: STOP is not issued after this byte."]
    #[inline(always)]
    pub fn dmode_stop(&self) -> DmodeStopR {
        DmodeStopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit controls whether a RESTART is issued after the byte is sent or received in DMA mode. * 1: a RESTART is issued after the data is sent/received (according to the value of CMD_RW), regardless of whether or not the transfer direction is changing from the previous command."]
    #[inline(always)]
    pub fn dmode_restart(&self) -> DmodeRestartR {
        DmodeRestartR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - * 1: Set to enable dma mode, clear when transfer is done"]
    #[inline(always)]
    pub fn dmode_enable(&mut self) -> DmodeEnableW<'_, IcDmaCmdSpec> {
        DmodeEnableW::new(self, 0)
    }
    #[doc = "Bit 5 - This bit controls whether a read or write is performed in DMA mode. This bit does not control the direction when the I2C acts as a slave. It controls only the direction when it acts as a master. * 1: Read * 0: Write"]
    #[inline(always)]
    pub fn dmode_cmd(&mut self) -> DmodeCmdW<'_, IcDmaCmdSpec> {
        DmodeCmdW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit controls whether a STOP is issued after the byte is sent or received in DMA mode. * 1: STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. * 0: STOP is not issued after this byte."]
    #[inline(always)]
    pub fn dmode_stop(&mut self) -> DmodeStopW<'_, IcDmaCmdSpec> {
        DmodeStopW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit controls whether a RESTART is issued after the byte is sent or received in DMA mode. * 1: a RESTART is issued after the data is sent/received (according to the value of CMD_RW), regardless of whether or not the transfer direction is changing from the previous command."]
    #[inline(always)]
    pub fn dmode_restart(&mut self) -> DmodeRestartW<'_, IcDmaCmdSpec> {
        DmodeRestartW::new(self, 7)
    }
}
#[doc = "I2C DMA Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_dma_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcDmaCmdSpec;
impl crate::RegisterSpec for IcDmaCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_dma_cmd::R`](R) reader structure"]
impl crate::Readable for IcDmaCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_dma_cmd::W`](W) writer structure"]
impl crate::Writable for IcDmaCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_DMA_CMD to value 0"]
impl crate::Resettable for IcDmaCmdSpec {}
