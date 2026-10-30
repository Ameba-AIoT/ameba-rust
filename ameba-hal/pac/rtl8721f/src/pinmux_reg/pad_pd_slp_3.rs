#[doc = "Register `PAD_PD_SLP_3` reader"]
pub type R = crate::R<PadPdSlp3Spec>;
#[doc = "Register `PAD_PD_SLP_3` writer"]
pub type W = crate::W<PadPdSlp3Spec>;
#[doc = "Field `pc_pd_slp` reader - PAD pull down enable when system is in sleep."]
pub type PcPdSlpR = crate::FieldReader<u32>;
#[doc = "Field `pc_pd_slp` writer - PAD pull down enable when system is in sleep."]
pub type PcPdSlpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull down enable when system is in sleep."]
    #[inline(always)]
    pub fn pc_pd_slp(&self) -> PcPdSlpR {
        PcPdSlpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull down enable when system is in sleep."]
    #[inline(always)]
    pub fn pc_pd_slp(&mut self) -> PcPdSlpW<'_, PadPdSlp3Spec> {
        PcPdSlpW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pd_slp_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pd_slp_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPdSlp3Spec;
impl crate::RegisterSpec for PadPdSlp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pd_slp_3::R`](R) reader structure"]
impl crate::Readable for PadPdSlp3Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pd_slp_3::W`](W) writer structure"]
impl crate::Writable for PadPdSlp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PD_SLP_3 to value 0"]
impl crate::Resettable for PadPdSlp3Spec {}
