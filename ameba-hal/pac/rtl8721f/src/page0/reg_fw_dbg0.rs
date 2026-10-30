#[doc = "Register `REG_FW_DBG0` reader"]
pub type R = crate::R<RegFwDbg0Spec>;
#[doc = "Register `REG_FW_DBG0` writer"]
pub type W = crate::W<RegFwDbg0Spec>;
#[doc = "Field `BIT_FW_DBG0` reader - fw debug0"]
pub type BitFwDbg0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_DBG0` writer - fw debug0"]
pub type BitFwDbg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw debug0"]
    #[inline(always)]
    pub fn bit_fw_dbg0(&self) -> BitFwDbg0R {
        BitFwDbg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw debug0"]
    #[inline(always)]
    pub fn bit_fw_dbg0(&mut self) -> BitFwDbg0W<'_, RegFwDbg0Spec> {
        BitFwDbg0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg0Spec;
impl crate::RegisterSpec for RegFwDbg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg0::R`](R) reader structure"]
impl crate::Readable for RegFwDbg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg0::W`](W) writer structure"]
impl crate::Writable for RegFwDbg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG0 to value 0"]
impl crate::Resettable for RegFwDbg0Spec {}
