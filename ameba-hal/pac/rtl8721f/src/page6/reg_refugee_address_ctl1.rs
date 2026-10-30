#[doc = "Register `REG_REFUGEE_ADDRESS_CTL1` reader"]
pub type R = crate::R<RegRefugeeAddressCtl1Spec>;
#[doc = "Register `REG_REFUGEE_ADDRESS_CTL1` writer"]
pub type W = crate::W<RegRefugeeAddressCtl1Spec>;
#[doc = "Field `r_refug_addr_clred_h` reader - high 16 bit of refugee mac address that need to be remove from refugee cam"]
pub type RRefugAddrClredHR = crate::FieldReader<u16>;
#[doc = "Field `r_refug_addr_clred_h` writer - high 16 bit of refugee mac address that need to be remove from refugee cam"]
pub type RRefugAddrClredHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `refug_clr_req` reader - 1 means hw is removing the refugee mac address. Set 1 when sw poll r_refug_clr_poll and set 0 when the refugee address is removed"]
pub type RefugClrReqR = crate::BitReader;
#[doc = "Field `r_refug_clr_poll` writer - gnenerate one pulse when set 1. sw request hw to clear refug_addr_clred in refugee cam"]
pub type RRefugClrPollW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_refug_clr_all` writer - gnenerate one pulse when set 1. clear all address in refugee cam"]
pub type RRefugClrAllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - high 16 bit of refugee mac address that need to be remove from refugee cam"]
    #[inline(always)]
    pub fn r_refug_addr_clred_h(&self) -> RRefugAddrClredHR {
        RRefugAddrClredHR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 1 means hw is removing the refugee mac address. Set 1 when sw poll r_refug_clr_poll and set 0 when the refugee address is removed"]
    #[inline(always)]
    pub fn refug_clr_req(&self) -> RefugClrReqR {
        RefugClrReqR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - high 16 bit of refugee mac address that need to be remove from refugee cam"]
    #[inline(always)]
    pub fn r_refug_addr_clred_h(&mut self) -> RRefugAddrClredHW<'_, RegRefugeeAddressCtl1Spec> {
        RRefugAddrClredHW::new(self, 0)
    }
    #[doc = "Bit 17 - gnenerate one pulse when set 1. sw request hw to clear refug_addr_clred in refugee cam"]
    #[inline(always)]
    pub fn r_refug_clr_poll(&mut self) -> RRefugClrPollW<'_, RegRefugeeAddressCtl1Spec> {
        RRefugClrPollW::new(self, 17)
    }
    #[doc = "Bit 18 - gnenerate one pulse when set 1. clear all address in refugee cam"]
    #[inline(always)]
    pub fn r_refug_clr_all(&mut self) -> RRefugClrAllW<'_, RegRefugeeAddressCtl1Spec> {
        RRefugClrAllW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_refugee_address_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_refugee_address_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRefugeeAddressCtl1Spec;
impl crate::RegisterSpec for RegRefugeeAddressCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_refugee_address_ctl1::R`](R) reader structure"]
impl crate::Readable for RegRefugeeAddressCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_refugee_address_ctl1::W`](W) writer structure"]
impl crate::Writable for RegRefugeeAddressCtl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_REFUGEE_ADDRESS_CTL1 to value 0"]
impl crate::Resettable for RegRefugeeAddressCtl1Spec {}
