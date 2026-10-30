#[doc = "Register `INPUT_LAYER3_WIN_MIN` reader"]
pub type R = crate::R<InputLayer3WinMinSpec>;
#[doc = "Register `INPUT_LAYER3_WIN_MIN` writer"]
pub type W = crate::W<InputLayer3WinMinSpec>;
#[doc = "Field `INPUT_LAYER3_X_MIN` reader - X coordinate"]
pub type InputLayer3XMinR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER3_X_MIN` writer - X coordinate"]
pub type InputLayer3XMinW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `INPUT_LAYER3_Y_MIN` reader - Y coordinate"]
pub type InputLayer3YMinR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER3_Y_MIN` writer - Y coordinate"]
pub type InputLayer3YMinW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - X coordinate"]
    #[inline(always)]
    pub fn input_layer3_x_min(&self) -> InputLayer3XMinR {
        InputLayer3XMinR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Y coordinate"]
    #[inline(always)]
    pub fn input_layer3_y_min(&self) -> InputLayer3YMinR {
        InputLayer3YMinR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - X coordinate"]
    #[inline(always)]
    pub fn input_layer3_x_min(&mut self) -> InputLayer3XMinW<'_, InputLayer3WinMinSpec> {
        InputLayer3XMinW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Y coordinate"]
    #[inline(always)]
    pub fn input_layer3_y_min(&mut self) -> InputLayer3YMinW<'_, InputLayer3WinMinSpec> {
        InputLayer3YMinW::new(self, 16)
    }
}
#[doc = "Minimum coordinate of the input window register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer3_win_min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer3_win_min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer3WinMinSpec;
impl crate::RegisterSpec for InputLayer3WinMinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer3_win_min::R`](R) reader structure"]
impl crate::Readable for InputLayer3WinMinSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer3_win_min::W`](W) writer structure"]
impl crate::Writable for InputLayer3WinMinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER3_WIN_MIN to value 0"]
impl crate::Resettable for InputLayer3WinMinSpec {}
