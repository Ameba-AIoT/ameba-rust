#[doc = "Register `REG_HMEBOX1` reader"]
pub type R = crate::R<RegHmebox1Spec>;
#[doc = "Register `REG_HMEBOX1` writer"]
pub type W = crate::W<RegHmebox1Spec>;
#[doc = "Field `BIT_HOST_MSG_1` reader - User Defined Message"]
pub type BitHostMsg1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_HOST_MSG_1` writer - User Defined Message"]
pub type BitHostMsg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_1(&self) -> BitHostMsg1R {
        BitHostMsg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_1(&mut self) -> BitHostMsg1W<'_, RegHmebox1Spec> {
        BitHostMsg1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hmebox1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hmebox1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHmebox1Spec;
impl crate::RegisterSpec for RegHmebox1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hmebox1::R`](R) reader structure"]
impl crate::Readable for RegHmebox1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_hmebox1::W`](W) writer structure"]
impl crate::Writable for RegHmebox1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HMEBOX1 to value 0"]
impl crate::Resettable for RegHmebox1Spec {}
