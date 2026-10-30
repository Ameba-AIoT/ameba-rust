#[doc = "Register `IC_RAW_INTR_STAT` reader"]
pub type R = crate::R<IcRawIntrStatSpec>;
#[doc = "Field `RX_UNDER` reader - Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register."]
pub type RxUnderR = crate::BitReader;
#[doc = "Field `RX_OVER` reader - Not used. Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device."]
pub type RxOverR = crate::BitReader;
#[doc = "Field `RX_FULL` reader - Set when the receive buffer goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level is equal to or less than the threshold."]
pub type RxFullR = crate::BitReader;
#[doc = "Field `TX_OVER` reader - Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register."]
pub type TxOverR = crate::BitReader;
#[doc = "Field `TX_EMPTY` reader - This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. It is automatically cleared by hardware when the buffer level goes above the threshold."]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `RD_REQ` reader - This bit is set to 1 when I2C is acting as a slave and another I2C master is attempting to read data from I2C. The I2C holds the I2C bus in a wait state(SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register."]
pub type RdReqR = crate::BitReader;
#[doc = "Field `TX_ABRT` reader - This bit indicates if I2C as a transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. * 1: the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort take places."]
pub type TxAbrtR = crate::BitReader;
#[doc = "Field `RX_DONE` reader - When the I2C is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done."]
pub type RxDoneR = crate::BitReader;
#[doc = "Field `ACTIVITY` reader - This bit captures I2C activity."]
pub type ActivityR = crate::BitReader;
#[doc = "Field `STOP_DET` reader - Indicates whether a STOP condition has occurred on the I2C interface regardless of whether I2C is operating in slave or master mode."]
pub type StopDetR = crate::BitReader;
#[doc = "Field `START_DET` reader - Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether I2C is operating in slave or master mode."]
pub type StartDetR = crate::BitReader;
#[doc = "Field `GEN_CALL` reader - Set only when a General Call address is received and it is acknowledged."]
pub type GenCallR = crate::BitReader;
#[doc = "Field `LP_WAKE_1` reader - Set when address SAR match with address sending on I2C BUS."]
pub type LpWake1R = crate::BitReader;
#[doc = "Field `LP_WAKE_2` reader - Set when address SAR_2 match with address sending on I2C BUS."]
pub type LpWake2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register."]
    #[inline(always)]
    pub fn rx_under(&self) -> RxUnderR {
        RxUnderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Not used. Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device."]
    #[inline(always)]
    pub fn rx_over(&self) -> RxOverR {
        RxOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set when the receive buffer goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level is equal to or less than the threshold."]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register."]
    #[inline(always)]
    pub fn tx_over(&self) -> TxOverR {
        TxOverR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. It is automatically cleared by hardware when the buffer level goes above the threshold."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set to 1 when I2C is acting as a slave and another I2C master is attempting to read data from I2C. The I2C holds the I2C bus in a wait state(SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register."]
    #[inline(always)]
    pub fn rd_req(&self) -> RdReqR {
        RdReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit indicates if I2C as a transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. * 1: the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort take places."]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TxAbrtR {
        TxAbrtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When the I2C is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done."]
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit captures I2C activity."]
    #[inline(always)]
    pub fn activity(&self) -> ActivityR {
        ActivityR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates whether a STOP condition has occurred on the I2C interface regardless of whether I2C is operating in slave or master mode."]
    #[inline(always)]
    pub fn stop_det(&self) -> StopDetR {
        StopDetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether I2C is operating in slave or master mode."]
    #[inline(always)]
    pub fn start_det(&self) -> StartDetR {
        StartDetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set only when a General Call address is received and it is acknowledged."]
    #[inline(always)]
    pub fn gen_call(&self) -> GenCallR {
        GenCallR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set when address SAR match with address sending on I2C BUS."]
    #[inline(always)]
    pub fn lp_wake_1(&self) -> LpWake1R {
        LpWake1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set when address SAR_2 match with address sending on I2C BUS."]
    #[inline(always)]
    pub fn lp_wake_2(&self) -> LpWake2R {
        LpWake2R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "I2C Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_raw_intr_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcRawIntrStatSpec;
impl crate::RegisterSpec for IcRawIntrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_raw_intr_stat::R`](R) reader structure"]
impl crate::Readable for IcRawIntrStatSpec {}
#[doc = "`reset()` method sets IC_RAW_INTR_STAT to value 0"]
impl crate::Resettable for IcRawIntrStatSpec {}
