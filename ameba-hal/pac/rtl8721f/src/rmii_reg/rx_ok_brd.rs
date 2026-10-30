#[doc = "Register `RxOkBrd` reader"]
pub type R = crate::R<RxOkBrdSpec>;
#[doc = "Field `RxOkBrd` reader - 16-bit counter of Rx Ok packets with broadcast destination address.. Rolls over automatically. Write to clear."]
pub type RxOkBrdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of Rx Ok packets with broadcast destination address.. Rolls over automatically. Write to clear."]
    #[inline(always)]
    pub fn rx_ok_brd(&self) -> RxOkBrdR {
        RxOkBrdR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ok_brd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOkBrdSpec;
impl crate::RegisterSpec for RxOkBrdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_ok_brd::R`](R) reader structure"]
impl crate::Readable for RxOkBrdSpec {}
#[doc = "`reset()` method sets RxOkBrd to value 0"]
impl crate::Resettable for RxOkBrdSpec {}
