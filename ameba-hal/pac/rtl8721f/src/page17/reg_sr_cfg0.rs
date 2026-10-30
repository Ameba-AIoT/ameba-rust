#[doc = "Register `REG_SR_CFG0` reader"]
pub type R = crate::R<RegSrCfg0Spec>;
#[doc = "Register `REG_SR_CFG0` writer"]
pub type W = crate::W<RegSrCfg0Spec>;
#[doc = "Field `r_sr_en` reader - enable spatial reuse function"]
pub type RSrEnR = crate::BitReader;
#[doc = "Field `r_sr_en` writer - enable spatial reuse function"]
pub type RSrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_srg_check_en` reader - enable srg ppdu check"]
pub type RSrgCheckEnR = crate::BitReader;
#[doc = "Field `r_srg_check_en` writer - enable srg ppdu check"]
pub type RSrgCheckEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_ctl_srg_check_en` reader - enable control packet srg ppdu check"]
pub type RCtlSrgCheckEnR = crate::BitReader;
#[doc = "Field `r_ctl_srg_check_en` writer - enable control packet srg ppdu check"]
pub type RCtlSrgCheckEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_spruse_rsvd_en` reader - enable spatial reuse when plcp header spatial reuse field value smaller than 14 or value is 15 and is srg ppdu"]
pub type RSpruseRsvdEnR = crate::BitReader;
#[doc = "Field `r_spruse_rsvd_en` writer - enable spatial reuse when plcp header spatial reuse field value smaller than 14 or value is 15 and is srg ppdu"]
pub type RSpruseRsvdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable spatial reuse function"]
    #[inline(always)]
    pub fn r_sr_en(&self) -> RSrEnR {
        RSrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable srg ppdu check"]
    #[inline(always)]
    pub fn r_srg_check_en(&self) -> RSrgCheckEnR {
        RSrgCheckEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable control packet srg ppdu check"]
    #[inline(always)]
    pub fn r_ctl_srg_check_en(&self) -> RCtlSrgCheckEnR {
        RCtlSrgCheckEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable spatial reuse when plcp header spatial reuse field value smaller than 14 or value is 15 and is srg ppdu"]
    #[inline(always)]
    pub fn r_spruse_rsvd_en(&self) -> RSpruseRsvdEnR {
        RSpruseRsvdEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable spatial reuse function"]
    #[inline(always)]
    pub fn r_sr_en(&mut self) -> RSrEnW<'_, RegSrCfg0Spec> {
        RSrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - enable srg ppdu check"]
    #[inline(always)]
    pub fn r_srg_check_en(&mut self) -> RSrgCheckEnW<'_, RegSrCfg0Spec> {
        RSrgCheckEnW::new(self, 1)
    }
    #[doc = "Bit 2 - enable control packet srg ppdu check"]
    #[inline(always)]
    pub fn r_ctl_srg_check_en(&mut self) -> RCtlSrgCheckEnW<'_, RegSrCfg0Spec> {
        RCtlSrgCheckEnW::new(self, 2)
    }
    #[doc = "Bit 3 - enable spatial reuse when plcp header spatial reuse field value smaller than 14 or value is 15 and is srg ppdu"]
    #[inline(always)]
    pub fn r_spruse_rsvd_en(&mut self) -> RSpruseRsvdEnW<'_, RegSrCfg0Spec> {
        RSpruseRsvdEnW::new(self, 3)
    }
}
#[doc = "spatial reuse configure register0\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sr_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sr_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSrCfg0Spec;
impl crate::RegisterSpec for RegSrCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sr_cfg0::R`](R) reader structure"]
impl crate::Readable for RegSrCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sr_cfg0::W`](W) writer structure"]
impl crate::Writable for RegSrCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SR_CFG0 to value 0"]
impl crate::Resettable for RegSrCfg0Spec {}
