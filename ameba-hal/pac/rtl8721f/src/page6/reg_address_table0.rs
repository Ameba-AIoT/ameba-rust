#[doc = "Register `REG_ADDRESS_TABLE0` reader"]
pub type R = crate::R<RegAddressTable0Spec>;
#[doc = "Register `REG_ADDRESS_TABLE0` writer"]
pub type W = crate::W<RegAddressTable0Spec>;
#[doc = "Field `r_addr_table_user_reg0_l` reader - low 32 bit of address table reg0"]
pub type RAddrTableUserReg0LR = crate::FieldReader<u32>;
#[doc = "Field `r_addr_table_user_reg0_l` writer - low 32 bit of address table reg0"]
pub type RAddrTableUserReg0LW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - low 32 bit of address table reg0"]
    #[inline(always)]
    pub fn r_addr_table_user_reg0_l(&self) -> RAddrTableUserReg0LR {
        RAddrTableUserReg0LR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - low 32 bit of address table reg0"]
    #[inline(always)]
    pub fn r_addr_table_user_reg0_l(&mut self) -> RAddrTableUserReg0LW<'_, RegAddressTable0Spec> {
        RAddrTableUserReg0LW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_address_table0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_address_table0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddressTable0Spec;
impl crate::RegisterSpec for RegAddressTable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_address_table0::R`](R) reader structure"]
impl crate::Readable for RegAddressTable0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_address_table0::W`](W) writer structure"]
impl crate::Writable for RegAddressTable0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESS_TABLE0 to value 0"]
impl crate::Resettable for RegAddressTable0Spec {}
