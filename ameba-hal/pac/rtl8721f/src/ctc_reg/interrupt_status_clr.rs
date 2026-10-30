#[doc = "Register `INTERRUPT_STATUS_CLR` reader"]
pub type R = crate::R<InterruptStatusClrSpec>;
#[doc = "Register `INTERRUPT_STATUS_CLR` writer"]
pub type W = crate::W<InterruptStatusClrSpec>;
#[doc = "Field `touch_press_clr` reader - Clear channelx single touch press interrupt and related register"]
pub type TouchPressClrR = crate::FieldReader<u16>;
#[doc = "Field `touch_press_clr` writer - Clear channelx single touch press interrupt and related register"]
pub type TouchPressClrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `touch_release_clr` reader - Clear channelx single touch release interrupt and related register"]
pub type TouchReleaseClrR = crate::FieldReader<u16>;
#[doc = "Field `touch_release_clr` writer - Clear channelx single touch release interrupt and related register"]
pub type TouchReleaseClrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `over_p_noise_th_clr` reader - Clear positive noise threshold overflow interrupt and related register"]
pub type OverPNoiseThClrR = crate::BitReader;
#[doc = "Field `over_p_noise_th_clr` writer - Clear positive noise threshold overflow interrupt and related register"]
pub type OverPNoiseThClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `afifo_overflow_clr` reader - Clear raw code FIFO overflow interrupt and related register"]
pub type AfifoOverflowClrR = crate::BitReader;
#[doc = "Field `afifo_overflow_clr` writer - Clear raw code FIFO overflow interrupt and related register"]
pub type AfifoOverflowClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `over_n_noise_th_clr` reader - Clear negative noise threshold overflow interrupt and related register"]
pub type OverNNoiseThClrR = crate::BitReader;
#[doc = "Field `over_n_noise_th_clr` writer - Clear negative noise threshold overflow interrupt and related register"]
pub type OverNNoiseThClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `afifo_overlvl_clr` reader - Clear raw code FIFO over level raw interrupt and related register"]
pub type AfifoOverlvlClrR = crate::BitReader;
#[doc = "Field `afifo_overlvl_clr` writer - Clear raw code FIFO over level raw interrupt and related register"]
pub type AfifoOverlvlClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctc_brk_by_adc_clr` reader - Clear captouch breaked by adc raw interrupt and related register"]
pub type CtcBrkByAdcClrR = crate::BitReader;
#[doc = "Field `ctc_brk_by_adc_clr` writer - Clear captouch breaked by adc raw interrupt and related register"]
pub type CtcBrkByAdcClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scan_end_clr` reader - sample scan finish raw interrupt and related register"]
pub type ScanEndClrR = crate::BitReader;
#[doc = "Field `scan_end_clr` writer - sample scan finish raw interrupt and related register"]
pub type ScanEndClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Clear channelx single touch press interrupt and related register"]
    #[inline(always)]
    pub fn touch_press_clr(&self) -> TouchPressClrR {
        TouchPressClrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Clear channelx single touch release interrupt and related register"]
    #[inline(always)]
    pub fn touch_release_clr(&self) -> TouchReleaseClrR {
        TouchReleaseClrR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 18 - Clear positive noise threshold overflow interrupt and related register"]
    #[inline(always)]
    pub fn over_p_noise_th_clr(&self) -> OverPNoiseThClrR {
        OverPNoiseThClrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear raw code FIFO overflow interrupt and related register"]
    #[inline(always)]
    pub fn afifo_overflow_clr(&self) -> AfifoOverflowClrR {
        AfifoOverflowClrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Clear negative noise threshold overflow interrupt and related register"]
    #[inline(always)]
    pub fn over_n_noise_th_clr(&self) -> OverNNoiseThClrR {
        OverNNoiseThClrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Clear raw code FIFO over level raw interrupt and related register"]
    #[inline(always)]
    pub fn afifo_overlvl_clr(&self) -> AfifoOverlvlClrR {
        AfifoOverlvlClrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Clear captouch breaked by adc raw interrupt and related register"]
    #[inline(always)]
    pub fn ctc_brk_by_adc_clr(&self) -> CtcBrkByAdcClrR {
        CtcBrkByAdcClrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - sample scan finish raw interrupt and related register"]
    #[inline(always)]
    pub fn scan_end_clr(&self) -> ScanEndClrR {
        ScanEndClrR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clear channelx single touch press interrupt and related register"]
    #[inline(always)]
    pub fn touch_press_clr(&mut self) -> TouchPressClrW<'_, InterruptStatusClrSpec> {
        TouchPressClrW::new(self, 0)
    }
    #[doc = "Bits 9:17 - Clear channelx single touch release interrupt and related register"]
    #[inline(always)]
    pub fn touch_release_clr(&mut self) -> TouchReleaseClrW<'_, InterruptStatusClrSpec> {
        TouchReleaseClrW::new(self, 9)
    }
    #[doc = "Bit 18 - Clear positive noise threshold overflow interrupt and related register"]
    #[inline(always)]
    pub fn over_p_noise_th_clr(&mut self) -> OverPNoiseThClrW<'_, InterruptStatusClrSpec> {
        OverPNoiseThClrW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear raw code FIFO overflow interrupt and related register"]
    #[inline(always)]
    pub fn afifo_overflow_clr(&mut self) -> AfifoOverflowClrW<'_, InterruptStatusClrSpec> {
        AfifoOverflowClrW::new(self, 19)
    }
    #[doc = "Bit 20 - Clear negative noise threshold overflow interrupt and related register"]
    #[inline(always)]
    pub fn over_n_noise_th_clr(&mut self) -> OverNNoiseThClrW<'_, InterruptStatusClrSpec> {
        OverNNoiseThClrW::new(self, 20)
    }
    #[doc = "Bit 21 - Clear raw code FIFO over level raw interrupt and related register"]
    #[inline(always)]
    pub fn afifo_overlvl_clr(&mut self) -> AfifoOverlvlClrW<'_, InterruptStatusClrSpec> {
        AfifoOverlvlClrW::new(self, 21)
    }
    #[doc = "Bit 22 - Clear captouch breaked by adc raw interrupt and related register"]
    #[inline(always)]
    pub fn ctc_brk_by_adc_clr(&mut self) -> CtcBrkByAdcClrW<'_, InterruptStatusClrSpec> {
        CtcBrkByAdcClrW::new(self, 22)
    }
    #[doc = "Bit 23 - sample scan finish raw interrupt and related register"]
    #[inline(always)]
    pub fn scan_end_clr(&mut self) -> ScanEndClrW<'_, InterruptStatusClrSpec> {
        ScanEndClrW::new(self, 23)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_status_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptStatusClrSpec;
impl crate::RegisterSpec for InterruptStatusClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_status_clr::R`](R) reader structure"]
impl crate::Readable for InterruptStatusClrSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_status_clr::W`](W) writer structure"]
impl crate::Writable for InterruptStatusClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT_STATUS_CLR to value 0"]
impl crate::Resettable for InterruptStatusClrSpec {}
