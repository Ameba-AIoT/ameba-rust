#[doc = "Register `REG_P2_TSFTR_H` reader"]
pub type R = crate::R<RegP2TsftrHSpec>;
#[doc = "Field `BIT_P2_TSF_TIMER_H` reader - high 32bits of port2's tsft(NAN)"]
pub type BitP2TsfTimerHR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - high 32bits of port2's tsft(NAN)"]
    #[inline(always)]
    pub fn bit_p2_tsf_timer_h(&self) -> BitP2TsfTimerHR {
        BitP2TsfTimerHR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p2_tsftr_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP2TsftrHSpec;
impl crate::RegisterSpec for RegP2TsftrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p2_tsftr_h::R`](R) reader structure"]
impl crate::Readable for RegP2TsftrHSpec {}
#[doc = "`reset()` method sets REG_P2_TSFTR_H to value 0x01"]
impl crate::Resettable for RegP2TsftrHSpec {
    const RESET_VALUE: u32 = 0x01;
}
