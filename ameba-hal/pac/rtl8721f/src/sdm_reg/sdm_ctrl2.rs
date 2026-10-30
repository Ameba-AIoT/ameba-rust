#[doc = "Register `SDM_CTRL2` reader"]
pub type R = crate::R<SdmCtrl2Spec>;
#[doc = "Register `SDM_CTRL2` writer"]
pub type W = crate::W<SdmCtrl2Spec>;
#[doc = "Field `SDM_OBS_REF_CYC` reader - Set in observed cycles, refer to counters value, default 312500 XTAL cycles"]
pub type SdmObsRefCycR = crate::FieldReader<u32>;
#[doc = "Field `SDM_OBS_REF_CYC` writer - Set in observed cycles, refer to counters value, default 312500 XTAL cycles"]
pub type SdmObsRefCycW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set in observed cycles, refer to counters value, default 312500 XTAL cycles"]
    #[inline(always)]
    pub fn sdm_obs_ref_cyc(&self) -> SdmObsRefCycR {
        SdmObsRefCycR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set in observed cycles, refer to counters value, default 312500 XTAL cycles"]
    #[inline(always)]
    pub fn sdm_obs_ref_cyc(&mut self) -> SdmObsRefCycW<'_, SdmCtrl2Spec> {
        SdmObsRefCycW::new(self, 0)
    }
}
#[doc = "Set ref counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmCtrl2Spec;
impl crate::RegisterSpec for SdmCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm_ctrl2::R`](R) reader structure"]
impl crate::Readable for SdmCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sdm_ctrl2::W`](W) writer structure"]
impl crate::Writable for SdmCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDM_CTRL2 to value 0"]
impl crate::Resettable for SdmCtrl2Spec {}
