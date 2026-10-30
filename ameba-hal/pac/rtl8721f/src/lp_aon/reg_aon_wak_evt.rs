#[doc = "Register `REG_AON_WAK_EVT` reader"]
pub type R = crate::R<RegAonWakEvtSpec>;
#[doc = "Field `chipen_sp_isr_evt` reader - Chipen short press interrupt event occurs."]
pub type ChipenSpIsrEvtR = crate::BitReader;
#[doc = "Field `chipen_lp_isr_evt` reader - Chipen long press interrupt event occurs."]
pub type ChipenLpIsrEvtR = crate::BitReader;
#[doc = "Field `bor_isr_evt` reader - bor interrupt event occurs."]
pub type BorIsrEvtR = crate::BitReader;
#[doc = "Field `aon_tim_isr_evt` reader - AON timer interrupt event occurs."]
pub type AonTimIsrEvtR = crate::BitReader;
#[doc = "Field `gpio_pin0_wakdet_evt` reader - Wake event for wake pin0 1: wake event detected 0: no wake event Note: aon gpio only used to wake hw pmc, to interrupt SW, it must enable GPIO interrupt in LP GPIO IP, and the poly must be same as AON GPIO setting."]
pub type GpioPin0WakdetEvtR = crate::BitReader;
#[doc = "Field `gpio_pin1_wakdet_evt` reader - Wake event for wake pin1 1: wake event detected 0: no wake event Note: aon gpio only used to wake hw pmc, to interrupt SW, it must enable GPIO interrupt in LP GPIO IP, and the poly must be same as AON GPIO setting."]
pub type GpioPin1WakdetEvtR = crate::BitReader;
#[doc = "Field `gpio_pin2_wakdet_evt` reader - Wake event for wake pin2 1: wake event detected 0: no wake event Note: aon gpio only used to wake hw pmc, to interrupt SW, it must enable GPIO interrupt in LP GPIO IP, and the poly must be same as AON GPIO setting."]
pub type GpioPin2WakdetEvtR = crate::BitReader;
#[doc = "Field `gpio_pin3_wakdet_evt` reader - Wake event for wake pin3 1: wake event detected 0: no wake event Note: aon gpio only used to wake hw pmc, to interrupt SW, it must enable GPIO interrupt in LP GPIO IP, and the poly must be same as AON GPIO setting."]
pub type GpioPin3WakdetEvtR = crate::BitReader;
#[doc = "Field `rtc_isr_evt` reader - RTC function interrupt wake event occurs."]
pub type RtcIsrEvtR = crate::BitReader;
#[doc = "Field `rtc_det_isr_evt` reader - RTC domain 131k detect interrupt wake event occurs."]
pub type RtcDetIsrEvtR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Chipen short press interrupt event occurs."]
    #[inline(always)]
    pub fn chipen_sp_isr_evt(&self) -> ChipenSpIsrEvtR {
        ChipenSpIsrEvtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Chipen long press interrupt event occurs."]
    #[inline(always)]
    pub fn chipen_lp_isr_evt(&self) -> ChipenLpIsrEvtR {
        ChipenLpIsrEvtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - bor interrupt event occurs."]
    #[inline(always)]
    pub fn bor_isr_evt(&self) -> BorIsrEvtR {
        BorIsrEvtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AON timer interrupt event occurs."]
    #[inline(always)]
    pub fn aon_tim_isr_evt(&self) -> AonTimIsrEvtR {
        AonTimIsrEvtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake event for wake pin0 1: wake event detected 0: no wake event Note: aon gpio only used to wake hw pmc, to interrupt SW, it must enable GPIO interrupt in LP GPIO IP, and the poly must be same as AON GPIO setting."]
    #[inline(always)]
    pub fn gpio_pin0_wakdet_evt(&self) -> GpioPin0WakdetEvtR {
        GpioPin0WakdetEvtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake event for wake pin1 1: wake event detected 0: no wake event Note: aon gpio only used to wake hw pmc, to interrupt SW, it must enable GPIO interrupt in LP GPIO IP, and the poly must be same as AON GPIO setting."]
    #[inline(always)]
    pub fn gpio_pin1_wakdet_evt(&self) -> GpioPin1WakdetEvtR {
        GpioPin1WakdetEvtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake event for wake pin2 1: wake event detected 0: no wake event Note: aon gpio only used to wake hw pmc, to interrupt SW, it must enable GPIO interrupt in LP GPIO IP, and the poly must be same as AON GPIO setting."]
    #[inline(always)]
    pub fn gpio_pin2_wakdet_evt(&self) -> GpioPin2WakdetEvtR {
        GpioPin2WakdetEvtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wake event for wake pin3 1: wake event detected 0: no wake event Note: aon gpio only used to wake hw pmc, to interrupt SW, it must enable GPIO interrupt in LP GPIO IP, and the poly must be same as AON GPIO setting."]
    #[inline(always)]
    pub fn gpio_pin3_wakdet_evt(&self) -> GpioPin3WakdetEvtR {
        GpioPin3WakdetEvtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC function interrupt wake event occurs."]
    #[inline(always)]
    pub fn rtc_isr_evt(&self) -> RtcIsrEvtR {
        RtcIsrEvtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC domain 131k detect interrupt wake event occurs."]
    #[inline(always)]
    pub fn rtc_det_isr_evt(&self) -> RtcDetIsrEvtR {
        RtcDetIsrEvtR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_aon_wak_evt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAonWakEvtSpec;
impl crate::RegisterSpec for RegAonWakEvtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_aon_wak_evt::R`](R) reader structure"]
impl crate::Readable for RegAonWakEvtSpec {}
#[doc = "`reset()` method sets REG_AON_WAK_EVT to value 0"]
impl crate::Resettable for RegAonWakEvtSpec {}
