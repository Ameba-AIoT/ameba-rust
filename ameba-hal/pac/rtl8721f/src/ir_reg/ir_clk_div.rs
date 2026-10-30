#[doc = "Register `IR_CLK_DIV` reader"]
pub type R = crate::R<IrClkDivSpec>;
#[doc = "Register `IR_CLK_DIV` writer"]
pub type W = crate::W<IrClkDivSpec>;
#[doc = "Field `IR_CLK_DIV` reader - IR_CLK = IO_CLK/(1 + IR_CLK_DIV) *Tx mode: divider number to generate IrDA modulation frequency. For example: sys_clk = 100MHz, modulation_freq = 455kHz, IR_DIV_NUM = (sys_clk/modulation_freq) - 1 *Rx mode: waveform sample clock. IR_DIV_NUM = (sys_clk/sample clock) - 1 For example: sample clock = 100MHz, IR_DIV_NUM = 0; sample clock = 50MHz, IR_DIV_NUM = 1"]
pub type IrClkDivR = crate::FieldReader<u16>;
#[doc = "Field `IR_CLK_DIV` writer - IR_CLK = IO_CLK/(1 + IR_CLK_DIV) *Tx mode: divider number to generate IrDA modulation frequency. For example: sys_clk = 100MHz, modulation_freq = 455kHz, IR_DIV_NUM = (sys_clk/modulation_freq) - 1 *Rx mode: waveform sample clock. IR_DIV_NUM = (sys_clk/sample clock) - 1 For example: sample clock = 100MHz, IR_DIV_NUM = 0; sample clock = 50MHz, IR_DIV_NUM = 1"]
pub type IrClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - IR_CLK = IO_CLK/(1 + IR_CLK_DIV) *Tx mode: divider number to generate IrDA modulation frequency. For example: sys_clk = 100MHz, modulation_freq = 455kHz, IR_DIV_NUM = (sys_clk/modulation_freq) - 1 *Rx mode: waveform sample clock. IR_DIV_NUM = (sys_clk/sample clock) - 1 For example: sample clock = 100MHz, IR_DIV_NUM = 0; sample clock = 50MHz, IR_DIV_NUM = 1"]
    #[inline(always)]
    pub fn ir_clk_div(&self) -> IrClkDivR {
        IrClkDivR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - IR_CLK = IO_CLK/(1 + IR_CLK_DIV) *Tx mode: divider number to generate IrDA modulation frequency. For example: sys_clk = 100MHz, modulation_freq = 455kHz, IR_DIV_NUM = (sys_clk/modulation_freq) - 1 *Rx mode: waveform sample clock. IR_DIV_NUM = (sys_clk/sample clock) - 1 For example: sample clock = 100MHz, IR_DIV_NUM = 0; sample clock = 50MHz, IR_DIV_NUM = 1"]
    #[inline(always)]
    pub fn ir_clk_div(&mut self) -> IrClkDivW<'_, IrClkDivSpec> {
        IrClkDivW::new(self, 0)
    }
}
#[doc = "IR clock division register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_clk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_clk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrClkDivSpec;
impl crate::RegisterSpec for IrClkDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_clk_div::R`](R) reader structure"]
impl crate::Readable for IrClkDivSpec {}
#[doc = "`write(|w| ..)` method takes [`ir_clk_div::W`](W) writer structure"]
impl crate::Writable for IrClkDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR_CLK_DIV to value 0"]
impl crate::Resettable for IrClkDivSpec {}
