#[doc = "Register `LL_CFG` reader"]
pub type R = crate::R<LlCfgSpec>;
#[doc = "Register `LL_CFG` writer"]
pub type W = crate::W<LlCfgSpec>;
#[doc = "Field `Layer_LL_EN` reader - Update layers registers by linked list enable Bit0~15: Layer0~15"]
pub type LayerLlEnR = crate::FieldReader;
#[doc = "Field `Layer_LL_EN` writer - Update layers registers by linked list enable Bit0~15: Layer0~15"]
pub type LayerLlEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LL_ARSIZE` reader - ARSize for reading linked list item (Reserved)"]
pub type LlArsizeR = crate::FieldReader;
#[doc = "Field `LL_INCR` reader - INCR or FIXED burst for reading linked list item (Reserved)"]
pub type LlIncrR = crate::BitReader;
#[doc = "Field `LL_ARCACHE` reader - ARCache (Reserved). Refer to AXI4 protocol."]
pub type LlArcacheR = crate::FieldReader;
#[doc = "Field `LL_MAX_ARLEN_LOG` reader - Limit of ARLen (Reserved)"]
pub type LlMaxArlenLogR = crate::FieldReader;
#[doc = "Field `LL_PRIOR` reader - Priority (ARQos) (Reserved)"]
pub type LlPriorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Update layers registers by linked list enable Bit0~15: Layer0~15"]
    #[inline(always)]
    pub fn layer_ll_en(&self) -> LayerLlEnR {
        LayerLlEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - ARSize for reading linked list item (Reserved)"]
    #[inline(always)]
    pub fn ll_arsize(&self) -> LlArsizeR {
        LlArsizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - INCR or FIXED burst for reading linked list item (Reserved)"]
    #[inline(always)]
    pub fn ll_incr(&self) -> LlIncrR {
        LlIncrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - ARCache (Reserved). Refer to AXI4 protocol."]
    #[inline(always)]
    pub fn ll_arcache(&self) -> LlArcacheR {
        LlArcacheR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Limit of ARLen (Reserved)"]
    #[inline(always)]
    pub fn ll_max_arlen_log(&self) -> LlMaxArlenLogR {
        LlMaxArlenLogR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Priority (ARQos) (Reserved)"]
    #[inline(always)]
    pub fn ll_prior(&self) -> LlPriorR {
        LlPriorR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update layers registers by linked list enable Bit0~15: Layer0~15"]
    #[inline(always)]
    pub fn layer_ll_en(&mut self) -> LayerLlEnW<'_, LlCfgSpec> {
        LayerLlEnW::new(self, 0)
    }
}
#[doc = "Linked List configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ll_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ll_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LlCfgSpec;
impl crate::RegisterSpec for LlCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ll_cfg::R`](R) reader structure"]
impl crate::Readable for LlCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ll_cfg::W`](W) writer structure"]
impl crate::Writable for LlCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LL_CFG to value 0x072a_0000"]
impl crate::Resettable for LlCfgSpec {
    const RESET_VALUE: u32 = 0x072a_0000;
}
