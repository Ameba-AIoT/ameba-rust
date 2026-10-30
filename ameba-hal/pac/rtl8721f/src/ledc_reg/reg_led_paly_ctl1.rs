#[doc = "Register `REG_LED_PALY_CTL1` reader"]
pub type R = crate::R<RegLedPalyCtl1Spec>;
#[doc = "Register `REG_LED_PALY_CTL1` writer"]
pub type W = crate::W<RegLedPalyCtl1Spec>;
#[doc = "Field `r_led_pwm_time` reader - one stage time of a frame, unit 255*250ns"]
pub type RLedPwmTimeR = crate::FieldReader<u16>;
#[doc = "Field `r_led_pwm_time` writer - one stage time of a frame, unit 255*250ns"]
pub type RLedPwmTimeW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `r_led_blank_time` reader - time is r_led_blank_time +1 , unit 4M clock"]
pub type RLedBlankTimeR = crate::FieldReader;
#[doc = "Field `r_led_blank_time` writer - time is r_led_blank_time +1 , unit 4M clock"]
pub type RLedBlankTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:12 - one stage time of a frame, unit 255*250ns"]
    #[inline(always)]
    pub fn r_led_pwm_time(&self) -> RLedPwmTimeR {
        RLedPwmTimeR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:23 - time is r_led_blank_time +1 , unit 4M clock"]
    #[inline(always)]
    pub fn r_led_blank_time(&self) -> RLedBlankTimeR {
        RLedBlankTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - one stage time of a frame, unit 255*250ns"]
    #[inline(always)]
    pub fn r_led_pwm_time(&mut self) -> RLedPwmTimeW<'_, RegLedPalyCtl1Spec> {
        RLedPwmTimeW::new(self, 0)
    }
    #[doc = "Bits 16:23 - time is r_led_blank_time +1 , unit 4M clock"]
    #[inline(always)]
    pub fn r_led_blank_time(&mut self) -> RLedBlankTimeW<'_, RegLedPalyCtl1Spec> {
        RLedBlankTimeW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_led_paly_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_led_paly_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLedPalyCtl1Spec;
impl crate::RegisterSpec for RegLedPalyCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_led_paly_ctl1::R`](R) reader structure"]
impl crate::Readable for RegLedPalyCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_led_paly_ctl1::W`](W) writer structure"]
impl crate::Writable for RegLedPalyCtl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LED_PALY_CTL1 to value 0x003b_0005"]
impl crate::Resettable for RegLedPalyCtl1Spec {
    const RESET_VALUE: u32 = 0x003b_0005;
}
