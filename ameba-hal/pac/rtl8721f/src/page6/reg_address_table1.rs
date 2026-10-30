#[doc = "Register `REG_ADDRESS_TABLE1` reader"]
pub type R = crate::R<RegAddressTable1Spec>;
#[doc = "Register `REG_ADDRESS_TABLE1` writer"]
pub type W = crate::W<RegAddressTable1Spec>;
#[doc = "Field `r_addr_table_user_reg0_h` reader - high 16 bit of address table reg0"]
pub type RAddrTableUserReg0HR = crate::FieldReader<u16>;
#[doc = "Field `r_addr_table_user_reg0_h` writer - high 16 bit of address table reg0"]
pub type RAddrTableUserReg0HW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_addr_table_user_reg1_l` reader - low 16 bit of address table reg1"]
pub type RAddrTableUserReg1LR = crate::FieldReader<u16>;
#[doc = "Field `r_addr_table_user_reg1_l` writer - low 16 bit of address table reg1"]
pub type RAddrTableUserReg1LW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - high 16 bit of address table reg0"]
    #[inline(always)]
    pub fn r_addr_table_user_reg0_h(&self) -> RAddrTableUserReg0HR {
        RAddrTableUserReg0HR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - low 16 bit of address table reg1"]
    #[inline(always)]
    pub fn r_addr_table_user_reg1_l(&self) -> RAddrTableUserReg1LR {
        RAddrTableUserReg1LR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - high 16 bit of address table reg0"]
    #[inline(always)]
    pub fn r_addr_table_user_reg0_h(&mut self) -> RAddrTableUserReg0HW<'_, RegAddressTable1Spec> {
        RAddrTableUserReg0HW::new(self, 0)
    }
    #[doc = "Bits 16:31 - low 16 bit of address table reg1"]
    #[inline(always)]
    pub fn r_addr_table_user_reg1_l(&mut self) -> RAddrTableUserReg1LW<'_, RegAddressTable1Spec> {
        RAddrTableUserReg1LW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_address_table1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_address_table1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAddressTable1Spec;
impl crate::RegisterSpec for RegAddressTable1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_address_table1::R`](R) reader structure"]
impl crate::Readable for RegAddressTable1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_address_table1::W`](W) writer structure"]
impl crate::Writable for RegAddressTable1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADDRESS_TABLE1 to value 0"]
impl crate::Resettable for RegAddressTable1Spec {}
