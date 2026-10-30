#[doc = "Register `REG_MSG4` reader"]
pub type R = crate::R<RegMsg4Spec>;
#[doc = "Register `REG_MSG4` writer"]
pub type W = crate::W<RegMsg4Spec>;
#[doc = "Field `BIT_FW_MSG4` reader - fw message 4"]
pub type BitFwMsg4R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_MSG4` writer - fw message 4"]
pub type BitFwMsg4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw message 4"]
    #[inline(always)]
    pub fn bit_fw_msg4(&self) -> BitFwMsg4R {
        BitFwMsg4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw message 4"]
    #[inline(always)]
    pub fn bit_fw_msg4(&mut self) -> BitFwMsg4W<'_, RegMsg4Spec> {
        BitFwMsg4W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_msg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_msg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMsg4Spec;
impl crate::RegisterSpec for RegMsg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_msg4::R`](R) reader structure"]
impl crate::Readable for RegMsg4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_msg4::W`](W) writer structure"]
impl crate::Writable for RegMsg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MSG4 to value 0"]
impl crate::Resettable for RegMsg4Spec {}
