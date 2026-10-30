#[doc = "Register `REG_FW_DBG5` reader"]
pub type R = crate::R<RegFwDbg5Spec>;
#[doc = "Register `REG_FW_DBG5` writer"]
pub type W = crate::W<RegFwDbg5Spec>;
#[doc = "Field `BIT_FW_DBG5` reader - fw debug5"]
pub type BitFwDbg5R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_DBG5` writer - fw debug5"]
pub type BitFwDbg5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw debug5"]
    #[inline(always)]
    pub fn bit_fw_dbg5(&self) -> BitFwDbg5R {
        BitFwDbg5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw debug5"]
    #[inline(always)]
    pub fn bit_fw_dbg5(&mut self) -> BitFwDbg5W<'_, RegFwDbg5Spec> {
        BitFwDbg5W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg5Spec;
impl crate::RegisterSpec for RegFwDbg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg5::R`](R) reader structure"]
impl crate::Readable for RegFwDbg5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg5::W`](W) writer structure"]
impl crate::Writable for RegFwDbg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG5 to value 0"]
impl crate::Resettable for RegFwDbg5Spec {}
