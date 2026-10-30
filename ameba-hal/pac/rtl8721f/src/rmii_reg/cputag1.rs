#[doc = "Register `CPUTAG1` reader"]
pub type R = crate::R<Cputag1Spec>;
#[doc = "Register `CPUTAG1` writer"]
pub type W = crate::W<Cputag1Spec>;
#[doc = "Field `Spa_pon` reader - The ingress cputag.SPA=PON field. Used in Apollo cputag."]
pub type SpaPonR = crate::FieldReader;
#[doc = "Field `Spa_pon` writer - The ingress cputag.SPA=PON field. Used in Apollo cputag."]
pub type SpaPonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Spa_dsl` reader - "]
pub type SpaDslR = crate::FieldReader;
#[doc = "Field `Spa_dsl` writer - "]
pub type SpaDslW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The ingress cputag.SPA=PON field. Used in Apollo cputag."]
    #[inline(always)]
    pub fn spa_pon(&self) -> SpaPonR {
        SpaPonR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn spa_dsl(&self) -> SpaDslR {
        SpaDslR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The ingress cputag.SPA=PON field. Used in Apollo cputag."]
    #[inline(always)]
    pub fn spa_pon(&mut self) -> SpaPonW<'_, Cputag1Spec> {
        SpaPonW::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn spa_dsl(&mut self) -> SpaDslW<'_, Cputag1Spec> {
        SpaDslW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cputag1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cputag1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cputag1Spec;
impl crate::RegisterSpec for Cputag1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cputag1::R`](R) reader structure"]
impl crate::Readable for Cputag1Spec {}
#[doc = "`write(|w| ..)` method takes [`cputag1::W`](W) writer structure"]
impl crate::Writable for Cputag1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPUTAG1 to value 0"]
impl crate::Resettable for Cputag1Spec {}
