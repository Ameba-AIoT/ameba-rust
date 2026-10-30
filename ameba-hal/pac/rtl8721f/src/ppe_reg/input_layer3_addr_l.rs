#[doc = "Register `INPUT_LAYER3_ADDR_L` reader"]
pub type R = crate::R<InputLayer3AddrLSpec>;
#[doc = "Register `INPUT_LAYER3_ADDR_L` writer"]
pub type W = crate::W<InputLayer3AddrLSpec>;
#[doc = "Field `INPUT_LAYER3_ADDR_L` reader - "]
pub type InputLayer3AddrLR = crate::FieldReader<u32>;
#[doc = "Field `INPUT_LAYER3_ADDR_L` writer - "]
pub type InputLayer3AddrLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn input_layer3_addr_l(&self) -> InputLayer3AddrLR {
        InputLayer3AddrLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn input_layer3_addr_l(&mut self) -> InputLayer3AddrLW<'_, InputLayer3AddrLSpec> {
        InputLayer3AddrLW::new(self, 0)
    }
}
#[doc = "Address of the first pixel register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer3_addr_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer3_addr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer3AddrLSpec;
impl crate::RegisterSpec for InputLayer3AddrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer3_addr_l::R`](R) reader structure"]
impl crate::Readable for InputLayer3AddrLSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer3_addr_l::W`](W) writer structure"]
impl crate::Writable for InputLayer3AddrLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER3_ADDR_L to value 0"]
impl crate::Resettable for InputLayer3AddrLSpec {}
