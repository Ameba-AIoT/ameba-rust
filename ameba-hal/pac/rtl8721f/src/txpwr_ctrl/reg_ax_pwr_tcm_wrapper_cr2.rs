#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR2` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr2Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR2` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr2Spec>;
#[doc = "Field `r_norm_2sts_path_en` reader - "]
pub type RNorm2stsPathEnR = crate::FieldReader;
#[doc = "Field `r_norm_2sts_path_en` writer - "]
pub type RNorm2stsPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_resp_1sts_path_en` reader - "]
pub type RResp1stsPathEnR = crate::FieldReader;
#[doc = "Field `r_resp_1sts_path_en` writer - "]
pub type RResp1stsPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_resp_2sts_path_en` reader - "]
pub type RResp2stsPathEnR = crate::FieldReader;
#[doc = "Field `r_resp_2sts_path_en` writer - "]
pub type RResp2stsPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_bf_1sts_spatial_expan_num` reader - "]
pub type RBf1stsSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_bf_1sts_spatial_expan_num` writer - "]
pub type RBf1stsSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_bf_2sts_spatial_expan_num` reader - "]
pub type RBf2stsSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_bf_2sts_spatial_expan_num` writer - "]
pub type RBf2stsSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_fc_ntx` reader - "]
pub type RFcNtxR = crate::FieldReader;
#[doc = "Field `r_fc_ntx` writer - "]
pub type RFcNtxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_fc_spatial_expan_num` reader - "]
pub type RFcSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_fc_spatial_expan_num` writer - "]
pub type RFcSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_force_macid_ant_spatial_expan_num_val` reader - "]
pub type RForceMacidAntSpatialExpanNumValR = crate::FieldReader;
#[doc = "Field `r_force_macid_ant_spatial_expan_num_val` writer - "]
pub type RForceMacidAntSpatialExpanNumValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_force_par_mode_val` reader - "]
pub type RForceParModeValR = crate::FieldReader;
#[doc = "Field `r_force_par_mode_val` writer - "]
pub type RForceParModeValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_bf_1sts_path_map_a` reader - "]
pub type RBf1stsPathMapAR = crate::FieldReader;
#[doc = "Field `r_bf_1sts_path_map_a` writer - "]
pub type RBf1stsPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn r_norm_2sts_path_en(&self) -> RNorm2stsPathEnR {
        RNorm2stsPathEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn r_resp_1sts_path_en(&self) -> RResp1stsPathEnR {
        RResp1stsPathEnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn r_resp_2sts_path_en(&self) -> RResp2stsPathEnR {
        RResp2stsPathEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn r_bf_1sts_spatial_expan_num(&self) -> RBf1stsSpatialExpanNumR {
        RBf1stsSpatialExpanNumR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn r_bf_2sts_spatial_expan_num(&self) -> RBf2stsSpatialExpanNumR {
        RBf2stsSpatialExpanNumR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn r_fc_ntx(&self) -> RFcNtxR {
        RFcNtxR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn r_fc_spatial_expan_num(&self) -> RFcSpatialExpanNumR {
        RFcSpatialExpanNumR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn r_force_macid_ant_spatial_expan_num_val(&self) -> RForceMacidAntSpatialExpanNumValR {
        RForceMacidAntSpatialExpanNumValR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn r_force_par_mode_val(&self) -> RForceParModeValR {
        RForceParModeValR::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_bf_1sts_path_map_a(&self) -> RBf1stsPathMapAR {
        RBf1stsPathMapAR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn r_norm_2sts_path_en(&mut self) -> RNorm2stsPathEnW<'_, RegAxPwrTcmWrapperCr2Spec> {
        RNorm2stsPathEnW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn r_resp_1sts_path_en(&mut self) -> RResp1stsPathEnW<'_, RegAxPwrTcmWrapperCr2Spec> {
        RResp1stsPathEnW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn r_resp_2sts_path_en(&mut self) -> RResp2stsPathEnW<'_, RegAxPwrTcmWrapperCr2Spec> {
        RResp2stsPathEnW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn r_bf_1sts_spatial_expan_num(
        &mut self,
    ) -> RBf1stsSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr2Spec> {
        RBf1stsSpatialExpanNumW::new(self, 12)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn r_bf_2sts_spatial_expan_num(
        &mut self,
    ) -> RBf2stsSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr2Spec> {
        RBf2stsSpatialExpanNumW::new(self, 15)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn r_fc_ntx(&mut self) -> RFcNtxW<'_, RegAxPwrTcmWrapperCr2Spec> {
        RFcNtxW::new(self, 18)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn r_fc_spatial_expan_num(&mut self) -> RFcSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr2Spec> {
        RFcSpatialExpanNumW::new(self, 21)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn r_force_macid_ant_spatial_expan_num_val(
        &mut self,
    ) -> RForceMacidAntSpatialExpanNumValW<'_, RegAxPwrTcmWrapperCr2Spec> {
        RForceMacidAntSpatialExpanNumValW::new(self, 24)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn r_force_par_mode_val(&mut self) -> RForceParModeValW<'_, RegAxPwrTcmWrapperCr2Spec> {
        RForceParModeValW::new(self, 27)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_bf_1sts_path_map_a(&mut self) -> RBf1stsPathMapAW<'_, RegAxPwrTcmWrapperCr2Spec> {
        RBf1stsPathMapAW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr2Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR2 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr2Spec {}
