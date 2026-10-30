#[doc = "Register `WRITE_QUAD_DATA` reader"]
pub type R = crate::R<WriteQuadDataSpec>;
#[doc = "Register `WRITE_QUAD_DATA` writer"]
pub type W = crate::W<WriteQuadDataSpec>;
#[doc = "Field `WR_QUAD_I_CMD` reader - Indicates SPI Flash command value of quad data write command."]
pub type WrQuadICmdR = crate::FieldReader;
#[doc = "Field `WR_QUAD_I_CMD` writer - Indicates SPI Flash command value of quad data write command."]
pub type WrQuadICmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of quad data write command."]
    #[inline(always)]
    pub fn wr_quad_i_cmd(&self) -> WrQuadICmdR {
        WrQuadICmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of quad data write command."]
    #[inline(always)]
    pub fn wr_quad_i_cmd(&mut self) -> WrQuadICmdW<'_, WriteQuadDataSpec> {
        WrQuadICmdW::new(self, 0)
    }
}
#[doc = "Quad Data Input Program Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`write_quad_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_quad_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteQuadDataSpec;
impl crate::RegisterSpec for WriteQuadDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`write_quad_data::R`](R) reader structure"]
impl crate::Readable for WriteQuadDataSpec {}
#[doc = "`write(|w| ..)` method takes [`write_quad_data::W`](W) writer structure"]
impl crate::Writable for WriteQuadDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITE_QUAD_DATA to value 0x20"]
impl crate::Resettable for WriteQuadDataSpec {
    const RESET_VALUE: u32 = 0x20;
}
