#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR3` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr3Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR3` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr3Spec>;
#[doc = "Field `r_grp_1tx_spatial_expan_num` reader - "]
pub type RGrp1txSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_grp_1tx_spatial_expan_num` writer - "]
pub type RGrp1txSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_grp_2tx_spatial_expan_num` reader - "]
pub type RGrp2txSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_grp_2tx_spatial_expan_num` writer - "]
pub type RGrp2txSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_grp_3tx_spatial_expan_num` reader - "]
pub type RGrp3txSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_grp_3tx_spatial_expan_num` writer - "]
pub type RGrp3txSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_grp_4tx_spatial_expan_num` reader - "]
pub type RGrp4txSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_grp_4tx_spatial_expan_num` writer - "]
pub type RGrp4txSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_grp_5tx_spatial_expan_num` reader - "]
pub type RGrp5txSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_grp_5tx_spatial_expan_num` writer - "]
pub type RGrp5txSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_grp_6tx_spatial_expan_num` reader - "]
pub type RGrp6txSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_grp_6tx_spatial_expan_num` writer - "]
pub type RGrp6txSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_grp_7tx_spatial_expan_num` reader - "]
pub type RGrp7txSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_grp_7tx_spatial_expan_num` writer - "]
pub type RGrp7txSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_grp_8tx_spatial_expan_num` reader - "]
pub type RGrp8txSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_grp_8tx_spatial_expan_num` writer - "]
pub type RGrp8txSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_hang_proof_spatial_expan_num` reader - "]
pub type RHangProofSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_hang_proof_spatial_expan_num` writer - "]
pub type RHangProofSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_norm_1sts_spatial_expan_num` reader - "]
pub type RNorm1stsSpatialExpanNumR = crate::FieldReader;
#[doc = "Field `r_norm_1sts_spatial_expan_num` writer - "]
pub type RNorm1stsSpatialExpanNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_bf_1sts_path_map_b` reader - "]
pub type RBf1stsPathMapBR = crate::FieldReader;
#[doc = "Field `r_bf_1sts_path_map_b` writer - "]
pub type RBf1stsPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn r_grp_1tx_spatial_expan_num(&self) -> RGrp1txSpatialExpanNumR {
        RGrp1txSpatialExpanNumR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn r_grp_2tx_spatial_expan_num(&self) -> RGrp2txSpatialExpanNumR {
        RGrp2txSpatialExpanNumR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn r_grp_3tx_spatial_expan_num(&self) -> RGrp3txSpatialExpanNumR {
        RGrp3txSpatialExpanNumR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn r_grp_4tx_spatial_expan_num(&self) -> RGrp4txSpatialExpanNumR {
        RGrp4txSpatialExpanNumR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn r_grp_5tx_spatial_expan_num(&self) -> RGrp5txSpatialExpanNumR {
        RGrp5txSpatialExpanNumR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn r_grp_6tx_spatial_expan_num(&self) -> RGrp6txSpatialExpanNumR {
        RGrp6txSpatialExpanNumR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn r_grp_7tx_spatial_expan_num(&self) -> RGrp7txSpatialExpanNumR {
        RGrp7txSpatialExpanNumR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn r_grp_8tx_spatial_expan_num(&self) -> RGrp8txSpatialExpanNumR {
        RGrp8txSpatialExpanNumR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn r_hang_proof_spatial_expan_num(&self) -> RHangProofSpatialExpanNumR {
        RHangProofSpatialExpanNumR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn r_norm_1sts_spatial_expan_num(&self) -> RNorm1stsSpatialExpanNumR {
        RNorm1stsSpatialExpanNumR::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_bf_1sts_path_map_b(&self) -> RBf1stsPathMapBR {
        RBf1stsPathMapBR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn r_grp_1tx_spatial_expan_num(
        &mut self,
    ) -> RGrp1txSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RGrp1txSpatialExpanNumW::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn r_grp_2tx_spatial_expan_num(
        &mut self,
    ) -> RGrp2txSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RGrp2txSpatialExpanNumW::new(self, 3)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn r_grp_3tx_spatial_expan_num(
        &mut self,
    ) -> RGrp3txSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RGrp3txSpatialExpanNumW::new(self, 6)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn r_grp_4tx_spatial_expan_num(
        &mut self,
    ) -> RGrp4txSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RGrp4txSpatialExpanNumW::new(self, 9)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn r_grp_5tx_spatial_expan_num(
        &mut self,
    ) -> RGrp5txSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RGrp5txSpatialExpanNumW::new(self, 12)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn r_grp_6tx_spatial_expan_num(
        &mut self,
    ) -> RGrp6txSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RGrp6txSpatialExpanNumW::new(self, 15)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn r_grp_7tx_spatial_expan_num(
        &mut self,
    ) -> RGrp7txSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RGrp7txSpatialExpanNumW::new(self, 18)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn r_grp_8tx_spatial_expan_num(
        &mut self,
    ) -> RGrp8txSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RGrp8txSpatialExpanNumW::new(self, 21)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn r_hang_proof_spatial_expan_num(
        &mut self,
    ) -> RHangProofSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RHangProofSpatialExpanNumW::new(self, 24)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn r_norm_1sts_spatial_expan_num(
        &mut self,
    ) -> RNorm1stsSpatialExpanNumW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RNorm1stsSpatialExpanNumW::new(self, 27)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_bf_1sts_path_map_b(&mut self) -> RBf1stsPathMapBW<'_, RegAxPwrTcmWrapperCr3Spec> {
        RBf1stsPathMapBW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr3Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr3::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr3::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR3 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr3Spec {}
