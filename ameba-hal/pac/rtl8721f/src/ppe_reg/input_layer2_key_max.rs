#[doc = "Register `INPUT_LAYER2_KEY_MAX` reader"]
pub type R = crate::R<InputLayer2KeyMaxSpec>;
#[doc = "Register `INPUT_LAYER2_KEY_MAX` writer"]
pub type W = crate::W<InputLayer2KeyMaxSpec>;
#[doc = "Field `INPUT_LAYER2_BGR_MAX` reader - Maximum BGR of the color key match condition In BGR888 format."]
pub type InputLayer2BgrMaxR = crate::FieldReader<u32>;
#[doc = "Field `INPUT_LAYER2_BGR_MAX` writer - Maximum BGR of the color key match condition In BGR888 format."]
pub type InputLayer2BgrMaxW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Maximum BGR of the color key match condition In BGR888 format."]
    #[inline(always)]
    pub fn input_layer2_bgr_max(&self) -> InputLayer2BgrMaxR {
        InputLayer2BgrMaxR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Maximum BGR of the color key match condition In BGR888 format."]
    #[inline(always)]
    pub fn input_layer2_bgr_max(&mut self) -> InputLayer2BgrMaxW<'_, InputLayer2KeyMaxSpec> {
        InputLayer2BgrMaxW::new(self, 0)
    }
}
#[doc = "Maximum of the color key match condition register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer2_key_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer2_key_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer2KeyMaxSpec;
impl crate::RegisterSpec for InputLayer2KeyMaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer2_key_max::R`](R) reader structure"]
impl crate::Readable for InputLayer2KeyMaxSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer2_key_max::W`](W) writer structure"]
impl crate::Writable for InputLayer2KeyMaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER2_KEY_MAX to value 0"]
impl crate::Resettable for InputLayer2KeyMaxSpec {}
