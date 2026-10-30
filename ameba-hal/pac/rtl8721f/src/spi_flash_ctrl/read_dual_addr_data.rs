#[doc = "Register `READ_DUAL_ADDR_DATA` reader"]
pub type R = crate::R<ReadDualAddrDataSpec>;
#[doc = "Register `READ_DUAL_ADDR_DATA` writer"]
pub type W = crate::W<ReadDualAddrDataSpec>;
#[doc = "Field `RD_DUAL_IO_CMD` reader - Indicates SPI Flash command value of dual address and data read command."]
pub type RdDualIoCmdR = crate::FieldReader;
#[doc = "Field `RD_DUAL_IO_CMD` writer - Indicates SPI Flash command value of dual address and data read command."]
pub type RdDualIoCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of dual address and data read command."]
    #[inline(always)]
    pub fn rd_dual_io_cmd(&self) -> RdDualIoCmdR {
        RdDualIoCmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of dual address and data read command."]
    #[inline(always)]
    pub fn rd_dual_io_cmd(&mut self) -> RdDualIoCmdW<'_, ReadDualAddrDataSpec> {
        RdDualIoCmdW::new(self, 0)
    }
}
#[doc = "Dual I/O Read Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`read_dual_addr_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_dual_addr_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadDualAddrDataSpec;
impl crate::RegisterSpec for ReadDualAddrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_dual_addr_data::R`](R) reader structure"]
impl crate::Readable for ReadDualAddrDataSpec {}
#[doc = "`write(|w| ..)` method takes [`read_dual_addr_data::W`](W) writer structure"]
impl crate::Writable for ReadDualAddrDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READ_DUAL_ADDR_DATA to value 0"]
impl crate::Resettable for ReadDualAddrDataSpec {}
