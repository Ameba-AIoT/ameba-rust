#[doc = "Register `REG_HMEBOX0` reader"]
pub type R = crate::R<RegHmebox0Spec>;
#[doc = "Register `REG_HMEBOX0` writer"]
pub type W = crate::W<RegHmebox0Spec>;
#[doc = "Field `BIT_HOST_MSG_0` reader - User Defined Message"]
pub type BitHostMsg0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_HOST_MSG_0` writer - User Defined Message"]
pub type BitHostMsg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_0(&self) -> BitHostMsg0R {
        BitHostMsg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_0(&mut self) -> BitHostMsg0W<'_, RegHmebox0Spec> {
        BitHostMsg0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hmebox0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hmebox0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHmebox0Spec;
impl crate::RegisterSpec for RegHmebox0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hmebox0::R`](R) reader structure"]
impl crate::Readable for RegHmebox0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_hmebox0::W`](W) writer structure"]
impl crate::Writable for RegHmebox0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HMEBOX0 to value 0"]
impl crate::Resettable for RegHmebox0Spec {}
