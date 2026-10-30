#[doc = "Register `SDM_CTRL1` reader"]
pub type R = crate::R<SdmCtrl1Spec>;
#[doc = "Register `SDM_CTRL1` writer"]
pub type W = crate::W<SdmCtrl1Spec>;
#[doc = "Field `SDM_OBS_CYC` reader - Set observed 128k cycle, default 1024 128k cycles"]
pub type SdmObsCycR = crate::FieldReader<u16>;
#[doc = "Field `SDM_OBS_CYC` writer - Set observed 128k cycle, default 1024 128k cycles"]
pub type SdmObsCycW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Set observed 128k cycle, default 1024 128k cycles"]
    #[inline(always)]
    pub fn sdm_obs_cyc(&self) -> SdmObsCycR {
        SdmObsCycR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set observed 128k cycle, default 1024 128k cycles"]
    #[inline(always)]
    pub fn sdm_obs_cyc(&mut self) -> SdmObsCycW<'_, SdmCtrl1Spec> {
        SdmObsCycW::new(self, 0)
    }
}
#[doc = "Config observed 128K cycle\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmCtrl1Spec;
impl crate::RegisterSpec for SdmCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm_ctrl1::R`](R) reader structure"]
impl crate::Readable for SdmCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sdm_ctrl1::W`](W) writer structure"]
impl crate::Writable for SdmCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDM_CTRL1 to value 0"]
impl crate::Resettable for SdmCtrl1Spec {}
