#[doc = "Register `REG_AX_PWR_MACID_REG` reader"]
pub type R = crate::R<RegAxPwrMacidRegSpec>;
#[doc = "Register `REG_AX_PWR_MACID_REG` writer"]
pub type W = crate::W<RegAxPwrMacidRegSpec>;
#[doc = "Field `r_txpwr_reg0` reader - S(8.1)pwr by macid when Pwr_mode=2 from MAC"]
pub type RTxpwrReg0R = crate::FieldReader;
#[doc = "Field `r_txpwr_reg0` writer - S(8.1)pwr by macid when Pwr_mode=2 from MAC"]
pub type RTxpwrReg0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_txpwr_reg1` reader - S(8.1)pwr by macid when Pwr_mode=3 from MAC"]
pub type RTxpwrReg1R = crate::FieldReader;
#[doc = "Field `r_txpwr_reg1` writer - S(8.1)pwr by macid when Pwr_mode=3 from MAC"]
pub type RTxpwrReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_txpwr_reg2` reader - S(8.1)pwr by macid when Pwr_mode=4 from MAC"]
pub type RTxpwrReg2R = crate::FieldReader;
#[doc = "Field `r_txpwr_reg2` writer - S(8.1)pwr by macid when Pwr_mode=4 from MAC"]
pub type RTxpwrReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_txpwr_reg3` reader - S(8.1)pwr by macid when Pwr_mode=5 from MAC"]
pub type RTxpwrReg3R = crate::FieldReader;
#[doc = "Field `r_txpwr_reg3` writer - S(8.1)pwr by macid when Pwr_mode=5 from MAC"]
pub type RTxpwrReg3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - S(8.1)pwr by macid when Pwr_mode=2 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg0(&self) -> RTxpwrReg0R {
        RTxpwrReg0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - S(8.1)pwr by macid when Pwr_mode=3 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg1(&self) -> RTxpwrReg1R {
        RTxpwrReg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - S(8.1)pwr by macid when Pwr_mode=4 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg2(&self) -> RTxpwrReg2R {
        RTxpwrReg2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - S(8.1)pwr by macid when Pwr_mode=5 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg3(&self) -> RTxpwrReg3R {
        RTxpwrReg3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - S(8.1)pwr by macid when Pwr_mode=2 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg0(&mut self) -> RTxpwrReg0W<'_, RegAxPwrMacidRegSpec> {
        RTxpwrReg0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - S(8.1)pwr by macid when Pwr_mode=3 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg1(&mut self) -> RTxpwrReg1W<'_, RegAxPwrMacidRegSpec> {
        RTxpwrReg1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - S(8.1)pwr by macid when Pwr_mode=4 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg2(&mut self) -> RTxpwrReg2W<'_, RegAxPwrMacidRegSpec> {
        RTxpwrReg2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - S(8.1)pwr by macid when Pwr_mode=5 from MAC"]
    #[inline(always)]
    pub fn r_txpwr_reg3(&mut self) -> RTxpwrReg3W<'_, RegAxPwrMacidRegSpec> {
        RTxpwrReg3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_macid_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_macid_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrMacidRegSpec;
impl crate::RegisterSpec for RegAxPwrMacidRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_macid_reg::R`](R) reader structure"]
impl crate::Readable for RegAxPwrMacidRegSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_macid_reg::W`](W) writer structure"]
impl crate::Writable for RegAxPwrMacidRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_MACID_REG to value 0"]
impl crate::Resettable for RegAxPwrMacidRegSpec {}
