#[doc = "Register `WRITE_ENABLE` reader"]
pub type R = crate::R<WriteEnableSpec>;
#[doc = "Register `WRITE_ENABLE` writer"]
pub type W = crate::W<WriteEnableSpec>;
#[doc = "Field `WR_EN_CMD_1st_BYTE` reader - Indicates SPI Flash command value of 1st byte of write enable."]
pub type WrEnCmd1stByteR = crate::FieldReader;
#[doc = "Field `WR_EN_CMD_1st_BYTE` writer - Indicates SPI Flash command value of 1st byte of write enable."]
pub type WrEnCmd1stByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WR_EN_CMD_2nd_BYTE` reader - Indicates SPI Flash command value of 2nd byte of write enable. Available only if configuration of DDR_EN is defined."]
pub type WrEnCmd2ndByteR = crate::FieldReader;
#[doc = "Field `WR_EN_CMD_2nd_BYTE` writer - Indicates SPI Flash command value of 2nd byte of write enable. Available only if configuration of DDR_EN is defined."]
pub type WrEnCmd2ndByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of 1st byte of write enable."]
    #[inline(always)]
    pub fn wr_en_cmd_1st_byte(&self) -> WrEnCmd1stByteR {
        WrEnCmd1stByteR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Indicates SPI Flash command value of 2nd byte of write enable. Available only if configuration of DDR_EN is defined."]
    #[inline(always)]
    pub fn wr_en_cmd_2nd_byte(&self) -> WrEnCmd2ndByteR {
        WrEnCmd2ndByteR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of 1st byte of write enable."]
    #[inline(always)]
    pub fn wr_en_cmd_1st_byte(&mut self) -> WrEnCmd1stByteW<'_, WriteEnableSpec> {
        WrEnCmd1stByteW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Indicates SPI Flash command value of 2nd byte of write enable. Available only if configuration of DDR_EN is defined."]
    #[inline(always)]
    pub fn wr_en_cmd_2nd_byte(&mut self) -> WrEnCmd2ndByteW<'_, WriteEnableSpec> {
        WrEnCmd2ndByteW::new(self, 8)
    }
}
#[doc = "Write Enable Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`write_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteEnableSpec;
impl crate::RegisterSpec for WriteEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`write_enable::R`](R) reader structure"]
impl crate::Readable for WriteEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`write_enable::W`](W) writer structure"]
impl crate::Writable for WriteEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITE_ENABLE to value 0x0606"]
impl crate::Resettable for WriteEnableSpec {
    const RESET_VALUE: u32 = 0x0606;
}
