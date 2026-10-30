#[doc = "Register `RAW_INTERRUPT_STATUS` reader"]
pub type R = crate::R<RawInterruptStatusSpec>;
#[doc = "Field `touch_press` reader - Channelx single touch press raw interrupt, x is 0~8"]
pub type TouchPressR = crate::FieldReader<u16>;
#[doc = "Field `touch_release` reader - Channelx single touch release raw interrupt, x is 0~8"]
pub type TouchReleaseR = crate::FieldReader<u16>;
#[doc = "Field `over_p_noise_th` reader - Positive noise threshold overflow raw interrupt"]
pub type OverPNoiseThR = crate::BitReader;
#[doc = "Field `afifo_overflow` reader - Raw code FIFO overflow raw interrupt"]
pub type AfifoOverflowR = crate::BitReader;
#[doc = "Field `over_n_noise_th` reader - Negative noise threshold overflow raw interrupt"]
pub type OverNNoiseThR = crate::BitReader;
#[doc = "Field `afifo_overlvl` reader - Raw code FIFO over level raw interrupt"]
pub type AfifoOverlvlR = crate::BitReader;
#[doc = "Field `ctc_brk_by_adc` reader - captouch breaked by adc raw interrupt"]
pub type CtcBrkByAdcR = crate::BitReader;
#[doc = "Field `scan_end` reader - sample scan finish raw interrupt"]
pub type ScanEndR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Channelx single touch press raw interrupt, x is 0~8"]
    #[inline(always)]
    pub fn touch_press(&self) -> TouchPressR {
        TouchPressR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Channelx single touch release raw interrupt, x is 0~8"]
    #[inline(always)]
    pub fn touch_release(&self) -> TouchReleaseR {
        TouchReleaseR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 18 - Positive noise threshold overflow raw interrupt"]
    #[inline(always)]
    pub fn over_p_noise_th(&self) -> OverPNoiseThR {
        OverPNoiseThR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw code FIFO overflow raw interrupt"]
    #[inline(always)]
    pub fn afifo_overflow(&self) -> AfifoOverflowR {
        AfifoOverflowR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Negative noise threshold overflow raw interrupt"]
    #[inline(always)]
    pub fn over_n_noise_th(&self) -> OverNNoiseThR {
        OverNNoiseThR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Raw code FIFO over level raw interrupt"]
    #[inline(always)]
    pub fn afifo_overlvl(&self) -> AfifoOverlvlR {
        AfifoOverlvlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - captouch breaked by adc raw interrupt"]
    #[inline(always)]
    pub fn ctc_brk_by_adc(&self) -> CtcBrkByAdcR {
        CtcBrkByAdcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - sample scan finish raw interrupt"]
    #[inline(always)]
    pub fn scan_end(&self) -> ScanEndR {
        ScanEndR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_interrupt_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawInterruptStatusSpec;
impl crate::RegisterSpec for RawInterruptStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_interrupt_status::R`](R) reader structure"]
impl crate::Readable for RawInterruptStatusSpec {}
#[doc = "`reset()` method sets RAW_INTERRUPT_STATUS to value 0"]
impl crate::Resettable for RawInterruptStatusSpec {}
