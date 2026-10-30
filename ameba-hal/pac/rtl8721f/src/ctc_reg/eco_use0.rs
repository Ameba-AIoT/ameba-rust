#[doc = "Register `ECO_USE0` reader"]
pub type R = crate::R<EcoUse0Spec>;
#[doc = "Register `ECO_USE0` writer"]
pub type W = crate::W<EcoUse0Spec>;
#[doc = "Field `eco_use0` reader - For ECO use"]
pub type EcoUse0R = crate::FieldReader<u32>;
#[doc = "Field `eco_use0` writer - For ECO use"]
pub type EcoUse0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For ECO use"]
    #[inline(always)]
    pub fn eco_use0(&self) -> EcoUse0R {
        EcoUse0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - For ECO use"]
    #[inline(always)]
    pub fn eco_use0(&mut self) -> EcoUse0W<'_, EcoUse0Spec> {
        EcoUse0W::new(self, 0)
    }
}
#[doc = "ECO USE0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_use0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_use0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcoUse0Spec;
impl crate::RegisterSpec for EcoUse0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_use0::R`](R) reader structure"]
impl crate::Readable for EcoUse0Spec {}
#[doc = "`write(|w| ..)` method takes [`eco_use0::W`](W) writer structure"]
impl crate::Writable for EcoUse0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECO_USE0 to value 0"]
impl crate::Resettable for EcoUse0Spec {}
