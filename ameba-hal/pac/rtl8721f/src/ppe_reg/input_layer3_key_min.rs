#[doc = "Register `INPUT_LAYER3_KEY_MIN` reader"]
pub type R = crate::R<InputLayer3KeyMinSpec>;
#[doc = "Register `INPUT_LAYER3_KEY_MIN` writer"]
pub type W = crate::W<InputLayer3KeyMinSpec>;
#[doc = "Field `INPUT_LAYER3_BGR_MIN` reader - Minimum BGR of the color key match condition In BGR888 format."]
pub type InputLayer3BgrMinR = crate::FieldReader<u32>;
#[doc = "Field `INPUT_LAYER3_BGR_MIN` writer - Minimum BGR of the color key match condition In BGR888 format."]
pub type InputLayer3BgrMinW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Minimum BGR of the color key match condition In BGR888 format."]
    #[inline(always)]
    pub fn input_layer3_bgr_min(&self) -> InputLayer3BgrMinR {
        InputLayer3BgrMinR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Minimum BGR of the color key match condition In BGR888 format."]
    #[inline(always)]
    pub fn input_layer3_bgr_min(&mut self) -> InputLayer3BgrMinW<'_, InputLayer3KeyMinSpec> {
        InputLayer3BgrMinW::new(self, 0)
    }
}
#[doc = "Minimum of the color key match condition register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer3_key_min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer3_key_min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer3KeyMinSpec;
impl crate::RegisterSpec for InputLayer3KeyMinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer3_key_min::R`](R) reader structure"]
impl crate::Readable for InputLayer3KeyMinSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer3_key_min::W`](W) writer structure"]
impl crate::Writable for InputLayer3KeyMinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER3_KEY_MIN to value 0"]
impl crate::Resettable for InputLayer3KeyMinSpec {}
