#[doc = "Register `RX_BEST_PHASE` reader"]
pub type R = crate::R<RxBestPhaseSpec>;
#[doc = "Field `rx_best_phase` reader - rx_best_phase"]
pub type RxBestPhaseR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - rx_best_phase"]
    #[inline(always)]
    pub fn rx_best_phase(&self) -> RxBestPhaseR {
        RxBestPhaseR::new(self.bits & 0x1f)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_best_phase::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxBestPhaseSpec;
impl crate::RegisterSpec for RxBestPhaseSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rx_best_phase::R`](R) reader structure"]
impl crate::Readable for RxBestPhaseSpec {}
#[doc = "`reset()` method sets RX_BEST_PHASE to value 0"]
impl crate::Resettable for RxBestPhaseSpec {}
