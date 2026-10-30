#[doc = "Register `REG_WLAN_PWRCTL` reader"]
pub type R = crate::R<RegWlanPwrctlSpec>;
#[doc = "Register `REG_WLAN_PWRCTL` writer"]
pub type W = crate::W<RegWlanPwrctlSpec>;
#[doc = "Field `WL_HW_RESP_FEN` reader - 1: enable HW auto response"]
pub type WlHwRespFenR = crate::BitReader;
#[doc = "Field `WL_HW_RESP_FEN` writer - 1: enable HW auto response"]
pub type WlHwRespFenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WL_REQ2ACT` reader - "]
pub type WlReq2actR = crate::BitReader;
#[doc = "Field `WL_RDY2ACT` reader - System infrom WLAN ready to active"]
pub type WlRdy2actR = crate::BitReader;
#[doc = "Field `SW_SET_RDY2ACT` reader - "]
pub type SwSetRdy2actR = crate::BitReader;
#[doc = "Field `SW_SET_RDY2ACT` writer - "]
pub type SwSetRdy2actW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CLR_RDY2ACT` reader - System infrom WLAN ready to active"]
pub type SwClrRdy2actR = crate::BitReader;
#[doc = "Field `SW_CLR_RDY2ACT` writer - System infrom WLAN ready to active"]
pub type SwClrRdy2actW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: enable HW auto response"]
    #[inline(always)]
    pub fn wl_hw_resp_fen(&self) -> WlHwRespFenR {
        WlHwRespFenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wl_req2act(&self) -> WlReq2actR {
        WlReq2actR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - System infrom WLAN ready to active"]
    #[inline(always)]
    pub fn wl_rdy2act(&self) -> WlRdy2actR {
        WlRdy2actR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sw_set_rdy2act(&self) -> SwSetRdy2actR {
        SwSetRdy2actR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - System infrom WLAN ready to active"]
    #[inline(always)]
    pub fn sw_clr_rdy2act(&self) -> SwClrRdy2actR {
        SwClrRdy2actR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable HW auto response"]
    #[inline(always)]
    pub fn wl_hw_resp_fen(&mut self) -> WlHwRespFenW<'_, RegWlanPwrctlSpec> {
        WlHwRespFenW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sw_set_rdy2act(&mut self) -> SwSetRdy2actW<'_, RegWlanPwrctlSpec> {
        SwSetRdy2actW::new(self, 16)
    }
    #[doc = "Bit 17 - System infrom WLAN ready to active"]
    #[inline(always)]
    pub fn sw_clr_rdy2act(&mut self) -> SwClrRdy2actW<'_, RegWlanPwrctlSpec> {
        SwClrRdy2actW::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wlan_pwrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wlan_pwrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlanPwrctlSpec;
impl crate::RegisterSpec for RegWlanPwrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wlan_pwrctl::R`](R) reader structure"]
impl crate::Readable for RegWlanPwrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wlan_pwrctl::W`](W) writer structure"]
impl crate::Writable for RegWlanPwrctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WLAN_PWRCTL to value 0x01"]
impl crate::Resettable for RegWlanPwrctlSpec {
    const RESET_VALUE: u32 = 0x01;
}
