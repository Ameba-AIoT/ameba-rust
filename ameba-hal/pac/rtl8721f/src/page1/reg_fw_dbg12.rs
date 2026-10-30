#[doc = "Register `REG_FW_DBG12` reader"]
pub type R = crate::R<RegFwDbg12Spec>;
#[doc = "Register `REG_FW_DBG12` writer"]
pub type W = crate::W<RegFwDbg12Spec>;
#[doc = "Field `BIT_SW_DEFINED_PAGE1_V1` reader - User Defined Message"]
pub type BitSwDefinedPage1V1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_SW_DEFINED_PAGE1_V1` writer - User Defined Message"]
pub type BitSwDefinedPage1V1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_sw_defined_page1_v1(&self) -> BitSwDefinedPage1V1R {
        BitSwDefinedPage1V1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_sw_defined_page1_v1(&mut self) -> BitSwDefinedPage1V1W<'_, RegFwDbg12Spec> {
        BitSwDefinedPage1V1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg12Spec;
impl crate::RegisterSpec for RegFwDbg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg12::R`](R) reader structure"]
impl crate::Readable for RegFwDbg12Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg12::W`](W) writer structure"]
impl crate::Writable for RegFwDbg12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG12 to value 0"]
impl crate::Resettable for RegFwDbg12Spec {}
