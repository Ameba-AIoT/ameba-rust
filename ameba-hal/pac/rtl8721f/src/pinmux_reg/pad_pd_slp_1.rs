#[doc = "Register `PAD_PD_SLP_1` reader"]
pub type R = crate::R<PadPdSlp1Spec>;
#[doc = "Register `PAD_PD_SLP_1` writer"]
pub type W = crate::W<PadPdSlp1Spec>;
#[doc = "Field `pa_pd_slp` reader - PAD pull down enable when system is in sleep."]
pub type PaPdSlpR = crate::FieldReader<u32>;
#[doc = "Field `pa_pd_slp` writer - PAD pull down enable when system is in sleep."]
pub type PaPdSlpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull down enable when system is in sleep."]
    #[inline(always)]
    pub fn pa_pd_slp(&self) -> PaPdSlpR {
        PaPdSlpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull down enable when system is in sleep."]
    #[inline(always)]
    pub fn pa_pd_slp(&mut self) -> PaPdSlpW<'_, PadPdSlp1Spec> {
        PaPdSlpW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pd_slp_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pd_slp_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPdSlp1Spec;
impl crate::RegisterSpec for PadPdSlp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pd_slp_1::R`](R) reader structure"]
impl crate::Readable for PadPdSlp1Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pd_slp_1::W`](W) writer structure"]
impl crate::Writable for PadPdSlp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PD_SLP_1 to value 0"]
impl crate::Resettable for PadPdSlp1Spec {}
