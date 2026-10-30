#[doc = "Register `INTERRUPT_ENABLE` reader"]
pub type R = crate::R<InterruptEnableSpec>;
#[doc = "Register `INTERRUPT_ENABLE` writer"]
pub type W = crate::W<InterruptEnableSpec>;
#[doc = "Field `touch_press_intr_en` reader - Channelx single touch press enable, x is 0~8"]
pub type TouchPressIntrEnR = crate::FieldReader<u16>;
#[doc = "Field `touch_press_intr_en` writer - Channelx single touch press enable, x is 0~8"]
pub type TouchPressIntrEnW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `touch_release_intr_en` reader - Channelx single touch release enable, x is 0~8"]
pub type TouchReleaseIntrEnR = crate::FieldReader<u16>;
#[doc = "Field `touch_release_intr_en` writer - Channelx single touch release enable, x is 0~8"]
pub type TouchReleaseIntrEnW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `over_p_noise_th_intr_en` reader - Positive noise threshold overflow enable"]
pub type OverPNoiseThIntrEnR = crate::BitReader;
#[doc = "Field `over_p_noise_th_intr_en` writer - Positive noise threshold overflow enable"]
pub type OverPNoiseThIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `afifo_overflow_intr_en` reader - Raw code FIFO overflow enable"]
pub type AfifoOverflowIntrEnR = crate::BitReader;
#[doc = "Field `afifo_overflow_intr_en` writer - Raw code FIFO overflow enable"]
pub type AfifoOverflowIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `over_n_noise_th_intr_en` reader - Negative noise threshold overflow enable"]
pub type OverNNoiseThIntrEnR = crate::BitReader;
#[doc = "Field `over_n_noise_th_intr_en` writer - Negative noise threshold overflow enable"]
pub type OverNNoiseThIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `afifo_overlvl_intr_en` reader - Raw code FIFO over level enable"]
pub type AfifoOverlvlIntrEnR = crate::BitReader;
#[doc = "Field `afifo_overlvl_intr_en` writer - Raw code FIFO over level enable"]
pub type AfifoOverlvlIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctc_brk_by_adc_intr_en` reader - captouch breaked by adc enable"]
pub type CtcBrkByAdcIntrEnR = crate::BitReader;
#[doc = "Field `ctc_brk_by_adc_intr_en` writer - captouch breaked by adc enable"]
pub type CtcBrkByAdcIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scan_end_intr_en` reader - sample scan finish enable"]
pub type ScanEndIntrEnR = crate::BitReader;
#[doc = "Field `scan_end_intr_en` writer - sample scan finish enable"]
pub type ScanEndIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Channelx single touch press enable, x is 0~8"]
    #[inline(always)]
    pub fn touch_press_intr_en(&self) -> TouchPressIntrEnR {
        TouchPressIntrEnR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Channelx single touch release enable, x is 0~8"]
    #[inline(always)]
    pub fn touch_release_intr_en(&self) -> TouchReleaseIntrEnR {
        TouchReleaseIntrEnR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 18 - Positive noise threshold overflow enable"]
    #[inline(always)]
    pub fn over_p_noise_th_intr_en(&self) -> OverPNoiseThIntrEnR {
        OverPNoiseThIntrEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw code FIFO overflow enable"]
    #[inline(always)]
    pub fn afifo_overflow_intr_en(&self) -> AfifoOverflowIntrEnR {
        AfifoOverflowIntrEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Negative noise threshold overflow enable"]
    #[inline(always)]
    pub fn over_n_noise_th_intr_en(&self) -> OverNNoiseThIntrEnR {
        OverNNoiseThIntrEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Raw code FIFO over level enable"]
    #[inline(always)]
    pub fn afifo_overlvl_intr_en(&self) -> AfifoOverlvlIntrEnR {
        AfifoOverlvlIntrEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - captouch breaked by adc enable"]
    #[inline(always)]
    pub fn ctc_brk_by_adc_intr_en(&self) -> CtcBrkByAdcIntrEnR {
        CtcBrkByAdcIntrEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - sample scan finish enable"]
    #[inline(always)]
    pub fn scan_end_intr_en(&self) -> ScanEndIntrEnR {
        ScanEndIntrEnR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Channelx single touch press enable, x is 0~8"]
    #[inline(always)]
    pub fn touch_press_intr_en(&mut self) -> TouchPressIntrEnW<'_, InterruptEnableSpec> {
        TouchPressIntrEnW::new(self, 0)
    }
    #[doc = "Bits 9:17 - Channelx single touch release enable, x is 0~8"]
    #[inline(always)]
    pub fn touch_release_intr_en(&mut self) -> TouchReleaseIntrEnW<'_, InterruptEnableSpec> {
        TouchReleaseIntrEnW::new(self, 9)
    }
    #[doc = "Bit 18 - Positive noise threshold overflow enable"]
    #[inline(always)]
    pub fn over_p_noise_th_intr_en(&mut self) -> OverPNoiseThIntrEnW<'_, InterruptEnableSpec> {
        OverPNoiseThIntrEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Raw code FIFO overflow enable"]
    #[inline(always)]
    pub fn afifo_overflow_intr_en(&mut self) -> AfifoOverflowIntrEnW<'_, InterruptEnableSpec> {
        AfifoOverflowIntrEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Negative noise threshold overflow enable"]
    #[inline(always)]
    pub fn over_n_noise_th_intr_en(&mut self) -> OverNNoiseThIntrEnW<'_, InterruptEnableSpec> {
        OverNNoiseThIntrEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Raw code FIFO over level enable"]
    #[inline(always)]
    pub fn afifo_overlvl_intr_en(&mut self) -> AfifoOverlvlIntrEnW<'_, InterruptEnableSpec> {
        AfifoOverlvlIntrEnW::new(self, 21)
    }
    #[doc = "Bit 22 - captouch breaked by adc enable"]
    #[inline(always)]
    pub fn ctc_brk_by_adc_intr_en(&mut self) -> CtcBrkByAdcIntrEnW<'_, InterruptEnableSpec> {
        CtcBrkByAdcIntrEnW::new(self, 22)
    }
    #[doc = "Bit 23 - sample scan finish enable"]
    #[inline(always)]
    pub fn scan_end_intr_en(&mut self) -> ScanEndIntrEnW<'_, InterruptEnableSpec> {
        ScanEndIntrEnW::new(self, 23)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptEnableSpec;
impl crate::RegisterSpec for InterruptEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for InterruptEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for InterruptEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT_ENABLE to value 0"]
impl crate::Resettable for InterruptEnableSpec {}
