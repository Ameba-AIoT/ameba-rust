#[doc = "Register `REG_RTC_DET_CTRL1` reader"]
pub type R = crate::R<RegRtcDetCtrl1Spec>;
#[doc = "Register `REG_RTC_DET_CTRL1` writer"]
pub type W = crate::W<RegRtcDetCtrl1Spec>;
#[doc = "Field `rtc_force_rst` reader - Software control to send a reset sequence to RTC domain . It will auto clear to 0"]
pub type RtcForceRstR = crate::BitReader;
#[doc = "Field `rtc_force_rst` writer - Software control to send a reset sequence to RTC domain . It will auto clear to 0"]
pub type RtcForceRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtc_force_clr` reader - Software control to send a iso clear sequence to RTC domain . It will auto clear to 0"]
pub type RtcForceClrR = crate::BitReader;
#[doc = "Field `rtc_force_clr` writer - Software control to send a iso clear sequence to RTC domain . It will auto clear to 0"]
pub type RtcForceClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtc_det_st` reader - Aon det rtc state machine flag, just for debug"]
pub type RtcDetStR = crate::FieldReader;
#[doc = "Field `rtc_rst_cnt` reader - It indicates How many times aon send reset sequence to RTC. write 4'b1 will clear this value"]
pub type RtcRstCntR = crate::FieldReader;
#[doc = "Field `rtc_det_intr` reader - Every time 131k re-detected , it will raise aon interrup . write 1'b1 will clear this value"]
pub type RtcDetIntrR = crate::BitReader;
impl R {
    #[doc = "Bit 10 - Software control to send a reset sequence to RTC domain . It will auto clear to 0"]
    #[inline(always)]
    pub fn rtc_force_rst(&self) -> RtcForceRstR {
        RtcForceRstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software control to send a iso clear sequence to RTC domain . It will auto clear to 0"]
    #[inline(always)]
    pub fn rtc_force_clr(&self) -> RtcForceClrR {
        RtcForceClrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Aon det rtc state machine flag, just for debug"]
    #[inline(always)]
    pub fn rtc_det_st(&self) -> RtcDetStR {
        RtcDetStR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - It indicates How many times aon send reset sequence to RTC. write 4'b1 will clear this value"]
    #[inline(always)]
    pub fn rtc_rst_cnt(&self) -> RtcRstCntR {
        RtcRstCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Every time 131k re-detected , it will raise aon interrup . write 1'b1 will clear this value"]
    #[inline(always)]
    pub fn rtc_det_intr(&self) -> RtcDetIntrR {
        RtcDetIntrR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Software control to send a reset sequence to RTC domain . It will auto clear to 0"]
    #[inline(always)]
    pub fn rtc_force_rst(&mut self) -> RtcForceRstW<'_, RegRtcDetCtrl1Spec> {
        RtcForceRstW::new(self, 10)
    }
    #[doc = "Bit 11 - Software control to send a iso clear sequence to RTC domain . It will auto clear to 0"]
    #[inline(always)]
    pub fn rtc_force_clr(&mut self) -> RtcForceClrW<'_, RegRtcDetCtrl1Spec> {
        RtcForceClrW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rtc_det_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rtc_det_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRtcDetCtrl1Spec;
impl crate::RegisterSpec for RegRtcDetCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rtc_det_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegRtcDetCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rtc_det_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegRtcDetCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RTC_DET_CTRL1 to value 0"]
impl crate::Resettable for RegRtcDetCtrl1Spec {}
