#[doc = "Register `REG_CKD_TIMER_GRP0` reader"]
pub type R = crate::R<RegCkdTimerGrp0Spec>;
#[doc = "Register `REG_CKD_TIMER_GRP0` writer"]
pub type W = crate::W<RegCkdTimerGrp0Spec>;
#[doc = "Field `ckd_xtal_ltim0` reader - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
pub type CkdXtalLtim0R = crate::FieldReader;
#[doc = "Field `ckd_xtal_ltim0` writer - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
pub type CkdXtalLtim0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ckd_xtal_ltim1` reader - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
pub type CkdXtalLtim1R = crate::FieldReader;
#[doc = "Field `ckd_xtal_ltim1` writer - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
pub type CkdXtalLtim1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ckd_xtal_ltim2` reader - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
pub type CkdXtalLtim2R = crate::FieldReader;
#[doc = "Field `ckd_xtal_ltim2` writer - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
pub type CkdXtalLtim2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ckd_xtal_ltim3` reader - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
pub type CkdXtalLtim3R = crate::FieldReader;
#[doc = "Field `ckd_xtal_ltim3` writer - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
pub type CkdXtalLtim3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_xtal_ltim0(&self) -> CkdXtalLtim0R {
        CkdXtalLtim0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_xtal_ltim1(&self) -> CkdXtalLtim1R {
        CkdXtalLtim1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_xtal_ltim2(&self) -> CkdXtalLtim2R {
        CkdXtalLtim2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_xtal_ltim3(&self) -> CkdXtalLtim3R {
        CkdXtalLtim3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_xtal_ltim0(&mut self) -> CkdXtalLtim0W<'_, RegCkdTimerGrp0Spec> {
        CkdXtalLtim0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_xtal_ltim1(&mut self) -> CkdXtalLtim1W<'_, RegCkdTimerGrp0Spec> {
        CkdXtalLtim1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_xtal_ltim2(&mut self) -> CkdXtalLtim2W<'_, RegCkdTimerGrp0Spec> {
        CkdXtalLtim2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - LS timer xtal clock divider when cksl_ltim = 1 0: not support 1: div2 n: divider is n+1 DIV_MIN:1"]
    #[inline(always)]
    pub fn ckd_xtal_ltim3(&mut self) -> CkdXtalLtim3W<'_, RegCkdTimerGrp0Spec> {
        CkdXtalLtim3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ckd_timer_grp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ckd_timer_grp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkdTimerGrp0Spec;
impl crate::RegisterSpec for RegCkdTimerGrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ckd_timer_grp0::R`](R) reader structure"]
impl crate::Readable for RegCkdTimerGrp0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ckd_timer_grp0::W`](W) writer structure"]
impl crate::Writable for RegCkdTimerGrp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKD_TIMER_GRP0 to value 0x2727_2727"]
impl crate::Resettable for RegCkdTimerGrp0Spec {
    const RESET_VALUE: u32 = 0x2727_2727;
}
