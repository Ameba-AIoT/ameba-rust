#[doc = "Register `REG_CKD_SYS_PLL_GRP1` reader"]
pub type R = crate::R<RegCkdSysPllGrp1Spec>;
#[doc = "Register `REG_CKD_SYS_PLL_GRP1` writer"]
pub type W = crate::W<RegCkdSysPllGrp1Spec>;
#[doc = "Field `ckd_sys_pll_vo` reader - sys_pll divider for vo/panel 0: div1 1: div2 ... Note: max 200M DIV_MIN:0"]
pub type CkdSysPllVoR = crate::FieldReader;
#[doc = "Field `ckd_sys_pll_vo` writer - sys_pll divider for vo/panel 0: div1 1: div2 ... Note: max 200M DIV_MIN:0"]
pub type CkdSysPllVoW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ckd_sys_pll_gmac` reader - sys_pll divider for gmac Note: output must fix to 50M. when sys_pll is 400M , this value default is 7"]
pub type CkdSysPllGmacR = crate::FieldReader;
#[doc = "Field `ckd_sys_pll_gmac` writer - sys_pll divider for gmac Note: output must fix to 50M. when sys_pll is 400M , this value default is 7"]
pub type CkdSysPllGmacW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_sys_pll_i2s` reader - sys_pll divider for i2s clk 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllI2sR = crate::FieldReader;
#[doc = "Field `ckd_sys_pll_i2s` writer - sys_pll divider for i2s clk 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllI2sW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ckd_sys_pll_sdh` reader - sys_pll divider for sdh clk 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllSdhR = crate::FieldReader;
#[doc = "Field `ckd_sys_pll_sdh` writer - sys_pll divider for sdh clk 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllSdhW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - sys_pll divider for vo/panel 0: div1 1: div2 ... Note: max 200M DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_vo(&self) -> CkdSysPllVoR {
        CkdSysPllVoR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - sys_pll divider for gmac Note: output must fix to 50M. when sys_pll is 400M , this value default is 7"]
    #[inline(always)]
    pub fn ckd_sys_pll_gmac(&self) -> CkdSysPllGmacR {
        CkdSysPllGmacR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - sys_pll divider for i2s clk 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_i2s(&self) -> CkdSysPllI2sR {
        CkdSysPllI2sR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - sys_pll divider for sdh clk 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_sdh(&self) -> CkdSysPllSdhR {
        CkdSysPllSdhR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - sys_pll divider for vo/panel 0: div1 1: div2 ... Note: max 200M DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_vo(&mut self) -> CkdSysPllVoW<'_, RegCkdSysPllGrp1Spec> {
        CkdSysPllVoW::new(self, 0)
    }
    #[doc = "Bits 8:11 - sys_pll divider for gmac Note: output must fix to 50M. when sys_pll is 400M , this value default is 7"]
    #[inline(always)]
    pub fn ckd_sys_pll_gmac(&mut self) -> CkdSysPllGmacW<'_, RegCkdSysPllGrp1Spec> {
        CkdSysPllGmacW::new(self, 8)
    }
    #[doc = "Bits 16:20 - sys_pll divider for i2s clk 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_i2s(&mut self) -> CkdSysPllI2sW<'_, RegCkdSysPllGrp1Spec> {
        CkdSysPllI2sW::new(self, 16)
    }
    #[doc = "Bits 24:26 - sys_pll divider for sdh clk 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_sdh(&mut self) -> CkdSysPllSdhW<'_, RegCkdSysPllGrp1Spec> {
        CkdSysPllSdhW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ckd_sys_pll_grp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ckd_sys_pll_grp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkdSysPllGrp1Spec;
impl crate::RegisterSpec for RegCkdSysPllGrp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ckd_sys_pll_grp1::R`](R) reader structure"]
impl crate::Readable for RegCkdSysPllGrp1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ckd_sys_pll_grp1::W`](W) writer structure"]
impl crate::Writable for RegCkdSysPllGrp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKD_SYS_PLL_GRP1 to value 0x0303_0707"]
impl crate::Resettable for RegCkdSysPllGrp1Spec {
    const RESET_VALUE: u32 = 0x0303_0707;
}
