#[doc = "Register `REG_FW_DBG13` reader"]
pub type R = crate::R<RegFwDbg13Spec>;
#[doc = "Register `REG_FW_DBG13` writer"]
pub type W = crate::W<RegFwDbg13Spec>;
#[doc = "Field `BIT_SW_DEFINED_PAGE2` reader - User Defined Message"]
pub type BitSwDefinedPage2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_SW_DEFINED_PAGE2` writer - User Defined Message"]
pub type BitSwDefinedPage2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_sw_defined_page2(&self) -> BitSwDefinedPage2R {
        BitSwDefinedPage2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_sw_defined_page2(&mut self) -> BitSwDefinedPage2W<'_, RegFwDbg13Spec> {
        BitSwDefinedPage2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg13Spec;
impl crate::RegisterSpec for RegFwDbg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg13::R`](R) reader structure"]
impl crate::Readable for RegFwDbg13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg13::W`](W) writer structure"]
impl crate::Writable for RegFwDbg13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG13 to value 0"]
impl crate::Resettable for RegFwDbg13Spec {}
