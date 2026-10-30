#[doc = "Register `IDR4` reader"]
pub type R = crate::R<Idr4Spec>;
#[doc = "Register `IDR4` writer"]
pub type W = crate::W<Idr4Spec>;
#[doc = "Field `IDR5` reader - "]
pub type Idr5R = crate::FieldReader;
#[doc = "Field `IDR5` writer - "]
pub type Idr5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDR4` reader - "]
pub type Idr4R = crate::FieldReader;
#[doc = "Field `IDR4` writer - "]
pub type Idr4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn idr5(&self) -> Idr5R {
        Idr5R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn idr4(&self) -> Idr4R {
        Idr4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn idr5(&mut self) -> Idr5W<'_, Idr4Spec> {
        Idr5W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn idr4(&mut self) -> Idr4W<'_, Idr4Spec> {
        Idr4W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`idr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idr4Spec;
impl crate::RegisterSpec for Idr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr4::R`](R) reader structure"]
impl crate::Readable for Idr4Spec {}
#[doc = "`write(|w| ..)` method takes [`idr4::W`](W) writer structure"]
impl crate::Writable for Idr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR4 to value 0"]
impl crate::Resettable for Idr4Spec {}
