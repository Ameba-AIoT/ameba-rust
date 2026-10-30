#[doc = "Register `INPUT_LAYER1_WIN_MIN` reader"]
pub type R = crate::R<InputLayer1WinMinSpec>;
#[doc = "Register `INPUT_LAYER1_WIN_MIN` writer"]
pub type W = crate::W<InputLayer1WinMinSpec>;
#[doc = "Field `INPUT_LAYER1_X_MIN` reader - X coordinate"]
pub type InputLayer1XMinR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER1_X_MIN` writer - X coordinate"]
pub type InputLayer1XMinW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `INPUT_LAYER1_Y_MIN` reader - Y coordinate"]
pub type InputLayer1YMinR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER1_Y_MIN` writer - Y coordinate"]
pub type InputLayer1YMinW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - X coordinate"]
    #[inline(always)]
    pub fn input_layer1_x_min(&self) -> InputLayer1XMinR {
        InputLayer1XMinR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Y coordinate"]
    #[inline(always)]
    pub fn input_layer1_y_min(&self) -> InputLayer1YMinR {
        InputLayer1YMinR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - X coordinate"]
    #[inline(always)]
    pub fn input_layer1_x_min(&mut self) -> InputLayer1XMinW<'_, InputLayer1WinMinSpec> {
        InputLayer1XMinW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Y coordinate"]
    #[inline(always)]
    pub fn input_layer1_y_min(&mut self) -> InputLayer1YMinW<'_, InputLayer1WinMinSpec> {
        InputLayer1YMinW::new(self, 16)
    }
}
#[doc = "Minimum coordinate of the input window register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer1_win_min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer1_win_min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer1WinMinSpec;
impl crate::RegisterSpec for InputLayer1WinMinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer1_win_min::R`](R) reader structure"]
impl crate::Readable for InputLayer1WinMinSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer1_win_min::W`](W) writer structure"]
impl crate::Writable for InputLayer1WinMinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER1_WIN_MIN to value 0"]
impl crate::Resettable for InputLayer1WinMinSpec {}
