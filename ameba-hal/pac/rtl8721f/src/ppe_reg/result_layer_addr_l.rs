#[doc = "Register `RESULT_LAYER_ADDR_L` reader"]
pub type R = crate::R<ResultLayerAddrLSpec>;
#[doc = "Register `RESULT_LAYER_ADDR_L` writer"]
pub type W = crate::W<ResultLayerAddrLSpec>;
#[doc = "Field `RESULT_LAYER_ADDR_L` reader - Address of the first pixel (Physical address)"]
pub type ResultLayerAddrLR = crate::FieldReader<u32>;
#[doc = "Field `RESULT_LAYER_ADDR_L` writer - Address of the first pixel (Physical address)"]
pub type ResultLayerAddrLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address of the first pixel (Physical address)"]
    #[inline(always)]
    pub fn result_layer_addr_l(&self) -> ResultLayerAddrLR {
        ResultLayerAddrLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address of the first pixel (Physical address)"]
    #[inline(always)]
    pub fn result_layer_addr_l(&mut self) -> ResultLayerAddrLW<'_, ResultLayerAddrLSpec> {
        ResultLayerAddrLW::new(self, 0)
    }
}
#[doc = "Address of the first pixel register\n\nYou can [`read`](crate::Reg::read) this register and get [`result_layer_addr_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result_layer_addr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultLayerAddrLSpec;
impl crate::RegisterSpec for ResultLayerAddrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result_layer_addr_l::R`](R) reader structure"]
impl crate::Readable for ResultLayerAddrLSpec {}
#[doc = "`write(|w| ..)` method takes [`result_layer_addr_l::W`](W) writer structure"]
impl crate::Writable for ResultLayerAddrLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESULT_LAYER_ADDR_L to value 0"]
impl crate::Resettable for ResultLayerAddrLSpec {}
