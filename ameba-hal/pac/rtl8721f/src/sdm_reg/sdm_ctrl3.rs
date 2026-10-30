#[doc = "Register `SDM_CTRL3` reader"]
pub type R = crate::R<SdmCtrl3Spec>;
#[doc = "Register `SDM_CTRL3` writer"]
pub type W = crate::W<SdmCtrl3Spec>;
#[doc = "Field `XTAL_PERIOD` reader - Set reference xtal period (ns)"]
pub type XtalPeriodR = crate::FieldReader<u32>;
#[doc = "Field `XTAL_PERIOD` writer - Set reference xtal period (ns)"]
pub type XtalPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set reference xtal period (ns)"]
    #[inline(always)]
    pub fn xtal_period(&self) -> XtalPeriodR {
        XtalPeriodR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set reference xtal period (ns)"]
    #[inline(always)]
    pub fn xtal_period(&mut self) -> XtalPeriodW<'_, SdmCtrl3Spec> {
        XtalPeriodW::new(self, 0)
    }
}
#[doc = "Set XTAL period\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmCtrl3Spec;
impl crate::RegisterSpec for SdmCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm_ctrl3::R`](R) reader structure"]
impl crate::Readable for SdmCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`sdm_ctrl3::W`](W) writer structure"]
impl crate::Writable for SdmCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDM_CTRL3 to value 0"]
impl crate::Resettable for SdmCtrl3Spec {}
