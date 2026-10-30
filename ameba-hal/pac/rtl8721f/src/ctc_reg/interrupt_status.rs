#[doc = "Register `INTERRUPT_STATUS` reader"]
pub type R = crate::R<InterruptStatusSpec>;
#[doc = "Field `touch_press_intr` reader - Channelx single touch press interrupt, x is 0~8"]
pub type TouchPressIntrR = crate::FieldReader<u16>;
#[doc = "Field `touch_release_intr` reader - Channelx single touch release interrupt, x is 0~8"]
pub type TouchReleaseIntrR = crate::FieldReader<u16>;
#[doc = "Field `over_p_noise_th_intr` reader - Positive noise threshold overflow interrupt"]
pub type OverPNoiseThIntrR = crate::BitReader;
#[doc = "Field `afifo_overflow_intr` reader - Raw code FIFO overflow interrupt"]
pub type AfifoOverflowIntrR = crate::BitReader;
#[doc = "Field `over_n_noise_th_intr` reader - Negative noise threshold overflow interrupt"]
pub type OverNNoiseThIntrR = crate::BitReader;
#[doc = "Field `afifo_overlvl_intr` reader - Raw code FIFO over level interrupt"]
pub type AfifoOverlvlIntrR = crate::BitReader;
#[doc = "Field `ctc_brk_by_adc_intr` reader - captouch breaked by adc interrupt"]
pub type CtcBrkByAdcIntrR = crate::BitReader;
#[doc = "Field `scan_end_intr` reader - sample scan finish interrupt"]
pub type ScanEndIntrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Channelx single touch press interrupt, x is 0~8"]
    #[inline(always)]
    pub fn touch_press_intr(&self) -> TouchPressIntrR {
        TouchPressIntrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Channelx single touch release interrupt, x is 0~8"]
    #[inline(always)]
    pub fn touch_release_intr(&self) -> TouchReleaseIntrR {
        TouchReleaseIntrR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 18 - Positive noise threshold overflow interrupt"]
    #[inline(always)]
    pub fn over_p_noise_th_intr(&self) -> OverPNoiseThIntrR {
        OverPNoiseThIntrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw code FIFO overflow interrupt"]
    #[inline(always)]
    pub fn afifo_overflow_intr(&self) -> AfifoOverflowIntrR {
        AfifoOverflowIntrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Negative noise threshold overflow interrupt"]
    #[inline(always)]
    pub fn over_n_noise_th_intr(&self) -> OverNNoiseThIntrR {
        OverNNoiseThIntrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Raw code FIFO over level interrupt"]
    #[inline(always)]
    pub fn afifo_overlvl_intr(&self) -> AfifoOverlvlIntrR {
        AfifoOverlvlIntrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - captouch breaked by adc interrupt"]
    #[inline(always)]
    pub fn ctc_brk_by_adc_intr(&self) -> CtcBrkByAdcIntrR {
        CtcBrkByAdcIntrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - sample scan finish interrupt"]
    #[inline(always)]
    pub fn scan_end_intr(&self) -> ScanEndIntrR {
        ScanEndIntrR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptStatusSpec;
impl crate::RegisterSpec for InterruptStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_status::R`](R) reader structure"]
impl crate::Readable for InterruptStatusSpec {}
#[doc = "`reset()` method sets INTERRUPT_STATUS to value 0"]
impl crate::Resettable for InterruptStatusSpec {}
