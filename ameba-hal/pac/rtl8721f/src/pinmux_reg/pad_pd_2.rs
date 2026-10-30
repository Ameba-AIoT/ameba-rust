#[doc = "Register `PAD_PD_2` reader"]
pub type R = crate::R<PadPd2Spec>;
#[doc = "Register `PAD_PD_2` writer"]
pub type W = crate::W<PadPd2Spec>;
#[doc = "Field `pb_pd` reader - PAD pull down enable when system is in active."]
pub type PbPdR = crate::FieldReader<u32>;
#[doc = "Field `pb_pd` writer - PAD pull down enable when system is in active."]
pub type PbPdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PAD pull down enable when system is in active."]
    #[inline(always)]
    pub fn pb_pd(&self) -> PbPdR {
        PbPdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAD pull down enable when system is in active."]
    #[inline(always)]
    pub fn pb_pd(&mut self) -> PbPdW<'_, PadPd2Spec> {
        PbPdW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_pd_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_pd_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadPd2Spec;
impl crate::RegisterSpec for PadPd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_pd_2::R`](R) reader structure"]
impl crate::Readable for PadPd2Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_pd_2::W`](W) writer structure"]
impl crate::Writable for PadPd2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_PD_2 to value 0"]
impl crate::Resettable for PadPd2Spec {}
