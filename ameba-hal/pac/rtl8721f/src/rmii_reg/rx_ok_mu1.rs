#[doc = "Register `RxOkMu1` reader"]
pub type R = crate::R<RxOkMu1Spec>;
#[doc = "Field `RxOkMul` reader - 16-bit counter of Rx Ok packets with multicast destination address. Rolls over automatically. Write to clear."]
pub type RxOkMulR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of Rx Ok packets with multicast destination address. Rolls over automatically. Write to clear."]
    #[inline(always)]
    pub fn rx_ok_mul(&self) -> RxOkMulR {
        RxOkMulR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ok_mu1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOkMu1Spec;
impl crate::RegisterSpec for RxOkMu1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_ok_mu1::R`](R) reader structure"]
impl crate::Readable for RxOkMu1Spec {}
#[doc = "`reset()` method sets RxOkMu1 to value 0"]
impl crate::Resettable for RxOkMu1Spec {}
