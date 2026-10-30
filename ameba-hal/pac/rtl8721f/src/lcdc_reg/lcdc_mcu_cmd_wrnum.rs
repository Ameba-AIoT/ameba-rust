#[doc = "Register `LCDC_MCU_CMD_WRNUM` reader"]
pub type R = crate::R<LcdcMcuCmdWrnumSpec>;
#[doc = "Register `LCDC_MCU_CMD_WRNUM` writer"]
pub type W = crate::W<LcdcMcuCmdWrnumSpec>;
#[doc = "Field `mcu_cmd_wrnum` reader - number of mcu predefined cmd, unit: byte"]
pub type McuCmdWrnumR = crate::FieldReader;
#[doc = "Field `mcu_cmd_wrnum` writer - number of mcu predefined cmd, unit: byte"]
pub type McuCmdWrnumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - number of mcu predefined cmd, unit: byte"]
    #[inline(always)]
    pub fn mcu_cmd_wrnum(&self) -> McuCmdWrnumR {
        McuCmdWrnumR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - number of mcu predefined cmd, unit: byte"]
    #[inline(always)]
    pub fn mcu_cmd_wrnum(&mut self) -> McuCmdWrnumW<'_, LcdcMcuCmdWrnumSpec> {
        McuCmdWrnumW::new(self, 0)
    }
}
#[doc = "number of mcu predefined cmd\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_mcu_cmd_wrnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_mcu_cmd_wrnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcMcuCmdWrnumSpec;
impl crate::RegisterSpec for LcdcMcuCmdWrnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_mcu_cmd_wrnum::R`](R) reader structure"]
impl crate::Readable for LcdcMcuCmdWrnumSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_mcu_cmd_wrnum::W`](W) writer structure"]
impl crate::Writable for LcdcMcuCmdWrnumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_MCU_CMD_WRNUM to value 0"]
impl crate::Resettable for LcdcMcuCmdWrnumSpec {}
