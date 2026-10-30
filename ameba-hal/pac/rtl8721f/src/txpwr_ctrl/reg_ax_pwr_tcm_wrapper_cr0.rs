#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR0` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr0Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR0` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr0Spec>;
#[doc = "Field `r_usr_list_tbl_ra` reader - "]
pub type RUsrListTblRaR = crate::FieldReader;
#[doc = "Field `r_usr_list_tbl_ra` writer - "]
pub type RUsrListTblRaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_bf_1sts_path_en` reader - "]
pub type RBf1stsPathEnR = crate::FieldReader;
#[doc = "Field `r_bf_1sts_path_en` writer - "]
pub type RBf1stsPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_bf_2sts_path_en` reader - "]
pub type RBf2stsPathEnR = crate::FieldReader;
#[doc = "Field `r_bf_2sts_path_en` writer - "]
pub type RBf2stsPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_fc_path_en` reader - "]
pub type RFcPathEnR = crate::FieldReader;
#[doc = "Field `r_fc_path_en` writer - "]
pub type RFcPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_force_macid_ant_path_en_val` reader - "]
pub type RForceMacidAntPathEnValR = crate::FieldReader;
#[doc = "Field `r_force_macid_ant_path_en_val` writer - "]
pub type RForceMacidAntPathEnValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_grp_1tx_path_en` reader - "]
pub type RGrp1txPathEnR = crate::FieldReader;
#[doc = "Field `r_grp_1tx_path_en` writer - "]
pub type RGrp1txPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_grp_2tx_path_en` reader - "]
pub type RGrp2txPathEnR = crate::FieldReader;
#[doc = "Field `r_grp_2tx_path_en` writer - "]
pub type RGrp2txPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_bf_1sts_ant_sel_a` reader - "]
pub type RBf1stsAntSelAR = crate::BitReader;
#[doc = "Field `r_bf_1sts_ant_sel_a` writer - "]
pub type RBf1stsAntSelAW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn r_usr_list_tbl_ra(&self) -> RUsrListTblRaR {
        RUsrListTblRaR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn r_bf_1sts_path_en(&self) -> RBf1stsPathEnR {
        RBf1stsPathEnR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn r_bf_2sts_path_en(&self) -> RBf2stsPathEnR {
        RBf2stsPathEnR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn r_fc_path_en(&self) -> RFcPathEnR {
        RFcPathEnR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn r_force_macid_ant_path_en_val(&self) -> RForceMacidAntPathEnValR {
        RForceMacidAntPathEnValR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26"]
    #[inline(always)]
    pub fn r_grp_1tx_path_en(&self) -> RGrp1txPathEnR {
        RGrp1txPathEnR::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:30"]
    #[inline(always)]
    pub fn r_grp_2tx_path_en(&self) -> RGrp2txPathEnR {
        RGrp2txPathEnR::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn r_bf_1sts_ant_sel_a(&self) -> RBf1stsAntSelAR {
        RBf1stsAntSelAR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn r_usr_list_tbl_ra(&mut self) -> RUsrListTblRaW<'_, RegAxPwrTcmWrapperCr0Spec> {
        RUsrListTblRaW::new(self, 0)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn r_bf_1sts_path_en(&mut self) -> RBf1stsPathEnW<'_, RegAxPwrTcmWrapperCr0Spec> {
        RBf1stsPathEnW::new(self, 7)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn r_bf_2sts_path_en(&mut self) -> RBf2stsPathEnW<'_, RegAxPwrTcmWrapperCr0Spec> {
        RBf2stsPathEnW::new(self, 11)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn r_fc_path_en(&mut self) -> RFcPathEnW<'_, RegAxPwrTcmWrapperCr0Spec> {
        RFcPathEnW::new(self, 15)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn r_force_macid_ant_path_en_val(
        &mut self,
    ) -> RForceMacidAntPathEnValW<'_, RegAxPwrTcmWrapperCr0Spec> {
        RForceMacidAntPathEnValW::new(self, 19)
    }
    #[doc = "Bits 23:26"]
    #[inline(always)]
    pub fn r_grp_1tx_path_en(&mut self) -> RGrp1txPathEnW<'_, RegAxPwrTcmWrapperCr0Spec> {
        RGrp1txPathEnW::new(self, 23)
    }
    #[doc = "Bits 27:30"]
    #[inline(always)]
    pub fn r_grp_2tx_path_en(&mut self) -> RGrp2txPathEnW<'_, RegAxPwrTcmWrapperCr0Spec> {
        RGrp2txPathEnW::new(self, 27)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn r_bf_1sts_ant_sel_a(&mut self) -> RBf1stsAntSelAW<'_, RegAxPwrTcmWrapperCr0Spec> {
        RBf1stsAntSelAW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr0Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR0 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr0Spec {}
