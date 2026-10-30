#[doc = "Register `INPUT_LAYER2_MATRIX_E33` reader"]
pub type R = crate::R<InputLayer2MatrixE33Spec>;
#[doc = "Field `INPUT_LAYER2_MATRIX_E33` reader - Element E11 ~ E33 of coordinates transformation matrix · Elements E12, E21, E31~E33 are RO if SUPPORT_ROTATION_i = 0. · Elements E31~E33 are RO if SUPPORT_PERSPECTIVE_i = 0."]
pub type InputLayer2MatrixE33R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Element E11 ~ E33 of coordinates transformation matrix · Elements E12, E21, E31~E33 are RO if SUPPORT_ROTATION_i = 0. · Elements E31~E33 are RO if SUPPORT_PERSPECTIVE_i = 0."]
    #[inline(always)]
    pub fn input_layer2_matrix_e33(&self) -> InputLayer2MatrixE33R {
        InputLayer2MatrixE33R::new(self.bits)
    }
}
#[doc = "Matrix E33 register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer2_matrix_e33::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer2MatrixE33Spec;
impl crate::RegisterSpec for InputLayer2MatrixE33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer2_matrix_e33::R`](R) reader structure"]
impl crate::Readable for InputLayer2MatrixE33Spec {}
#[doc = "`reset()` method sets INPUT_LAYER2_MATRIX_E33 to value 0x10"]
impl crate::Resettable for InputLayer2MatrixE33Spec {
    const RESET_VALUE: u32 = 0x10;
}
