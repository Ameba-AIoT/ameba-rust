#[doc = "Register `REG_FW_DBG14` reader"]
pub type R = crate::R<RegFwDbg14Spec>;
#[doc = "Register `REG_FW_DBG14` writer"]
pub type W = crate::W<RegFwDbg14Spec>;
#[doc = "Field `BIT_MCUDMSG_I` reader - User Defined Message"]
pub type BitMcudmsgIR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MCUDMSG_I` writer - User Defined Message"]
pub type BitMcudmsgIW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_mcudmsg_i(&self) -> BitMcudmsgIR {
        BitMcudmsgIR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_mcudmsg_i(&mut self) -> BitMcudmsgIW<'_, RegFwDbg14Spec> {
        BitMcudmsgIW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg14Spec;
impl crate::RegisterSpec for RegFwDbg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg14::R`](R) reader structure"]
impl crate::Readable for RegFwDbg14Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg14::W`](W) writer structure"]
impl crate::Writable for RegFwDbg14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG14 to value 0"]
impl crate::Resettable for RegFwDbg14Spec {}
