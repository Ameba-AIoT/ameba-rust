#[doc = "Register `INPUT_LAYER1_MATRIX_E23` reader"]
pub type R = crate::R<InputLayer1MatrixE23Spec>;
#[doc = "Register `INPUT_LAYER1_MATRIX_E23` writer"]
pub type W = crate::W<InputLayer1MatrixE23Spec>;
#[doc = "Field `INPUT_LAYER1_MATRIX_E23` reader - Element E11 ~ E33 of coordinates transformation matrix"]
pub type InputLayer1MatrixE23R = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER1_MATRIX_E23` writer - Element E11 ~ E33 of coordinates transformation matrix"]
pub type InputLayer1MatrixE23W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Element E11 ~ E33 of coordinates transformation matrix"]
    #[inline(always)]
    pub fn input_layer1_matrix_e23(&self) -> InputLayer1MatrixE23R {
        InputLayer1MatrixE23R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Element E11 ~ E33 of coordinates transformation matrix"]
    #[inline(always)]
    pub fn input_layer1_matrix_e23(
        &mut self,
    ) -> InputLayer1MatrixE23W<'_, InputLayer1MatrixE23Spec> {
        InputLayer1MatrixE23W::new(self, 0)
    }
}
#[doc = "Matrix E23 register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer1_matrix_e23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer1_matrix_e23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer1MatrixE23Spec;
impl crate::RegisterSpec for InputLayer1MatrixE23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer1_matrix_e23::R`](R) reader structure"]
impl crate::Readable for InputLayer1MatrixE23Spec {}
#[doc = "`write(|w| ..)` method takes [`input_layer1_matrix_e23::W`](W) writer structure"]
impl crate::Writable for InputLayer1MatrixE23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER1_MATRIX_E23 to value 0"]
impl crate::Resettable for InputLayer1MatrixE23Spec {}
