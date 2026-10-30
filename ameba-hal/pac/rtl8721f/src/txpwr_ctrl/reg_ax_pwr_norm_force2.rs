#[doc = "Register `REG_AX_PWR_NORM_FORCE2` reader"]
pub type R = crate::R<RegAxPwrNormForce2Spec>;
#[doc = "Register `REG_AX_PWR_NORM_FORCE2` writer"]
pub type W = crate::W<RegAxPwrNormForce2Spec>;
#[doc = "Field `r_txpwr_ctrl_dbg_sel` reader - (RTL_only) dbg port address"]
pub type RTxpwrCtrlDbgSelR = crate::FieldReader;
#[doc = "Field `r_txpwr_ctrl_dbg_sel` writer - (RTL_only) dbg port address"]
pub type RTxpwrCtrlDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_force_out_CCA_PWR_TH` reader - (RTL_only) Force output to MAC"]
pub type RForceOutCcaPwrThR = crate::FieldReader;
#[doc = "Field `r_force_out_CCA_PWR_TH` writer - (RTL_only) Force output to MAC"]
pub type RForceOutCcaPwrThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_force_out_CCA_PWR_TH_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutCcaPwrThOnR = crate::BitReader;
#[doc = "Field `r_force_out_CCA_PWR_TH_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutCcaPwrThOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_CCA_PWR_TH_en` reader - (RTL_only) Force output to MAC"]
pub type RForceOutCcaPwrThEnR = crate::BitReader;
#[doc = "Field `r_force_out_CCA_PWR_TH_en` writer - (RTL_only) Force output to MAC"]
pub type RForceOutCcaPwrThEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_out_CCA_PWR_TH_en_on` reader - (RTL_only) Force output to MAC en"]
pub type RForceOutCcaPwrThEnOnR = crate::BitReader;
#[doc = "Field `r_force_out_CCA_PWR_TH_en_on` writer - (RTL_only) Force output to MAC en"]
pub type RForceOutCcaPwrThEnOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - (RTL_only) dbg port address"]
    #[inline(always)]
    pub fn r_txpwr_ctrl_dbg_sel(&self) -> RTxpwrCtrlDbgSelR {
        RTxpwrCtrlDbgSelR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_cca_pwr_th(&self) -> RForceOutCcaPwrThR {
        RForceOutCcaPwrThR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_cca_pwr_th_on(&self) -> RForceOutCcaPwrThOnR {
        RForceOutCcaPwrThOnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_cca_pwr_th_en(&self) -> RForceOutCcaPwrThEnR {
        RForceOutCcaPwrThEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_cca_pwr_th_en_on(&self) -> RForceOutCcaPwrThEnOnR {
        RForceOutCcaPwrThEnOnR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - (RTL_only) dbg port address"]
    #[inline(always)]
    pub fn r_txpwr_ctrl_dbg_sel(&mut self) -> RTxpwrCtrlDbgSelW<'_, RegAxPwrNormForce2Spec> {
        RTxpwrCtrlDbgSelW::new(self, 0)
    }
    #[doc = "Bits 8:15 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_cca_pwr_th(&mut self) -> RForceOutCcaPwrThW<'_, RegAxPwrNormForce2Spec> {
        RForceOutCcaPwrThW::new(self, 8)
    }
    #[doc = "Bit 16 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_cca_pwr_th_on(
        &mut self,
    ) -> RForceOutCcaPwrThOnW<'_, RegAxPwrNormForce2Spec> {
        RForceOutCcaPwrThOnW::new(self, 16)
    }
    #[doc = "Bit 17 - (RTL_only) Force output to MAC"]
    #[inline(always)]
    pub fn r_force_out_cca_pwr_th_en(
        &mut self,
    ) -> RForceOutCcaPwrThEnW<'_, RegAxPwrNormForce2Spec> {
        RForceOutCcaPwrThEnW::new(self, 17)
    }
    #[doc = "Bit 18 - (RTL_only) Force output to MAC en"]
    #[inline(always)]
    pub fn r_force_out_cca_pwr_th_en_on(
        &mut self,
    ) -> RForceOutCcaPwrThEnOnW<'_, RegAxPwrNormForce2Spec> {
        RForceOutCcaPwrThEnOnW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_norm_force2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_norm_force2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrNormForce2Spec;
impl crate::RegisterSpec for RegAxPwrNormForce2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_norm_force2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrNormForce2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_norm_force2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrNormForce2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_NORM_FORCE2 to value 0"]
impl crate::Resettable for RegAxPwrNormForce2Spec {}
