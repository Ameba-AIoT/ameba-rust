#[doc = "Register `RESULT_LAYER_CONST_BG` reader"]
pub type R = crate::R<ResultLayerConstBgSpec>;
#[doc = "Register `RESULT_LAYER_CONST_BG` writer"]
pub type W = crate::W<ResultLayerConstBgSpec>;
#[doc = "Field `RESULT_LAYER_ABGR` reader - Background ABGR (in ABGR8888 format)"]
pub type ResultLayerAbgrR = crate::FieldReader<u32>;
#[doc = "Field `RESULT_LAYER_ABGR` writer - Background ABGR (in ABGR8888 format)"]
pub type ResultLayerAbgrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Background ABGR (in ABGR8888 format)"]
    #[inline(always)]
    pub fn result_layer_abgr(&self) -> ResultLayerAbgrR {
        ResultLayerAbgrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Background ABGR (in ABGR8888 format)"]
    #[inline(always)]
    pub fn result_layer_abgr(&mut self) -> ResultLayerAbgrW<'_, ResultLayerConstBgSpec> {
        ResultLayerAbgrW::new(self, 0)
    }
}
#[doc = "Constant background color register\n\nYou can [`read`](crate::Reg::read) this register and get [`result_layer_const_bg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result_layer_const_bg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultLayerConstBgSpec;
impl crate::RegisterSpec for ResultLayerConstBgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result_layer_const_bg::R`](R) reader structure"]
impl crate::Readable for ResultLayerConstBgSpec {}
#[doc = "`write(|w| ..)` method takes [`result_layer_const_bg::W`](W) writer structure"]
impl crate::Writable for ResultLayerConstBgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESULT_LAYER_CONST_BG to value 0"]
impl crate::Resettable for ResultLayerConstBgSpec {}
