#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR6` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr6Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR6` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr6Spec>;
#[doc = "Field `r_grp_4tx_path_map_b` reader - "]
pub type RGrp4txPathMapBR = crate::FieldReader;
#[doc = "Field `r_grp_4tx_path_map_b` writer - "]
pub type RGrp4txPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_4tx_path_map_c` reader - "]
pub type RGrp4txPathMapCR = crate::FieldReader;
#[doc = "Field `r_grp_4tx_path_map_c` writer - "]
pub type RGrp4txPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_4tx_path_map_d` reader - "]
pub type RGrp4txPathMapDR = crate::FieldReader;
#[doc = "Field `r_grp_4tx_path_map_d` writer - "]
pub type RGrp4txPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_5tx_path_map_a` reader - "]
pub type RGrp5txPathMapAR = crate::FieldReader;
#[doc = "Field `r_grp_5tx_path_map_a` writer - "]
pub type RGrp5txPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_5tx_path_map_b` reader - "]
pub type RGrp5txPathMapBR = crate::FieldReader;
#[doc = "Field `r_grp_5tx_path_map_b` writer - "]
pub type RGrp5txPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_5tx_path_map_c` reader - "]
pub type RGrp5txPathMapCR = crate::FieldReader;
#[doc = "Field `r_grp_5tx_path_map_c` writer - "]
pub type RGrp5txPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_5tx_path_map_d` reader - "]
pub type RGrp5txPathMapDR = crate::FieldReader;
#[doc = "Field `r_grp_5tx_path_map_d` writer - "]
pub type RGrp5txPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_6tx_path_map_a` reader - "]
pub type RGrp6txPathMapAR = crate::FieldReader;
#[doc = "Field `r_grp_6tx_path_map_a` writer - "]
pub type RGrp6txPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_6tx_path_map_b` reader - "]
pub type RGrp6txPathMapBR = crate::FieldReader;
#[doc = "Field `r_grp_6tx_path_map_b` writer - "]
pub type RGrp6txPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_6tx_path_map_c` reader - "]
pub type RGrp6txPathMapCR = crate::FieldReader;
#[doc = "Field `r_grp_6tx_path_map_c` writer - "]
pub type RGrp6txPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_6tx_path_map_d` reader - "]
pub type RGrp6txPathMapDR = crate::FieldReader;
#[doc = "Field `r_grp_6tx_path_map_d` writer - "]
pub type RGrp6txPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_7tx_path_map_a` reader - "]
pub type RGrp7txPathMapAR = crate::FieldReader;
#[doc = "Field `r_grp_7tx_path_map_a` writer - "]
pub type RGrp7txPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_7tx_path_map_b` reader - "]
pub type RGrp7txPathMapBR = crate::FieldReader;
#[doc = "Field `r_grp_7tx_path_map_b` writer - "]
pub type RGrp7txPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_7tx_path_map_c` reader - "]
pub type RGrp7txPathMapCR = crate::FieldReader;
#[doc = "Field `r_grp_7tx_path_map_c` writer - "]
pub type RGrp7txPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_7tx_path_map_d` reader - "]
pub type RGrp7txPathMapDR = crate::FieldReader;
#[doc = "Field `r_grp_7tx_path_map_d` writer - "]
pub type RGrp7txPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_8tx_path_map_a` reader - "]
pub type RGrp8txPathMapAR = crate::FieldReader;
#[doc = "Field `r_grp_8tx_path_map_a` writer - "]
pub type RGrp8txPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn r_grp_4tx_path_map_b(&self) -> RGrp4txPathMapBR {
        RGrp4txPathMapBR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn r_grp_4tx_path_map_c(&self) -> RGrp4txPathMapCR {
        RGrp4txPathMapCR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn r_grp_4tx_path_map_d(&self) -> RGrp4txPathMapDR {
        RGrp4txPathMapDR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn r_grp_5tx_path_map_a(&self) -> RGrp5txPathMapAR {
        RGrp5txPathMapAR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn r_grp_5tx_path_map_b(&self) -> RGrp5txPathMapBR {
        RGrp5txPathMapBR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn r_grp_5tx_path_map_c(&self) -> RGrp5txPathMapCR {
        RGrp5txPathMapCR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn r_grp_5tx_path_map_d(&self) -> RGrp5txPathMapDR {
        RGrp5txPathMapDR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn r_grp_6tx_path_map_a(&self) -> RGrp6txPathMapAR {
        RGrp6txPathMapAR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn r_grp_6tx_path_map_b(&self) -> RGrp6txPathMapBR {
        RGrp6txPathMapBR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn r_grp_6tx_path_map_c(&self) -> RGrp6txPathMapCR {
        RGrp6txPathMapCR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn r_grp_6tx_path_map_d(&self) -> RGrp6txPathMapDR {
        RGrp6txPathMapDR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn r_grp_7tx_path_map_a(&self) -> RGrp7txPathMapAR {
        RGrp7txPathMapAR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn r_grp_7tx_path_map_b(&self) -> RGrp7txPathMapBR {
        RGrp7txPathMapBR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn r_grp_7tx_path_map_c(&self) -> RGrp7txPathMapCR {
        RGrp7txPathMapCR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn r_grp_7tx_path_map_d(&self) -> RGrp7txPathMapDR {
        RGrp7txPathMapDR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_grp_8tx_path_map_a(&self) -> RGrp8txPathMapAR {
        RGrp8txPathMapAR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn r_grp_4tx_path_map_b(&mut self) -> RGrp4txPathMapBW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp4txPathMapBW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn r_grp_4tx_path_map_c(&mut self) -> RGrp4txPathMapCW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp4txPathMapCW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn r_grp_4tx_path_map_d(&mut self) -> RGrp4txPathMapDW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp4txPathMapDW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn r_grp_5tx_path_map_a(&mut self) -> RGrp5txPathMapAW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp5txPathMapAW::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn r_grp_5tx_path_map_b(&mut self) -> RGrp5txPathMapBW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp5txPathMapBW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn r_grp_5tx_path_map_c(&mut self) -> RGrp5txPathMapCW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp5txPathMapCW::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn r_grp_5tx_path_map_d(&mut self) -> RGrp5txPathMapDW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp5txPathMapDW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn r_grp_6tx_path_map_a(&mut self) -> RGrp6txPathMapAW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp6txPathMapAW::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn r_grp_6tx_path_map_b(&mut self) -> RGrp6txPathMapBW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp6txPathMapBW::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn r_grp_6tx_path_map_c(&mut self) -> RGrp6txPathMapCW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp6txPathMapCW::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn r_grp_6tx_path_map_d(&mut self) -> RGrp6txPathMapDW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp6txPathMapDW::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn r_grp_7tx_path_map_a(&mut self) -> RGrp7txPathMapAW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp7txPathMapAW::new(self, 22)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn r_grp_7tx_path_map_b(&mut self) -> RGrp7txPathMapBW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp7txPathMapBW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn r_grp_7tx_path_map_c(&mut self) -> RGrp7txPathMapCW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp7txPathMapCW::new(self, 26)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn r_grp_7tx_path_map_d(&mut self) -> RGrp7txPathMapDW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp7txPathMapDW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_grp_8tx_path_map_a(&mut self) -> RGrp8txPathMapAW<'_, RegAxPwrTcmWrapperCr6Spec> {
        RGrp8txPathMapAW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr6Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr6::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr6::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR6 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr6Spec {}
