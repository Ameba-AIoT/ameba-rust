#[doc = "Register `ECO_USE1` reader"]
pub type R = crate::R<EcoUse1Spec>;
#[doc = "Register `ECO_USE1` writer"]
pub type W = crate::W<EcoUse1Spec>;
#[doc = "Field `eco_use1` reader - For ECO use"]
pub type EcoUse1R = crate::FieldReader<u32>;
#[doc = "Field `eco_use1` writer - For ECO use"]
pub type EcoUse1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For ECO use"]
    #[inline(always)]
    pub fn eco_use1(&self) -> EcoUse1R {
        EcoUse1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - For ECO use"]
    #[inline(always)]
    pub fn eco_use1(&mut self) -> EcoUse1W<'_, EcoUse1Spec> {
        EcoUse1W::new(self, 0)
    }
}
#[doc = "ECO USE1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_use1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_use1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcoUse1Spec;
impl crate::RegisterSpec for EcoUse1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_use1::R`](R) reader structure"]
impl crate::Readable for EcoUse1Spec {}
#[doc = "`write(|w| ..)` method takes [`eco_use1::W`](W) writer structure"]
impl crate::Writable for EcoUse1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECO_USE1 to value 0"]
impl crate::Resettable for EcoUse1Spec {}
