#[doc = "Register `REG_HMEBOX_E3` reader"]
pub type R = crate::R<RegHmeboxE3Spec>;
#[doc = "Register `REG_HMEBOX_E3` writer"]
pub type W = crate::W<RegHmeboxE3Spec>;
#[doc = "Field `BIT_HMEBOX_E3` reader - User Defined Message"]
pub type BitHmeboxE3R = crate::FieldReader<u32>;
#[doc = "Field `BIT_HMEBOX_E3` writer - User Defined Message"]
pub type BitHmeboxE3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_hmebox_e3(&self) -> BitHmeboxE3R {
        BitHmeboxE3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_hmebox_e3(&mut self) -> BitHmeboxE3W<'_, RegHmeboxE3Spec> {
        BitHmeboxE3W::new(self, 0)
    }
}
#[doc = "Host Message Mail Box 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hmebox_e3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hmebox_e3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHmeboxE3Spec;
impl crate::RegisterSpec for RegHmeboxE3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hmebox_e3::R`](R) reader structure"]
impl crate::Readable for RegHmeboxE3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_hmebox_e3::W`](W) writer structure"]
impl crate::Writable for RegHmeboxE3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HMEBOX_E3 to value 0"]
impl crate::Resettable for RegHmeboxE3Spec {}
