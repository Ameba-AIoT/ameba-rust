#[doc = "Register `PAD_PD_3` reader"]
pub type R = crate::R<PadPd3Spec>;
#[doc = "Register `PAD_PD_3` writer"]
pub type W = crate::W<PadPd3Spec>;
#[doc = "Field `pc_pd` reader - PAD pull down enable when system is in active."]
pub type PcPdR = crate::FieldReader<u32>;
#[doc = "Field `pc_pd` writer - PAD pull down enable when system is in active."]
pub type PcPdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull down enable when system is in active."]
    #[inline(always)]
    pub fn pc_pd(&self) -> PcPdR {
        PcPdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull down enable when system is in active."]
    #[inline(always)]
    pub fn pc_pd(&mut self) -> PcPdW<'_, PadPd3Spec> {
        PcPdW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pd_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pd_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPd3Spec;
impl crate::RegisterSpec for PadPd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pd_3::R`](R) reader structure"]
impl crate::Readable for PadPd3Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pd_3::W`](W) writer structure"]
impl crate::Writable for PadPd3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PD_3 to value 0"]
impl crate::Resettable for PadPd3Spec {}
