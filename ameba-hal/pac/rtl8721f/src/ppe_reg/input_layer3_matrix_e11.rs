#[doc = "Register `INPUT_LAYER3_MATRIX_E11` reader"]
pub type R = crate::R<InputLayer3MatrixE11Spec>;
#[doc = "Register `INPUT_LAYER3_MATRIX_E11` writer"]
pub type W = crate::W<InputLayer3MatrixE11Spec>;
#[doc = "Field `INPUT_LAYER3_MATRIX_E11` reader - Element E11 ~ E33 of coordinates transformation matrix"]
pub type InputLayer3MatrixE11R = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER3_MATRIX_E11` writer - Element E11 ~ E33 of coordinates transformation matrix"]
pub type InputLayer3MatrixE11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Element E11 ~ E33 of coordinates transformation matrix"]
    #[inline(always)]
    pub fn input_layer3_matrix_e11(&self) -> InputLayer3MatrixE11R {
        InputLayer3MatrixE11R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Element E11 ~ E33 of coordinates transformation matrix"]
    #[inline(always)]
    pub fn input_layer3_matrix_e11(
        &mut self,
    ) -> InputLayer3MatrixE11W<'_, InputLayer3MatrixE11Spec> {
        InputLayer3MatrixE11W::new(self, 0)
    }
}
#[doc = "Matrix E11 register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer3_matrix_e11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer3_matrix_e11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer3MatrixE11Spec;
impl crate::RegisterSpec for InputLayer3MatrixE11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer3_matrix_e11::R`](R) reader structure"]
impl crate::Readable for InputLayer3MatrixE11Spec {}
#[doc = "`write(|w| ..)` method takes [`input_layer3_matrix_e11::W`](W) writer structure"]
impl crate::Writable for InputLayer3MatrixE11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER3_MATRIX_E11 to value 0x10"]
impl crate::Resettable for InputLayer3MatrixE11Spec {
    const RESET_VALUE: u32 = 0x10;
}
