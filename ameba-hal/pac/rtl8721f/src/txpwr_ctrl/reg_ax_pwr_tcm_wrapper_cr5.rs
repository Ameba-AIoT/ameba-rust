#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR5` reader"]
pub type R = crate::R<RegAxPwrTcmWrapperCr5Spec>;
#[doc = "Register `REG_AX_PWR_TCM_WRAPPER_CR5` writer"]
pub type W = crate::W<RegAxPwrTcmWrapperCr5Spec>;
#[doc = "Field `r_force_macid_ant_map_b_val` reader - "]
pub type RForceMacidAntMapBValR = crate::FieldReader;
#[doc = "Field `r_force_macid_ant_map_b_val` writer - "]
pub type RForceMacidAntMapBValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_force_macid_ant_map_c_val` reader - "]
pub type RForceMacidAntMapCValR = crate::FieldReader;
#[doc = "Field `r_force_macid_ant_map_c_val` writer - "]
pub type RForceMacidAntMapCValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_force_macid_ant_map_d_val` reader - "]
pub type RForceMacidAntMapDValR = crate::FieldReader;
#[doc = "Field `r_force_macid_ant_map_d_val` writer - "]
pub type RForceMacidAntMapDValW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_1tx_path_map_a` reader - "]
pub type RGrp1txPathMapAR = crate::FieldReader;
#[doc = "Field `r_grp_1tx_path_map_a` writer - "]
pub type RGrp1txPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_1tx_path_map_b` reader - "]
pub type RGrp1txPathMapBR = crate::FieldReader;
#[doc = "Field `r_grp_1tx_path_map_b` writer - "]
pub type RGrp1txPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_1tx_path_map_c` reader - "]
pub type RGrp1txPathMapCR = crate::FieldReader;
#[doc = "Field `r_grp_1tx_path_map_c` writer - "]
pub type RGrp1txPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_1tx_path_map_d` reader - "]
pub type RGrp1txPathMapDR = crate::FieldReader;
#[doc = "Field `r_grp_1tx_path_map_d` writer - "]
pub type RGrp1txPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_2tx_path_map_a` reader - "]
pub type RGrp2txPathMapAR = crate::FieldReader;
#[doc = "Field `r_grp_2tx_path_map_a` writer - "]
pub type RGrp2txPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_2tx_path_map_b` reader - "]
pub type RGrp2txPathMapBR = crate::FieldReader;
#[doc = "Field `r_grp_2tx_path_map_b` writer - "]
pub type RGrp2txPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_2tx_path_map_c` reader - "]
pub type RGrp2txPathMapCR = crate::FieldReader;
#[doc = "Field `r_grp_2tx_path_map_c` writer - "]
pub type RGrp2txPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_2tx_path_map_d` reader - "]
pub type RGrp2txPathMapDR = crate::FieldReader;
#[doc = "Field `r_grp_2tx_path_map_d` writer - "]
pub type RGrp2txPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_3tx_path_map_a` reader - "]
pub type RGrp3txPathMapAR = crate::FieldReader;
#[doc = "Field `r_grp_3tx_path_map_a` writer - "]
pub type RGrp3txPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_3tx_path_map_b` reader - "]
pub type RGrp3txPathMapBR = crate::FieldReader;
#[doc = "Field `r_grp_3tx_path_map_b` writer - "]
pub type RGrp3txPathMapBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_3tx_path_map_c` reader - "]
pub type RGrp3txPathMapCR = crate::FieldReader;
#[doc = "Field `r_grp_3tx_path_map_c` writer - "]
pub type RGrp3txPathMapCW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_3tx_path_map_d` reader - "]
pub type RGrp3txPathMapDR = crate::FieldReader;
#[doc = "Field `r_grp_3tx_path_map_d` writer - "]
pub type RGrp3txPathMapDW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_grp_4tx_path_map_a` reader - "]
pub type RGrp4txPathMapAR = crate::FieldReader;
#[doc = "Field `r_grp_4tx_path_map_a` writer - "]
pub type RGrp4txPathMapAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn r_force_macid_ant_map_b_val(&self) -> RForceMacidAntMapBValR {
        RForceMacidAntMapBValR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn r_force_macid_ant_map_c_val(&self) -> RForceMacidAntMapCValR {
        RForceMacidAntMapCValR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn r_force_macid_ant_map_d_val(&self) -> RForceMacidAntMapDValR {
        RForceMacidAntMapDValR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn r_grp_1tx_path_map_a(&self) -> RGrp1txPathMapAR {
        RGrp1txPathMapAR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn r_grp_1tx_path_map_b(&self) -> RGrp1txPathMapBR {
        RGrp1txPathMapBR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn r_grp_1tx_path_map_c(&self) -> RGrp1txPathMapCR {
        RGrp1txPathMapCR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn r_grp_1tx_path_map_d(&self) -> RGrp1txPathMapDR {
        RGrp1txPathMapDR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn r_grp_2tx_path_map_a(&self) -> RGrp2txPathMapAR {
        RGrp2txPathMapAR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn r_grp_2tx_path_map_b(&self) -> RGrp2txPathMapBR {
        RGrp2txPathMapBR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn r_grp_2tx_path_map_c(&self) -> RGrp2txPathMapCR {
        RGrp2txPathMapCR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn r_grp_2tx_path_map_d(&self) -> RGrp2txPathMapDR {
        RGrp2txPathMapDR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn r_grp_3tx_path_map_a(&self) -> RGrp3txPathMapAR {
        RGrp3txPathMapAR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn r_grp_3tx_path_map_b(&self) -> RGrp3txPathMapBR {
        RGrp3txPathMapBR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn r_grp_3tx_path_map_c(&self) -> RGrp3txPathMapCR {
        RGrp3txPathMapCR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn r_grp_3tx_path_map_d(&self) -> RGrp3txPathMapDR {
        RGrp3txPathMapDR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_grp_4tx_path_map_a(&self) -> RGrp4txPathMapAR {
        RGrp4txPathMapAR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn r_force_macid_ant_map_b_val(
        &mut self,
    ) -> RForceMacidAntMapBValW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RForceMacidAntMapBValW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn r_force_macid_ant_map_c_val(
        &mut self,
    ) -> RForceMacidAntMapCValW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RForceMacidAntMapCValW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn r_force_macid_ant_map_d_val(
        &mut self,
    ) -> RForceMacidAntMapDValW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RForceMacidAntMapDValW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn r_grp_1tx_path_map_a(&mut self) -> RGrp1txPathMapAW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp1txPathMapAW::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn r_grp_1tx_path_map_b(&mut self) -> RGrp1txPathMapBW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp1txPathMapBW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn r_grp_1tx_path_map_c(&mut self) -> RGrp1txPathMapCW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp1txPathMapCW::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn r_grp_1tx_path_map_d(&mut self) -> RGrp1txPathMapDW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp1txPathMapDW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn r_grp_2tx_path_map_a(&mut self) -> RGrp2txPathMapAW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp2txPathMapAW::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn r_grp_2tx_path_map_b(&mut self) -> RGrp2txPathMapBW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp2txPathMapBW::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn r_grp_2tx_path_map_c(&mut self) -> RGrp2txPathMapCW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp2txPathMapCW::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn r_grp_2tx_path_map_d(&mut self) -> RGrp2txPathMapDW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp2txPathMapDW::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn r_grp_3tx_path_map_a(&mut self) -> RGrp3txPathMapAW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp3txPathMapAW::new(self, 22)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn r_grp_3tx_path_map_b(&mut self) -> RGrp3txPathMapBW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp3txPathMapBW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn r_grp_3tx_path_map_c(&mut self) -> RGrp3txPathMapCW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp3txPathMapCW::new(self, 26)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn r_grp_3tx_path_map_d(&mut self) -> RGrp3txPathMapDW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp3txPathMapDW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn r_grp_4tx_path_map_a(&mut self) -> RGrp4txPathMapAW<'_, RegAxPwrTcmWrapperCr5Spec> {
        RGrp4txPathMapAW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tcm_wrapper_cr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tcm_wrapper_cr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTcmWrapperCr5Spec;
impl crate::RegisterSpec for RegAxPwrTcmWrapperCr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tcm_wrapper_cr5::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTcmWrapperCr5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tcm_wrapper_cr5::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTcmWrapperCr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TCM_WRAPPER_CR5 to value 0"]
impl crate::Resettable for RegAxPwrTcmWrapperCr5Spec {}
