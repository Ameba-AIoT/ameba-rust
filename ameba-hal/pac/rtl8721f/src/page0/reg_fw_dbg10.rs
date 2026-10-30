#[doc = "Register `REG_FW_DBG10` reader"]
pub type R = crate::R<RegFwDbg10Spec>;
#[doc = "Register `REG_FW_DBG10` writer"]
pub type W = crate::W<RegFwDbg10Spec>;
#[doc = "Field `BIT_HOST_MSG_E2` reader - User Defined Message"]
pub type BitHostMsgE2R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HOST_MSG_E2` writer - User Defined Message"]
pub type BitHostMsgE2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_HOST_MSG_E3` reader - User Defined Message"]
pub type BitHostMsgE3R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HOST_MSG_E3` writer - User Defined Message"]
pub type BitHostMsgE3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_e2(&self) -> BitHostMsgE2R {
        BitHostMsgE2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_e3(&self) -> BitHostMsgE3R {
        BitHostMsgE3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_e2(&mut self) -> BitHostMsgE2W<'_, RegFwDbg10Spec> {
        BitHostMsgE2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_e3(&mut self) -> BitHostMsgE3W<'_, RegFwDbg10Spec> {
        BitHostMsgE3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg10Spec;
impl crate::RegisterSpec for RegFwDbg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg10::R`](R) reader structure"]
impl crate::Readable for RegFwDbg10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg10::W`](W) writer structure"]
impl crate::Writable for RegFwDbg10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG10 to value 0"]
impl crate::Resettable for RegFwDbg10Spec {}
