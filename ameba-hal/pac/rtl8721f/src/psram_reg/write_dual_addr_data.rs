#[doc = "Register `WRITE_DUAL_ADDR_DATA` reader"]
pub type R = crate::R<WriteDualAddrDataSpec>;
#[doc = "Register `WRITE_DUAL_ADDR_DATA` writer"]
pub type W = crate::W<WriteDualAddrDataSpec>;
#[doc = "Field `WR_DUAL_II_CMD` reader - Indicates SPI Flash command value of dual address and data write command."]
pub type WrDualIiCmdR = crate::FieldReader;
#[doc = "Field `WR_DUAL_II_CMD` writer - Indicates SPI Flash command value of dual address and data write command."]
pub type WrDualIiCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of dual address and data write command."]
    #[inline(always)]
    pub fn wr_dual_ii_cmd(&self) -> WrDualIiCmdR {
        WrDualIiCmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of dual address and data write command."]
    #[inline(always)]
    pub fn wr_dual_ii_cmd(&mut self) -> WrDualIiCmdW<'_, WriteDualAddrDataSpec> {
        WrDualIiCmdW::new(self, 0)
    }
}
#[doc = "Dual Address and Data Output Read Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`write_dual_addr_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_dual_addr_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteDualAddrDataSpec;
impl crate::RegisterSpec for WriteDualAddrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`write_dual_addr_data::R`](R) reader structure"]
impl crate::Readable for WriteDualAddrDataSpec {}
#[doc = "`write(|w| ..)` method takes [`write_dual_addr_data::W`](W) writer structure"]
impl crate::Writable for WriteDualAddrDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITE_DUAL_ADDR_DATA to value 0"]
impl crate::Resettable for WriteDualAddrDataSpec {}
