#[doc = "Register `REG_FW_DBG3` reader"]
pub type R = crate::R<RegFwDbg3Spec>;
#[doc = "Register `REG_FW_DBG3` writer"]
pub type W = crate::W<RegFwDbg3Spec>;
#[doc = "Field `BIT_FW_DBG3` reader - fw debug3"]
pub type BitFwDbg3R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_DBG3` writer - fw debug3"]
pub type BitFwDbg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw debug3"]
    #[inline(always)]
    pub fn bit_fw_dbg3(&self) -> BitFwDbg3R {
        BitFwDbg3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw debug3"]
    #[inline(always)]
    pub fn bit_fw_dbg3(&mut self) -> BitFwDbg3W<'_, RegFwDbg3Spec> {
        BitFwDbg3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg3Spec;
impl crate::RegisterSpec for RegFwDbg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg3::R`](R) reader structure"]
impl crate::Readable for RegFwDbg3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg3::W`](W) writer structure"]
impl crate::Writable for RegFwDbg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG3 to value 0"]
impl crate::Resettable for RegFwDbg3Spec {}
