#[doc = "Register `REG_ADDRESS_TABLE2` reader"]
pub type R = crate::R<RegAddressTable2Spec>;
#[doc = "Register `REG_ADDRESS_TABLE2` writer"]
pub type W = crate::W<RegAddressTable2Spec>;
#[doc = "Field `r_addr_table_user_reg1_h` reader - high 32 bit of address table reg1"]
pub type RAddrTableUserReg1HR = crate::FieldReader<u32>;
#[doc = "Field `r_addr_table_user_reg1_h` writer - high 32 bit of address table reg1"]
pub type RAddrTableUserReg1HW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - high 32 bit of address table reg1"]
    #[inline(always)]
    pub fn r_addr_table_user_reg1_h(&self) -> RAddrTableUserReg1HR {
        RAddrTableUserReg1HR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - high 32 bit of address table reg1"]
    #[inline(always)]
    pub fn r_addr_table_user_reg1_h(&mut self) -> RAddrTableUserReg1HW<'_, RegAddressTable2Spec> {
        RAddrTableUserReg1HW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_address_table2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_address_table2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddressTable2Spec;
impl crate::RegisterSpec for RegAddressTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_address_table2::R`](R) reader structure"]
impl crate::Readable for RegAddressTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_address_table2::W`](W) writer structure"]
impl crate::Writable for RegAddressTable2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESS_TABLE2 to value 0"]
impl crate::Resettable for RegAddressTable2Spec {}
