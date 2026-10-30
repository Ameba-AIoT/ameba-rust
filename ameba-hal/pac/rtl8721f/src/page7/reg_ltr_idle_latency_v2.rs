#[doc = "Register `REG_LTR_IDLE_LATENCY_V2` reader"]
pub type R = crate::R<RegLtrIdleLatencyV2Spec>;
#[doc = "Register `REG_LTR_IDLE_LATENCY_V2` writer"]
pub type W = crate::W<RegLtrIdleLatencyV2Spec>;
#[doc = "Field `BIT_OBFF_CTRL_0` reader - "]
pub type BitObffCtrl0R = crate::FieldReader;
#[doc = "Field `BIT_OBFF_CTRL_1` reader - "]
pub type BitObffCtrl1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_OBFF_CTRL_1` writer - "]
pub type BitObffCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BIT_OBFF_STATE` reader - "]
pub type BitObffStateR = crate::FieldReader;
#[doc = "Field `BIT_LTR_IDLE_LATENCY` reader - LTR idle latency."]
pub type BitLtrIdleLatencyR = crate::FieldReader;
#[doc = "Field `BIT_LTR_IDLE_LATENCY` writer - LTR idle latency."]
pub type BitLtrIdleLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn bit_obff_ctrl_0(&self) -> BitObffCtrl0R {
        BitObffCtrl0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:27"]
    #[inline(always)]
    pub fn bit_obff_ctrl_1(&self) -> BitObffCtrl1R {
        BitObffCtrl1R::new((self.bits >> 4) & 0x00ff_ffff)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn bit_obff_state(&self) -> BitObffStateR {
        BitObffStateR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - LTR idle latency."]
    #[inline(always)]
    pub fn bit_ltr_idle_latency(&self) -> BitLtrIdleLatencyR {
        BitLtrIdleLatencyR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:27"]
    #[inline(always)]
    pub fn bit_obff_ctrl_1(&mut self) -> BitObffCtrl1W<'_, RegLtrIdleLatencyV2Spec> {
        BitObffCtrl1W::new(self, 4)
    }
    #[doc = "Bits 30:31 - LTR idle latency."]
    #[inline(always)]
    pub fn bit_ltr_idle_latency(&mut self) -> BitLtrIdleLatencyW<'_, RegLtrIdleLatencyV2Spec> {
        BitLtrIdleLatencyW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ltr_idle_latency_v2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ltr_idle_latency_v2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLtrIdleLatencyV2Spec;
impl crate::RegisterSpec for RegLtrIdleLatencyV2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ltr_idle_latency_v2::R`](R) reader structure"]
impl crate::Readable for RegLtrIdleLatencyV2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ltr_idle_latency_v2::W`](W) writer structure"]
impl crate::Writable for RegLtrIdleLatencyV2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LTR_IDLE_LATENCY_V2 to value 0x0444_4400"]
impl crate::Resettable for RegLtrIdleLatencyV2Spec {
    const RESET_VALUE: u32 = 0x0444_4400;
}
