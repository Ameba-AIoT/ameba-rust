#[doc = "Register `INPUT_LAYER3_WIN_MAX` reader"]
pub type R = crate::R<InputLayer3WinMaxSpec>;
#[doc = "Register `INPUT_LAYER3_WIN_MAX` writer"]
pub type W = crate::W<InputLayer3WinMaxSpec>;
#[doc = "Field `INPUT_LAYER3_X_MAX` reader - X coordinate"]
pub type InputLayer3XMaxR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER3_X_MAX` writer - X coordinate"]
pub type InputLayer3XMaxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `INPUT_LAYER3_Y_MAX` reader - Y coordinate"]
pub type InputLayer3YMaxR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER3_Y_MAX` writer - Y coordinate"]
pub type InputLayer3YMaxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - X coordinate"]
    #[inline(always)]
    pub fn input_layer3_x_max(&self) -> InputLayer3XMaxR {
        InputLayer3XMaxR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Y coordinate"]
    #[inline(always)]
    pub fn input_layer3_y_max(&self) -> InputLayer3YMaxR {
        InputLayer3YMaxR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - X coordinate"]
    #[inline(always)]
    pub fn input_layer3_x_max(&mut self) -> InputLayer3XMaxW<'_, InputLayer3WinMaxSpec> {
        InputLayer3XMaxW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Y coordinate"]
    #[inline(always)]
    pub fn input_layer3_y_max(&mut self) -> InputLayer3YMaxW<'_, InputLayer3WinMaxSpec> {
        InputLayer3YMaxW::new(self, 16)
    }
}
#[doc = "Maximum coordinate of the input window register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer3_win_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer3_win_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer3WinMaxSpec;
impl crate::RegisterSpec for InputLayer3WinMaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer3_win_max::R`](R) reader structure"]
impl crate::Readable for InputLayer3WinMaxSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer3_win_max::W`](W) writer structure"]
impl crate::Writable for InputLayer3WinMaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER3_WIN_MAX to value 0"]
impl crate::Resettable for InputLayer3WinMaxSpec {}
