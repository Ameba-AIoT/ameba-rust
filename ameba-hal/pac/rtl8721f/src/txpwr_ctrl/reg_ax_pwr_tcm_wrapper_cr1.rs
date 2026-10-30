#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR1` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr1Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR1` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr1Spec>;
#[doc = "Field `r_grp_3tx_path_en` reader - "]
pub type RGrp3txPathEnR = crate::FieldReader;
#[doc = "Field `r_grp_3tx_path_en` writer - "]
pub type RGrp3txPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_grp_4tx_path_en` reader - "]
pub type RGrp4txPathEnR = crate::FieldReader;
#[doc = "Field `r_grp_4tx_path_en` writer - "]
pub type RGrp4txPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_grp_5tx_path_en` reader - "]
pub type RGrp5txPathEnR = crate::FieldReader;
#[doc = "Field `r_grp_5tx_path_en` writer - "]
pub type RGrp5txPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_grp_6tx_path_en` reader - "]
pub type RGrp6txPathEnR = crate::FieldReader;
#[doc = "Field `r_grp_6tx_path_en` writer - "]
pub type RGrp6txPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_grp_7tx_path_en` reader - "]
pub type RGrp7txPathEnR = crate::FieldReader;
#[doc = "Field `r_grp_7tx_path_en` writer - "]
pub type RGrp7txPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_grp_8tx_path_en` reader - "]
pub type RGrp8txPathEnR = crate::FieldReader;
#[doc = "Field `r_grp_8tx_path_en` writer - "]
pub type RGrp8txPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_hang_proof_path_en` reader - "]
pub type RHangProofPathEnR = crate::FieldReader;
#[doc = "Field `r_hang_proof_path_en` writer - "]
pub type RHangProofPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_norm_1sts_path_en` reader - "]
pub type RNorm1stsPathEnR = crate::FieldReader;
#[doc = "Field `r_norm_1sts_path_en` writer - "]
pub type RNorm1stsPathEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn r_grp_3tx_path_en(&self) -> RGrp3txPathEnR {
        RGrp3txPathEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn r_grp_4tx_path_en(&self) -> RGrp4txPathEnR {
        RGrp4txPathEnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn r_grp_5tx_path_en(&self) -> RGrp5txPathEnR {
        RGrp5txPathEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn r_grp_6tx_path_en(&self) -> RGrp6txPathEnR {
        RGrp6txPathEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn r_grp_7tx_path_en(&self) -> RGrp7txPathEnR {
        RGrp7txPathEnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn r_grp_8tx_path_en(&self) -> RGrp8txPathEnR {
        RGrp8txPathEnR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn r_hang_proof_path_en(&self) -> RHangProofPathEnR {
        RHangProofPathEnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn r_norm_1sts_path_en(&self) -> RNorm1stsPathEnR {
        RNorm1stsPathEnR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn r_grp_3tx_path_en(&mut self) -> RGrp3txPathEnW<'_, RegAxPwrTcmWrapperCr1Spec> {
        RGrp3txPathEnW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn r_grp_4tx_path_en(&mut self) -> RGrp4txPathEnW<'_, RegAxPwrTcmWrapperCr1Spec> {
        RGrp4txPathEnW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn r_grp_5tx_path_en(&mut self) -> RGrp5txPathEnW<'_, RegAxPwrTcmWrapperCr1Spec> {
        RGrp5txPathEnW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn r_grp_6tx_path_en(&mut self) -> RGrp6txPathEnW<'_, RegAxPwrTcmWrapperCr1Spec> {
        RGrp6txPathEnW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn r_grp_7tx_path_en(&mut self) -> RGrp7txPathEnW<'_, RegAxPwrTcmWrapperCr1Spec> {
        RGrp7txPathEnW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn r_grp_8tx_path_en(&mut self) -> RGrp8txPathEnW<'_, RegAxPwrTcmWrapperCr1Spec> {
        RGrp8txPathEnW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn r_hang_proof_path_en(&mut self) -> RHangProofPathEnW<'_, RegAxPwrTcmWrapperCr1Spec> {
        RHangProofPathEnW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn r_norm_1sts_path_en(&mut self) -> RNorm1stsPathEnW<'_, RegAxPwrTcmWrapperCr1Spec> {
        RNorm1stsPathEnW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr1Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR1 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr1Spec {}
