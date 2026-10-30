#[doc = "Register `PAD_PU_2` reader"]
pub type R = crate::R<PadPu2Spec>;
#[doc = "Register `PAD_PU_2` writer"]
pub type W = crate::W<PadPu2Spec>;
#[doc = "Field `pb_pu` reader - PAD pull up enable when system is in active."]
pub type PbPuR = crate::FieldReader<u32>;
#[doc = "Field `pb_pu` writer - PAD pull up enable when system is in active."]
pub type PbPuW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in active."]
    #[inline(always)]
    pub fn pb_pu(&self) -> PbPuR {
        PbPuR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in active."]
    #[inline(always)]
    pub fn pb_pu(&mut self) -> PbPuW<'_, PadPu2Spec> {
        PbPuW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pu_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pu_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPu2Spec;
impl crate::RegisterSpec for PadPu2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pu_2::R`](R) reader structure"]
impl crate::Readable for PadPu2Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pu_2::W`](W) writer structure"]
impl crate::Writable for PadPu2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PU_2 to value 0x0016_0000"]
impl crate::Resettable for PadPu2Spec {
    const RESET_VALUE: u32 = 0x0016_0000;
}
