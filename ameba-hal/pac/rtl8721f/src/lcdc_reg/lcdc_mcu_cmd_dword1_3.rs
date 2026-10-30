#[doc = "Register `LCDC_MCU_CMD_DWORD1_3` reader"]
pub type R = crate::R<LcdcMcuCmdDword1_3Spec>;
#[doc = "Register `LCDC_MCU_CMD_DWORD1_3` writer"]
pub type W = crate::W<LcdcMcuCmdDword1_3Spec>;
#[doc = "Field `mcu_cmd_dword1_3` reader - mcu predefined cmd before data"]
pub type McuCmdDword1_3R = crate::FieldReader;
#[doc = "Field `mcu_cmd_dword1_3` writer - mcu predefined cmd before data"]
pub type McuCmdDword1_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - mcu predefined cmd before data"]
    #[inline(always)]
    pub fn mcu_cmd_dword1_3(&self) -> McuCmdDword1_3R {
        McuCmdDword1_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - mcu predefined cmd before data"]
    #[inline(always)]
    pub fn mcu_cmd_dword1_3(&mut self) -> McuCmdDword1_3W<'_, LcdcMcuCmdDword1_3Spec> {
        McuCmdDword1_3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_mcu_cmd_dword1_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_mcu_cmd_dword1_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcMcuCmdDword1_3Spec;
impl crate::RegisterSpec for LcdcMcuCmdDword1_3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdc_mcu_cmd_dword1_3::R`](R) reader structure"]
impl crate::Readable for LcdcMcuCmdDword1_3Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_mcu_cmd_dword1_3::W`](W) writer structure"]
impl crate::Writable for LcdcMcuCmdDword1_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_MCU_CMD_DWORD1_3 to value 0"]
impl crate::Resettable for LcdcMcuCmdDword1_3Spec {}
