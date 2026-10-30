#[doc = "Register `READ_QUAD_DATA` reader"]
pub type R = crate::R<ReadQuadDataSpec>;
#[doc = "Register `READ_QUAD_DATA` writer"]
pub type W = crate::W<ReadQuadDataSpec>;
#[doc = "Field `RD_QUAD_O_CMD` reader - Indicates SPI Flash command value of quad data read command."]
pub type RdQuadOCmdR = crate::FieldReader;
#[doc = "Field `RD_QUAD_O_CMD` writer - Indicates SPI Flash command value of quad data read command."]
pub type RdQuadOCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of quad data read command."]
    #[inline(always)]
    pub fn rd_quad_o_cmd(&self) -> RdQuadOCmdR {
        RdQuadOCmdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of quad data read command."]
    #[inline(always)]
    pub fn rd_quad_o_cmd(&mut self) -> RdQuadOCmdW<'_, ReadQuadDataSpec> {
        RdQuadOCmdW::new(self, 0)
    }
}
#[doc = "Qaud Output Read Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`read_quad_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_quad_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadQuadDataSpec;
impl crate::RegisterSpec for ReadQuadDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_quad_data::R`](R) reader structure"]
impl crate::Readable for ReadQuadDataSpec {}
#[doc = "`write(|w| ..)` method takes [`read_quad_data::W`](W) writer structure"]
impl crate::Writable for ReadQuadDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READ_QUAD_DATA to value 0"]
impl crate::Resettable for ReadQuadDataSpec {}
