#[doc = "Register `REG_AX_PWR_MACID_REG2` reader"]
pub type R = crate::R<RegAxPwrMacidReg2Spec>;
#[doc = "Register `REG_AX_PWR_MACID_REG2` writer"]
pub type W = crate::W<RegAxPwrMacidReg2Spec>;
#[doc = "Field `r_txpwr_reg4` reader - S(8.1)pwr by macid when Pwr_mode=6 from MAC"]
pub type RTxpwrReg4R = crate::FieldReader;
#[doc = "Field `r_txpwr_reg4` writer - S(8.1)pwr by macid when Pwr_mode=6 from MAC"]
pub type RTxpwrReg4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_txpwr_reg5` reader - S(8.1)pwr by macid when Pwr_mode=7 from MAC"]
pub type RTxpwrReg5R = crate::FieldReader;
#[doc = "Field `r_txpwr_reg5` writer - S(8.1)pwr by macid when Pwr_mode=7 from MAC"]
pub type RTxpwrReg5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_txpwr_reg0_en` reader - pwr by macid en when Pwr_mode=2 from MAC"]
pub type RTxpwrReg0EnR = crate::BitReader;
#[doc = "Field `r_txpwr_reg0_en` writer - pwr by macid en when Pwr_mode=2 from MAC"]
pub type RTxpwrReg0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_reg1_en` reader - pwr by macid en when Pwr_mode=3 from MAC"]
pub type RTxpwrReg1EnR = crate::BitReader;
#[doc = "Field `r_txpwr_reg1_en` writer - pwr by macid en when Pwr_mode=3 from MAC"]
pub type RTxpwrReg1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_reg2_en` reader - pwr by macid en when pwr_mode=4 from MAC"]
pub type RTxpwrReg2EnR = crate::BitReader;
#[doc = "Field `r_txpwr_reg2_en` writer - pwr by macid en when pwr_mode=4 from MAC"]
pub type RTxpwrReg2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_reg3_en` reader - pwr by macid en when Pwr_mode=5 from MAC"]
pub type RTxpwrReg3EnR = crate::BitReader;
#[doc = "Field `r_txpwr_reg3_en` writer - pwr by macid en when Pwr_mode=5 from MAC"]
pub type RTxpwrReg3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_reg4_en` reader - pwr by macid en when Pwr_mode=6 from MAC"]
pub type RTxpwrReg4EnR = crate::BitReader;
#[doc = "Field `r_txpwr_reg4_en` writer - pwr by macid en when Pwr_mode=6 from MAC"]
pub type RTxpwrReg4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_reg5_en` reader - pwr by macid en when Pwr_mode=7 from MAC"]
pub type RTxpwrReg5EnR = crate::BitReader;
#[doc = "Field `r_txpwr_reg5_en` writer - pwr by macid en when Pwr_mode=7 from MAC"]
pub type RTxpwrReg5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_txpwr_by_macid_tbl_ra` reader - (RTL_only) Pwr_by_macid indirect read addr"]
pub type RTxpwrByMacidTblRaR = crate::FieldReader;
#[doc = "Field `r_txpwr_by_macid_tbl_ra` writer - (RTL_only) Pwr_by_macid indirect read addr"]
pub type RTxpwrByMacidTblRaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_txpwr_by_macid_tbl_rd` reader - (RTL_only) 1:Pwr_by_macid indirect read"]
pub type RTxpwrByMacidTblRdR = crate::BitReader;
#[doc = "Field `r_txpwr_by_macid_tbl_rd` writer - (RTL_only) 1:Pwr_by_macid indirect read"]
pub type RTxpwrByMacidTblRdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - S(8.1)pwr by macid when Pwr_mode=6 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg4(&self) -> RTxpwrReg4R {
        RTxpwrReg4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - S(8.1)pwr by macid when Pwr_mode=7 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg5(&self) -> RTxpwrReg5R {
        RTxpwrReg5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - pwr by macid en when Pwr_mode=2 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg0_en(&self) -> RTxpwrReg0EnR {
        RTxpwrReg0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - pwr by macid en when Pwr_mode=3 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg1_en(&self) -> RTxpwrReg1EnR {
        RTxpwrReg1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - pwr by macid en when pwr_mode=4 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg2_en(&self) -> RTxpwrReg2EnR {
        RTxpwrReg2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - pwr by macid en when Pwr_mode=5 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg3_en(&self) -> RTxpwrReg3EnR {
        RTxpwrReg3EnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - pwr by macid en when Pwr_mode=6 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg4_en(&self) -> RTxpwrReg4EnR {
        RTxpwrReg4EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - pwr by macid en when Pwr_mode=7 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg5_en(&self) -> RTxpwrReg5EnR {
        RTxpwrReg5EnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:30 - (RTL_only) Pwr_by_macid indirect read addr"]
    #[inline(always)]
    pub fn r_txpwr_by_macid_tbl_ra(&self) -> RTxpwrByMacidTblRaR {
        RTxpwrByMacidTblRaR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - (RTL_only) 1:Pwr_by_macid indirect read"]
    #[inline(always)]
    pub fn r_txpwr_by_macid_tbl_rd(&self) -> RTxpwrByMacidTblRdR {
        RTxpwrByMacidTblRdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - S(8.1)pwr by macid when Pwr_mode=6 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg4(&mut self) -> RTxpwrReg4W<'_, RegAxPwrMacidReg2Spec> {
        RTxpwrReg4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - S(8.1)pwr by macid when Pwr_mode=7 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg5(&mut self) -> RTxpwrReg5W<'_, RegAxPwrMacidReg2Spec> {
        RTxpwrReg5W::new(self, 8)
    }
    #[doc = "Bit 16 - pwr by macid en when Pwr_mode=2 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg0_en(&mut self) -> RTxpwrReg0EnW<'_, RegAxPwrMacidReg2Spec> {
        RTxpwrReg0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - pwr by macid en when Pwr_mode=3 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg1_en(&mut self) -> RTxpwrReg1EnW<'_, RegAxPwrMacidReg2Spec> {
        RTxpwrReg1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - pwr by macid en when pwr_mode=4 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg2_en(&mut self) -> RTxpwrReg2EnW<'_, RegAxPwrMacidReg2Spec> {
        RTxpwrReg2EnW::new(self, 18)
    }
    #[doc = "Bit 19 - pwr by macid en when Pwr_mode=5 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg3_en(&mut self) -> RTxpwrReg3EnW<'_, RegAxPwrMacidReg2Spec> {
        RTxpwrReg3EnW::new(self, 19)
    }
    #[doc = "Bit 20 - pwr by macid en when Pwr_mode=6 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg4_en(&mut self) -> RTxpwrReg4EnW<'_, RegAxPwrMacidReg2Spec> {
        RTxpwrReg4EnW::new(self, 20)
    }
    #[doc = "Bit 21 - pwr by macid en when Pwr_mode=7 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg5_en(&mut self) -> RTxpwrReg5EnW<'_, RegAxPwrMacidReg2Spec> {
        RTxpwrReg5EnW::new(self, 21)
    }
    #[doc = "Bits 24:30 - (RTL_only) Pwr_by_macid indirect read addr"]
    #[inline(always)]
    pub fn r_txpwr_by_macid_tbl_ra(&mut self) -> RTxpwrByMacidTblRaW<'_, RegAxPwrMacidReg2Spec> {
        RTxpwrByMacidTblRaW::new(self, 24)
    }
    #[doc = "Bit 31 - (RTL_only) 1:Pwr_by_macid indirect read"]
    #[inline(always)]
    pub fn r_txpwr_by_macid_tbl_rd(&mut self) -> RTxpwrByMacidTblRdW<'_, RegAxPwrMacidReg2Spec> {
        RTxpwrByMacidTblRdW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_macid_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_macid_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrMacidReg2Spec;
impl crate::RegisterSpec for RegAxPwrMacidReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_macid_reg2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrMacidReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_macid_reg2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrMacidReg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_MACID_REG2 to value 0"]
impl crate::Resettable for RegAxPwrMacidReg2Spec {}
