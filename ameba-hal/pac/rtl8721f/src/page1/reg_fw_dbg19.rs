#[doc = "Register `REG_FW_DBG19` reader"]
pub type R = crate::R<RegFwDbg19Spec>;
#[doc = "Register `REG_FW_DBG19` writer"]
pub type W = crate::W<RegFwDbg19Spec>;
#[doc = "Field `BIT_FS_FW_DEBUG_3` reader - User Defined Message"]
pub type BitFsFwDebug3R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FS_FW_DEBUG_3` writer - User Defined Message"]
pub type BitFsFwDebug3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_fs_fw_debug_3(&self) -> BitFsFwDebug3R {
        BitFsFwDebug3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_fs_fw_debug_3(&mut self) -> BitFsFwDebug3W<'_, RegFwDbg19Spec> {
        BitFsFwDebug3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg19Spec;
impl crate::RegisterSpec for RegFwDbg19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg19::R`](R) reader structure"]
impl crate::Readable for RegFwDbg19Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg19::W`](W) writer structure"]
impl crate::Writable for RegFwDbg19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG19 to value 0"]
impl crate::Resettable for RegFwDbg19Spec {}
