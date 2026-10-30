#[doc = "Register `REG_RX_BCN_TM` reader"]
pub type R = crate::R<RegRxBcnTmSpec>;
#[doc = "Field `BIT_RX_BCN_TIMER` reader - indicate rx beacon timer from cca up to fcs ok, units us"]
pub type BitRxBcnTimerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - indicate rx beacon timer from cca up to fcs ok, units us"]
    #[inline(always)]
    pub fn bit_rx_bcn_timer(&self) -> BitRxBcnTimerR {
        BitRxBcnTimerR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "rx beacon timer\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_bcn_tm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxBcnTmSpec;
impl crate::RegisterSpec for RegRxBcnTmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_bcn_tm::R`](R) reader structure"]
impl crate::Readable for RegRxBcnTmSpec {}
#[doc = "`reset()` method sets REG_RX_BCN_TM to value 0"]
impl crate::Resettable for RegRxBcnTmSpec {}
