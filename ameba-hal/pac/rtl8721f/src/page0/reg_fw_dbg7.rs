#[doc = "Register `REG_FW_DBG7` reader"]
pub type R = crate::R<RegFwDbg7Spec>;
#[doc = "Register `REG_FW_DBG7` writer"]
pub type W = crate::W<RegFwDbg7Spec>;
#[doc = "Field `BIT_FW_DBG7` reader - fw debug7"]
pub type BitFwDbg7R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_DBG7` writer - fw debug7"]
pub type BitFwDbg7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw debug7"]
    #[inline(always)]
    pub fn bit_fw_dbg7(&self) -> BitFwDbg7R {
        BitFwDbg7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw debug7"]
    #[inline(always)]
    pub fn bit_fw_dbg7(&mut self) -> BitFwDbg7W<'_, RegFwDbg7Spec> {
        BitFwDbg7W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg7Spec;
impl crate::RegisterSpec for RegFwDbg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg7::R`](R) reader structure"]
impl crate::Readable for RegFwDbg7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg7::W`](W) writer structure"]
impl crate::Writable for RegFwDbg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG7 to value 0"]
impl crate::Resettable for RegFwDbg7Spec {}
