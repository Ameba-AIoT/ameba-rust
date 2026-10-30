#[doc = "Register `USER_LENGTH` reader"]
pub type R = crate::R<UserLengthSpec>;
#[doc = "Register `USER_LENGTH` writer"]
pub type W = crate::W<UserLengthSpec>;
#[doc = "Field `USER_RD_DUMMY_LENGTH` reader - Indicates delay cycles for receiving data in user mode (USER_MODE == 1). It is referenced by bus_clk."]
pub type UserRdDummyLengthR = crate::FieldReader<u16>;
#[doc = "Field `USER_RD_DUMMY_LENGTH` writer - Indicates delay cycles for receiving data in user mode (USER_MODE == 1). It is referenced by bus_clk."]
pub type UserRdDummyLengthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `USER_CMD_LENGHT` reader - Indicates number of bytes in command phase in user mode. USER_CMD_LENGHT can be 0~3."]
pub type UserCmdLenghtR = crate::FieldReader;
#[doc = "Field `USER_CMD_LENGHT` writer - Indicates number of bytes in command phase in user mode. USER_CMD_LENGHT can be 0~3."]
pub type UserCmdLenghtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USER_ADDR_LENGTH` reader - Indicates number of bytes in address phase (between command phase and write/read phase) in user mode. If it is set to 4, it will transmit 4-byte Address to support 4-byte address mode in SPI Flash."]
pub type UserAddrLengthR = crate::FieldReader;
#[doc = "Field `USER_ADDR_LENGTH` writer - Indicates number of bytes in address phase (between command phase and write/read phase) in user mode. If it is set to 4, it will transmit 4-byte Address to support 4-byte address mode in SPI Flash."]
pub type UserAddrLengthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - Indicates delay cycles for receiving data in user mode (USER_MODE == 1). It is referenced by bus_clk."]
    #[inline(always)]
    pub fn user_rd_dummy_length(&self) -> UserRdDummyLengthR {
        UserRdDummyLengthR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Indicates number of bytes in command phase in user mode. USER_CMD_LENGHT can be 0~3."]
    #[inline(always)]
    pub fn user_cmd_lenght(&self) -> UserCmdLenghtR {
        UserCmdLenghtR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Indicates number of bytes in address phase (between command phase and write/read phase) in user mode. If it is set to 4, it will transmit 4-byte Address to support 4-byte address mode in SPI Flash."]
    #[inline(always)]
    pub fn user_addr_length(&self) -> UserAddrLengthR {
        UserAddrLengthR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Indicates delay cycles for receiving data in user mode (USER_MODE == 1). It is referenced by bus_clk."]
    #[inline(always)]
    pub fn user_rd_dummy_length(&mut self) -> UserRdDummyLengthW<'_, UserLengthSpec> {
        UserRdDummyLengthW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Indicates number of bytes in command phase in user mode. USER_CMD_LENGHT can be 0~3."]
    #[inline(always)]
    pub fn user_cmd_lenght(&mut self) -> UserCmdLenghtW<'_, UserLengthSpec> {
        UserCmdLenghtW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Indicates number of bytes in address phase (between command phase and write/read phase) in user mode. If it is set to 4, it will transmit 4-byte Address to support 4-byte address mode in SPI Flash."]
    #[inline(always)]
    pub fn user_addr_length(&mut self) -> UserAddrLengthW<'_, UserLengthSpec> {
        UserAddrLengthW::new(self, 16)
    }
}
#[doc = "User Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user_length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserLengthSpec;
impl crate::RegisterSpec for UserLengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user_length::R`](R) reader structure"]
impl crate::Readable for UserLengthSpec {}
#[doc = "`write(|w| ..)` method takes [`user_length::W`](W) writer structure"]
impl crate::Writable for UserLengthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER_LENGTH to value 0x0003_0000"]
impl crate::Resettable for UserLengthSpec {
    const RESET_VALUE: u32 = 0x0003_0000;
}
