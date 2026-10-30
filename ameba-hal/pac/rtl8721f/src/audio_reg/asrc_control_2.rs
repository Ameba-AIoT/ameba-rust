#[doc = "Register `asrc_control_2` reader"]
pub type R = crate::R<AsrcControl2Spec>;
#[doc = "Register `asrc_control_2` writer"]
pub type W = crate::W<AsrcControl2Spec>;
#[doc = "Field `asrc_rate_sel_rx_0` reader - 0: fs supports 4kHz ~ 60kHz 1: fs supports 60kHz ~ 120kHz 2/3: Reserved"]
pub type AsrcRateSelRx0R = crate::FieldReader;
#[doc = "Field `asrc_rate_sel_rx_0` writer - 0: fs supports 4kHz ~ 60kHz 1: fs supports 60kHz ~ 120kHz 2/3: Reserved"]
pub type AsrcRateSelRx0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `asrc_gain_sel_rx_0` reader - ASRC convergence rate: larger is faster but more noisy"]
pub type AsrcGainSelRx0R = crate::FieldReader;
#[doc = "Field `asrc_gain_sel_rx_0` writer - ASRC convergence rate: larger is faster but more noisy"]
pub type AsrcGainSelRx0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `asrc_auto_adjust_rx_0` reader - HW auto adjust convergence rate 0: Disable 1: Enable"]
pub type AsrcAutoAdjustRx0R = crate::BitReader;
#[doc = "Field `asrc_auto_adjust_rx_0` writer - HW auto adjust convergence rate 0: Disable 1: Enable"]
pub type AsrcAutoAdjustRx0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 0: fs supports 4kHz ~ 60kHz 1: fs supports 60kHz ~ 120kHz 2/3: Reserved"]
    #[inline(always)]
    pub fn asrc_rate_sel_rx_0(&self) -> AsrcRateSelRx0R {
        AsrcRateSelRx0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - ASRC convergence rate: larger is faster but more noisy"]
    #[inline(always)]
    pub fn asrc_gain_sel_rx_0(&self) -> AsrcGainSelRx0R {
        AsrcGainSelRx0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - HW auto adjust convergence rate 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn asrc_auto_adjust_rx_0(&self) -> AsrcAutoAdjustRx0R {
        AsrcAutoAdjustRx0R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 0: fs supports 4kHz ~ 60kHz 1: fs supports 60kHz ~ 120kHz 2/3: Reserved"]
    #[inline(always)]
    pub fn asrc_rate_sel_rx_0(&mut self) -> AsrcRateSelRx0W<'_, AsrcControl2Spec> {
        AsrcRateSelRx0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - ASRC convergence rate: larger is faster but more noisy"]
    #[inline(always)]
    pub fn asrc_gain_sel_rx_0(&mut self) -> AsrcGainSelRx0W<'_, AsrcControl2Spec> {
        AsrcGainSelRx0W::new(self, 2)
    }
    #[doc = "Bit 4 - HW auto adjust convergence rate 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn asrc_auto_adjust_rx_0(&mut self) -> AsrcAutoAdjustRx0W<'_, AsrcControl2Spec> {
        AsrcAutoAdjustRx0W::new(self, 4)
    }
}
#[doc = "ASRC Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`asrc_control_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asrc_control_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsrcControl2Spec;
impl crate::RegisterSpec for AsrcControl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asrc_control_2::R`](R) reader structure"]
impl crate::Readable for AsrcControl2Spec {}
#[doc = "`write(|w| ..)` method takes [`asrc_control_2::W`](W) writer structure"]
impl crate::Writable for AsrcControl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets asrc_control_2 to value 0x1c"]
impl crate::Resettable for AsrcControl2Spec {
    const RESET_VALUE: u32 = 0x1c;
}
