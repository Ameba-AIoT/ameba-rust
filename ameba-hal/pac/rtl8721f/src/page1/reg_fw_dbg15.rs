#[doc = "Register `REG_FW_DBG15` reader"]
pub type R = crate::R<RegFwDbg15Spec>;
#[doc = "Register `REG_FW_DBG15` writer"]
pub type W = crate::W<RegFwDbg15Spec>;
#[doc = "Field `BIT_MCUDMSG_II` reader - User Defined Message"]
pub type BitMcudmsgIiR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MCUDMSG_II` writer - User Defined Message"]
pub type BitMcudmsgIiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_mcudmsg_ii(&self) -> BitMcudmsgIiR {
        BitMcudmsgIiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_mcudmsg_ii(&mut self) -> BitMcudmsgIiW<'_, RegFwDbg15Spec> {
        BitMcudmsgIiW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg15Spec;
impl crate::RegisterSpec for RegFwDbg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg15::R`](R) reader structure"]
impl crate::Readable for RegFwDbg15Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg15::W`](W) writer structure"]
impl crate::Writable for RegFwDbg15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG15 to value 0"]
impl crate::Resettable for RegFwDbg15Spec {}
