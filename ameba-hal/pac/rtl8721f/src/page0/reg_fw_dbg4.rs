#[doc = "Register `REG_FW_DBG4` reader"]
pub type R = crate::R<RegFwDbg4Spec>;
#[doc = "Register `REG_FW_DBG4` writer"]
pub type W = crate::W<RegFwDbg4Spec>;
#[doc = "Field `BIT_FW_DBG4` reader - fw debug4"]
pub type BitFwDbg4R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_DBG4` writer - fw debug4"]
pub type BitFwDbg4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw debug4"]
    #[inline(always)]
    pub fn bit_fw_dbg4(&self) -> BitFwDbg4R {
        BitFwDbg4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw debug4"]
    #[inline(always)]
    pub fn bit_fw_dbg4(&mut self) -> BitFwDbg4W<'_, RegFwDbg4Spec> {
        BitFwDbg4W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg4Spec;
impl crate::RegisterSpec for RegFwDbg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg4::R`](R) reader structure"]
impl crate::Readable for RegFwDbg4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg4::W`](W) writer structure"]
impl crate::Writable for RegFwDbg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG4 to value 0"]
impl crate::Resettable for RegFwDbg4Spec {}
