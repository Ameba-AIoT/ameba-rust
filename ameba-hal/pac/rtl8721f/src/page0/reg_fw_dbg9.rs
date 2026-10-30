#[doc = "Register `REG_FW_DBG9` reader"]
pub type R = crate::R<RegFwDbg9Spec>;
#[doc = "Register `REG_FW_DBG9` writer"]
pub type W = crate::W<RegFwDbg9Spec>;
#[doc = "Field `BIT_HOST_MSG_E0` reader - User Defined Message"]
pub type BitHostMsgE0R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HOST_MSG_E0` writer - User Defined Message"]
pub type BitHostMsgE0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_HOST_MSG_E1` reader - User Defined Message"]
pub type BitHostMsgE1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_HOST_MSG_E1` writer - User Defined Message"]
pub type BitHostMsgE1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_e0(&self) -> BitHostMsgE0R {
        BitHostMsgE0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_e1(&self) -> BitHostMsgE1R {
        BitHostMsgE1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_e0(&mut self) -> BitHostMsgE0W<'_, RegFwDbg9Spec> {
        BitHostMsgE0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - User Defined Message"]
    #[inline(always)]
    pub fn bit_host_msg_e1(&mut self) -> BitHostMsgE1W<'_, RegFwDbg9Spec> {
        BitHostMsgE1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg9Spec;
impl crate::RegisterSpec for RegFwDbg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg9::R`](R) reader structure"]
impl crate::Readable for RegFwDbg9Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg9::W`](W) writer structure"]
impl crate::Writable for RegFwDbg9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG9 to value 0"]
impl crate::Resettable for RegFwDbg9Spec {}
