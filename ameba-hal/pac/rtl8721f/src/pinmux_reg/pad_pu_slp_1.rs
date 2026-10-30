#[doc = "Register `PAD_PU_SLP_1` reader"]
pub type R = crate::R<PadPuSlp1Spec>;
#[doc = "Register `PAD_PU_SLP_1` writer"]
pub type W = crate::W<PadPuSlp1Spec>;
#[doc = "Field `pa_pu_slp` reader - PAD pull up enable when system is in sleep."]
pub type PaPuSlpR = crate::FieldReader<u32>;
#[doc = "Field `pa_pu_slp` writer - PAD pull up enable when system is in sleep."]
pub type PaPuSlpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in sleep."]
    #[inline(always)]
    pub fn pa_pu_slp(&self) -> PaPuSlpR {
        PaPuSlpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in sleep."]
    #[inline(always)]
    pub fn pa_pu_slp(&mut self) -> PaPuSlpW<'_, PadPuSlp1Spec> {
        PaPuSlpW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pu_slp_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pu_slp_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPuSlp1Spec;
impl crate::RegisterSpec for PadPuSlp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pu_slp_1::R`](R) reader structure"]
impl crate::Readable for PadPuSlp1Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pu_slp_1::W`](W) writer structure"]
impl crate::Writable for PadPuSlp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PU_SLP_1 to value 0x0c"]
impl crate::Resettable for PadPuSlp1Spec {
    const RESET_VALUE: u32 = 0x0c;
}
