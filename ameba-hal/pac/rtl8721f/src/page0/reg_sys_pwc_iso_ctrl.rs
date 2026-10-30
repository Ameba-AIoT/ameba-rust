#[doc = "Register `REG_SYS_PWC_ISO_CTRL` reader"]
pub type R = crate::R<RegSysPwcIsoCtrlSpec>;
#[doc = "Register `REG_SYS_PWC_ISO_CTRL` writer"]
pub type W = crate::W<RegSysPwcIsoCtrlSpec>;
#[doc = "Field `BIT_SPC_MAC_EN` reader - 1: Enable WLOFF power cut"]
pub type BitSpcMacEnR = crate::BitReader;
#[doc = "Field `BIT_SPC_MAC_EN` writer - 1: Enable WLOFF power cut"]
pub type BitSpcMacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_POF_WAIT_ALL` reader - gen req of swr and xtal and wait rdy together when power off"]
pub type BitWlPofWaitAllR = crate::BitReader;
#[doc = "Field `BIT_WL_POF_WAIT_ALL` writer - gen req of swr and xtal and wait rdy together when power off"]
pub type BitWlPofWaitAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_PON_WAIT_ALL` reader - gen req of pwc and swr and xtal and wait rdy together when power on"]
pub type BitWlPonWaitAllR = crate::BitReader;
#[doc = "Field `BIT_WL_PON_WAIT_ALL` writer - gen req of pwc and swr and xtal and wait rdy together when power on"]
pub type BitWlPonWaitAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_SWR_REQ_TRG` writer - trigger switch swr mode according to 0x0\\[11:9\\] when wlan is active, write 1 to generate one pulse"]
pub type BitWlSwrReqTrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_SWR_STS_REQ_LPS` reader - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
pub type BitWlSwrStsReqLpsR = crate::FieldReader;
#[doc = "Field `BIT_WL_SWR_STS_REQ_LPS` writer - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
pub type BitWlSwrStsReqLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_WL_SWR_STS_REQ_ACT` reader - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
pub type BitWlSwrStsReqActR = crate::FieldReader;
#[doc = "Field `BIT_WL_SWR_STS_REQ_ACT` writer - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
pub type BitWlSwrStsReqActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_WL_PWR_SWITCH_DMY_EN` reader - 1:enable wl_dummy_en when wlan is switching power mode 0:disable wl_dummy_en when wlan is switching power mode"]
pub type BitWlPwrSwitchDmyEnR = crate::BitReader;
#[doc = "Field `BIT_WL_PWR_SWITCH_DMY_EN` writer - 1:enable wl_dummy_en when wlan is switching power mode 0:disable wl_dummy_en when wlan is switching power mode"]
pub type BitWlPwrSwitchDmyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WL_SWR_STS_REQ` reader - the swr request mode of wifi"]
pub type BitWlSwrStsReqR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 1: Enable WLOFF power cut"]
    #[inline(always)]
    pub fn bit_spc_mac_en(&self) -> BitSpcMacEnR {
        BitSpcMacEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - gen req of swr and xtal and wait rdy together when power off"]
    #[inline(always)]
    pub fn bit_wl_pof_wait_all(&self) -> BitWlPofWaitAllR {
        BitWlPofWaitAllR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - gen req of pwc and swr and xtal and wait rdy together when power on"]
    #[inline(always)]
    pub fn bit_wl_pon_wait_all(&self) -> BitWlPonWaitAllR {
        BitWlPonWaitAllR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
    #[inline(always)]
    pub fn bit_wl_swr_sts_req_lps(&self) -> BitWlSwrStsReqLpsR {
        BitWlSwrStsReqLpsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
    #[inline(always)]
    pub fn bit_wl_swr_sts_req_act(&self) -> BitWlSwrStsReqActR {
        BitWlSwrStsReqActR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - 1:enable wl_dummy_en when wlan is switching power mode 0:disable wl_dummy_en when wlan is switching power mode"]
    #[inline(always)]
    pub fn bit_wl_pwr_switch_dmy_en(&self) -> BitWlPwrSwitchDmyEnR {
        BitWlPwrSwitchDmyEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - the swr request mode of wifi"]
    #[inline(always)]
    pub fn bit_wl_swr_sts_req(&self) -> BitWlSwrStsReqR {
        BitWlSwrStsReqR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable WLOFF power cut"]
    #[inline(always)]
    pub fn bit_spc_mac_en(&mut self) -> BitSpcMacEnW<'_, RegSysPwcIsoCtrlSpec> {
        BitSpcMacEnW::new(self, 0)
    }
    #[doc = "Bit 2 - gen req of swr and xtal and wait rdy together when power off"]
    #[inline(always)]
    pub fn bit_wl_pof_wait_all(&mut self) -> BitWlPofWaitAllW<'_, RegSysPwcIsoCtrlSpec> {
        BitWlPofWaitAllW::new(self, 2)
    }
    #[doc = "Bit 3 - gen req of pwc and swr and xtal and wait rdy together when power on"]
    #[inline(always)]
    pub fn bit_wl_pon_wait_all(&mut self) -> BitWlPonWaitAllW<'_, RegSysPwcIsoCtrlSpec> {
        BitWlPonWaitAllW::new(self, 3)
    }
    #[doc = "Bit 4 - trigger switch swr mode according to 0x0\\[11:9\\] when wlan is active, write 1 to generate one pulse"]
    #[inline(always)]
    pub fn bit_wl_swr_req_trg(&mut self) -> BitWlSwrReqTrgW<'_, RegSysPwcIsoCtrlSpec> {
        BitWlSwrReqTrgW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
    #[inline(always)]
    pub fn bit_wl_swr_sts_req_lps(&mut self) -> BitWlSwrStsReqLpsW<'_, RegSysPwcIsoCtrlSpec> {
        BitWlSwrStsReqLpsW::new(self, 5)
    }
    #[doc = "Bits 9:11 - 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
    #[inline(always)]
    pub fn bit_wl_swr_sts_req_act(&mut self) -> BitWlSwrStsReqActW<'_, RegSysPwcIsoCtrlSpec> {
        BitWlSwrStsReqActW::new(self, 9)
    }
    #[doc = "Bit 12 - 1:enable wl_dummy_en when wlan is switching power mode 0:disable wl_dummy_en when wlan is switching power mode"]
    #[inline(always)]
    pub fn bit_wl_pwr_switch_dmy_en(&mut self) -> BitWlPwrSwitchDmyEnW<'_, RegSysPwcIsoCtrlSpec> {
        BitWlPwrSwitchDmyEnW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sys_pwc_iso_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sys_pwc_iso_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSysPwcIsoCtrlSpec;
impl crate::RegisterSpec for RegSysPwcIsoCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_sys_pwc_iso_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSysPwcIsoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sys_pwc_iso_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSysPwcIsoCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SYS_PWC_ISO_CTRL to value 0x1c4c"]
impl crate::Resettable for RegSysPwcIsoCtrlSpec {
    const RESET_VALUE: u16 = 0x1c4c;
}
