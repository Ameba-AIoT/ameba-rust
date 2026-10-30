#[doc = "Register `RX_FIFO_1_RD_ADDR` reader"]
pub type R = crate::R<RxFifo1RdAddrSpec>;
#[doc = "Field `RX_FIFO_1_RD_ADDR` reader - RX_FIFO_1_RD_ADDR"]
pub type RxFifo1RdAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RX_FIFO_1_RD_ADDR"]
    #[inline(always)]
    pub fn rx_fifo_1_rd_addr(&self) -> RxFifo1RdAddrR {
        RxFifo1RdAddrR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_1_rd_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifo1RdAddrSpec;
impl crate::RegisterSpec for RxFifo1RdAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_1_rd_addr::R`](R) reader structure"]
impl crate::Readable for RxFifo1RdAddrSpec {}
#[doc = "`reset()` method sets RX_FIFO_1_RD_ADDR to value 0"]
impl crate::Resettable for RxFifo1RdAddrSpec {}
