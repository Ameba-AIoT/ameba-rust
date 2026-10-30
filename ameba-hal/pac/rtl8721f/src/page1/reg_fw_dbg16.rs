#[doc = "Register `REG_FW_DBG16` reader"]
pub type R = crate::R<RegFwDbg16Spec>;
#[doc = "Register `REG_FW_DBG16` writer"]
pub type W = crate::W<RegFwDbg16Spec>;
#[doc = "Field `BIT_FS_FW_DEBUG_0` reader - User Defined Message"]
pub type BitFsFwDebug0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FS_FW_DEBUG_0` writer - User Defined Message"]
pub type BitFsFwDebug0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_fs_fw_debug_0(&self) -> BitFsFwDebug0R {
        BitFsFwDebug0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_fs_fw_debug_0(&mut self) -> BitFsFwDebug0W<'_, RegFwDbg16Spec> {
        BitFsFwDebug0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg16Spec;
impl crate::RegisterSpec for RegFwDbg16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg16::R`](R) reader structure"]
impl crate::Readable for RegFwDbg16Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg16::W`](W) writer structure"]
impl crate::Writable for RegFwDbg16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG16 to value 0"]
impl crate::Resettable for RegFwDbg16Spec {}
