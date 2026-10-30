#[doc = "Register `LCDC_MCU_TIMING_CFG` reader"]
pub type R = crate::R<LcdcMcuTimingCfgSpec>;
#[doc = "Register `LCDC_MCU_TIMING_CFG` writer"]
pub type W = crate::W<LcdcMcuTimingCfgSpec>;
#[doc = "Field `wrpulw` reader - Write clock divider depends on this value. The relationship between them is as follows: * 0: The write clock divider is 2 * 1: The write clock divider is 4 * 2: The write clock divider is 6 * 3: The write clock divider is 8 *... The frequency of write clock is derived from the following equation: Write clock = SYS_CLK/((WRPULW+1) * 2) where WRPULW is any even value between 0 and 1022."]
pub type WrpulwR = crate::FieldReader<u16>;
#[doc = "Field `wrpulw` writer - Write clock divider depends on this value. The relationship between them is as follows: * 0: The write clock divider is 2 * 1: The write clock divider is 4 * 2: The write clock divider is 6 * 3: The write clock divider is 8 *... The frequency of write clock is derived from the following equation: Write clock = SYS_CLK/((WRPULW+1) * 2) where WRPULW is any even value between 0 and 1022."]
pub type WrpulwW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `rdactw` reader - RD active pulse width -1. Unit: system clock"]
pub type RdactwR = crate::FieldReader<u16>;
#[doc = "Field `rdactw` writer - RD active pulse width -1. Unit: system clock"]
pub type RdactwW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `rdinactw` reader - RD inactive pulse width -1. Unit: system clock"]
pub type RdinactwR = crate::FieldReader<u16>;
#[doc = "Field `rdinactw` writer - RD inactive pulse width -1. Unit: system clock"]
pub type RdinactwW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:9 - Write clock divider depends on this value. The relationship between them is as follows: * 0: The write clock divider is 2 * 1: The write clock divider is 4 * 2: The write clock divider is 6 * 3: The write clock divider is 8 *... The frequency of write clock is derived from the following equation: Write clock = SYS_CLK/((WRPULW+1) * 2) where WRPULW is any even value between 0 and 1022."]
    #[inline(always)]
    pub fn wrpulw(&self) -> WrpulwR {
        WrpulwR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:20 - RD active pulse width -1. Unit: system clock"]
    #[inline(always)]
    pub fn rdactw(&self) -> RdactwR {
        RdactwR::new(((self.bits >> 10) & 0x07ff) as u16)
    }
    #[doc = "Bits 21:31 - RD inactive pulse width -1. Unit: system clock"]
    #[inline(always)]
    pub fn rdinactw(&self) -> RdinactwR {
        RdinactwR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Write clock divider depends on this value. The relationship between them is as follows: * 0: The write clock divider is 2 * 1: The write clock divider is 4 * 2: The write clock divider is 6 * 3: The write clock divider is 8 *... The frequency of write clock is derived from the following equation: Write clock = SYS_CLK/((WRPULW+1) * 2) where WRPULW is any even value between 0 and 1022."]
    #[inline(always)]
    pub fn wrpulw(&mut self) -> WrpulwW<'_, LcdcMcuTimingCfgSpec> {
        WrpulwW::new(self, 0)
    }
    #[doc = "Bits 10:20 - RD active pulse width -1. Unit: system clock"]
    #[inline(always)]
    pub fn rdactw(&mut self) -> RdactwW<'_, LcdcMcuTimingCfgSpec> {
        RdactwW::new(self, 10)
    }
    #[doc = "Bits 21:31 - RD inactive pulse width -1. Unit: system clock"]
    #[inline(always)]
    pub fn rdinactw(&mut self) -> RdinactwW<'_, LcdcMcuTimingCfgSpec> {
        RdinactwW::new(self, 21)
    }
}
#[doc = "MCU timing config\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_mcu_timing_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_mcu_timing_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcMcuTimingCfgSpec;
impl crate::RegisterSpec for LcdcMcuTimingCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_mcu_timing_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcMcuTimingCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_mcu_timing_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcMcuTimingCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_MCU_TIMING_CFG to value 0x01"]
impl crate::Resettable for LcdcMcuTimingCfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
