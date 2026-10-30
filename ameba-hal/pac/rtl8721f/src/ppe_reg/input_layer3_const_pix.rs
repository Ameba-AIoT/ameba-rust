#[doc = "Register `INPUT_LAYER3_CONST_PIX` reader"]
pub type R = crate::R<InputLayer3ConstPixSpec>;
#[doc = "Register `INPUT_LAYER3_CONST_PIX` writer"]
pub type W = crate::W<InputLayer3ConstPixSpec>;
#[doc = "Field `INPUT_LAYER3_ABGR` reader - In ABGR8888 format"]
pub type InputLayer3AbgrR = crate::FieldReader<u32>;
#[doc = "Field `INPUT_LAYER3_ABGR` writer - In ABGR8888 format"]
pub type InputLayer3AbgrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - In ABGR8888 format"]
    #[inline(always)]
    pub fn input_layer3_abgr(&self) -> InputLayer3AbgrR {
        InputLayer3AbgrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - In ABGR8888 format"]
    #[inline(always)]
    pub fn input_layer3_abgr(&mut self) -> InputLayer3AbgrW<'_, InputLayer3ConstPixSpec> {
        InputLayer3AbgrW::new(self, 0)
    }
}
#[doc = "Constant pixel register\n\nYou can [`read`](crate::Reg::read) this register and get [`input_layer3_const_pix::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input_layer3_const_pix::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputLayer3ConstPixSpec;
impl crate::RegisterSpec for InputLayer3ConstPixSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_layer3_const_pix::R`](R) reader structure"]
impl crate::Readable for InputLayer3ConstPixSpec {}
#[doc = "`write(|w| ..)` method takes [`input_layer3_const_pix::W`](W) writer structure"]
impl crate::Writable for InputLayer3ConstPixSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUT_LAYER3_CONST_PIX to value 0xffff_ffff"]
impl crate::Resettable for InputLayer3ConstPixSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
