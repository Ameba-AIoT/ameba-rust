#[doc = "Register `LCDC_DEBUG_STATUS` reader"]
pub type R = crate::R<LcdcDebugStatusSpec>;
#[doc = "Field `lcdc_debug_states` reader - LCDC critial signal for debug"]
pub type LcdcDebugStatesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - LCDC critial signal for debug"]
    #[inline(always)]
    pub fn lcdc_debug_states(&self) -> LcdcDebugStatesR {
        LcdcDebugStatesR::new(self.bits)
    }
}
#[doc = "Debug signal\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_debug_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcDebugStatusSpec;
impl crate::RegisterSpec for LcdcDebugStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_debug_status::R`](R) reader structure"]
impl crate::Readable for LcdcDebugStatusSpec {}
#[doc = "`reset()` method sets LCDC_DEBUG_STATUS to value 0"]
impl crate::Resettable for LcdcDebugStatusSpec {}
