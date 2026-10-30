#[doc = "Register `INPUT_LAYER2_BUS_CFG` reader"]
pub type R = crate::R<InputLayer2BusCfgSpec>;
#[doc = "Register `INPUT_LAYER2_BUS_CFG` writer"]
pub type W = crate::W<InputLayer2BusCfgSpec>;
#[doc = "Field `INPUT_LAYER2_AXSIZE` reader - ARSize Refer to AXI4 protocol. · AXI cmd size must be less than or equal to BUS width. · Must be 0x2 when AXCACHE=6."]
pub type InputLayer2AxsizeR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER2_AXSIZE` writer - ARSize Refer to AXI4 protocol. · AXI cmd size must be less than or equal to BUS width. · Must be 0x2 when AXCACHE=6."]
pub type InputLayer2AxsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INPUT_LAYER2_INCR` reader - ARBurst 0x1: ARBurst = INCR 0x0: ARBurst = FIXED"]
pub type InputLayer2IncrR = crate::BitReader;
#[doc = "Field `INPUT_LAYER2_AXCACHE` reader - ARCache Refer to AXI4 protocol. Note: Set to 0x6 to enable read buffer for input layer"]
pub type InputLayer2AxcacheR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER2_AXCACHE` writer - ARCache Refer to AXI4 protocol. Note: Set to 0x6 to enable read buffer for input layer"]
pub type InputLayer2AxcacheW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INPUT_LAYER2_MAX_AXLEN_LOG` reader - Limit of ARLen"]
pub type InputLayer2MaxAxlenLogR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER2_MAX_AXLEN_LOG` writer - Limit of ARLen"]
pub type InputLayer2MaxAxlenLogW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - ARSize Refer to AXI4 protocol. · AXI cmd size must be less than or equal to BUS width. · Must be 0x2 when AXCACHE=6."]
    #[inline(always)]
    pub fn input_layer2_axsize(&self) -> InputLayer2AxsizeR {
        InputLayer2AxsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - ARBurst 0x1: ARBurst = INCR 0x0: ARBurst = FIXED"]
    #[inline(always)]
    pub fn input_layer2_incr(&self) -> InputLayer2IncrR {
        InputLayer2IncrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - ARCache Refer to AXI4 protocol. Note: Set to 0x6 to enable read buffer for input layer"]
    #[inline(always)]
    pub fn input_layer2_axcache(&self) -> InputLayer2AxcacheR {
        InputLayer2AxcacheR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Limit of ARLen"]
    #[inline(always)]
    pub fn input_layer2_max_axlen_log(&self) -> InputLayer2MaxAxlenLogR {
        InputLayer2MaxAxlenLogR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ARSize Refer to AXI4 protocol. · AXI cmd size must be less than or equal to BUS width. · Must be 0x2 when AXCACHE=6."]
    #[inline(always)]
    pub fn input_layer2_axsize(&mut self) -> InputLayer2AxsizeW<'_, InputLayer2BusCfgSpec> {
        InputLayer2AxsizeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - ARCache Refer to AXI4 protocol. Note: Set to 0x6 to enable read buffer for input layer"]
    #[inline(always)]
    pub fn input_layer2_axcache(&mut self) -> InputLayer2AxcacheW<'_, InputLayer2BusCfgSpec> {
        InputLayer2AxcacheW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Limit of ARLen"]
    #[inline(always)]
    pub fn input_layer2_max_axlen_log(
        &mut self,
    ) -> InputLayer2MaxAxlenLogW<'_, InputLayer2BusCfgSpec> {
        InputLayer2MaxAxlenLogW::new(self, 8)
    }
}
#[doc = "Master port BUS configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer2_bus_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer2_bus_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer2BusCfgSpec;
impl crate::RegisterSpec for InputLayer2BusCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer2_bus_cfg::R`](R) reader structure"]
impl crate::Readable for InputLayer2BusCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer2_bus_cfg::W`](W) writer structure"]
impl crate::Writable for InputLayer2BusCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER2_BUS_CFG to value 0x070a"]
impl crate::Resettable for InputLayer2BusCfgSpec {
    const RESET_VALUE: u32 = 0x070a;
}
