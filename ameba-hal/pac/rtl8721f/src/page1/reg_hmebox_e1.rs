#[doc = "Register `REG_HMEBOX_E1` reader"]
pub type R = crate::R<RegHmeboxE1Spec>;
#[doc = "Register `REG_HMEBOX_E1` writer"]
pub type W = crate::W<RegHmeboxE1Spec>;
#[doc = "Field `BIT_HMEBOX_E1` reader - User Defined Message"]
pub type BitHmeboxE1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_HMEBOX_E1` writer - User Defined Message"]
pub type BitHmeboxE1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_hmebox_e1(&self) -> BitHmeboxE1R {
        BitHmeboxE1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_hmebox_e1(&mut self) -> BitHmeboxE1W<'_, RegHmeboxE1Spec> {
        BitHmeboxE1W::new(self, 0)
    }
}
#[doc = "Host Message Mail Box 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hmebox_e1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hmebox_e1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHmeboxE1Spec;
impl crate::RegisterSpec for RegHmeboxE1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hmebox_e1::R`](R) reader structure"]
impl crate::Readable for RegHmeboxE1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_hmebox_e1::W`](W) writer structure"]
impl crate::Writable for RegHmeboxE1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HMEBOX_E1 to value 0"]
impl crate::Resettable for RegHmeboxE1Spec {}
