#[doc = "Register `PAD_PU_SLP_2` reader"]
pub type R = crate::R<PadPuSlp2Spec>;
#[doc = "Register `PAD_PU_SLP_2` writer"]
pub type W = crate::W<PadPuSlp2Spec>;
#[doc = "Field `pb_pu_slp` reader - PAD pull up enable when system is in sleep."]
pub type PbPuSlpR = crate::FieldReader<u32>;
#[doc = "Field `pb_pu_slp` writer - PAD pull up enable when system is in sleep."]
pub type PbPuSlpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in sleep."]
    #[inline(always)]
    pub fn pb_pu_slp(&self) -> PbPuSlpR {
        PbPuSlpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in sleep."]
    #[inline(always)]
    pub fn pb_pu_slp(&mut self) -> PbPuSlpW<'_, PadPuSlp2Spec> {
        PbPuSlpW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pu_slp_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pu_slp_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPuSlp2Spec;
impl crate::RegisterSpec for PadPuSlp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pu_slp_2::R`](R) reader structure"]
impl crate::Readable for PadPuSlp2Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pu_slp_2::W`](W) writer structure"]
impl crate::Writable for PadPuSlp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PU_SLP_2 to value 0x0016_0000"]
impl crate::Resettable for PadPuSlp2Spec {
    const RESET_VALUE: u32 = 0x0016_0000;
}
