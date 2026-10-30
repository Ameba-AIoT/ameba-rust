#[doc = "Register `REG_FW_DBG6` reader"]
pub type R = crate::R<RegFwDbg6Spec>;
#[doc = "Register `REG_FW_DBG6` writer"]
pub type W = crate::W<RegFwDbg6Spec>;
#[doc = "Field `BIT_FW_DBG6` reader - fw debug6"]
pub type BitFwDbg6R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_DBG6` writer - fw debug6"]
pub type BitFwDbg6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw debug6"]
    #[inline(always)]
    pub fn bit_fw_dbg6(&self) -> BitFwDbg6R {
        BitFwDbg6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw debug6"]
    #[inline(always)]
    pub fn bit_fw_dbg6(&mut self) -> BitFwDbg6W<'_, RegFwDbg6Spec> {
        BitFwDbg6W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg6Spec;
impl crate::RegisterSpec for RegFwDbg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg6::R`](R) reader structure"]
impl crate::Readable for RegFwDbg6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg6::W`](W) writer structure"]
impl crate::Writable for RegFwDbg6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG6 to value 0"]
impl crate::Resettable for RegFwDbg6Spec {}
