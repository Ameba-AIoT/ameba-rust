#[doc = "Register `RESULT_LAYER_BUS_CFG` reader"]
pub type R = crate::R<ResultLayerBusCfgSpec>;
#[doc = "Register `RESULT_LAYER_BUS_CFG` writer"]
pub type W = crate::W<ResultLayerBusCfgSpec>;
#[doc = "Field `RESULT_LAYER_AXSIZE` reader - AWSize Refer to AXI4 protocol. (AXI cmd size must be less than or equal to BUS width)"]
pub type ResultLayerAxsizeR = crate::FieldReader;
#[doc = "Field `RESULT_LAYER_AXSIZE` writer - AWSize Refer to AXI4 protocol. (AXI cmd size must be less than or equal to BUS width)"]
pub type ResultLayerAxsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESULT_LAYER_INCR` reader - AWBurst 0x1: AWBurst = INCR 0x0: AWBurst = FIXED"]
pub type ResultLayerIncrR = crate::BitReader;
#[doc = "Field `RESULT_LAYER_AXCACHE` reader - AWCache Refer to AXI4 protocol."]
pub type ResultLayerAxcacheR = crate::FieldReader;
#[doc = "Field `RESULT_LAYER_AXCACHE` writer - AWCache Refer to AXI4 protocol."]
pub type ResultLayerAxcacheW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESULT_LAYER_MAX_AXLEN_LOG` reader - Limit of AWLen"]
pub type ResultLayerMaxAxlenLogR = crate::FieldReader;
#[doc = "Field `RESULT_LAYER_MAX_AXLEN_LOG` writer - Limit of AWLen"]
pub type ResultLayerMaxAxlenLogW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESULT_LAYER_PRIOR` reader - Priority (AWQos)"]
pub type ResultLayerPriorR = crate::FieldReader;
#[doc = "Field `RESULT_LAYER_PRIOR` writer - Priority (AWQos)"]
pub type ResultLayerPriorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - AWSize Refer to AXI4 protocol. (AXI cmd size must be less than or equal to BUS width)"]
    #[inline(always)]
    pub fn result_layer_axsize(&self) -> ResultLayerAxsizeR {
        ResultLayerAxsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - AWBurst 0x1: AWBurst = INCR 0x0: AWBurst = FIXED"]
    #[inline(always)]
    pub fn result_layer_incr(&self) -> ResultLayerIncrR {
        ResultLayerIncrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - AWCache Refer to AXI4 protocol."]
    #[inline(always)]
    pub fn result_layer_axcache(&self) -> ResultLayerAxcacheR {
        ResultLayerAxcacheR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Limit of AWLen"]
    #[inline(always)]
    pub fn result_layer_max_axlen_log(&self) -> ResultLayerMaxAxlenLogR {
        ResultLayerMaxAxlenLogR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - Priority (AWQos)"]
    #[inline(always)]
    pub fn result_layer_prior(&self) -> ResultLayerPriorR {
        ResultLayerPriorR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AWSize Refer to AXI4 protocol. (AXI cmd size must be less than or equal to BUS width)"]
    #[inline(always)]
    pub fn result_layer_axsize(&mut self) -> ResultLayerAxsizeW<'_, ResultLayerBusCfgSpec> {
        ResultLayerAxsizeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AWCache Refer to AXI4 protocol."]
    #[inline(always)]
    pub fn result_layer_axcache(&mut self) -> ResultLayerAxcacheW<'_, ResultLayerBusCfgSpec> {
        ResultLayerAxcacheW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Limit of AWLen"]
    #[inline(always)]
    pub fn result_layer_max_axlen_log(
        &mut self,
    ) -> ResultLayerMaxAxlenLogW<'_, ResultLayerBusCfgSpec> {
        ResultLayerMaxAxlenLogW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Priority (AWQos)"]
    #[inline(always)]
    pub fn result_layer_prior(&mut self) -> ResultLayerPriorW<'_, ResultLayerBusCfgSpec> {
        ResultLayerPriorW::new(self, 12)
    }
}
#[doc = "Master port BUS configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`result_layer_bus_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result_layer_bus_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultLayerBusCfgSpec;
impl crate::RegisterSpec for ResultLayerBusCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result_layer_bus_cfg::R`](R) reader structure"]
impl crate::Readable for ResultLayerBusCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`result_layer_bus_cfg::W`](W) writer structure"]
impl crate::Writable for ResultLayerBusCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESULT_LAYER_BUS_CFG to value 0x070a"]
impl crate::Resettable for ResultLayerBusCfgSpec {
    const RESET_VALUE: u32 = 0x070a;
}
