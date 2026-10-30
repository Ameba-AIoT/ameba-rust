#[doc = "Register `REG_MSG5` reader"]
pub type R = crate::R<RegMsg5Spec>;
#[doc = "Register `REG_MSG5` writer"]
pub type W = crate::W<RegMsg5Spec>;
#[doc = "Field `BIT_FW_MSG5` reader - fw message 5"]
pub type BitFwMsg5R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_MSG5` writer - fw message 5"]
pub type BitFwMsg5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw message 5"]
    #[inline(always)]
    pub fn bit_fw_msg5(&self) -> BitFwMsg5R {
        BitFwMsg5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw message 5"]
    #[inline(always)]
    pub fn bit_fw_msg5(&mut self) -> BitFwMsg5W<'_, RegMsg5Spec> {
        BitFwMsg5W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_msg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_msg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMsg5Spec;
impl crate::RegisterSpec for RegMsg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_msg5::R`](R) reader structure"]
impl crate::Readable for RegMsg5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_msg5::W`](W) writer structure"]
impl crate::Writable for RegMsg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MSG5 to value 0"]
impl crate::Resettable for RegMsg5Spec {}
