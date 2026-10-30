#[doc = "Register `INPUT_LAYER1_BUS_CFG` reader"]
pub type R = crate::R<InputLayer1BusCfgSpec>;
#[doc = "Register `INPUT_LAYER1_BUS_CFG` writer"]
pub type W = crate::W<InputLayer1BusCfgSpec>;
#[doc = "Field `INPUT_LAYER1_AXSIZE` reader - ARSize Refer to AXI4 protocol. · AXI cmd size must be less than or equal to BUS width. · Must be 0x2 when AXCACHE=6."]
pub type InputLayer1AxsizeR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER1_AXSIZE` writer - ARSize Refer to AXI4 protocol. · AXI cmd size must be less than or equal to BUS width. · Must be 0x2 when AXCACHE=6."]
pub type InputLayer1AxsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INPUT_LAYER1_INCR` reader - ARBurst 0x1: ARBurst = INCR 0x0: ARBurst = FIXED"]
pub type InputLayer1IncrR = crate::BitReader;
#[doc = "Field `INPUT_LAYER1_AXCACHE` reader - ARCache Refer to AXI4 protocol. Note: Set to 0x6 to enable read buffer for input layer"]
pub type InputLayer1AxcacheR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER1_AXCACHE` writer - ARCache Refer to AXI4 protocol. Note: Set to 0x6 to enable read buffer for input layer"]
pub type InputLayer1AxcacheW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INPUT_LAYER1_MAX_AXLEN_LOG` reader - Limit of ARLen"]
pub type InputLayer1MaxAxlenLogR = crate::FieldReader;
#[doc = "Field `INPUT_LAYER1_MAX_AXLEN_LOG` writer - Limit of ARLen"]
pub type InputLayer1MaxAxlenLogW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - ARSize Refer to AXI4 protocol. · AXI cmd size must be less than or equal to BUS width. · Must be 0x2 when AXCACHE=6."]
    #[inline(always)]
    pub fn input_layer1_axsize(&self) -> InputLayer1AxsizeR {
        InputLayer1AxsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - ARBurst 0x1: ARBurst = INCR 0x0: ARBurst = FIXED"]
    #[inline(always)]
    pub fn input_layer1_incr(&self) -> InputLayer1IncrR {
        InputLayer1IncrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - ARCache Refer to AXI4 protocol. Note: Set to 0x6 to enable read buffer for input layer"]
    #[inline(always)]
    pub fn input_layer1_axcache(&self) -> InputLayer1AxcacheR {
        InputLayer1AxcacheR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Limit of ARLen"]
    #[inline(always)]
    pub fn input_layer1_max_axlen_log(&self) -> InputLayer1MaxAxlenLogR {
        InputLayer1MaxAxlenLogR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ARSize Refer to AXI4 protocol. · AXI cmd size must be less than or equal to BUS width. · Must be 0x2 when AXCACHE=6."]
    #[inline(always)]
    pub fn input_layer1_axsize(&mut self) -> InputLayer1AxsizeW<'_, InputLayer1BusCfgSpec> {
        InputLayer1AxsizeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - ARCache Refer to AXI4 protocol. Note: Set to 0x6 to enable read buffer for input layer"]
    #[inline(always)]
    pub fn input_layer1_axcache(&mut self) -> InputLayer1AxcacheW<'_, InputLayer1BusCfgSpec> {
        InputLayer1AxcacheW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Limit of ARLen"]
    #[inline(always)]
    pub fn input_layer1_max_axlen_log(
        &mut self,
    ) -> InputLayer1MaxAxlenLogW<'_, InputLayer1BusCfgSpec> {
        InputLayer1MaxAxlenLogW::new(self, 8)
    }
}
#[doc = "Master port BUS configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer1_bus_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer1_bus_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer1BusCfgSpec;
impl crate::RegisterSpec for InputLayer1BusCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer1_bus_cfg::R`](R) reader structure"]
impl crate::Readable for InputLayer1BusCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer1_bus_cfg::W`](W) writer structure"]
impl crate::Writable for InputLayer1BusCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER1_BUS_CFG to value 0x070a"]
impl crate::Resettable for InputLayer1BusCfgSpec {
    const RESET_VALUE: u32 = 0x070a;
}
