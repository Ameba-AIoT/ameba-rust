#[doc = "Register `PAD_PD_1` reader"]
pub type R = crate::R<PadPd1Spec>;
#[doc = "Register `PAD_PD_1` writer"]
pub type W = crate::W<PadPd1Spec>;
#[doc = "Field `pa_pd` reader - PAD pull down enable when system is in active."]
pub type PaPdR = crate::FieldReader<u32>;
#[doc = "Field `pa_pd` writer - PAD pull down enable when system is in active."]
pub type PaPdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull down enable when system is in active."]
    #[inline(always)]
    pub fn pa_pd(&self) -> PaPdR {
        PaPdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull down enable when system is in active."]
    #[inline(always)]
    pub fn pa_pd(&mut self) -> PaPdW<'_, PadPd1Spec> {
        PaPdW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pd_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pd_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPd1Spec;
impl crate::RegisterSpec for PadPd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pd_1::R`](R) reader structure"]
impl crate::Readable for PadPd1Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pd_1::W`](W) writer structure"]
impl crate::Writable for PadPd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PD_1 to value 0"]
impl crate::Resettable for PadPd1Spec {}
