#[doc = "Register `TXUICR` reader"]
pub type R = crate::R<TxuicrSpec>;
#[doc = "Field `TXUICR` reader - When SPI is configured as serial-slave, this register is used to Clear Transmit FIFO Underflow Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_txu_intr interrupt; writing has no effect."]
pub type TxuicrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When SPI is configured as serial-slave, this register is used to Clear Transmit FIFO Underflow Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_txu_intr interrupt; writing has no effect."]
    #[inline(always)]
    pub fn txuicr(&self) -> TxuicrR {
        TxuicrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Transmit FIFO Underflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txuicr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxuicrSpec;
impl crate::RegisterSpec for TxuicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txuicr::R`](R) reader structure"]
impl crate::Readable for TxuicrSpec {}
#[doc = "`reset()` method sets TXUICR to value 0"]
impl crate::Resettable for TxuicrSpec {}
