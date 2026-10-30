#[doc = "Register `LCDC_RGB_CLK_DIV` reader"]
pub type R = crate::R<LcdcRgbClkDivSpec>;
#[doc = "Register `LCDC_RGB_CLK_DIV` writer"]
pub type W = crate::W<LcdcRgbClkDivSpec>;
#[doc = "Field `clkdiv` reader - DCLK clock divider depends on this value. The relationship between them is as follows: *0 : The DCLK clock divider is 2 *1 : The DCLK clock divider is 4 *2 : The DCLK clock divider is 6 *3 : The DCLK clock divider is 8 *... The frequency of DCLK is derived from the following equation: DCLK = SYS_CLK/((CLKDIV+1)*2) where CLKDIV is any even value between 0 and 65534."]
pub type ClkdivR = crate::FieldReader<u16>;
#[doc = "Field `clkdiv` writer - DCLK clock divider depends on this value. The relationship between them is as follows: *0 : The DCLK clock divider is 2 *1 : The DCLK clock divider is 4 *2 : The DCLK clock divider is 6 *3 : The DCLK clock divider is 8 *... The frequency of DCLK is derived from the following equation: DCLK = SYS_CLK/((CLKDIV+1)*2) where CLKDIV is any even value between 0 and 65534."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DCLK clock divider depends on this value. The relationship between them is as follows: *0 : The DCLK clock divider is 2 *1 : The DCLK clock divider is 4 *2 : The DCLK clock divider is 6 *3 : The DCLK clock divider is 8 *... The frequency of DCLK is derived from the following equation: DCLK = SYS_CLK/((CLKDIV+1)*2) where CLKDIV is any even value between 0 and 65534."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DCLK clock divider depends on this value. The relationship between them is as follows: *0 : The DCLK clock divider is 2 *1 : The DCLK clock divider is 4 *2 : The DCLK clock divider is 6 *3 : The DCLK clock divider is 8 *... The frequency of DCLK is derived from the following equation: DCLK = SYS_CLK/((CLKDIV+1)*2) where CLKDIV is any even value between 0 and 65534."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, LcdcRgbClkDivSpec> {
        ClkdivW::new(self, 0)
    }
}
#[doc = "RGB clock divide\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_rgb_clk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_rgb_clk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcRgbClkDivSpec;
impl crate::RegisterSpec for LcdcRgbClkDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_rgb_clk_div::R`](R) reader structure"]
impl crate::Readable for LcdcRgbClkDivSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_rgb_clk_div::W`](W) writer structure"]
impl crate::Writable for LcdcRgbClkDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_RGB_CLK_DIV to value 0x02"]
impl crate::Resettable for LcdcRgbClkDivSpec {
    const RESET_VALUE: u32 = 0x02;
}
