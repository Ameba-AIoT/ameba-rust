#[doc = "Register `REG_LED_PALY_CTL0` reader"]
pub type R = crate::R<RegLedPalyCtl0Spec>;
#[doc = "Register `REG_LED_PALY_CTL0` writer"]
pub type W = crate::W<RegLedPalyCtl0Spec>;
#[doc = "Field `r_led_display_en` reader - 1: enable led driver to display;0:disable led driver to display"]
pub type RLedDisplayEnR = crate::BitReader;
#[doc = "Field `r_led_display_en` writer - 1: enable led driver to display;0:disable led driver to display"]
pub type RLedDisplayEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ledbuf_flag` reader - 1: display data of ram1; 0: display data of ram0. HW check in when display enable and sw trigger hw switch ram"]
pub type LedbufFlagR = crate::BitReader;
#[doc = "Field `ledbuf_switch_on` reader - 1: sw trigger hw switch ram; 0: hw switch finish"]
pub type LedbufSwitchOnR = crate::BitReader;
#[doc = "Field `r_led_column` reader - led column number - 1"]
pub type RLedColumnR = crate::FieldReader;
#[doc = "Field `r_led_column` writer - led column number - 1"]
pub type RLedColumnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_led_row` reader - led row number - 1"]
pub type RLedRowR = crate::FieldReader;
#[doc = "Field `r_led_row` writer - led row number - 1"]
pub type RLedRowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_overlap_time` reader - time is r_overlap_time +1, delay beweent stage, unit 4M clock"]
pub type ROverlapTimeR = crate::FieldReader;
#[doc = "Field `r_overlap_time` writer - time is r_overlap_time +1, delay beweent stage, unit 4M clock"]
pub type ROverlapTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_led_buf_clear` writer - write 1 generate one pulse. Clear and reset ram data"]
pub type RLedBufClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_led_column_idle_status` reader - 1: output 0: floating"]
pub type RLedColumnIdleStatusR = crate::BitReader;
#[doc = "Field `r_led_column_idle_status` writer - 1: output 0: floating"]
pub type RLedColumnIdleStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_led_row_idle_status` reader - 1: output 0: floating"]
pub type RLedRowIdleStatusR = crate::BitReader;
#[doc = "Field `r_led_row_idle_status` writer - 1: output 0: floating"]
pub type RLedRowIdleStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_seq_pass_flag` reader - 1: indicate 0x0c\\[31:0\\] sequence check pass, clear 0 when read"]
pub type RSeqPassFlagR = crate::BitReader;
#[doc = "Field `r_led_display_mode` reader - 1: show rows in sequence 0: show columns in sequence"]
pub type RLedDisplayModeR = crate::BitReader;
#[doc = "Field `r_led_display_mode` writer - 1: show rows in sequence 0: show columns in sequence"]
pub type RLedDisplayModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_led_col_disp_inv` reader - 1: inverse column output logic of the current display mode; 0: default column output logic of the current display mode"]
pub type RLedColDispInvR = crate::BitReader;
#[doc = "Field `r_led_col_disp_inv` writer - 1: inverse column output logic of the current display mode; 0: default column output logic of the current display mode"]
pub type RLedColDispInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_led_row_disp_inv` reader - 1: inverse row output logic of the current display mode; 0: default row output logic of the current display mode"]
pub type RLedRowDispInvR = crate::BitReader;
#[doc = "Field `r_led_row_disp_inv` writer - 1: inverse row output logic of the current display mode; 0: default row output logic of the current display mode"]
pub type RLedRowDispInvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: enable led driver to display;0:disable led driver to display"]
    #[inline(always)]
    pub fn r_led_display_en(&self) -> RLedDisplayEnR {
        RLedDisplayEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: display data of ram1; 0: display data of ram0. HW check in when display enable and sw trigger hw switch ram"]
    #[inline(always)]
    pub fn ledbuf_flag(&self) -> LedbufFlagR {
        LedbufFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: sw trigger hw switch ram; 0: hw switch finish"]
    #[inline(always)]
    pub fn ledbuf_switch_on(&self) -> LedbufSwitchOnR {
        LedbufSwitchOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - led column number - 1"]
    #[inline(always)]
    pub fn r_led_column(&self) -> RLedColumnR {
        RLedColumnR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - led row number - 1"]
    #[inline(always)]
    pub fn r_led_row(&self) -> RLedRowR {
        RLedRowR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:16 - time is r_overlap_time +1, delay beweent stage, unit 4M clock"]
    #[inline(always)]
    pub fn r_overlap_time(&self) -> ROverlapTimeR {
        ROverlapTimeR::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 18 - 1: output 0: floating"]
    #[inline(always)]
    pub fn r_led_column_idle_status(&self) -> RLedColumnIdleStatusR {
        RLedColumnIdleStatusR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: output 0: floating"]
    #[inline(always)]
    pub fn r_led_row_idle_status(&self) -> RLedRowIdleStatusR {
        RLedRowIdleStatusR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: indicate 0x0c\\[31:0\\] sequence check pass, clear 0 when read"]
    #[inline(always)]
    pub fn r_seq_pass_flag(&self) -> RSeqPassFlagR {
        RSeqPassFlagR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: show rows in sequence 0: show columns in sequence"]
    #[inline(always)]
    pub fn r_led_display_mode(&self) -> RLedDisplayModeR {
        RLedDisplayModeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: inverse column output logic of the current display mode; 0: default column output logic of the current display mode"]
    #[inline(always)]
    pub fn r_led_col_disp_inv(&self) -> RLedColDispInvR {
        RLedColDispInvR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: inverse row output logic of the current display mode; 0: default row output logic of the current display mode"]
    #[inline(always)]
    pub fn r_led_row_disp_inv(&self) -> RLedRowDispInvR {
        RLedRowDispInvR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable led driver to display;0:disable led driver to display"]
    #[inline(always)]
    pub fn r_led_display_en(&mut self) -> RLedDisplayEnW<'_, RegLedPalyCtl0Spec> {
        RLedDisplayEnW::new(self, 0)
    }
    #[doc = "Bits 3:5 - led column number - 1"]
    #[inline(always)]
    pub fn r_led_column(&mut self) -> RLedColumnW<'_, RegLedPalyCtl0Spec> {
        RLedColumnW::new(self, 3)
    }
    #[doc = "Bits 6:8 - led row number - 1"]
    #[inline(always)]
    pub fn r_led_row(&mut self) -> RLedRowW<'_, RegLedPalyCtl0Spec> {
        RLedRowW::new(self, 6)
    }
    #[doc = "Bits 9:16 - time is r_overlap_time +1, delay beweent stage, unit 4M clock"]
    #[inline(always)]
    pub fn r_overlap_time(&mut self) -> ROverlapTimeW<'_, RegLedPalyCtl0Spec> {
        ROverlapTimeW::new(self, 9)
    }
    #[doc = "Bit 17 - write 1 generate one pulse. Clear and reset ram data"]
    #[inline(always)]
    pub fn r_led_buf_clear(&mut self) -> RLedBufClearW<'_, RegLedPalyCtl0Spec> {
        RLedBufClearW::new(self, 17)
    }
    #[doc = "Bit 18 - 1: output 0: floating"]
    #[inline(always)]
    pub fn r_led_column_idle_status(&mut self) -> RLedColumnIdleStatusW<'_, RegLedPalyCtl0Spec> {
        RLedColumnIdleStatusW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: output 0: floating"]
    #[inline(always)]
    pub fn r_led_row_idle_status(&mut self) -> RLedRowIdleStatusW<'_, RegLedPalyCtl0Spec> {
        RLedRowIdleStatusW::new(self, 19)
    }
    #[doc = "Bit 21 - 1: show rows in sequence 0: show columns in sequence"]
    #[inline(always)]
    pub fn r_led_display_mode(&mut self) -> RLedDisplayModeW<'_, RegLedPalyCtl0Spec> {
        RLedDisplayModeW::new(self, 21)
    }
    #[doc = "Bit 22 - 1: inverse column output logic of the current display mode; 0: default column output logic of the current display mode"]
    #[inline(always)]
    pub fn r_led_col_disp_inv(&mut self) -> RLedColDispInvW<'_, RegLedPalyCtl0Spec> {
        RLedColDispInvW::new(self, 22)
    }
    #[doc = "Bit 23 - 1: inverse row output logic of the current display mode; 0: default row output logic of the current display mode"]
    #[inline(always)]
    pub fn r_led_row_disp_inv(&mut self) -> RLedRowDispInvW<'_, RegLedPalyCtl0Spec> {
        RLedRowDispInvW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_led_paly_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_led_paly_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLedPalyCtl0Spec;
impl crate::RegisterSpec for RegLedPalyCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_led_paly_ctl0::R`](R) reader structure"]
impl crate::Readable for RegLedPalyCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_led_paly_ctl0::W`](W) writer structure"]
impl crate::Writable for RegLedPalyCtl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LED_PALY_CTL0 to value 0x000c_0602"]
impl crate::Resettable for RegLedPalyCtl0Spec {
    const RESET_VALUE: u32 = 0x000c_0602;
}
