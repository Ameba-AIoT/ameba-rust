#[doc = "Register `DEBUG_MODE_CTRL` reader"]
pub type R = crate::R<DebugModeCtrlSpec>;
#[doc = "Register `DEBUG_MODE_CTRL` writer"]
pub type W = crate::W<DebugModeCtrlSpec>;
#[doc = "Field `debug_en` reader - Debug mode enable 0: Disable 1: Enable"]
pub type DebugEnR = crate::BitReader;
#[doc = "Field `debug_en` writer - Debug mode enable 0: Disable 1: Enable"]
pub type DebugEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `continue_sample_en` reader - continue sample one channel mode enable, only take effect in debug mode 1: enable 0: disable"]
pub type ContinueSampleEnR = crate::BitReader;
#[doc = "Field `continue_sample_en` writer - continue sample one channel mode enable, only take effect in debug mode 1: enable 0: disable"]
pub type ContinueSampleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `norm_fifo_en` reader - when debug mode disable, select whether to write the ctc processed data into the fifo 1: enable 0: disable"]
pub type NormFifoEnR = crate::BitReader;
#[doc = "Field `norm_fifo_en` writer - when debug mode disable, select whether to write the ctc processed data into the fifo 1: enable 0: disable"]
pub type NormFifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_switch_ctrl` reader - Scan channel switch control 0: Auto switch function disable (manual switch by ch_ctrl) 1: Auto switch to next channel (channel 0 --> channel 3 --> channel 0)"]
pub type ChSwitchCtrlR = crate::BitReader;
#[doc = "Field `ch_switch_ctrl` writer - Scan channel switch control 0: Auto switch function disable (manual switch by ch_ctrl) 1: Auto switch to next channel (channel 0 --> channel 3 --> channel 0)"]
pub type ChSwitchCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_ctrl` reader - Scan channel control 0000: channel 0 0001: channel 1 0010: channel 2 0011: channel 3 0100: channel 4 0101: channel 5 0110: channel 6 0111: channel 7 1000: channel 8"]
pub type ChCtrlR = crate::FieldReader;
#[doc = "Field `ch_ctrl` writer - Scan channel control 0000: channel 0 0001: channel 1 0010: channel 2 0011: channel 3 0100: channel 4 0101: channel 5 0110: channel 6 0111: channel 7 1000: channel 8"]
pub type ChCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Debug mode enable 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn debug_en(&self) -> DebugEnR {
        DebugEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - continue sample one channel mode enable, only take effect in debug mode 1: enable 0: disable"]
    #[inline(always)]
    pub fn continue_sample_en(&self) -> ContinueSampleEnR {
        ContinueSampleEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - when debug mode disable, select whether to write the ctc processed data into the fifo 1: enable 0: disable"]
    #[inline(always)]
    pub fn norm_fifo_en(&self) -> NormFifoEnR {
        NormFifoEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan channel switch control 0: Auto switch function disable (manual switch by ch_ctrl) 1: Auto switch to next channel (channel 0 --> channel 3 --> channel 0)"]
    #[inline(always)]
    pub fn ch_switch_ctrl(&self) -> ChSwitchCtrlR {
        ChSwitchCtrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Scan channel control 0000: channel 0 0001: channel 1 0010: channel 2 0011: channel 3 0100: channel 4 0101: channel 5 0110: channel 6 0111: channel 7 1000: channel 8"]
    #[inline(always)]
    pub fn ch_ctrl(&self) -> ChCtrlR {
        ChCtrlR::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug mode enable 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn debug_en(&mut self) -> DebugEnW<'_, DebugModeCtrlSpec> {
        DebugEnW::new(self, 0)
    }
    #[doc = "Bit 1 - continue sample one channel mode enable, only take effect in debug mode 1: enable 0: disable"]
    #[inline(always)]
    pub fn continue_sample_en(&mut self) -> ContinueSampleEnW<'_, DebugModeCtrlSpec> {
        ContinueSampleEnW::new(self, 1)
    }
    #[doc = "Bit 2 - when debug mode disable, select whether to write the ctc processed data into the fifo 1: enable 0: disable"]
    #[inline(always)]
    pub fn norm_fifo_en(&mut self) -> NormFifoEnW<'_, DebugModeCtrlSpec> {
        NormFifoEnW::new(self, 2)
    }
    #[doc = "Bit 4 - Scan channel switch control 0: Auto switch function disable (manual switch by ch_ctrl) 1: Auto switch to next channel (channel 0 --> channel 3 --> channel 0)"]
    #[inline(always)]
    pub fn ch_switch_ctrl(&mut self) -> ChSwitchCtrlW<'_, DebugModeCtrlSpec> {
        ChSwitchCtrlW::new(self, 4)
    }
    #[doc = "Bits 5:8 - Scan channel control 0000: channel 0 0001: channel 1 0010: channel 2 0011: channel 3 0100: channel 4 0101: channel 5 0110: channel 6 0111: channel 7 1000: channel 8"]
    #[inline(always)]
    pub fn ch_ctrl(&mut self) -> ChCtrlW<'_, DebugModeCtrlSpec> {
        ChCtrlW::new(self, 5)
    }
}
#[doc = "Debug and Channel Swith Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_mode_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_mode_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugModeCtrlSpec;
impl crate::RegisterSpec for DebugModeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for DebugModeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`debug_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for DebugModeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_MODE_CTRL to value 0x10"]
impl crate::Resettable for DebugModeCtrlSpec {
    const RESET_VALUE: u32 = 0x10;
}
