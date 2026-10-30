#[doc = "Register `INPUT_LAYER3_MATRIX_E13` reader"]
pub type R = crate::R<InputLayer3MatrixE13Spec>;
#[doc = "Register `INPUT_LAYER3_MATRIX_E13` writer"]
pub type W = crate::W<InputLayer3MatrixE13Spec>;
#[doc = "Field `INPUT_LAYER3_MATRIX_E13` reader - Element E11 ~ E33 of coordinates transformation matrix"]
pub type InputLayer3MatrixE13R = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER3_MATRIX_E13` writer - Element E11 ~ E33 of coordinates transformation matrix"]
pub type InputLayer3MatrixE13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Element E11 ~ E33 of coordinates transformation matrix"]
    #[inline(always)]
    pub fn input_layer3_matrix_e13(&self) -> InputLayer3MatrixE13R {
        InputLayer3MatrixE13R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Element E11 ~ E33 of coordinates transformation matrix"]
    #[inline(always)]
    pub fn input_layer3_matrix_e13(
        &mut self,
    ) -> InputLayer3MatrixE13W<'_, InputLayer3MatrixE13Spec> {
        InputLayer3MatrixE13W::new(self, 0)
    }
}
#[doc = "Matrix E13 register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer3_matrix_e13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer3_matrix_e13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer3MatrixE13Spec;
impl crate::RegisterSpec for InputLayer3MatrixE13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer3_matrix_e13::R`](R) reader structure"]
impl crate::Readable for InputLayer3MatrixE13Spec {}
#[doc = "`write(|w| ..)` method takes [`input_layer3_matrix_e13::W`](W) writer structure"]
impl crate::Writable for InputLayer3MatrixE13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER3_MATRIX_E13 to value 0"]
impl crate::Resettable for InputLayer3MatrixE13Spec {}
