#[doc = "Register `REG_SYSON_FSM_MON_V1` reader"]
pub type R = crate::R<RegSysonFsmMonV1Spec>;
#[doc = "Register `REG_SYSON_FSM_MON_V1` writer"]
pub type W = crate::W<RegSysonFsmMonV1Spec>;
#[doc = "Field `BIT_WLPMC_DEBUG` reader - debug signal of WL PMC"]
pub type BitWlpmcDebugR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WLPMC_DEBUG_SEL` reader - debug select signal for WL PMC"]
pub type BitWlpmcDebugSelR = crate::FieldReader;
#[doc = "Field `BIT_WLPMC_DEBUG_SEL` writer - debug select signal for WL PMC"]
pub type BitWlpmcDebugSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - Reserved"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - debug signal of WL PMC"]
    #[inline(always)]
    pub fn bit_wlpmc_debug(&self) -> BitWlpmcDebugR {
        BitWlpmcDebugR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - debug select signal for WL PMC"]
    #[inline(always)]
    pub fn bit_wlpmc_debug_sel(&self) -> BitWlpmcDebugSelR {
        BitWlpmcDebugSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - debug select signal for WL PMC"]
    #[inline(always)]
    pub fn bit_wlpmc_debug_sel(&mut self) -> BitWlpmcDebugSelW<'_, RegSysonFsmMonV1Spec> {
        BitWlpmcDebugSelW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegSysonFsmMonV1Spec> {
        RsvdW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_syson_fsm_mon_v1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_syson_fsm_mon_v1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSysonFsmMonV1Spec;
impl crate::RegisterSpec for RegSysonFsmMonV1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_syson_fsm_mon_v1::R`](R) reader structure"]
impl crate::Readable for RegSysonFsmMonV1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_syson_fsm_mon_v1::W`](W) writer structure"]
impl crate::Writable for RegSysonFsmMonV1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SYSON_FSM_MON_V1 to value 0"]
impl crate::Resettable for RegSysonFsmMonV1Spec {}
