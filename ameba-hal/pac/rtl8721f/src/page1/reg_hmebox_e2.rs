#[doc = "Register `REG_HMEBOX_E2` reader"]
pub type R = crate::R<RegHmeboxE2Spec>;
#[doc = "Register `REG_HMEBOX_E2` writer"]
pub type W = crate::W<RegHmeboxE2Spec>;
#[doc = "Field `BIT_HMEBOX_E2` reader - User Defined Message"]
pub type BitHmeboxE2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_HMEBOX_E2` writer - User Defined Message"]
pub type BitHmeboxE2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_hmebox_e2(&self) -> BitHmeboxE2R {
        BitHmeboxE2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_hmebox_e2(&mut self) -> BitHmeboxE2W<'_, RegHmeboxE2Spec> {
        BitHmeboxE2W::new(self, 0)
    }
}
#[doc = "Host Message Mail Box 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hmebox_e2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hmebox_e2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHmeboxE2Spec;
impl crate::RegisterSpec for RegHmeboxE2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hmebox_e2::R`](R) reader structure"]
impl crate::Readable for RegHmeboxE2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_hmebox_e2::W`](W) writer structure"]
impl crate::Writable for RegHmeboxE2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HMEBOX_E2 to value 0"]
impl crate::Resettable for RegHmeboxE2Spec {}
