#[doc = "Register `PAD_PU_SLP_3` reader"]
pub type R = crate::R<PadPuSlp3Spec>;
#[doc = "Register `PAD_PU_SLP_3` writer"]
pub type W = crate::W<PadPuSlp3Spec>;
#[doc = "Field `pc_pu_slp` reader - PAD pull up enable when system is in sleep."]
pub type PcPuSlpR = crate::FieldReader<u32>;
#[doc = "Field `pc_pu_slp` writer - PAD pull up enable when system is in sleep."]
pub type PcPuSlpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in sleep."]
    #[inline(always)]
    pub fn pc_pu_slp(&self) -> PcPuSlpR {
        PcPuSlpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in sleep."]
    #[inline(always)]
    pub fn pc_pu_slp(&mut self) -> PcPuSlpW<'_, PadPuSlp3Spec> {
        PcPuSlpW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pu_slp_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pu_slp_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPuSlp3Spec;
impl crate::RegisterSpec for PadPuSlp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pu_slp_3::R`](R) reader structure"]
impl crate::Readable for PadPuSlp3Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pu_slp_3::W`](W) writer structure"]
impl crate::Writable for PadPuSlp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PU_SLP_3 to value 0x0010_4008"]
impl crate::Resettable for PadPuSlp3Spec {
    const RESET_VALUE: u32 = 0x0010_4008;
}
