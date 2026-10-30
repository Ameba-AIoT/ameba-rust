#[doc = "Register `LCDC_MCU_IO_TO_CFG` reader"]
pub type R = crate::R<LcdcMcuIoToCfgSpec>;
#[doc = "Register `LCDC_MCU_IO_TO_CFG` writer"]
pub type W = crate::W<LcdcMcuIoToCfgSpec>;
#[doc = "Field `io_timeout_num` reader - The I/O write/read timeout interrupthappens, When writing or reading is timeout for IO_RDY_TO_DOTCLK_NUM system clocks in MCU I/F I/O mode, t. When MCU I/F I/O mode is used, the recommand value is: 16 * WRPULW + RDACTW + RDINACTW"]
pub type IoTimeoutNumR = crate::FieldReader<u16>;
#[doc = "Field `io_timeout_num` writer - The I/O write/read timeout interrupthappens, When writing or reading is timeout for IO_RDY_TO_DOTCLK_NUM system clocks in MCU I/F I/O mode, t. When MCU I/F I/O mode is used, the recommand value is: 16 * WRPULW + RDACTW + RDINACTW"]
pub type IoTimeoutNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The I/O write/read timeout interrupthappens, When writing or reading is timeout for IO_RDY_TO_DOTCLK_NUM system clocks in MCU I/F I/O mode, t. When MCU I/F I/O mode is used, the recommand value is: 16 * WRPULW + RDACTW + RDINACTW"]
    #[inline(always)]
    pub fn io_timeout_num(&self) -> IoTimeoutNumR {
        IoTimeoutNumR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The I/O write/read timeout interrupthappens, When writing or reading is timeout for IO_RDY_TO_DOTCLK_NUM system clocks in MCU I/F I/O mode, t. When MCU I/F I/O mode is used, the recommand value is: 16 * WRPULW + RDACTW + RDINACTW"]
    #[inline(always)]
    pub fn io_timeout_num(&mut self) -> IoTimeoutNumW<'_, LcdcMcuIoToCfgSpec> {
        IoTimeoutNumW::new(self, 0)
    }
}
#[doc = "MCU IO timeout config\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_mcu_io_to_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_mcu_io_to_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcMcuIoToCfgSpec;
impl crate::RegisterSpec for LcdcMcuIoToCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_mcu_io_to_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcMcuIoToCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_mcu_io_to_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcMcuIoToCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_MCU_IO_TO_CFG to value 0x42"]
impl crate::Resettable for LcdcMcuIoToCfgSpec {
    const RESET_VALUE: u32 = 0x42;
}
