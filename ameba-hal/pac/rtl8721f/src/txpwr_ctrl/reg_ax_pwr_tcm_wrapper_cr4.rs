#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR4` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr4Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR4` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr4Spec>;
#[doc = "Field `r_norm_2sts_spatial_expan_num` reader - "]
pub type RNorm2stsSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_norm_2sts_spatial_expan_num` writer - "]
pub type RNorm2stsSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_resp_1sts_spatial_expan_num` reader - "]
pub type RResp1stsSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_resp_1sts_spatial_expan_num` writer - "]
pub type RResp1stsSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_resp_2sts_spatial_expan_num` reader - "]
pub type RResp2stsSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_resp_2sts_spatial_expan_num` writer - "]
pub type RResp2stsSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_bf_1sts_path_map_c` reader - "]
pub type RBf1stsPathMapCR = crate::FieldReader;
#[doc = "Field `r_bf_1sts_path_map_c` writer - "]
pub type RBf1stsPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_bf_1sts_path_map_d` reader - "]
pub type RBf1stsPathMapDR = crate::FieldReader;
#[doc = "Field `r_bf_1sts_path_map_d` writer - "]
pub type RBf1stsPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_bf_2sts_path_map_a` reader - "]
pub type RBf2stsPathMapAR = crate::FieldReader;
#[doc = "Field `r_bf_2sts_path_map_a` writer - "]
pub type RBf2stsPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_bf_2sts_path_map_b` reader - "]
pub type RBf2stsPathMapBR = crate::FieldReader;
#[doc = "Field `r_bf_2sts_path_map_b` writer - "]
pub type RBf2stsPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_bf_2sts_path_map_c` reader - "]
pub type RBf2stsPathMapCR = crate::FieldReader;
#[doc = "Field `r_bf_2sts_path_map_c` writer - "]
pub type RBf2stsPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_bf_2sts_path_map_d` reader - "]
pub type RBf2stsPathMapDR = crate::FieldReader;
#[doc = "Field `r_bf_2sts_path_map_d` writer - "]
pub type RBf2stsPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_fc_path_map_a` reader - "]
pub type RFcPathMapAR = crate::FieldReader;
#[doc = "Field `r_fc_path_map_a` writer - "]
pub type RFcPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_fc_path_map_b` reader - "]
pub type RFcPathMapBR = crate::FieldReader;
#[doc = "Field `r_fc_path_map_b` writer - "]
pub type RFcPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_fc_path_map_c` reader - "]
pub type RFcPathMapCR = crate::FieldReader;
#[doc = "Field `r_fc_path_map_c` writer - "]
pub type RFcPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_fc_path_map_d` reader - "]
pub type RFcPathMapDR = crate::FieldReader;
#[doc = "Field `r_fc_path_map_d` writer - "]
pub type RFcPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_force_macid_ant_map_a_val` reader - "]
pub type RForceMacidAntMapAValR = crate::FieldReader;
#[doc = "Field `r_force_macid_ant_map_a_val` writer - "]
pub type RForceMacidAntMapAValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_bf_1sts_ant_sel_b` reader - "]
pub type RBf1stsAntSelBR = crate::BitReader;
#[doc = "Field `r_bf_1sts_ant_sel_b` writer - "]
pub type RBf1stsAntSelBW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn r_norm_2sts_spatial_expan_num(&self) -> RNorm2stsSpatialExpanNumR {
        RNorm2stsSpatialExpanNumR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn r_resp_1sts_spatial_expan_num(&self) -> RResp1stsSpatialExpanNumR {
        RResp1stsSpatialExpanNumR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn r_resp_2sts_spatial_expan_num(&self) -> RResp2stsSpatialExpanNumR {
        RResp2stsSpatialExpanNumR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn r_bf_1sts_path_map_c(&self) -> RBf1stsPathMapCR {
        RBf1stsPathMapCR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn r_bf_1sts_path_map_d(&self) -> RBf1stsPathMapDR {
        RBf1stsPathMapDR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn r_bf_2sts_path_map_a(&self) -> RBf2stsPathMapAR {
        RBf2stsPathMapAR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn r_bf_2sts_path_map_b(&self) -> RBf2stsPathMapBR {
        RBf2stsPathMapBR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn r_bf_2sts_path_map_c(&self) -> RBf2stsPathMapCR {
        RBf2stsPathMapCR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn r_bf_2sts_path_map_d(&self) -> RBf2stsPathMapDR {
        RBf2stsPathMapDR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn r_fc_path_map_a(&self) -> RFcPathMapAR {
        RFcPathMapAR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn r_fc_path_map_b(&self) -> RFcPathMapBR {
        RFcPathMapBR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn r_fc_path_map_c(&self) -> RFcPathMapCR {
        RFcPathMapCR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn r_fc_path_map_d(&self) -> RFcPathMapDR {
        RFcPathMapDR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn r_force_macid_ant_map_a_val(&self) -> RForceMacidAntMapAValR {
        RForceMacidAntMapAValR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn r_bf_1sts_ant_sel_b(&self) -> RBf1stsAntSelBR {
        RBf1stsAntSelBR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn r_norm_2sts_spatial_expan_num(
        &mut self,
    ) -> RNorm2stsSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RNorm2stsSpatialExpanNumW::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn r_resp_1sts_spatial_expan_num(
        &mut self,
    ) -> RResp1stsSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RResp1stsSpatialExpanNumW::new(self, 3)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn r_resp_2sts_spatial_expan_num(
        &mut self,
    ) -> RResp2stsSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RResp2stsSpatialExpanNumW::new(self, 6)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn r_bf_1sts_path_map_c(&mut self) -> RBf1stsPathMapCW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RBf1stsPathMapCW::new(self, 9)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn r_bf_1sts_path_map_d(&mut self) -> RBf1stsPathMapDW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RBf1stsPathMapDW::new(self, 11)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn r_bf_2sts_path_map_a(&mut self) -> RBf2stsPathMapAW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RBf2stsPathMapAW::new(self, 13)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn r_bf_2sts_path_map_b(&mut self) -> RBf2stsPathMapBW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RBf2stsPathMapBW::new(self, 15)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn r_bf_2sts_path_map_c(&mut self) -> RBf2stsPathMapCW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RBf2stsPathMapCW::new(self, 17)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn r_bf_2sts_path_map_d(&mut self) -> RBf2stsPathMapDW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RBf2stsPathMapDW::new(self, 19)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn r_fc_path_map_a(&mut self) -> RFcPathMapAW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RFcPathMapAW::new(self, 21)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn r_fc_path_map_b(&mut self) -> RFcPathMapBW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RFcPathMapBW::new(self, 23)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn r_fc_path_map_c(&mut self) -> RFcPathMapCW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RFcPathMapCW::new(self, 25)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn r_fc_path_map_d(&mut self) -> RFcPathMapDW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RFcPathMapDW::new(self, 27)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn r_force_macid_ant_map_a_val(
        &mut self,
    ) -> RForceMacidAntMapAValW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RForceMacidAntMapAValW::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn r_bf_1sts_ant_sel_b(&mut self) -> RBf1stsAntSelBW<'_, RegAxPwrTcmWrapperCr4Spec> {
        RBf1stsAntSelBW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr4Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr4::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr4::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR4 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr4Spec {}
