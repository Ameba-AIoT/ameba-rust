#[doc = "Register `PAD_PU_3` reader"]
pub type R = crate::R<PadPu3Spec>;
#[doc = "Register `PAD_PU_3` writer"]
pub type W = crate::W<PadPu3Spec>;
#[doc = "Field `pc_pu` reader - PAD pull up enable when system is in active."]
pub type PcPuR = crate::FieldReader<u32>;
#[doc = "Field `pc_pu` writer - PAD pull up enable when system is in active."]
pub type PcPuW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in active."]
    #[inline(always)]
    pub fn pc_pu(&self) -> PcPuR {
        PcPuR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in active."]
    #[inline(always)]
    pub fn pc_pu(&mut self) -> PcPuW<'_, PadPu3Spec> {
        PcPuW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pu_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pu_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPu3Spec;
impl crate::RegisterSpec for PadPu3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pu_3::R`](R) reader structure"]
impl crate::Readable for PadPu3Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pu_3::W`](W) writer structure"]
impl crate::Writable for PadPu3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PU_3 to value 0x0010_4008"]
impl crate::Resettable for PadPu3Spec {
    const RESET_VALUE: u32 = 0x0010_4008;
}
