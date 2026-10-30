#[doc = "Register `EEE_LPI_TMO_REG` reader"]
pub type R = crate::R<EeeLpiTmoRegSpec>;
#[doc = "Field `EEE_RXLPI_Time` reader - RX in LPI time. (unit: 1us) (accumulation)Valid as (PORT_MASK\\[0\\]==1&&RXTimer_EN==1)"]
pub type EeeRxlpiTimeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - RX in LPI time. (unit: 1us) (accumulation)Valid as (PORT_MASK\\[0\\]==1&&RXTimer_EN==1)"]
    #[inline(always)]
    pub fn eee_rxlpi_time(&self) -> EeeRxlpiTimeR {
        EeeRxlpiTimeR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eee_lpi_tmo_reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeeLpiTmoRegSpec;
impl crate::RegisterSpec for EeeLpiTmoRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eee_lpi_tmo_reg::R`](R) reader structure"]
impl crate::Readable for EeeLpiTmoRegSpec {}
#[doc = "`reset()` method sets EEE_LPI_TMO_REG to value 0"]
impl crate::Resettable for EeeLpiTmoRegSpec {}
