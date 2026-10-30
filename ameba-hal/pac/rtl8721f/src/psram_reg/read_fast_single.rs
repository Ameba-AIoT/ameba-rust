#[doc = "Register `READ_FAST_SINGLE` reader"]
pub type R = crate::R<ReadFastSingleSpec>;
#[doc = "Register `READ_FAST_SINGLE` writer"]
pub type W = crate::W<ReadFastSingleSpec>;
#[doc = "Field `FRD_CMD` reader - Indicate SPI Flash command value of fast read command. The baud rate is used with FBAUDR. FRD_CMD\\[15:8\\] is for RD_OCTA_IO_CMD_2nd_BYTE (only available when configuration of DDR_EN is defined) FRD_CMD\\[7:0\\] is for RD_OCTA_IO_CMD_1st_BYTE"]
pub type FrdCmdR = crate::FieldReader<u16>;
#[doc = "Field `FRD_CMD` writer - Indicate SPI Flash command value of fast read command. The baud rate is used with FBAUDR. FRD_CMD\\[15:8\\] is for RD_OCTA_IO_CMD_2nd_BYTE (only available when configuration of DDR_EN is defined) FRD_CMD\\[7:0\\] is for RD_OCTA_IO_CMD_1st_BYTE"]
pub type FrdCmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicate SPI Flash command value of fast read command. The baud rate is used with FBAUDR. FRD_CMD\\[15:8\\] is for RD_OCTA_IO_CMD_2nd_BYTE (only available when configuration of DDR_EN is defined) FRD_CMD\\[7:0\\] is for RD_OCTA_IO_CMD_1st_BYTE"]
    #[inline(always)]
    pub fn frd_cmd(&self) -> FrdCmdR {
        FrdCmdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicate SPI Flash command value of fast read command. The baud rate is used with FBAUDR. FRD_CMD\\[15:8\\] is for RD_OCTA_IO_CMD_2nd_BYTE (only available when configuration of DDR_EN is defined) FRD_CMD\\[7:0\\] is for RD_OCTA_IO_CMD_1st_BYTE"]
    #[inline(always)]
    pub fn frd_cmd(&mut self) -> FrdCmdW<'_, ReadFastSingleSpec> {
        FrdCmdW::new(self, 0)
    }
}
#[doc = "Fast Read Data Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`read_fast_single::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_fast_single::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadFastSingleSpec;
impl crate::RegisterSpec for ReadFastSingleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_fast_single::R`](R) reader structure"]
impl crate::Readable for ReadFastSingleSpec {}
#[doc = "`write(|w| ..)` method takes [`read_fast_single::W`](W) writer structure"]
impl crate::Writable for ReadFastSingleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READ_FAST_SINGLE to value 0"]
impl crate::Resettable for ReadFastSingleSpec {}
