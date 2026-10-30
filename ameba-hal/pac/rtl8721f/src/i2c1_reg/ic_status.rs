#[doc = "Register `IC_STATUS` reader"]
pub type R = crate::R<IcStatusSpec>;
#[doc = "Field `IC_ACTIVITY` reader - I2C Activity Status."]
pub type IcActivityR = crate::BitReader;
#[doc = "Field `TFNF` reader - Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. * 0: Transmit FIFO is full * 1: Transmit FIFO is not full"]
pub type TfnfR = crate::BitReader;
#[doc = "Field `TFE` reader - Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. * 0: Transmit FIFO is not empty * 1: Transmit FIFO is empty"]
pub type TfeR = crate::BitReader;
#[doc = "Field `RFNE` reader - Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. * 0: Receive FIFO is empty * 1: Receive FIFO is not empty"]
pub type RfneR = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. * 0: Receive FIFO is not full * 1: Receive FIFO is full"]
pub type RffR = crate::BitReader;
#[doc = "Field `MST_ACTIVITY` reader - Master FSM Activity Status. When the Master FSM is not in the IDLE state, this bit is set. * 0: Master FSM is in the IDLE state so the master part of I2C is not active * 1: Master FSM is not he IDLE state so the master part of I2C is active"]
pub type MstActivityR = crate::BitReader;
#[doc = "Field `SLV_ACTIVITY` reader - Slave FSM Activity Status. When the Slave FSM is not in the IDLE state, this bit is set. * 0: Slave FSM is in the IDLE state so the slave part of I2C is not active * 1: Slave FSM is not he IDLE state so the slave part of I2C is active"]
pub type SlvActivityR = crate::BitReader;
#[doc = "Field `bus_monitor_status` reader - BUS monitor status: * 2'b00: IDLE * 2'b01: ADDR * 2'b10: DATA * 2'b11:STRTCH"]
pub type BusMonitorStatusR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - I2C Activity Status."]
    #[inline(always)]
    pub fn ic_activity(&self) -> IcActivityR {
        IcActivityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. * 0: Transmit FIFO is full * 1: Transmit FIFO is not full"]
    #[inline(always)]
    pub fn tfnf(&self) -> TfnfR {
        TfnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. * 0: Transmit FIFO is not empty * 1: Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. * 0: Receive FIFO is empty * 1: Receive FIFO is not empty"]
    #[inline(always)]
    pub fn rfne(&self) -> RfneR {
        RfneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. * 0: Receive FIFO is not full * 1: Receive FIFO is full"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master FSM Activity Status. When the Master FSM is not in the IDLE state, this bit is set. * 0: Master FSM is in the IDLE state so the master part of I2C is not active * 1: Master FSM is not he IDLE state so the master part of I2C is active"]
    #[inline(always)]
    pub fn mst_activity(&self) -> MstActivityR {
        MstActivityR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slave FSM Activity Status. When the Slave FSM is not in the IDLE state, this bit is set. * 0: Slave FSM is in the IDLE state so the slave part of I2C is not active * 1: Slave FSM is not he IDLE state so the slave part of I2C is active"]
    #[inline(always)]
    pub fn slv_activity(&self) -> SlvActivityR {
        SlvActivityR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - BUS monitor status: * 2'b00: IDLE * 2'b01: ADDR * 2'b10: DATA * 2'b11:STRTCH"]
    #[inline(always)]
    pub fn bus_monitor_status(&self) -> BusMonitorStatusR {
        BusMonitorStatusR::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "I2C Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcStatusSpec;
impl crate::RegisterSpec for IcStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_status::R`](R) reader structure"]
impl crate::Readable for IcStatusSpec {}
#[doc = "`reset()` method sets IC_STATUS to value 0x06"]
impl crate::Resettable for IcStatusSpec {
    const RESET_VALUE: u32 = 0x06;
}
