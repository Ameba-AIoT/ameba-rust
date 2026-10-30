#[doc = "Register `REG_FW_DBG2` reader"]
pub type R = crate::R<RegFwDbg2Spec>;
#[doc = "Register `REG_FW_DBG2` writer"]
pub type W = crate::W<RegFwDbg2Spec>;
#[doc = "Field `BIT_FW_DBG2` reader - fw debug2"]
pub type BitFwDbg2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_DBG2` writer - fw debug2"]
pub type BitFwDbg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw debug2"]
    #[inline(always)]
    pub fn bit_fw_dbg2(&self) -> BitFwDbg2R {
        BitFwDbg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw debug2"]
    #[inline(always)]
    pub fn bit_fw_dbg2(&mut self) -> BitFwDbg2W<'_, RegFwDbg2Spec> {
        BitFwDbg2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg2Spec;
impl crate::RegisterSpec for RegFwDbg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg2::R`](R) reader structure"]
impl crate::Readable for RegFwDbg2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg2::W`](W) writer structure"]
impl crate::Writable for RegFwDbg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG2 to value 0"]
impl crate::Resettable for RegFwDbg2Spec {}
