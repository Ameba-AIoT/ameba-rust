#[doc = "Register `REG_LTR_ACTIVE_LATENCY_V2` reader"]
pub type R = crate::R<RegLtrActiveLatencyV2Spec>;
#[doc = "Register `REG_LTR_ACTIVE_LATENCY_V2` writer"]
pub type W = crate::W<RegLtrActiveLatencyV2Spec>;
#[doc = "Field `BIT_LTR_ACTIVE_LATENCY` reader - LTR Active latency."]
pub type BitLtrActiveLatencyR = crate::FieldReader<u32>;
#[doc = "Field `BIT_LTR_ACTIVE_LATENCY` writer - LTR Active latency."]
pub type BitLtrActiveLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LTR Active latency."]
    #[inline(always)]
    pub fn bit_ltr_active_latency(&self) -> BitLtrActiveLatencyR {
        BitLtrActiveLatencyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LTR Active latency."]
    #[inline(always)]
    pub fn bit_ltr_active_latency(
        &mut self,
    ) -> BitLtrActiveLatencyW<'_, RegLtrActiveLatencyV2Spec> {
        BitLtrActiveLatencyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ltr_active_latency_v2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ltr_active_latency_v2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLtrActiveLatencyV2Spec;
impl crate::RegisterSpec for RegLtrActiveLatencyV2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ltr_active_latency_v2::R`](R) reader structure"]
impl crate::Readable for RegLtrActiveLatencyV2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ltr_active_latency_v2::W`](W) writer structure"]
impl crate::Writable for RegLtrActiveLatencyV2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LTR_ACTIVE_LATENCY_V2 to value 0x0203_0405"]
impl crate::Resettable for RegLtrActiveLatencyV2Spec {
    const RESET_VALUE: u32 = 0x0203_0405;
}
