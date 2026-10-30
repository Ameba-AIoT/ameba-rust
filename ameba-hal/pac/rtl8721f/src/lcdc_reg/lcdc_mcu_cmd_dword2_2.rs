#[doc = "Register `LCDC_MCU_CMD_DWORD2_2` reader"]
pub type R = crate::R<LcdcMcuCmdDword2_2Spec>;
#[doc = "Register `LCDC_MCU_CMD_DWORD2_2` writer"]
pub type W = crate::W<LcdcMcuCmdDword2_2Spec>;
#[doc = "Field `mcu_cmd_dword2_2` reader - mcu predefined cmd before data"]
pub type McuCmdDword2_2R = crate::FieldReader;
#[doc = "Field `mcu_cmd_dword2_2` writer - mcu predefined cmd before data"]
pub type McuCmdDword2_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - mcu predefined cmd before data"]
    #[inline(always)]
    pub fn mcu_cmd_dword2_2(&self) -> McuCmdDword2_2R {
        McuCmdDword2_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - mcu predefined cmd before data"]
    #[inline(always)]
    pub fn mcu_cmd_dword2_2(&mut self) -> McuCmdDword2_2W<'_, LcdcMcuCmdDword2_2Spec> {
        McuCmdDword2_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_mcu_cmd_dword2_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_mcu_cmd_dword2_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcMcuCmdDword2_2Spec;
impl crate::RegisterSpec for LcdcMcuCmdDword2_2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdc_mcu_cmd_dword2_2::R`](R) reader structure"]
impl crate::Readable for LcdcMcuCmdDword2_2Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_mcu_cmd_dword2_2::W`](W) writer structure"]
impl crate::Writable for LcdcMcuCmdDword2_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_MCU_CMD_DWORD2_2 to value 0"]
impl crate::Resettable for LcdcMcuCmdDword2_2Spec {}
