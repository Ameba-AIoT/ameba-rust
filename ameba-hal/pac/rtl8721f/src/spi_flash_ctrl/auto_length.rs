#[doc = "Register `AUTO_LENGTH` reader"]
pub type R = crate::R<AutoLengthSpec>;
#[doc = "Register `AUTO_LENGTH` writer"]
pub type W = crate::W<AutoLengthSpec>;
#[doc = "Field `RD_DUMMY_LENGTH` reader - Indicates delay cycles for receiving data. It is referenced by bus_clk. Note: Don't include SPIC CLK (bus_clk) cycles from pad to internal SPIC"]
pub type RdDummyLengthR = crate::FieldReader<u16>;
#[doc = "Field `RD_DUMMY_LENGTH` writer - Indicates delay cycles for receiving data. It is referenced by bus_clk. Note: Don't include SPIC CLK (bus_clk) cycles from pad to internal SPIC"]
pub type RdDummyLengthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `IN_PHYSICAL_CYC` reader - Indicates how many SPIC CLK (bus_clk) cycles from pad to internal SPIC. It is used in both auto mode and user mode."]
pub type InPhysicalCycR = crate::FieldReader;
#[doc = "Field `IN_PHYSICAL_CYC` writer - Indicates how many SPIC CLK (bus_clk) cycles from pad to internal SPIC. It is used in both auto mode and user mode."]
pub type InPhysicalCycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AUTO_ADDR_LENGTH` reader - Indicates number of bytes address in read/write command in auto mode. AUTO_ADDR_LENGTH should be 1, 2, 3, 4 bytes. If it is set to 4, it will transmit 4-byte Address to support 4-byte address mode in SPI Flash. Note: PRM in auto mode should set VALID_CMD\\[11\\] (PRM_EN) with correct AUTO_ADDR_LENGTH."]
pub type AutoAddrLengthR = crate::FieldReader;
#[doc = "Field `AUTO_ADDR_LENGTH` writer - Indicates number of bytes address in read/write command in auto mode. AUTO_ADDR_LENGTH should be 1, 2, 3, 4 bytes. If it is set to 4, it will transmit 4-byte Address to support 4-byte address mode in SPI Flash. Note: PRM in auto mode should set VALID_CMD\\[11\\] (PRM_EN) with correct AUTO_ADDR_LENGTH."]
pub type AutoAddrLengthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDSR_DUMMY_LENGTH` reader - Indicates delay cycles for receiving data after Read Status Register (RDSR) command. (auto write or user mode with auto check) It is referenced by bus_clk."]
pub type RdsrDummyLengthR = crate::FieldReader;
#[doc = "Field `RDSR_DUMMY_LENGTH` writer - Indicates delay cycles for receiving data after Read Status Register (RDSR) command. (auto write or user mode with auto check) It is referenced by bus_clk."]
pub type RdsrDummyLengthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AUTO_CMD_LENGTH` reader - Indicate byte numbers of read/write command in auto mode. AUTO_CMD_LENGTH should be 1 or 2 bytes. Note: If set 0, it will transmit 1 byte command to device. Exists: TAG >= 2.0.8."]
pub type AutoCmdLengthR = crate::FieldReader;
#[doc = "Field `AUTO_CMD_LENGTH` writer - Indicate byte numbers of read/write command in auto mode. AUTO_CMD_LENGTH should be 1 or 2 bytes. Note: If set 0, it will transmit 1 byte command to device. Exists: TAG >= 2.0.8."]
pub type AutoCmdLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - Indicates delay cycles for receiving data. It is referenced by bus_clk. Note: Don't include SPIC CLK (bus_clk) cycles from pad to internal SPIC"]
    #[inline(always)]
    pub fn rd_dummy_length(&self) -> RdDummyLengthR {
        RdDummyLengthR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Indicates how many SPIC CLK (bus_clk) cycles from pad to internal SPIC. It is used in both auto mode and user mode."]
    #[inline(always)]
    pub fn in_physical_cyc(&self) -> InPhysicalCycR {
        InPhysicalCycR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates number of bytes address in read/write command in auto mode. AUTO_ADDR_LENGTH should be 1, 2, 3, 4 bytes. If it is set to 4, it will transmit 4-byte Address to support 4-byte address mode in SPI Flash. Note: PRM in auto mode should set VALID_CMD\\[11\\] (PRM_EN) with correct AUTO_ADDR_LENGTH."]
    #[inline(always)]
    pub fn auto_addr_length(&self) -> AutoAddrLengthR {
        AutoAddrLengthR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Indicates delay cycles for receiving data after Read Status Register (RDSR) command. (auto write or user mode with auto check) It is referenced by bus_clk."]
    #[inline(always)]
    pub fn rdsr_dummy_length(&self) -> RdsrDummyLengthR {
        RdsrDummyLengthR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:29 - Indicate byte numbers of read/write command in auto mode. AUTO_CMD_LENGTH should be 1 or 2 bytes. Note: If set 0, it will transmit 1 byte command to device. Exists: TAG >= 2.0.8."]
    #[inline(always)]
    pub fn auto_cmd_length(&self) -> AutoCmdLengthR {
        AutoCmdLengthR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Indicates delay cycles for receiving data. It is referenced by bus_clk. Note: Don't include SPIC CLK (bus_clk) cycles from pad to internal SPIC"]
    #[inline(always)]
    pub fn rd_dummy_length(&mut self) -> RdDummyLengthW<'_, AutoLengthSpec> {
        RdDummyLengthW::new(self, 0)
    }
    #[doc = "Bits 12:15 - Indicates how many SPIC CLK (bus_clk) cycles from pad to internal SPIC. It is used in both auto mode and user mode."]
    #[inline(always)]
    pub fn in_physical_cyc(&mut self) -> InPhysicalCycW<'_, AutoLengthSpec> {
        InPhysicalCycW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Indicates number of bytes address in read/write command in auto mode. AUTO_ADDR_LENGTH should be 1, 2, 3, 4 bytes. If it is set to 4, it will transmit 4-byte Address to support 4-byte address mode in SPI Flash. Note: PRM in auto mode should set VALID_CMD\\[11\\] (PRM_EN) with correct AUTO_ADDR_LENGTH."]
    #[inline(always)]
    pub fn auto_addr_length(&mut self) -> AutoAddrLengthW<'_, AutoLengthSpec> {
        AutoAddrLengthW::new(self, 16)
    }
    #[doc = "Bits 20:27 - Indicates delay cycles for receiving data after Read Status Register (RDSR) command. (auto write or user mode with auto check) It is referenced by bus_clk."]
    #[inline(always)]
    pub fn rdsr_dummy_length(&mut self) -> RdsrDummyLengthW<'_, AutoLengthSpec> {
        RdsrDummyLengthW::new(self, 20)
    }
    #[doc = "Bits 28:29 - Indicate byte numbers of read/write command in auto mode. AUTO_CMD_LENGTH should be 1 or 2 bytes. Note: If set 0, it will transmit 1 byte command to device. Exists: TAG >= 2.0.8."]
    #[inline(always)]
    pub fn auto_cmd_length(&mut self) -> AutoCmdLengthW<'_, AutoLengthSpec> {
        AutoCmdLengthW::new(self, 28)
    }
}
#[doc = "Auto Address Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`auto_length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auto_length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutoLengthSpec;
impl crate::RegisterSpec for AutoLengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auto_length::R`](R) reader structure"]
impl crate::Readable for AutoLengthSpec {}
#[doc = "`write(|w| ..)` method takes [`auto_length::W`](W) writer structure"]
impl crate::Writable for AutoLengthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTO_LENGTH to value 0x0003_0000"]
impl crate::Resettable for AutoLengthSpec {
    const RESET_VALUE: u32 = 0x0003_0000;
}
