#[doc = "Register `REG_REFUGEE_ADDRESS_CTL0` reader"]
pub type R = crate::R<RegRefugeeAddressCtl0Spec>;
#[doc = "Register `REG_REFUGEE_ADDRESS_CTL0` writer"]
pub type W = crate::W<RegRefugeeAddressCtl0Spec>;
#[doc = "Field `r_refug_addr_clred_l` reader - low 32 bit of refugee mac address that need to be remove from refugee cam"]
pub type RRefugAddrClredLR = crate::FieldReader<u32>;
#[doc = "Field `r_refug_addr_clred_l` writer - low 32 bit of refugee mac address that need to be remove from refugee cam"]
pub type RRefugAddrClredLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low 32 bit of refugee mac address that need to be remove from refugee cam"]
    #[inline(always)]
    pub fn r_refug_addr_clred_l(&self) -> RRefugAddrClredLR {
        RRefugAddrClredLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low 32 bit of refugee mac address that need to be remove from refugee cam"]
    #[inline(always)]
    pub fn r_refug_addr_clred_l(&mut self) -> RRefugAddrClredLW<'_, RegRefugeeAddressCtl0Spec> {
        RRefugAddrClredLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_refugee_address_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_refugee_address_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRefugeeAddressCtl0Spec;
impl crate::RegisterSpec for RegRefugeeAddressCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_refugee_address_ctl0::R`](R) reader structure"]
impl crate::Readable for RegRefugeeAddressCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_refugee_address_ctl0::W`](W) writer structure"]
impl crate::Writable for RegRefugeeAddressCtl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_REFUGEE_ADDRESS_CTL0 to value 0"]
impl crate::Resettable for RegRefugeeAddressCtl0Spec {}
