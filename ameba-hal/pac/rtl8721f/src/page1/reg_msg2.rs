#[doc = "Register `REG_MSG2` reader"]
pub type R = crate::R<RegMsg2Spec>;
#[doc = "Register `REG_MSG2` writer"]
pub type W = crate::W<RegMsg2Spec>;
#[doc = "Field `BIT_FW_MSG2` reader - fw message 2"]
pub type BitFwMsg2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_MSG2` writer - fw message 2"]
pub type BitFwMsg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw message 2"]
    #[inline(always)]
    pub fn bit_fw_msg2(&self) -> BitFwMsg2R {
        BitFwMsg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw message 2"]
    #[inline(always)]
    pub fn bit_fw_msg2(&mut self) -> BitFwMsg2W<'_, RegMsg2Spec> {
        BitFwMsg2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_msg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_msg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMsg2Spec;
impl crate::RegisterSpec for RegMsg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_msg2::R`](R) reader structure"]
impl crate::Readable for RegMsg2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_msg2::W`](W) writer structure"]
impl crate::Writable for RegMsg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MSG2 to value 0"]
impl crate::Resettable for RegMsg2Spec {}
