#[doc = "Register `WRITE_SIGNLE` reader"]
pub type R = crate::R<WriteSignleSpec>;
#[doc = "Register `WRITE_SIGNLE` writer"]
pub type W = crate::W<WriteSignleSpec>;
#[doc = "Field `WR_CMD` reader - Indicate SPI Flash command value of write command. WR_CMD\\[15:8\\] is for WR_OCTA_IO_CMD_2nd_BYTE (only available when configuration of DDR_EN is defined) WR_CMD\\[7:0\\] is for WR_OCTA_IO_CMD_1st_BYTE"]
pub type WrCmdR = crate::FieldReader<u16>;
#[doc = "Field `WR_CMD` writer - Indicate SPI Flash command value of write command. WR_CMD\\[15:8\\] is for WR_OCTA_IO_CMD_2nd_BYTE (only available when configuration of DDR_EN is defined) WR_CMD\\[7:0\\] is for WR_OCTA_IO_CMD_1st_BYTE"]
pub type WrCmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicate SPI Flash command value of write command. WR_CMD\\[15:8\\] is for WR_OCTA_IO_CMD_2nd_BYTE (only available when configuration of DDR_EN is defined) WR_CMD\\[7:0\\] is for WR_OCTA_IO_CMD_1st_BYTE"]
    #[inline(always)]
    pub fn wr_cmd(&self) -> WrCmdR {
        WrCmdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicate SPI Flash command value of write command. WR_CMD\\[15:8\\] is for WR_OCTA_IO_CMD_2nd_BYTE (only available when configuration of DDR_EN is defined) WR_CMD\\[7:0\\] is for WR_OCTA_IO_CMD_1st_BYTE"]
    #[inline(always)]
    pub fn wr_cmd(&mut self) -> WrCmdW<'_, WriteSignleSpec> {
        WrCmdW::new(self, 0)
    }
}
#[doc = "Page Program Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`write_signle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_signle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteSignleSpec;
impl crate::RegisterSpec for WriteSignleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`write_signle::R`](R) reader structure"]
impl crate::Readable for WriteSignleSpec {}
#[doc = "`write(|w| ..)` method takes [`write_signle::W`](W) writer structure"]
impl crate::Writable for WriteSignleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITE_SIGNLE to value 0xca"]
impl crate::Resettable for WriteSignleSpec {
    const RESET_VALUE: u32 = 0xca;
}
