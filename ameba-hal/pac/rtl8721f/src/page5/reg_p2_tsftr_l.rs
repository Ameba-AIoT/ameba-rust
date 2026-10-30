#[doc = "Register `REG_P2_TSFTR_L` reader"]
pub type R = crate::R<RegP2TsftrLSpec>;
#[doc = "Field `BIT_P2_TSF_TIMER_L` reader - low 32bits of port2's tsft (NAN)"]
pub type BitP2TsfTimerLR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - low 32bits of port2's tsft (NAN)"]
    #[inline(always)]
    pub fn bit_p2_tsf_timer_l(&self) -> BitP2TsfTimerLR {
        BitP2TsfTimerLR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p2_tsftr_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP2TsftrLSpec;
impl crate::RegisterSpec for RegP2TsftrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p2_tsftr_l::R`](R) reader structure"]
impl crate::Readable for RegP2TsftrLSpec {}
#[doc = "`reset()` method sets REG_P2_TSFTR_L to value 0x01"]
impl crate::Resettable for RegP2TsftrLSpec {
    const RESET_VALUE: u32 = 0x01;
}
