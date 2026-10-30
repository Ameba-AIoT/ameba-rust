#[doc = "Register `RX_FIFO_0_RD_ADDR` reader"]
pub type R = crate::R<RxFifo0RdAddrSpec>;
#[doc = "Field `RX_FIFO_0_RD_ADDR` reader - RX_FIFO_0_RD_ADDR"]
pub type RxFifo0RdAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RX_FIFO_0_RD_ADDR"]
    #[inline(always)]
    pub fn rx_fifo_0_rd_addr(&self) -> RxFifo0RdAddrR {
        RxFifo0RdAddrR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_0_rd_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifo0RdAddrSpec;
impl crate::RegisterSpec for RxFifo0RdAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_0_rd_addr::R`](R) reader structure"]
impl crate::Readable for RxFifo0RdAddrSpec {}
#[doc = "`reset()` method sets RX_FIFO_0_RD_ADDR to value 0"]
impl crate::Resettable for RxFifo0RdAddrSpec {}
