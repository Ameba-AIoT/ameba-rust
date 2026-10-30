#[doc = "Register `REG_AX_PWR_SR_CTRL1` reader"]
pub type R = crate::R<RegAxPwrSrCtrl1Spec>;
#[doc = "Register `REG_AX_PWR_SR_CTRL1` writer"]
pub type W = crate::W<RegAxPwrSrCtrl1Spec>;
#[doc = "Field `r_TXPWR_ref` reader - SR txpwr ref(txpwr_mcs diff)"]
pub type RTxpwrRefR = crate::FieldReader;
#[doc = "Field `r_TXPWR_ref` writer - SR txpwr ref(txpwr_mcs diff)"]
pub type RTxpwrRefW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_MCS_th_cck` reader - SR 0~3"]
pub type RMcsThCckR = crate::FieldReader;
#[doc = "Field `r_MCS_th_cck` writer - SR 0~3"]
pub type RMcsThCckW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS_th_legacy` reader - SR 4~11"]
pub type RMcsThLegacyR = crate::FieldReader;
#[doc = "Field `r_MCS_th_legacy` writer - SR 4~11"]
pub type RMcsThLegacyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS_th_ht_mod8` reader - SR MCS0~MCS7"]
pub type RMcsThHtMod8R = crate::FieldReader;
#[doc = "Field `r_MCS_th_ht_mod8` writer - SR MCS0~MCS7"]
pub type RMcsThHtMod8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS_th_vht` reader - SR MCS0~MCS11"]
pub type RMcsThVhtR = crate::FieldReader;
#[doc = "Field `r_MCS_th_vht` writer - SR MCS0~MCS11"]
pub type RMcsThVhtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_MCS_th_he` reader - SR MCS0~MCS11"]
pub type RMcsThHeR = crate::FieldReader;
#[doc = "Field `r_MCS_th_he` writer - SR MCS0~MCS11"]
pub type RMcsThHeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - SR txpwr ref(txpwr_mcs diff)"]
    #[inline(always)]
    pub fn r_txpwr_ref(&self) -> RTxpwrRefR {
        RTxpwrRefR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - SR 0~3"]
    #[inline(always)]
    pub fn r_mcs_th_cck(&self) -> RMcsThCckR {
        RMcsThCckR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SR 4~11"]
    #[inline(always)]
    pub fn r_mcs_th_legacy(&self) -> RMcsThLegacyR {
        RMcsThLegacyR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SR MCS0~MCS7"]
    #[inline(always)]
    pub fn r_mcs_th_ht_mod8(&self) -> RMcsThHtMod8R {
        RMcsThHtMod8R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SR MCS0~MCS11"]
    #[inline(always)]
    pub fn r_mcs_th_vht(&self) -> RMcsThVhtR {
        RMcsThVhtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SR MCS0~MCS11"]
    #[inline(always)]
    pub fn r_mcs_th_he(&self) -> RMcsThHeR {
        RMcsThHeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SR txpwr ref(txpwr_mcs diff)"]
    #[inline(always)]
    pub fn r_txpwr_ref(&mut self) -> RTxpwrRefW<'_, RegAxPwrSrCtrl1Spec> {
        RTxpwrRefW::new(self, 0)
    }
    #[doc = "Bits 8:11 - SR 0~3"]
    #[inline(always)]
    pub fn r_mcs_th_cck(&mut self) -> RMcsThCckW<'_, RegAxPwrSrCtrl1Spec> {
        RMcsThCckW::new(self, 8)
    }
    #[doc = "Bits 12:15 - SR 4~11"]
    #[inline(always)]
    pub fn r_mcs_th_legacy(&mut self) -> RMcsThLegacyW<'_, RegAxPwrSrCtrl1Spec> {
        RMcsThLegacyW::new(self, 12)
    }
    #[doc = "Bits 16:19 - SR MCS0~MCS7"]
    #[inline(always)]
    pub fn r_mcs_th_ht_mod8(&mut self) -> RMcsThHtMod8W<'_, RegAxPwrSrCtrl1Spec> {
        RMcsThHtMod8W::new(self, 16)
    }
    #[doc = "Bits 20:23 - SR MCS0~MCS11"]
    #[inline(always)]
    pub fn r_mcs_th_vht(&mut self) -> RMcsThVhtW<'_, RegAxPwrSrCtrl1Spec> {
        RMcsThVhtW::new(self, 20)
    }
    #[doc = "Bits 24:27 - SR MCS0~MCS11"]
    #[inline(always)]
    pub fn r_mcs_th_he(&mut self) -> RMcsThHeW<'_, RegAxPwrSrCtrl1Spec> {
        RMcsThHeW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrCtrl1Spec;
impl crate::RegisterSpec for RegAxPwrSrCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_CTRL1 to value 0x0bb7_7315"]
impl crate::Resettable for RegAxPwrSrCtrl1Spec {
    const RESET_VALUE: u32 = 0x0bb7_7315;
}
