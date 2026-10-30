#[doc = "Register `PINMUX_SUB_CTRL` reader"]
pub type R = crate::R<PinmuxSubCtrlSpec>;
#[doc = "Register `PINMUX_SUB_CTRL` writer"]
pub type W = crate::W<PinmuxSubCtrlSpec>;
#[doc = "Field `dbg_clk0_sel` reader - debug clock1 in CLOCK testmode 0: xtal 40M 1: xtal 20M 2: xtal 2M 3: xtal lps 312.5K 4: sdm_32.768k 5: sdm_1Hz 6.usb pll div 7.sys pll div 8: osc131k 9: osc4M 10: osc100K 11: RMII gmac 50M 12: RMII gmac 25M"]
pub type DbgClk0SelR = crate::FieldReader;
#[doc = "Field `dbg_clk0_sel` writer - debug clock1 in CLOCK testmode 0: xtal 40M 1: xtal 20M 2: xtal 2M 3: xtal lps 312.5K 4: sdm_32.768k 5: sdm_1Hz 6.usb pll div 7.sys pll div 8: osc131k 9: osc4M 10: osc100K 11: RMII gmac 50M 12: RMII gmac 25M"]
pub type DbgClk0SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dbg_clk1_sel` reader - debug clock1 in CLOCK testmode 0: xtal 40M 1: xtal 20M 2: xtal 2M 3: xtal lps 312.5K 4: sdm_32.768k 5: sdm_1Hz 6.usb pll div 7.sys pll div 8: osc131k 9: osc4M 10: osc100K 11: RMII gmac 50M 12: RMII gmac 25M"]
pub type DbgClk1SelR = crate::FieldReader;
#[doc = "Field `dbg_clk1_sel` writer - debug clock1 in CLOCK testmode 0: xtal 40M 1: xtal 20M 2: xtal 2M 3: xtal lps 312.5K 4: sdm_32.768k 5: sdm_1Hz 6.usb pll div 7.sys pll div 8: osc131k 9: osc4M 10: osc100K 11: RMII gmac 50M 12: RMII gmac 25M"]
pub type DbgClk1SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dbg_clk_force` reader - Force debug clock divider enable if testmode in clock debug ICFG. This maybe used when EXT_CLK_OUT was used."]
pub type DbgClkForceR = crate::BitReader;
#[doc = "Field `dbg_clk_force` writer - Force debug clock divider enable if testmode in clock debug ICFG. This maybe used when EXT_CLK_OUT was used."]
pub type DbgClkForceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - debug clock1 in CLOCK testmode 0: xtal 40M 1: xtal 20M 2: xtal 2M 3: xtal lps 312.5K 4: sdm_32.768k 5: sdm_1Hz 6.usb pll div 7.sys pll div 8: osc131k 9: osc4M 10: osc100K 11: RMII gmac 50M 12: RMII gmac 25M"]
    #[inline(always)]
    pub fn dbg_clk0_sel(&self) -> DbgClk0SelR {
        DbgClk0SelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - debug clock1 in CLOCK testmode 0: xtal 40M 1: xtal 20M 2: xtal 2M 3: xtal lps 312.5K 4: sdm_32.768k 5: sdm_1Hz 6.usb pll div 7.sys pll div 8: osc131k 9: osc4M 10: osc100K 11: RMII gmac 50M 12: RMII gmac 25M"]
    #[inline(always)]
    pub fn dbg_clk1_sel(&self) -> DbgClk1SelR {
        DbgClk1SelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Force debug clock divider enable if testmode in clock debug ICFG. This maybe used when EXT_CLK_OUT was used."]
    #[inline(always)]
    pub fn dbg_clk_force(&self) -> DbgClkForceR {
        DbgClkForceR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - debug clock1 in CLOCK testmode 0: xtal 40M 1: xtal 20M 2: xtal 2M 3: xtal lps 312.5K 4: sdm_32.768k 5: sdm_1Hz 6.usb pll div 7.sys pll div 8: osc131k 9: osc4M 10: osc100K 11: RMII gmac 50M 12: RMII gmac 25M"]
    #[inline(always)]
    pub fn dbg_clk0_sel(&mut self) -> DbgClk0SelW<'_, PinmuxSubCtrlSpec> {
        DbgClk0SelW::new(self, 0)
    }
    #[doc = "Bits 4:7 - debug clock1 in CLOCK testmode 0: xtal 40M 1: xtal 20M 2: xtal 2M 3: xtal lps 312.5K 4: sdm_32.768k 5: sdm_1Hz 6.usb pll div 7.sys pll div 8: osc131k 9: osc4M 10: osc100K 11: RMII gmac 50M 12: RMII gmac 25M"]
    #[inline(always)]
    pub fn dbg_clk1_sel(&mut self) -> DbgClk1SelW<'_, PinmuxSubCtrlSpec> {
        DbgClk1SelW::new(self, 4)
    }
    #[doc = "Bit 8 - Force debug clock divider enable if testmode in clock debug ICFG. This maybe used when EXT_CLK_OUT was used."]
    #[inline(always)]
    pub fn dbg_clk_force(&mut self) -> DbgClkForceW<'_, PinmuxSubCtrlSpec> {
        DbgClkForceW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pinmux_sub_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pinmux_sub_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PinmuxSubCtrlSpec;
impl crate::RegisterSpec for PinmuxSubCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmux_sub_ctrl::R`](R) reader structure"]
impl crate::Readable for PinmuxSubCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pinmux_sub_ctrl::W`](W) writer structure"]
impl crate::Writable for PinmuxSubCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PINMUX_SUB_CTRL to value 0"]
impl crate::Resettable for PinmuxSubCtrlSpec {}
