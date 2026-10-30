#[doc = "Register `PAD_PU_1` reader"]
pub type R = crate::R<PadPu1Spec>;
#[doc = "Register `PAD_PU_1` writer"]
pub type W = crate::W<PadPu1Spec>;
#[doc = "Field `pa_pu` reader - PAD pull up enable when system is in active."]
pub type PaPuR = crate::FieldReader<u32>;
#[doc = "Field `pa_pu` writer - PAD pull up enable when system is in active."]
pub type PaPuW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in active."]
    #[inline(always)]
    pub fn pa_pu(&self) -> PaPuR {
        PaPuR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull up enable when system is in active."]
    #[inline(always)]
    pub fn pa_pu(&mut self) -> PaPuW<'_, PadPu1Spec> {
        PaPuW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pu_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pu_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPu1Spec;
impl crate::RegisterSpec for PadPu1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pu_1::R`](R) reader structure"]
impl crate::Readable for PadPu1Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pu_1::W`](W) writer structure"]
impl crate::Writable for PadPu1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PU_1 to value 0x0c"]
impl crate::Resettable for PadPu1Spec {
    const RESET_VALUE: u32 = 0x0c;
}
