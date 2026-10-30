#[doc = "Register `INPUT_LAYER2_WIN_MIN` reader"]
pub type R = crate::R<InputLayer2WinMinSpec>;
#[doc = "Register `INPUT_LAYER2_WIN_MIN` writer"]
pub type W = crate::W<InputLayer2WinMinSpec>;
#[doc = "Field `INPUT_LAYER2_X_MIN` reader - X coordinate"]
pub type InputLayer2XMinR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER2_X_MIN` writer - X coordinate"]
pub type InputLayer2XMinW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `INPUT_LAYER2_Y_MIN` reader - Y coordinate"]
pub type InputLayer2YMinR = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER2_Y_MIN` writer - Y coordinate"]
pub type InputLayer2YMinW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - X coordinate"]
    #[inline(always)]
    pub fn input_layer2_x_min(&self) -> InputLayer2XMinR {
        InputLayer2XMinR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Y coordinate"]
    #[inline(always)]
    pub fn input_layer2_y_min(&self) -> InputLayer2YMinR {
        InputLayer2YMinR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - X coordinate"]
    #[inline(always)]
    pub fn input_layer2_x_min(&mut self) -> InputLayer2XMinW<'_, InputLayer2WinMinSpec> {
        InputLayer2XMinW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Y coordinate"]
    #[inline(always)]
    pub fn input_layer2_y_min(&mut self) -> InputLayer2YMinW<'_, InputLayer2WinMinSpec> {
        InputLayer2YMinW::new(self, 16)
    }
}
#[doc = "Minimum coordinate of the input window register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer2_win_min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer2_win_min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer2WinMinSpec;
impl crate::RegisterSpec for InputLayer2WinMinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer2_win_min::R`](R) reader structure"]
impl crate::Readable for InputLayer2WinMinSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer2_win_min::W`](W) writer structure"]
impl crate::Writable for InputLayer2WinMinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER2_WIN_MIN to value 0"]
impl crate::Resettable for InputLayer2WinMinSpec {}
