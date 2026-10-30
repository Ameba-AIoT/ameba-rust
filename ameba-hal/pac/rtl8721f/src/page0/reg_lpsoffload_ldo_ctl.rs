#[doc = "Register `REG_LPSOFFLOAD_LDO_CTL` reader"]
pub type R = crate::R<RegLpsoffloadLdoCtlSpec>;
#[doc = "Register `REG_LPSOFFLOAD_LDO_CTL` writer"]
pub type W = crate::W<RegLpsoffloadLdoCtlSpec>;
#[doc = "Field `r_rxbcn_lpsoffload_en` reader - when set 1, enable lpsoffload function"]
pub type RRxbcnLpsoffloadEnR = crate::BitReader;
#[doc = "Field `r_rxbcn_lpsoffload_en` writer - when set 1, enable lpsoffload function"]
pub type RRxbcnLpsoffloadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_hw_auto_enter_lps_en` reader - when set 1, enable hw auto enter lps when triggered by lpsoffload"]
pub type RHwAutoEnterLpsEnR = crate::BitReader;
#[doc = "Field `r_hw_auto_enter_lps_en` writer - when set 1, enable hw auto enter lps when triggered by lpsoffload"]
pub type RHwAutoEnterLpsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_bypass_hioe_chn_on` reader - when set 1, bypass request to CHON of hioe"]
pub type RBypassHioeChnOnR = crate::BitReader;
#[doc = "Field `r_bypass_hioe_chn_on` writer - when set 1, bypass request to CHON of hioe"]
pub type RBypassHioeChnOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_bypass_hioe_chn_off` reader - when set 1, bypass request to CHOFF of hioe"]
pub type RBypassHioeChnOffR = crate::BitReader;
#[doc = "Field `r_bypass_hioe_chn_off` writer - when set 1, bypass request to CHOFF of hioe"]
pub type RBypassHioeChnOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_bypass_hioe_chn_bcn_mode` reader - when set 1, bypass request to CHN_BCN_MODE of hioe"]
pub type RBypassHioeChnBcnModeR = crate::BitReader;
#[doc = "Field `r_bypass_hioe_chn_bcn_mode` writer - when set 1, bypass request to CHN_BCN_MODE of hioe"]
pub type RBypassHioeChnBcnModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_wl_mldo_mode_act` reader - analog ldo mode lps option 00:off 01:sleep 10:normal-dummy load 11:normal-no dummy load"]
pub type RWlMldoModeActR = crate::FieldReader;
#[doc = "Field `r_wl_mldo_mode_act` writer - analog ldo mode lps option 00:off 01:sleep 10:normal-dummy load 11:normal-no dummy load"]
pub type RWlMldoModeActW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_wl_mldo_mode_lps` reader - analog ldo mode lps option 00:off 01:sleep 10:normal-dummy load 11:normal-no dummy load"]
pub type RWlMldoModeLpsR = crate::FieldReader;
#[doc = "Field `r_wl_mldo_mode_lps` writer - analog ldo mode lps option 00:off 01:sleep 10:normal-dummy load 11:normal-no dummy load"]
pub type RWlMldoModeLpsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wl_mldo_mode_req` reader - analog ldo mode lps option 00:off 01:sleep 10:normal-dummy load 11:normal-no dummy load"]
pub type WlMldoModeReqR = crate::FieldReader;
#[doc = "Field `r_wl_mldo_mode_req_trg` writer - trigger switch LDO mode according to 0x2C\\[17:16\\] when wlan is active, write 1 to generate one pulse"]
pub type RWlMldoModeReqTrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_wl_mldo_timer` reader - wait analog ldo stable time 00:16us 01:32us 10:64us 11:128us"]
pub type RWlMldoTimerR = crate::FieldReader;
#[doc = "Field `r_wl_mldo_timer` writer - wait analog ldo stable time 00:16us 01:32us 10:64us 11:128us"]
pub type RWlMldoTimerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - when set 1, enable lpsoffload function"]
    #[inline(always)]
    pub fn r_rxbcn_lpsoffload_en(&self) -> RRxbcnLpsoffloadEnR {
        RRxbcnLpsoffloadEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when set 1, enable hw auto enter lps when triggered by lpsoffload"]
    #[inline(always)]
    pub fn r_hw_auto_enter_lps_en(&self) -> RHwAutoEnterLpsEnR {
        RHwAutoEnterLpsEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - when set 1, bypass request to CHON of hioe"]
    #[inline(always)]
    pub fn r_bypass_hioe_chn_on(&self) -> RBypassHioeChnOnR {
        RBypassHioeChnOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - when set 1, bypass request to CHOFF of hioe"]
    #[inline(always)]
    pub fn r_bypass_hioe_chn_off(&self) -> RBypassHioeChnOffR {
        RBypassHioeChnOffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - when set 1, bypass request to CHN_BCN_MODE of hioe"]
    #[inline(always)]
    pub fn r_bypass_hioe_chn_bcn_mode(&self) -> RBypassHioeChnBcnModeR {
        RBypassHioeChnBcnModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:17 - analog ldo mode lps option 00:off 01:sleep 10:normal-dummy load 11:normal-no dummy load"]
    #[inline(always)]
    pub fn r_wl_mldo_mode_act(&self) -> RWlMldoModeActR {
        RWlMldoModeActR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - analog ldo mode lps option 00:off 01:sleep 10:normal-dummy load 11:normal-no dummy load"]
    #[inline(always)]
    pub fn r_wl_mldo_mode_lps(&self) -> RWlMldoModeLpsR {
        RWlMldoModeLpsR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - analog ldo mode lps option 00:off 01:sleep 10:normal-dummy load 11:normal-no dummy load"]
    #[inline(always)]
    pub fn wl_mldo_mode_req(&self) -> WlMldoModeReqR {
        WlMldoModeReqR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 23:24 - wait analog ldo stable time 00:16us 01:32us 10:64us 11:128us"]
    #[inline(always)]
    pub fn r_wl_mldo_timer(&self) -> RWlMldoTimerR {
        RWlMldoTimerR::new(((self.bits >> 23) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - when set 1, enable lpsoffload function"]
    #[inline(always)]
    pub fn r_rxbcn_lpsoffload_en(&mut self) -> RRxbcnLpsoffloadEnW<'_, RegLpsoffloadLdoCtlSpec> {
        RRxbcnLpsoffloadEnW::new(self, 0)
    }
    #[doc = "Bit 1 - when set 1, enable hw auto enter lps when triggered by lpsoffload"]
    #[inline(always)]
    pub fn r_hw_auto_enter_lps_en(&mut self) -> RHwAutoEnterLpsEnW<'_, RegLpsoffloadLdoCtlSpec> {
        RHwAutoEnterLpsEnW::new(self, 1)
    }
    #[doc = "Bit 2 - when set 1, bypass request to CHON of hioe"]
    #[inline(always)]
    pub fn r_bypass_hioe_chn_on(&mut self) -> RBypassHioeChnOnW<'_, RegLpsoffloadLdoCtlSpec> {
        RBypassHioeChnOnW::new(self, 2)
    }
    #[doc = "Bit 3 - when set 1, bypass request to CHOFF of hioe"]
    #[inline(always)]
    pub fn r_bypass_hioe_chn_off(&mut self) -> RBypassHioeChnOffW<'_, RegLpsoffloadLdoCtlSpec> {
        RBypassHioeChnOffW::new(self, 3)
    }
    #[doc = "Bit 4 - when set 1, bypass request to CHN_BCN_MODE of hioe"]
    #[inline(always)]
    pub fn r_bypass_hioe_chn_bcn_mode(
        &mut self,
    ) -> RBypassHioeChnBcnModeW<'_, RegLpsoffloadLdoCtlSpec> {
        RBypassHioeChnBcnModeW::new(self, 4)
    }
    #[doc = "Bits 16:17 - analog ldo mode lps option 00:off 01:sleep 10:normal-dummy load 11:normal-no dummy load"]
    #[inline(always)]
    pub fn r_wl_mldo_mode_act(&mut self) -> RWlMldoModeActW<'_, RegLpsoffloadLdoCtlSpec> {
        RWlMldoModeActW::new(self, 16)
    }
    #[doc = "Bits 18:19 - analog ldo mode lps option 00:off 01:sleep 10:normal-dummy load 11:normal-no dummy load"]
    #[inline(always)]
    pub fn r_wl_mldo_mode_lps(&mut self) -> RWlMldoModeLpsW<'_, RegLpsoffloadLdoCtlSpec> {
        RWlMldoModeLpsW::new(self, 18)
    }
    #[doc = "Bit 22 - trigger switch LDO mode according to 0x2C\\[17:16\\] when wlan is active, write 1 to generate one pulse"]
    #[inline(always)]
    pub fn r_wl_mldo_mode_req_trg(&mut self) -> RWlMldoModeReqTrgW<'_, RegLpsoffloadLdoCtlSpec> {
        RWlMldoModeReqTrgW::new(self, 22)
    }
    #[doc = "Bits 23:24 - wait analog ldo stable time 00:16us 01:32us 10:64us 11:128us"]
    #[inline(always)]
    pub fn r_wl_mldo_timer(&mut self) -> RWlMldoTimerW<'_, RegLpsoffloadLdoCtlSpec> {
        RWlMldoTimerW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_lpsoffload_ldo_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_lpsoffload_ldo_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLpsoffloadLdoCtlSpec;
impl crate::RegisterSpec for RegLpsoffloadLdoCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_lpsoffload_ldo_ctl::R`](R) reader structure"]
impl crate::Readable for RegLpsoffloadLdoCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_lpsoffload_ldo_ctl::W`](W) writer structure"]
impl crate::Writable for RegLpsoffloadLdoCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LPSOFFLOAD_LDO_CTL to value 0x0003_001c"]
impl crate::Resettable for RegLpsoffloadLdoCtlSpec {
    const RESET_VALUE: u32 = 0x0003_001c;
}
