#[doc = "Register `RxErr` reader"]
pub type R = crate::R<RxErrSpec>;
#[doc = "Field `RxErr` reader - 16-bit counter of RxErr packets. Rolls over automatically. Write to clear. The source of RxErr are GMAC meets the following condition when rx fifo is not full.: 1. RCR.accept runt = 0, but the ingress pkt length < 64B. 2. RCR.accepterr = 0, but the ingress pkt is a CRC error pkt."]
pub type RxErrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of RxErr packets. Rolls over automatically. Write to clear. The source of RxErr are GMAC meets the following condition when rx fifo is not full.: 1. RCR.accept runt = 0, but the ingress pkt length < 64B. 2. RCR.accepterr = 0, but the ingress pkt is a CRC error pkt."]
    #[inline(always)]
    pub fn rx_err(&self) -> RxErrR {
        RxErrR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_err::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxErrSpec;
impl crate::RegisterSpec for RxErrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_err::R`](R) reader structure"]
impl crate::Readable for RxErrSpec {}
#[doc = "`reset()` method sets RxErr to value 0"]
impl crate::Resettable for RxErrSpec {}
