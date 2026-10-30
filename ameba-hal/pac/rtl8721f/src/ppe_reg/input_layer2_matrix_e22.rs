#[doc = "Register `INPUT_LAYER2_MATRIX_E22` reader"]
pub type R = crate::R<InputLayer2MatrixE22Spec>;
#[doc = "Register `INPUT_LAYER2_MATRIX_E22` writer"]
pub type W = crate::W<InputLayer2MatrixE22Spec>;
#[doc = "Field `INPUT_LAYER2_MATRIX_E22` reader - Element E11 ~ E33 of coordinates transformation matrix"]
pub type InputLayer2MatrixE22R = crate::FieldReader<u16>;
#[doc = "Field `INPUT_LAYER2_MATRIX_E22` writer - Element E11 ~ E33 of coordinates transformation matrix"]
pub type InputLayer2MatrixE22W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Element E11 ~ E33 of coordinates transformation matrix"]
    #[inline(always)]
    pub fn input_layer2_matrix_e22(&self) -> InputLayer2MatrixE22R {
        InputLayer2MatrixE22R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Element E11 ~ E33 of coordinates transformation matrix"]
    #[inline(always)]
    pub fn input_layer2_matrix_e22(
        &mut self,
    ) -> InputLayer2MatrixE22W<'_, InputLayer2MatrixE22Spec> {
        InputLayer2MatrixE22W::new(self, 0)
    }
}
#[doc = "Matrix E22 register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer2_matrix_e22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer2_matrix_e22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer2MatrixE22Spec;
impl crate::RegisterSpec for InputLayer2MatrixE22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer2_matrix_e22::R`](R) reader structure"]
impl crate::Readable for InputLayer2MatrixE22Spec {}
#[doc = "`write(|w| ..)` method takes [`input_layer2_matrix_e22::W`](W) writer structure"]
impl crate::Writable for InputLayer2MatrixE22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER2_MATRIX_E22 to value 0x10"]
impl crate::Resettable for InputLayer2MatrixE22Spec {
    const RESET_VALUE: u32 = 0x10;
}
