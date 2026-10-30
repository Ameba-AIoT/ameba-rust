#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR7` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr7Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR7` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr7Spec>;
#[doc = "Field `r_grp_8tx_path_map_b` reader - "]
pub type RGrp8txPathMapBR = crate::FieldReader;
#[doc = "Field `r_grp_8tx_path_map_b` writer - "]
pub type RGrp8txPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_8tx_path_map_c` reader - "]
pub type RGrp8txPathMapCR = crate::FieldReader;
#[doc = "Field `r_grp_8tx_path_map_c` writer - "]
pub type RGrp8txPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_8tx_path_map_d` reader - "]
pub type RGrp8txPathMapDR = crate::FieldReader;
#[doc = "Field `r_grp_8tx_path_map_d` writer - "]
pub type RGrp8txPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_hang_proof_path_map_a` reader - "]
pub type RHangProofPathMapAR = crate::FieldReader;
#[doc = "Field `r_hang_proof_path_map_a` writer - "]
pub type RHangProofPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_hang_proof_path_map_b` reader - "]
pub type RHangProofPathMapBR = crate::FieldReader;
#[doc = "Field `r_hang_proof_path_map_b` writer - "]
pub type RHangProofPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_hang_proof_path_map_c` reader - "]
pub type RHangProofPathMapCR = crate::FieldReader;
#[doc = "Field `r_hang_proof_path_map_c` writer - "]
pub type RHangProofPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_hang_proof_path_map_d` reader - "]
pub type RHangProofPathMapDR = crate::FieldReader;
#[doc = "Field `r_hang_proof_path_map_d` writer - "]
pub type RHangProofPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_norm_1sts_path_map_a` reader - "]
pub type RNorm1stsPathMapAR = crate::FieldReader;
#[doc = "Field `r_norm_1sts_path_map_a` writer - "]
pub type RNorm1stsPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_norm_1sts_path_map_b` reader - "]
pub type RNorm1stsPathMapBR = crate::FieldReader;
#[doc = "Field `r_norm_1sts_path_map_b` writer - "]
pub type RNorm1stsPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_norm_1sts_path_map_c` reader - "]
pub type RNorm1stsPathMapCR = crate::FieldReader;
#[doc = "Field `r_norm_1sts_path_map_c` writer - "]
pub type RNorm1stsPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_norm_1sts_path_map_d` reader - "]
pub type RNorm1stsPathMapDR = crate::FieldReader;
#[doc = "Field `r_norm_1sts_path_map_d` writer - "]
pub type RNorm1stsPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_norm_2sts_path_map_a` reader - "]
pub type RNorm2stsPathMapAR = crate::FieldReader;
#[doc = "Field `r_norm_2sts_path_map_a` writer - "]
pub type RNorm2stsPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_norm_2sts_path_map_b` reader - "]
pub type RNorm2stsPathMapBR = crate::FieldReader;
#[doc = "Field `r_norm_2sts_path_map_b` writer - "]
pub type RNorm2stsPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_norm_2sts_path_map_c` reader - "]
pub type RNorm2stsPathMapCR = crate::FieldReader;
#[doc = "Field `r_norm_2sts_path_map_c` writer - "]
pub type RNorm2stsPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_norm_2sts_path_map_d` reader - "]
pub type RNorm2stsPathMapDR = crate::FieldReader;
#[doc = "Field `r_norm_2sts_path_map_d` writer - "]
pub type RNorm2stsPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_resp_1sts_path_map_a` reader - "]
pub type RResp1stsPathMapAR = crate::FieldReader;
#[doc = "Field `r_resp_1sts_path_map_a` writer - "]
pub type RResp1stsPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn r_grp_8tx_path_map_b(&self) -> RGrp8txPathMapBR {
        RGrp8txPathMapBR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn r_grp_8tx_path_map_c(&self) -> RGrp8txPathMapCR {
        RGrp8txPathMapCR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn r_grp_8tx_path_map_d(&self) -> RGrp8txPathMapDR {
        RGrp8txPathMapDR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn r_hang_proof_path_map_a(&self) -> RHangProofPathMapAR {
        RHangProofPathMapAR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn r_hang_proof_path_map_b(&self) -> RHangProofPathMapBR {
        RHangProofPathMapBR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn r_hang_proof_path_map_c(&self) -> RHangProofPathMapCR {
        RHangProofPathMapCR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn r_hang_proof_path_map_d(&self) -> RHangProofPathMapDR {
        RHangProofPathMapDR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn r_norm_1sts_path_map_a(&self) -> RNorm1stsPathMapAR {
        RNorm1stsPathMapAR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn r_norm_1sts_path_map_b(&self) -> RNorm1stsPathMapBR {
        RNorm1stsPathMapBR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn r_norm_1sts_path_map_c(&self) -> RNorm1stsPathMapCR {
        RNorm1stsPathMapCR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn r_norm_1sts_path_map_d(&self) -> RNorm1stsPathMapDR {
        RNorm1stsPathMapDR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn r_norm_2sts_path_map_a(&self) -> RNorm2stsPathMapAR {
        RNorm2stsPathMapAR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn r_norm_2sts_path_map_b(&self) -> RNorm2stsPathMapBR {
        RNorm2stsPathMapBR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn r_norm_2sts_path_map_c(&self) -> RNorm2stsPathMapCR {
        RNorm2stsPathMapCR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn r_norm_2sts_path_map_d(&self) -> RNorm2stsPathMapDR {
        RNorm2stsPathMapDR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_resp_1sts_path_map_a(&self) -> RResp1stsPathMapAR {
        RResp1stsPathMapAR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn r_grp_8tx_path_map_b(&mut self) -> RGrp8txPathMapBW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RGrp8txPathMapBW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn r_grp_8tx_path_map_c(&mut self) -> RGrp8txPathMapCW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RGrp8txPathMapCW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn r_grp_8tx_path_map_d(&mut self) -> RGrp8txPathMapDW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RGrp8txPathMapDW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn r_hang_proof_path_map_a(
        &mut self,
    ) -> RHangProofPathMapAW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RHangProofPathMapAW::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn r_hang_proof_path_map_b(
        &mut self,
    ) -> RHangProofPathMapBW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RHangProofPathMapBW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn r_hang_proof_path_map_c(
        &mut self,
    ) -> RHangProofPathMapCW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RHangProofPathMapCW::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn r_hang_proof_path_map_d(
        &mut self,
    ) -> RHangProofPathMapDW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RHangProofPathMapDW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn r_norm_1sts_path_map_a(&mut self) -> RNorm1stsPathMapAW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RNorm1stsPathMapAW::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn r_norm_1sts_path_map_b(&mut self) -> RNorm1stsPathMapBW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RNorm1stsPathMapBW::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn r_norm_1sts_path_map_c(&mut self) -> RNorm1stsPathMapCW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RNorm1stsPathMapCW::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn r_norm_1sts_path_map_d(&mut self) -> RNorm1stsPathMapDW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RNorm1stsPathMapDW::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn r_norm_2sts_path_map_a(&mut self) -> RNorm2stsPathMapAW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RNorm2stsPathMapAW::new(self, 22)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn r_norm_2sts_path_map_b(&mut self) -> RNorm2stsPathMapBW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RNorm2stsPathMapBW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn r_norm_2sts_path_map_c(&mut self) -> RNorm2stsPathMapCW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RNorm2stsPathMapCW::new(self, 26)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn r_norm_2sts_path_map_d(&mut self) -> RNorm2stsPathMapDW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RNorm2stsPathMapDW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_resp_1sts_path_map_a(&mut self) -> RResp1stsPathMapAW<'_, RegAxPwrTcmWrapperCr7Spec> {
        RResp1stsPathMapAW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr7Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr7::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr7::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR7 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr7Spec {}
