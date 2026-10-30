#[doc = "Register `IC_TX_ABRT_SOURCE` reader"]
pub type R = crate::R<IcTxAbrtSourceSpec>;
#[doc = "Field `ABRT_7B_ADDR_NOACK` reader - * 1: Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave"]
pub type Abrt7bAddrNoackR = crate::BitReader;
#[doc = "Field `ABRT_10ADDR1_NOACK` reader - * 1: Master is in * 10-bit address mode and the first * 10-bit address byte was not acknowledged by any slave."]
pub type Abrt10addr1NoackR = crate::BitReader;
#[doc = "Field `ABRT_10ADDR2_NOACK` reader - * 1: Master is in * 10-bit address mode and the second byte of the * 10-bit address was not acknowledged by any slave."]
pub type Abrt10addr2NoackR = crate::BitReader;
#[doc = "Field `ABRT_TXDATA_NOACK` reader - * 1: This is a master-mode only bit. Master has received an acknowledgement for the address, but when it sent data byte(s) following the address, it did not receive an acknowledge from the remote slave(s)."]
pub type AbrtTxdataNoackR = crate::BitReader;
#[doc = "Field `ABRT_GCALL_NOACK` reader - * 1: I2C in master mode sent a General Call and no slave on the bus acknowledged the General Call."]
pub type AbrtGcallNoackR = crate::BitReader;
#[doc = "Field `ABRT_GCALL_READ` reader - * 1: I2C in master mode sent a General Call but the user programmed the byte following the General Call to be a read from the bus."]
pub type AbrtGcallReadR = crate::BitReader;
#[doc = "Field `ABRT_HS_ACKDET` reader - * 1: Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior)."]
pub type AbrtHsAckdetR = crate::BitReader;
#[doc = "Field `ABRT_SBYTE_ACKDET` reader - * 1: Master has sent a START Byte and the START Byte was acknowledged (wrong behavior)."]
pub type AbrtSbyteAckdetR = crate::BitReader;
#[doc = "Field `ABRT_HS_NORSTRT` reader - * 1: The restart is disabled and the user is trying to use the master to transfer data in High Speed mode."]
pub type AbrtHsNorstrtR = crate::BitReader;
#[doc = "Field `ABRT_SBYTE_NORSTRT` reader - * 1: The restart is disabled and the user is trying to send a START Byte."]
pub type AbrtSbyteNorstrtR = crate::BitReader;
#[doc = "Field `ABRT_10B_RD_NORSTRT` reader - * 1: The restart is disabled and the master sends a read command in * 10-bit addressing mode."]
pub type Abrt10bRdNorstrtR = crate::BitReader;
#[doc = "Field `ABRT_MASTER_DIS` reader - * 1: User tries to initiate a Master operation with the Master mode disabled."]
pub type AbrtMasterDisR = crate::BitReader;
#[doc = "Field `ARBT_LOST` reader - * 1: Master has lost arbitration, or if IC_TX_ABRT_SOURCE \\[* 14\\] is also set, then the slave transmitter has lost arbitration."]
pub type ArbtLostR = crate::BitReader;
#[doc = "Field `ABRT_SLVFLUSH_TXFIFO` reader - * 1: Slave has received a read command and some data exists in the TXFIFO so the slave issues a TX_ABRT interrupt to flush old data in TXFIFO."]
pub type AbrtSlvflushTxfifoR = crate::BitReader;
#[doc = "Field `ABRT_SLV_ARBLOST` reader - * 1: Slave lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE \\[12\\] is set at the same time."]
pub type AbrtSlvArblostR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - * 1: Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave"]
    #[inline(always)]
    pub fn abrt_7b_addr_noack(&self) -> Abrt7bAddrNoackR {
        Abrt7bAddrNoackR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - * 1: Master is in * 10-bit address mode and the first * 10-bit address byte was not acknowledged by any slave."]
    #[inline(always)]
    pub fn abrt_10addr1_noack(&self) -> Abrt10addr1NoackR {
        Abrt10addr1NoackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - * 1: Master is in * 10-bit address mode and the second byte of the * 10-bit address was not acknowledged by any slave."]
    #[inline(always)]
    pub fn abrt_10addr2_noack(&self) -> Abrt10addr2NoackR {
        Abrt10addr2NoackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - * 1: This is a master-mode only bit. Master has received an acknowledgement for the address, but when it sent data byte(s) following the address, it did not receive an acknowledge from the remote slave(s)."]
    #[inline(always)]
    pub fn abrt_txdata_noack(&self) -> AbrtTxdataNoackR {
        AbrtTxdataNoackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - * 1: I2C in master mode sent a General Call and no slave on the bus acknowledged the General Call."]
    #[inline(always)]
    pub fn abrt_gcall_noack(&self) -> AbrtGcallNoackR {
        AbrtGcallNoackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - * 1: I2C in master mode sent a General Call but the user programmed the byte following the General Call to be a read from the bus."]
    #[inline(always)]
    pub fn abrt_gcall_read(&self) -> AbrtGcallReadR {
        AbrtGcallReadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - * 1: Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior)."]
    #[inline(always)]
    pub fn abrt_hs_ackdet(&self) -> AbrtHsAckdetR {
        AbrtHsAckdetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - * 1: Master has sent a START Byte and the START Byte was acknowledged (wrong behavior)."]
    #[inline(always)]
    pub fn abrt_sbyte_ackdet(&self) -> AbrtSbyteAckdetR {
        AbrtSbyteAckdetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - * 1: The restart is disabled and the user is trying to use the master to transfer data in High Speed mode."]
    #[inline(always)]
    pub fn abrt_hs_norstrt(&self) -> AbrtHsNorstrtR {
        AbrtHsNorstrtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - * 1: The restart is disabled and the user is trying to send a START Byte."]
    #[inline(always)]
    pub fn abrt_sbyte_norstrt(&self) -> AbrtSbyteNorstrtR {
        AbrtSbyteNorstrtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - * 1: The restart is disabled and the master sends a read command in * 10-bit addressing mode."]
    #[inline(always)]
    pub fn abrt_10b_rd_norstrt(&self) -> Abrt10bRdNorstrtR {
        Abrt10bRdNorstrtR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - * 1: User tries to initiate a Master operation with the Master mode disabled."]
    #[inline(always)]
    pub fn abrt_master_dis(&self) -> AbrtMasterDisR {
        AbrtMasterDisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - * 1: Master has lost arbitration, or if IC_TX_ABRT_SOURCE \\[* 14\\] is also set, then the slave transmitter has lost arbitration."]
    #[inline(always)]
    pub fn arbt_lost(&self) -> ArbtLostR {
        ArbtLostR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - * 1: Slave has received a read command and some data exists in the TXFIFO so the slave issues a TX_ABRT interrupt to flush old data in TXFIFO."]
    #[inline(always)]
    pub fn abrt_slvflush_txfifo(&self) -> AbrtSlvflushTxfifoR {
        AbrtSlvflushTxfifoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - * 1: Slave lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE \\[12\\] is set at the same time."]
    #[inline(always)]
    pub fn abrt_slv_arblost(&self) -> AbrtSlvArblostR {
        AbrtSlvArblostR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "I2C Transmit Abort Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_tx_abrt_source::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcTxAbrtSourceSpec;
impl crate::RegisterSpec for IcTxAbrtSourceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_tx_abrt_source::R`](R) reader structure"]
impl crate::Readable for IcTxAbrtSourceSpec {}
#[doc = "`reset()` method sets IC_TX_ABRT_SOURCE to value 0"]
impl crate::Resettable for IcTxAbrtSourceSpec {}
