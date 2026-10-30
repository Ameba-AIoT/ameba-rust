#[doc = "Register `INPUT_LAYER1_MATRIX_E12` reader"]
pub type R = crate::R<InputLayer1MatrixE12Spec>;
#[doc = "Register `INPUT_LAYER1_MATRIX_E12` writer"]
pub type W = crate::W<InputLayer1MatrixE12Spec>;
#[doc = "Field `INPUT_LAYER1_MATRIX_E12` reader - Element E11 ~ E33 of coordinates transformation matrix · Elements E12, E21, E31~E33 are RO if SUPPORT_ROTATION_i = 0."]
pub type InputLayer1MatrixE12R = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER1_MATRIX_E12` writer - Element E11 ~ E33 of coordinates transformation matrix · Elements E12, E21, E31~E33 are RO if SUPPORT_ROTATION_i = 0."]
pub type InputLayer1MatrixE12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Element E11 ~ E33 of coordinates transformation matrix · Elements E12, E21, E31~E33 are RO if SUPPORT_ROTATION_i = 0."]
    #[inline(always)]
    pub fn input_layer1_matrix_e12(&self) -> InputLayer1MatrixE12R {
        InputLayer1MatrixE12R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Element E11 ~ E33 of coordinates transformation matrix · Elements E12, E21, E31~E33 are RO if SUPPORT_ROTATION_i = 0."]
    #[inline(always)]
    pub fn input_layer1_matrix_e12(
        &mut self,
    ) -> InputLayer1MatrixE12W<'_, InputLayer1MatrixE12Spec> {
        InputLayer1MatrixE12W::new(self, 0)
    }
}
#[doc = "Matrix E12 register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer1_matrix_e12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer1_matrix_e12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer1MatrixE12Spec;
impl crate::RegisterSpec for InputLayer1MatrixE12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer1_matrix_e12::R`](R) reader structure"]
impl crate::Readable for InputLayer1MatrixE12Spec {}
#[doc = "`write(|w| ..)` method takes [`input_layer1_matrix_e12::W`](W) writer structure"]
impl crate::Writable for InputLayer1MatrixE12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER1_MATRIX_E12 to value 0"]
impl crate::Resettable for InputLayer1MatrixE12Spec {}
