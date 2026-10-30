#[doc = "Register `GRP_PWR_MSK3` reader"]
pub type R = crate::R<GrpPwrMsk3Spec>;
#[doc = "Register `GRP_PWR_MSK3` writer"]
pub type W = crate::W<GrpPwrMsk3Spec>;
#[doc = "Field `grp_pmsk_ppe_e1` reader - "]
pub type GrpPmskPpeE1R = crate::FieldReader;
#[doc = "Field `grp_pmsk_ppe_e1` writer - "]
pub type GrpPmskPpeE1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn grp_pmsk_ppe_e1(&self) -> GrpPmskPpeE1R {
        GrpPmskPpeE1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn grp_pmsk_ppe_e1(&mut self) -> GrpPmskPpeE1W<'_, GrpPwrMsk3Spec> {
        GrpPmskPpeE1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_pwr_msk3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_pwr_msk3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpPwrMsk3Spec;
impl crate::RegisterSpec for GrpPwrMsk3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_pwr_msk3::R`](R) reader structure"]
impl crate::Readable for GrpPwrMsk3Spec {}
#[doc = "`write(|w| ..)` method takes [`grp_pwr_msk3::W`](W) writer structure"]
impl crate::Writable for GrpPwrMsk3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_PWR_MSK3 to value 0xff"]
impl crate::Resettable for GrpPwrMsk3Spec {
    const RESET_VALUE: u32 = 0xff;
}
