#[doc = "Register `REG_WL_PIN_FUNC_CTRL0` reader"]
pub type R = crate::R<RegWlPinFuncCtrl0Spec>;
#[doc = "Register `REG_WL_PIN_FUNC_CTRL0` writer"]
pub type W = crate::W<RegWlPinFuncCtrl0Spec>;
#[doc = "Field `BIT_WL_DBG_SEL` reader - WL debug select"]
pub type BitWlDbgSelR = crate::FieldReader;
#[doc = "Field `BIT_WL_DBG_SEL` writer - WL debug select"]
pub type BitWlDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 24:31 - WL debug select"]
    #[inline(always)]
    pub fn bit_wl_dbg_sel(&self) -> BitWlDbgSelR {
        BitWlDbgSelR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - WL debug select"]
    #[inline(always)]
    pub fn bit_wl_dbg_sel(&mut self) -> BitWlDbgSelW<'_, RegWlPinFuncCtrl0Spec> {
        BitWlDbgSelW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wl_pin_func_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wl_pin_func_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlPinFuncCtrl0Spec;
impl crate::RegisterSpec for RegWlPinFuncCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wl_pin_func_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegWlPinFuncCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_wl_pin_func_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegWlPinFuncCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WL_PIN_FUNC_CTRL0 to value 0"]
impl crate::Resettable for RegWlPinFuncCtrl0Spec {}
