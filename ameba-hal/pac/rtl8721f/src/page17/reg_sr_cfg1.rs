#[doc = "Register `REG_SR_CFG1` reader"]
pub type R = crate::R<RegSrCfg1Spec>;
#[doc = "Register `REG_SR_CFG1` writer"]
pub type W = crate::W<RegSrCfg1Spec>;
#[doc = "Field `r_srg_obss_pd_min` reader - the minimum value of OBSS_PD"]
pub type RSrgObssPdMinR = crate::FieldReader;
#[doc = "Field `r_srg_obss_pd_min` writer - the minimum value of OBSS_PD"]
pub type RSrgObssPdMinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_srg_obss_pd_max` reader - the maximum value of OBSS_PD"]
pub type RSrgObssPdMaxR = crate::FieldReader;
#[doc = "Field `r_srg_obss_pd_max` writer - the maximum value of OBSS_PD"]
pub type RSrgObssPdMaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_nonsrg_obss_pd_min` reader - the minimum value of non_SRG OBSS_PD"]
pub type RNonsrgObssPdMinR = crate::FieldReader;
#[doc = "Field `r_nonsrg_obss_pd_min` writer - the minimum value of non_SRG OBSS_PD"]
pub type RNonsrgObssPdMinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_nonsrg_obss_pd_max` reader - the maximum value of non_SRG OBSS_PD"]
pub type RNonsrgObssPdMaxR = crate::FieldReader;
#[doc = "Field `r_nonsrg_obss_pd_max` writer - the maximum value of non_SRG OBSS_PD"]
pub type RNonsrgObssPdMaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - the minimum value of OBSS_PD"]
    #[inline(always)]
    pub fn r_srg_obss_pd_min(&self) -> RSrgObssPdMinR {
        RSrgObssPdMinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - the maximum value of OBSS_PD"]
    #[inline(always)]
    pub fn r_srg_obss_pd_max(&self) -> RSrgObssPdMaxR {
        RSrgObssPdMaxR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - the minimum value of non_SRG OBSS_PD"]
    #[inline(always)]
    pub fn r_nonsrg_obss_pd_min(&self) -> RNonsrgObssPdMinR {
        RNonsrgObssPdMinR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - the maximum value of non_SRG OBSS_PD"]
    #[inline(always)]
    pub fn r_nonsrg_obss_pd_max(&self) -> RNonsrgObssPdMaxR {
        RNonsrgObssPdMaxR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - the minimum value of OBSS_PD"]
    #[inline(always)]
    pub fn r_srg_obss_pd_min(&mut self) -> RSrgObssPdMinW<'_, RegSrCfg1Spec> {
        RSrgObssPdMinW::new(self, 0)
    }
    #[doc = "Bits 8:15 - the maximum value of OBSS_PD"]
    #[inline(always)]
    pub fn r_srg_obss_pd_max(&mut self) -> RSrgObssPdMaxW<'_, RegSrCfg1Spec> {
        RSrgObssPdMaxW::new(self, 8)
    }
    #[doc = "Bits 16:23 - the minimum value of non_SRG OBSS_PD"]
    #[inline(always)]
    pub fn r_nonsrg_obss_pd_min(&mut self) -> RNonsrgObssPdMinW<'_, RegSrCfg1Spec> {
        RNonsrgObssPdMinW::new(self, 16)
    }
    #[doc = "Bits 24:31 - the maximum value of non_SRG OBSS_PD"]
    #[inline(always)]
    pub fn r_nonsrg_obss_pd_max(&mut self) -> RNonsrgObssPdMaxW<'_, RegSrCfg1Spec> {
        RNonsrgObssPdMaxW::new(self, 24)
    }
}
#[doc = "spatial reuse configure register1\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sr_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sr_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSrCfg1Spec;
impl crate::RegisterSpec for RegSrCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sr_cfg1::R`](R) reader structure"]
impl crate::Readable for RegSrCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_sr_cfg1::W`](W) writer structure"]
impl crate::Writable for RegSrCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SR_CFG1 to value 0"]
impl crate::Resettable for RegSrCfg1Spec {}
