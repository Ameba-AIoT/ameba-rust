#[doc = "Register `WRITE_QUAD_ADDR_DATA` reader"]
pub type R = crate::R<WriteQuadAddrDataSpec>;
#[doc = "Register `WRITE_QUAD_ADDR_DATA` writer"]
pub type W = crate::W<WriteQuadAddrDataSpec>;
#[doc = "Field `WR_QUAD_II_CMD` reader - Indicates SPI Flash command value of quad address and data write command."]
pub type WrQuadIiCmdR = crate::FieldReader;
#[doc = "Field `WR_QUAD_II_CMD` writer - Indicates SPI Flash command value of quad address and data write command."]
pub type WrQuadIiCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of quad address and data write command."]
    #[inline(always)]
    pub fn wr_quad_ii_cmd(&self) -> WrQuadIiCmdR {
        WrQuadIiCmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of quad address and data write command."]
    #[inline(always)]
    pub fn wr_quad_ii_cmd(&mut self) -> WrQuadIiCmdW<'_, WriteQuadAddrDataSpec> {
        WrQuadIiCmdW::new(self, 0)
    }
}
#[doc = "Quad Address and Data Output Read Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`write_quad_addr_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_quad_addr_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteQuadAddrDataSpec;
impl crate::RegisterSpec for WriteQuadAddrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`write_quad_addr_data::R`](R) reader structure"]
impl crate::Readable for WriteQuadAddrDataSpec {}
#[doc = "`write(|w| ..)` method takes [`write_quad_addr_data::W`](W) writer structure"]
impl crate::Writable for WriteQuadAddrDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITE_QUAD_ADDR_DATA to value 0x26"]
impl crate::Resettable for WriteQuadAddrDataSpec {
    const RESET_VALUE: u32 = 0x26;
}
