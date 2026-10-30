#[doc = "Register `LAY_EN` reader"]
pub type R = crate::R<LayEnSpec>;
#[doc = "Register `LAY_EN` writer"]
pub type W = crate::W<LayEnSpec>;
#[doc = "Field `INPUT_LAY_EN` reader - Input layers enable field. Bit i (1 <= i <=3 ) 0x0: Input layer i is disabled. 0x1: Input layer i is enabled."]
pub type InputLayEnR = crate::FieldReader;
#[doc = "Field `INPUT_LAY_EN` writer - Input layers enable field. Bit i (1 <= i <=3 ) 0x0: Input layer i is disabled. 0x1: Input layer i is enabled."]
pub type InputLayEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 1:3 - Input layers enable field. Bit i (1 <= i <=3 ) 0x0: Input layer i is disabled. 0x1: Input layer i is enabled."]
    #[inline(always)]
    pub fn input_lay_en(&self) -> InputLayEnR {
        InputLayEnR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Input layers enable field. Bit i (1 <= i <=3 ) 0x0: Input layer i is disabled. 0x1: Input layer i is enabled."]
    #[inline(always)]
    pub fn input_lay_en(&mut self) -> InputLayEnW<'_, LayEnSpec> {
        InputLayEnW::new(self, 1)
    }
}
#[doc = "Layer enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`lay_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lay_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LayEnSpec;
impl crate::RegisterSpec for LayEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lay_en::R`](R) reader structure"]
impl crate::Readable for LayEnSpec {}
#[doc = "`write(|w| ..)` method takes [`lay_en::W`](W) writer structure"]
impl crate::Writable for LayEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LAY_EN to value 0"]
impl crate::Resettable for LayEnSpec {}
