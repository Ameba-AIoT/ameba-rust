#[doc = "Register `INPUT_LAYER1_MATRIX_E31` reader"]
pub type R = crate::R<InputLayer1MatrixE31Spec>;
#[doc = "Field `INPUT_LAYER1_MATRIX_E31` reader - Element E11 ~ E33 of coordinates transformation matrix · Elements E12, E21, E31~E33 are RO if SUPPORT_ROTATION_i = 0. · Elements E31~E33 are RO if SUPPORT_PERSPECTIVE_i = 0."]
pub type InputLayer1MatrixE31R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Element E11 ~ E33 of coordinates transformation matrix · Elements E12, E21, E31~E33 are RO if SUPPORT_ROTATION_i = 0. · Elements E31~E33 are RO if SUPPORT_PERSPECTIVE_i = 0."]
    #[inline(always)]
    pub fn input_layer1_matrix_e31(&self) -> InputLayer1MatrixE31R {
        InputLayer1MatrixE31R::new(self.bits)
    }
}
#[doc = "Matrix E31 register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer1_matrix_e31::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer1MatrixE31Spec;
impl crate::RegisterSpec for InputLayer1MatrixE31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer1_matrix_e31::R`](R) reader structure"]
impl crate::Readable for InputLayer1MatrixE31Spec {}
#[doc = "`reset()` method sets INPUT_LAYER1_MATRIX_E31 to value 0"]
impl crate::Resettable for InputLayer1MatrixE31Spec {}
