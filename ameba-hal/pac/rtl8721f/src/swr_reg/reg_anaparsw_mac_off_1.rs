#[doc = "Register `REG_ANAPARSW_MAC_OFF_1` reader"]
pub type R = crate::R<RegAnaparswMacOff1Spec>;
#[doc = "Register `REG_ANAPARSW_MAC_OFF_1` writer"]
pub type W = crate::W<RegAnaparswMacOff1Spec>;
#[doc = "Field `SDZP_L` reader - Nonoverlap delay cell for PMOS"]
pub type SdzpLR = crate::FieldReader;
#[doc = "Field `SDZP_L` writer - Nonoverlap delay cell for PMOS"]
pub type SdzpLW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENSR_L` reader - 1:with SSR latch"]
pub type EnsrLR = crate::BitReader;
#[doc = "Field `ENSR_L` writer - 1:with SSR latch"]
pub type EnsrLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_PWM_CTRL_L` reader - 0:internal PWM 1:external PWM"]
pub type RegPwmCtrlLR = crate::BitReader;
#[doc = "Field `reg_PWM_CTRL_L` writer - 0:internal PWM 1:external PWM"]
pub type RegPwmCtrlLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mux_PI_L` reader - 1:PI (original) 0:i"]
pub type RegMuxPiLR = crate::BitReader;
#[doc = "Field `reg_mux_PI_L` writer - 1:PI (original) 0:i"]
pub type RegMuxPiLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_nmos_off_L` reader - 0:normal 1:NMOS always off"]
pub type RegNmosOffLR = crate::BitReader;
#[doc = "Field `null8` reader - dummy"]
pub type Null8R = crate::FieldReader;
#[doc = "Field `null8` writer - dummy"]
pub type Null8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `null9` reader - dummy"]
pub type Null9R = crate::FieldReader;
#[doc = "Field `null9` writer - dummy"]
pub type Null9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Nonoverlap delay cell for PMOS"]
    #[inline(always)]
    pub fn sdzp_l(&self) -> SdzpLR {
        SdzpLR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 1:with SSR latch"]
    #[inline(always)]
    pub fn ensr_l(&self) -> EnsrLR {
        EnsrLR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0:internal PWM 1:external PWM"]
    #[inline(always)]
    pub fn reg_pwm_ctrl_l(&self) -> RegPwmCtrlLR {
        RegPwmCtrlLR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:PI (original) 0:i"]
    #[inline(always)]
    pub fn reg_mux_pi_l(&self) -> RegMuxPiLR {
        RegMuxPiLR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0:normal 1:NMOS always off"]
    #[inline(always)]
    pub fn reg_nmos_off_l(&self) -> RegNmosOffLR {
        RegNmosOffLR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - dummy"]
    #[inline(always)]
    pub fn null8(&self) -> Null8R {
        Null8R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - dummy"]
    #[inline(always)]
    pub fn null9(&self) -> Null9R {
        Null9R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Nonoverlap delay cell for PMOS"]
    #[inline(always)]
    pub fn sdzp_l(&mut self) -> SdzpLW<'_, RegAnaparswMacOff1Spec> {
        SdzpLW::new(self, 0)
    }
    #[doc = "Bit 2 - 1:with SSR latch"]
    #[inline(always)]
    pub fn ensr_l(&mut self) -> EnsrLW<'_, RegAnaparswMacOff1Spec> {
        EnsrLW::new(self, 2)
    }
    #[doc = "Bit 3 - 0:internal PWM 1:external PWM"]
    #[inline(always)]
    pub fn reg_pwm_ctrl_l(&mut self) -> RegPwmCtrlLW<'_, RegAnaparswMacOff1Spec> {
        RegPwmCtrlLW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:PI (original) 0:i"]
    #[inline(always)]
    pub fn reg_mux_pi_l(&mut self) -> RegMuxPiLW<'_, RegAnaparswMacOff1Spec> {
        RegMuxPiLW::new(self, 4)
    }
    #[doc = "Bits 6:7 - dummy"]
    #[inline(always)]
    pub fn null8(&mut self) -> Null8W<'_, RegAnaparswMacOff1Spec> {
        Null8W::new(self, 6)
    }
    #[doc = "Bits 8:15 - dummy"]
    #[inline(always)]
    pub fn null9(&mut self) -> Null9W<'_, RegAnaparswMacOff1Spec> {
        Null9W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anaparsw_mac_off_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anaparsw_mac_off_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparswMacOff1Spec;
impl crate::RegisterSpec for RegAnaparswMacOff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anaparsw_mac_off_1::R`](R) reader structure"]
impl crate::Readable for RegAnaparswMacOff1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anaparsw_mac_off_1::W`](W) writer structure"]
impl crate::Writable for RegAnaparswMacOff1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPARSW_MAC_OFF_1 to value 0xc0c5"]
impl crate::Resettable for RegAnaparswMacOff1Spec {
    const RESET_VALUE: u32 = 0xc0c5;
}
