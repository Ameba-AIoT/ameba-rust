#[doc = "Register `REG_CKD_SYS_PLL_GRP0` reader"]
pub type R = crate::R<RegCkdSysPllGrp0Spec>;
#[doc = "Register `REG_CKD_SYS_PLL_GRP0` writer"]
pub type W = crate::W<RegCkdSysPllGrp0Spec>;
#[doc = "Field `ckd_sys_pll_hp` reader - sys_pll divider for hp 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllHpR = crate::FieldReader;
#[doc = "Field `ckd_sys_pll_hp` writer - sys_pll divider for hp 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllHpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_sys_pll_hperi` reader - sys_pll divider for hperi 0: div1 1: div2 ..."]
pub type CkdSysPllHperiR = crate::FieldReader;
#[doc = "Field `ckd_sys_pll_hperi` writer - sys_pll divider for hperi 0: div1 1: div2 ..."]
pub type CkdSysPllHperiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_sys_pll_spic` reader - sys_pll divider for spic 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllSpicR = crate::FieldReader;
#[doc = "Field `ckd_sys_pll_spic` writer - sys_pll divider for spic 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllSpicW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_sys_pll_psram` reader - sys_pll divider for psram 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllPsramR = crate::FieldReader;
#[doc = "Field `ckd_sys_pll_psram` writer - sys_pll divider for psram 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllPsramW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_sys_pll_shperi` reader - sys_pll divider for shperi 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllShperiR = crate::FieldReader;
#[doc = "Field `ckd_sys_pll_shperi` writer - sys_pll divider for shperi 0: div1 1: div2 ... DIV_MIN:0"]
pub type CkdSysPllShperiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ckd_sys_pll_can` reader - sys_pll divider for can 0: div1 1: div2 ... Note: default 400M/4=100M DIV_MIN:0"]
pub type CkdSysPllCanR = crate::FieldReader;
#[doc = "Field `ckd_sys_pll_can` writer - sys_pll divider for can 0: div1 1: div2 ... Note: default 400M/4=100M DIV_MIN:0"]
pub type CkdSysPllCanW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - sys_pll divider for hp 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_hp(&self) -> CkdSysPllHpR {
        CkdSysPllHpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - sys_pll divider for hperi 0: div1 1: div2 ..."]
    #[inline(always)]
    pub fn ckd_sys_pll_hperi(&self) -> CkdSysPllHperiR {
        CkdSysPllHperiR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - sys_pll divider for spic 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_spic(&self) -> CkdSysPllSpicR {
        CkdSysPllSpicR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - sys_pll divider for psram 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_psram(&self) -> CkdSysPllPsramR {
        CkdSysPllPsramR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - sys_pll divider for shperi 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_shperi(&self) -> CkdSysPllShperiR {
        CkdSysPllShperiR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - sys_pll divider for can 0: div1 1: div2 ... Note: default 400M/4=100M DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_can(&self) -> CkdSysPllCanR {
        CkdSysPllCanR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - sys_pll divider for hp 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_hp(&mut self) -> CkdSysPllHpW<'_, RegCkdSysPllGrp0Spec> {
        CkdSysPllHpW::new(self, 0)
    }
    #[doc = "Bits 4:7 - sys_pll divider for hperi 0: div1 1: div2 ..."]
    #[inline(always)]
    pub fn ckd_sys_pll_hperi(&mut self) -> CkdSysPllHperiW<'_, RegCkdSysPllGrp0Spec> {
        CkdSysPllHperiW::new(self, 4)
    }
    #[doc = "Bits 8:11 - sys_pll divider for spic 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_spic(&mut self) -> CkdSysPllSpicW<'_, RegCkdSysPllGrp0Spec> {
        CkdSysPllSpicW::new(self, 8)
    }
    #[doc = "Bits 12:15 - sys_pll divider for psram 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_psram(&mut self) -> CkdSysPllPsramW<'_, RegCkdSysPllGrp0Spec> {
        CkdSysPllPsramW::new(self, 12)
    }
    #[doc = "Bits 16:19 - sys_pll divider for shperi 0: div1 1: div2 ... DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_shperi(&mut self) -> CkdSysPllShperiW<'_, RegCkdSysPllGrp0Spec> {
        CkdSysPllShperiW::new(self, 16)
    }
    #[doc = "Bits 20:23 - sys_pll divider for can 0: div1 1: div2 ... Note: default 400M/4=100M DIV_MIN:0"]
    #[inline(always)]
    pub fn ckd_sys_pll_can(&mut self) -> CkdSysPllCanW<'_, RegCkdSysPllGrp0Spec> {
        CkdSysPllCanW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ckd_sys_pll_grp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ckd_sys_pll_grp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkdSysPllGrp0Spec;
impl crate::RegisterSpec for RegCkdSysPllGrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ckd_sys_pll_grp0::R`](R) reader structure"]
impl crate::Readable for RegCkdSysPllGrp0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ckd_sys_pll_grp0::W`](W) writer structure"]
impl crate::Writable for RegCkdSysPllGrp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKD_SYS_PLL_GRP0 to value 0x0031_0131"]
impl crate::Resettable for RegCkdSysPllGrp0Spec {
    const RESET_VALUE: u32 = 0x0031_0131;
}
