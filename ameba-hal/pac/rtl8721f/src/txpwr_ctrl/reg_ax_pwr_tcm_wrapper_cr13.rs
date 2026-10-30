#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR13` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr13Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR13` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr13Spec>;
#[doc = "Field `r_force_pwrru_ch20_with_data_val` reader - "]
pub type RForcePwrruCh20WithDataValR = crate::FieldReader;
#[doc = "Field `r_force_pwrru_ch20_with_data_val` writer - "]
pub type RForcePwrruCh20WithDataValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_force_pwrcom_macid_val` reader - "]
pub type RForcePwrcomMacidValR = crate::FieldReader;
#[doc = "Field `r_force_pwrcom_macid_val` writer - "]
pub type RForcePwrcomMacidValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_force_pwrul_tar_rssi_val` reader - "]
pub type RForcePwrulTarRssiValR = crate::FieldReader;
#[doc = "Field `r_force_pwrul_tar_rssi_val` writer - "]
pub type RForcePwrulTarRssiValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_force_pwrsr_txpwr_toler_val` reader - "]
pub type RForcePwrsrTxpwrTolerValR = crate::FieldReader;
#[doc = "Field `r_force_pwrsr_txpwr_toler_val` writer - "]
pub type RForcePwrsrTxpwrTolerValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_dbg_ant_sel` reader - "]
pub type RDbgAntSelR = crate::FieldReader;
#[doc = "Field `r_dbg_ant_sel` writer - "]
pub type RDbgAntSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_force_pwrru_ch20_with_data_val(&self) -> RForcePwrruCh20WithDataValR {
        RForcePwrruCh20WithDataValR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn r_force_pwrcom_macid_val(&self) -> RForcePwrcomMacidValR {
        RForcePwrcomMacidValR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:21"]
    #[inline(always)]
    pub fn r_force_pwrul_tar_rssi_val(&self) -> RForcePwrulTarRssiValR {
        RForcePwrulTarRssiValR::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn r_force_pwrsr_txpwr_toler_val(&self) -> RForcePwrsrTxpwrTolerValR {
        RForcePwrsrTxpwrTolerValR::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn r_dbg_ant_sel(&self) -> RDbgAntSelR {
        RDbgAntSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_force_pwrru_ch20_with_data_val(
        &mut self,
    ) -> RForcePwrruCh20WithDataValW<'_, RegAxPwrTcmWrapperCr13Spec> {
        RForcePwrruCh20WithDataValW::new(self, 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn r_force_pwrcom_macid_val(
        &mut self,
    ) -> RForcePwrcomMacidValW<'_, RegAxPwrTcmWrapperCr13Spec> {
        RForcePwrcomMacidValW::new(self, 8)
    }
    #[doc = "Bits 15:21"]
    #[inline(always)]
    pub fn r_force_pwrul_tar_rssi_val(
        &mut self,
    ) -> RForcePwrulTarRssiValW<'_, RegAxPwrTcmWrapperCr13Spec> {
        RForcePwrulTarRssiValW::new(self, 15)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn r_force_pwrsr_txpwr_toler_val(
        &mut self,
    ) -> RForcePwrsrTxpwrTolerValW<'_, RegAxPwrTcmWrapperCr13Spec> {
        RForcePwrsrTxpwrTolerValW::new(self, 22)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn r_dbg_ant_sel(&mut self) -> RDbgAntSelW<'_, RegAxPwrTcmWrapperCr13Spec> {
        RDbgAntSelW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr13Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr13::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr13::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR13 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr13Spec {}
