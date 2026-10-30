#[doc = "Register `REG_HMEBOX_E0` reader"]
pub type R = crate::R<RegHmeboxE0Spec>;
#[doc = "Register `REG_HMEBOX_E0` writer"]
pub type W = crate::W<RegHmeboxE0Spec>;
#[doc = "Field `BIT_HMEBOX_E0` reader - User Defined Message"]
pub type BitHmeboxE0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_HMEBOX_E0` writer - User Defined Message"]
pub type BitHmeboxE0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_hmebox_e0(&self) -> BitHmeboxE0R {
        BitHmeboxE0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_hmebox_e0(&mut self) -> BitHmeboxE0W<'_, RegHmeboxE0Spec> {
        BitHmeboxE0W::new(self, 0)
    }
}
#[doc = "Host Message Mail Box 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hmebox_e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hmebox_e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHmeboxE0Spec;
impl crate::RegisterSpec for RegHmeboxE0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hmebox_e0::R`](R) reader structure"]
impl crate::Readable for RegHmeboxE0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_hmebox_e0::W`](W) writer structure"]
impl crate::Writable for RegHmeboxE0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HMEBOX_E0 to value 0"]
impl crate::Resettable for RegHmeboxE0Spec {}
