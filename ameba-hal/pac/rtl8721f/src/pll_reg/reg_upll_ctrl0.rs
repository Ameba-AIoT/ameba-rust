#[doc = "Register `REG_UPLL_CTRL0` reader"]
pub type R = crate::R<RegUpllCtrl0Spec>;
#[doc = "Register `REG_UPLL_CTRL0` writer"]
pub type W = crate::W<RegUpllCtrl0Spec>;
#[doc = "Field `ss_scaledown_mode` reader - use scaled-down timing values for usb, resulting in faster simulations"]
pub type SsScaledownModeR = crate::FieldReader;
#[doc = "Field `ss_scaledown_mode` writer - use scaled-down timing values for usb, resulting in faster simulations"]
pub type SsScaledownModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `suspend_wak_msk` reader - set 0 to mask usb suspendm(keep 1) at normal state after receiving usb suspend interrupt, set this bit to 1 to enable resume(from suspend) wake event"]
pub type SuspendWakMskR = crate::BitReader;
#[doc = "Field `suspend_wak_msk` writer - set 0 to mask usb suspendm(keep 1) at normal state after receiving usb suspend interrupt, set this bit to 1 to enable resume(from suspend) wake event"]
pub type SuspendWakMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTG_ANA_EN` reader - 1: enable usb aphy lv2hv, hv2lv check for otg"]
pub type OtgAnaEnR = crate::BitReader;
#[doc = "Field `OTG_ANA_EN` writer - 1: enable usb aphy lv2hv, hv2lv check for otg"]
pub type OtgAnaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWC_UAPC_EN` reader - enable USBPHY power cut UA33PC_EN = PWC_UAHV_ALIVE | PWC_UAPC_EN UA10PC_EN = PWC_UAPC_EN"]
pub type PwcUapcEnR = crate::BitReader;
#[doc = "Field `PWC_UAPC_EN` writer - enable USBPHY power cut UA33PC_EN = PWC_UAHV_ALIVE | PWC_UAPC_EN UA10PC_EN = PWC_UAPC_EN"]
pub type PwcUapcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWC_UAHV_ALIVE` reader - keep UA33PC_EN alive for usb suspend"]
pub type PwcUahvAliveR = crate::BitReader;
#[doc = "Field `PWC_UAHV_ALIVE` writer - keep UA33PC_EN alive for usb suspend"]
pub type PwcUahvAliveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWC_UABG_EN` reader - 1: enable USBPHY BG power cut"]
pub type PwcUabgEnR = crate::BitReader;
#[doc = "Field `PWC_UABG_EN` writer - 1: enable USBPHY BG power cut"]
pub type PwcUabgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DPHY_EN` reader - usb dphy power on reset, low active"]
pub type UsbDphyEnR = crate::BitReader;
#[doc = "Field `USB_DPHY_EN` writer - usb dphy power on reset, low active"]
pub type UsbDphyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_APHY_EN` reader - usb aphy power on reset, low active this bit also controls uphy pll: can only write 1 after xtal normal 40M is ready write 0, uphy pll will be disabled Note: when power on , HW pmc only set this bit to 1 , other using default value , and USB pll can power up"]
pub type UsbAphyEnR = crate::BitReader;
#[doc = "Field `USB_APHY_EN` writer - usb aphy power on reset, low active this bit also controls uphy pll: can only write 1 after xtal normal 40M is ready write 0, uphy pll will be disabled Note: when power on , HW pmc only set this bit to 1 , other using default value , and USB pll can power up"]
pub type UsbAphyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_PLL_EN` reader - uphy pll enable request, keep pll alive when usb entering suspend state both usb_aphy_en and sys_pll_en should be set 0 to disable uphy pll"]
pub type SysPllEnR = crate::BitReader;
#[doc = "Field `SYS_PLL_EN` writer - uphy pll enable request, keep pll alive when usb entering suspend state both usb_aphy_en and sys_pll_en should be set 0 to disable uphy pll"]
pub type SysPllEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO_USBA_EN` reader - 1: Enable isolation signal from USBPHY analog 0: disable"]
pub type IsoUsbaEnR = crate::BitReader;
#[doc = "Field `ISO_USBA_EN` writer - 1: Enable isolation signal from USBPHY analog 0: disable"]
pub type IsoUsbaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB2_DIGPADEN` reader - 1: usb pad used as digital IO pad"]
pub type Usb2DigpadenR = crate::BitReader;
#[doc = "Field `USB2_DIGPADEN` writer - 1: usb pad used as digital IO pad"]
pub type Usb2DigpadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_OTG_EN` reader - 0: disable otg signals to usb mac"]
pub type UsbOtgEnR = crate::BitReader;
#[doc = "Field `USB_OTG_EN` writer - 0: disable otg signals to usb mac"]
pub type UsbOtgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB2_DIGOTGPADEN` reader - 1: usb otg pad used as digital IO pad"]
pub type Usb2DigotgpadenR = crate::BitReader;
#[doc = "Field `USB2_DIGOTGPADEN` writer - 1: usb otg pad used as digital IO pad"]
pub type Usb2DigotgpadenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - use scaled-down timing values for usb, resulting in faster simulations"]
    #[inline(always)]
    pub fn ss_scaledown_mode(&self) -> SsScaledownModeR {
        SsScaledownModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - set 0 to mask usb suspendm(keep 1) at normal state after receiving usb suspend interrupt, set this bit to 1 to enable resume(from suspend) wake event"]
    #[inline(always)]
    pub fn suspend_wak_msk(&self) -> SuspendWakMskR {
        SuspendWakMskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable usb aphy lv2hv, hv2lv check for otg"]
    #[inline(always)]
    pub fn otg_ana_en(&self) -> OtgAnaEnR {
        OtgAnaEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - enable USBPHY power cut UA33PC_EN = PWC_UAHV_ALIVE | PWC_UAPC_EN UA10PC_EN = PWC_UAPC_EN"]
    #[inline(always)]
    pub fn pwc_uapc_en(&self) -> PwcUapcEnR {
        PwcUapcEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - keep UA33PC_EN alive for usb suspend"]
    #[inline(always)]
    pub fn pwc_uahv_alive(&self) -> PwcUahvAliveR {
        PwcUahvAliveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: enable USBPHY BG power cut"]
    #[inline(always)]
    pub fn pwc_uabg_en(&self) -> PwcUabgEnR {
        PwcUabgEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - usb dphy power on reset, low active"]
    #[inline(always)]
    pub fn usb_dphy_en(&self) -> UsbDphyEnR {
        UsbDphyEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - usb aphy power on reset, low active this bit also controls uphy pll: can only write 1 after xtal normal 40M is ready write 0, uphy pll will be disabled Note: when power on , HW pmc only set this bit to 1 , other using default value , and USB pll can power up"]
    #[inline(always)]
    pub fn usb_aphy_en(&self) -> UsbAphyEnR {
        UsbAphyEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - uphy pll enable request, keep pll alive when usb entering suspend state both usb_aphy_en and sys_pll_en should be set 0 to disable uphy pll"]
    #[inline(always)]
    pub fn sys_pll_en(&self) -> SysPllEnR {
        SysPllEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Enable isolation signal from USBPHY analog 0: disable"]
    #[inline(always)]
    pub fn iso_usba_en(&self) -> IsoUsbaEnR {
        IsoUsbaEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: usb pad used as digital IO pad"]
    #[inline(always)]
    pub fn usb2_digpaden(&self) -> Usb2DigpadenR {
        Usb2DigpadenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 0: disable otg signals to usb mac"]
    #[inline(always)]
    pub fn usb_otg_en(&self) -> UsbOtgEnR {
        UsbOtgEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: usb otg pad used as digital IO pad"]
    #[inline(always)]
    pub fn usb2_digotgpaden(&self) -> Usb2DigotgpadenR {
        Usb2DigotgpadenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - use scaled-down timing values for usb, resulting in faster simulations"]
    #[inline(always)]
    pub fn ss_scaledown_mode(&mut self) -> SsScaledownModeW<'_, RegUpllCtrl0Spec> {
        SsScaledownModeW::new(self, 0)
    }
    #[doc = "Bit 4 - set 0 to mask usb suspendm(keep 1) at normal state after receiving usb suspend interrupt, set this bit to 1 to enable resume(from suspend) wake event"]
    #[inline(always)]
    pub fn suspend_wak_msk(&mut self) -> SuspendWakMskW<'_, RegUpllCtrl0Spec> {
        SuspendWakMskW::new(self, 4)
    }
    #[doc = "Bit 16 - 1: enable usb aphy lv2hv, hv2lv check for otg"]
    #[inline(always)]
    pub fn otg_ana_en(&mut self) -> OtgAnaEnW<'_, RegUpllCtrl0Spec> {
        OtgAnaEnW::new(self, 16)
    }
    #[doc = "Bit 17 - enable USBPHY power cut UA33PC_EN = PWC_UAHV_ALIVE | PWC_UAPC_EN UA10PC_EN = PWC_UAPC_EN"]
    #[inline(always)]
    pub fn pwc_uapc_en(&mut self) -> PwcUapcEnW<'_, RegUpllCtrl0Spec> {
        PwcUapcEnW::new(self, 17)
    }
    #[doc = "Bit 18 - keep UA33PC_EN alive for usb suspend"]
    #[inline(always)]
    pub fn pwc_uahv_alive(&mut self) -> PwcUahvAliveW<'_, RegUpllCtrl0Spec> {
        PwcUahvAliveW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: enable USBPHY BG power cut"]
    #[inline(always)]
    pub fn pwc_uabg_en(&mut self) -> PwcUabgEnW<'_, RegUpllCtrl0Spec> {
        PwcUabgEnW::new(self, 19)
    }
    #[doc = "Bit 20 - usb dphy power on reset, low active"]
    #[inline(always)]
    pub fn usb_dphy_en(&mut self) -> UsbDphyEnW<'_, RegUpllCtrl0Spec> {
        UsbDphyEnW::new(self, 20)
    }
    #[doc = "Bit 21 - usb aphy power on reset, low active this bit also controls uphy pll: can only write 1 after xtal normal 40M is ready write 0, uphy pll will be disabled Note: when power on , HW pmc only set this bit to 1 , other using default value , and USB pll can power up"]
    #[inline(always)]
    pub fn usb_aphy_en(&mut self) -> UsbAphyEnW<'_, RegUpllCtrl0Spec> {
        UsbAphyEnW::new(self, 21)
    }
    #[doc = "Bit 22 - uphy pll enable request, keep pll alive when usb entering suspend state both usb_aphy_en and sys_pll_en should be set 0 to disable uphy pll"]
    #[inline(always)]
    pub fn sys_pll_en(&mut self) -> SysPllEnW<'_, RegUpllCtrl0Spec> {
        SysPllEnW::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Enable isolation signal from USBPHY analog 0: disable"]
    #[inline(always)]
    pub fn iso_usba_en(&mut self) -> IsoUsbaEnW<'_, RegUpllCtrl0Spec> {
        IsoUsbaEnW::new(self, 23)
    }
    #[doc = "Bit 26 - 1: usb pad used as digital IO pad"]
    #[inline(always)]
    pub fn usb2_digpaden(&mut self) -> Usb2DigpadenW<'_, RegUpllCtrl0Spec> {
        Usb2DigpadenW::new(self, 26)
    }
    #[doc = "Bit 27 - 0: disable otg signals to usb mac"]
    #[inline(always)]
    pub fn usb_otg_en(&mut self) -> UsbOtgEnW<'_, RegUpllCtrl0Spec> {
        UsbOtgEnW::new(self, 27)
    }
    #[doc = "Bit 28 - 1: usb otg pad used as digital IO pad"]
    #[inline(always)]
    pub fn usb2_digotgpaden(&mut self) -> Usb2DigotgpadenW<'_, RegUpllCtrl0Spec> {
        Usb2DigotgpadenW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_upll_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_upll_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegUpllCtrl0Spec;
impl crate::RegisterSpec for RegUpllCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_upll_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegUpllCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_upll_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegUpllCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_UPLL_CTRL0 to value 0x14da_0010"]
impl crate::Resettable for RegUpllCtrl0Spec {
    const RESET_VALUE: u32 = 0x14da_0010;
}
