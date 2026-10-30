#[doc = "Register `REG_NOA_PARAM_4` reader"]
pub type R = crate::R<RegNoaParam4Spec>;
#[doc = "Register `REG_NOA_PARAM_4` writer"]
pub type W = crate::W<RegNoaParam4Spec>;
#[doc = "Field `BIT_NOA_COUNT_V4` reader - indicates the number of absence intervals"]
pub type BitNoaCountV4R = crate::FieldReader;
#[doc = "Field `BIT_NOA_COUNT_V4` writer - indicates the number of absence intervals"]
pub type BitNoaCountV4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - indicates the number of absence intervals"]
    #[inline(always)]
    pub fn bit_noa_count_v4(&self) -> BitNoaCountV4R {
        BitNoaCountV4R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - indicates the number of absence intervals"]
    #[inline(always)]
    pub fn bit_noa_count_v4(&mut self) -> BitNoaCountV4W<'_, RegNoaParam4Spec> {
        BitNoaCountV4W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa_param_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa_param_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoaParam4Spec;
impl crate::RegisterSpec for RegNoaParam4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa_param_4::R`](R) reader structure"]
impl crate::Readable for RegNoaParam4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa_param_4::W`](W) writer structure"]
impl crate::Writable for RegNoaParam4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA_PARAM_4 to value 0x03"]
impl crate::Resettable for RegNoaParam4Spec {
    const RESET_VALUE: u32 = 0x03;
}
