#[doc = "Register `RxOkPhy` reader"]
pub type R = crate::R<RxOkPhySpec>;
#[doc = "Field `RxOkPhy` reader - 16-bit counter of Rx Ok packets with physical address matching destination address. Rolls over automatically. Write to clear."]
pub type RxOkPhyR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of Rx Ok packets with physical address matching destination address. Rolls over automatically. Write to clear."]
    #[inline(always)]
    pub fn rx_ok_phy(&self) -> RxOkPhyR {
        RxOkPhyR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ok_phy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOkPhySpec;
impl crate::RegisterSpec for RxOkPhySpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_ok_phy::R`](R) reader structure"]
impl crate::Readable for RxOkPhySpec {}
#[doc = "`reset()` method sets RxOkPhy to value 0"]
impl crate::Resettable for RxOkPhySpec {}
