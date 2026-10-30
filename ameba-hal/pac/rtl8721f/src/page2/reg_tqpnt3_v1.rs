#[doc = "Register `REG_TQPNT3_V1` reader"]
pub type R = crate::R<RegTqpnt3V1Spec>;
#[doc = "Register `REG_TQPNT3_V1` writer"]
pub type W = crate::W<RegTqpnt3V1Spec>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTqpnt3V1Spec> {
        RsvdW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tqpnt3_v1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tqpnt3_v1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTqpnt3V1Spec;
impl crate::RegisterSpec for RegTqpnt3V1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tqpnt3_v1::R`](R) reader structure"]
impl crate::Readable for RegTqpnt3V1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_tqpnt3_v1::W`](W) writer structure"]
impl crate::Writable for RegTqpnt3V1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TQPNT3_V1 to value 0"]
impl crate::Resettable for RegTqpnt3V1Spec {}
