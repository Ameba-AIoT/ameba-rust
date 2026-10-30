#[doc = "Register `REG_CKD_TIMER_GRP1` reader"]
pub type R = crate::R<RegCkdTimerGrp1Spec>;
#[doc = "Register `REG_CKD_TIMER_GRP1` writer"]
pub type W = crate::W<RegCkdTimerGrp1Spec>;
#[doc = "Field `ckd_xtal_dtim` reader - xtal 1m divider for dtim Default is /40 which output 1M"]
pub type CkdXtalDtimR = crate::FieldReader;
#[doc = "Field `ckd_xtal_dtim` writer - xtal 1m divider for dtim Default is /40 which output 1M"]
pub type CkdXtalDtimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 16:21 - xtal 1m divider for dtim Default is /40 which output 1M"]
    #[inline(always)]
    pub fn ckd_xtal_dtim(&self) -> CkdXtalDtimR {
        CkdXtalDtimR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:21 - xtal 1m divider for dtim Default is /40 which output 1M"]
    #[inline(always)]
    pub fn ckd_xtal_dtim(&mut self) -> CkdXtalDtimW<'_, RegCkdTimerGrp1Spec> {
        CkdXtalDtimW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ckd_timer_grp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ckd_timer_grp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCkdTimerGrp1Spec;
impl crate::RegisterSpec for RegCkdTimerGrp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ckd_timer_grp1::R`](R) reader structure"]
impl crate::Readable for RegCkdTimerGrp1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ckd_timer_grp1::W`](W) writer structure"]
impl crate::Writable for RegCkdTimerGrp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CKD_TIMER_GRP1 to value 0x0027_0000"]
impl crate::Resettable for RegCkdTimerGrp1Spec {
    const RESET_VALUE: u32 = 0x0027_0000;
}
