#[doc = "Register `MAR4` reader"]
pub type R = crate::R<Mar4Spec>;
#[doc = "Register `MAR4` writer"]
pub type W = crate::W<Mar4Spec>;
#[doc = "Field `MAR7` reader - Multicast Register 7"]
pub type Mar7R = crate::FieldReader;
#[doc = "Field `MAR7` writer - Multicast Register 7"]
pub type Mar7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAR6` reader - "]
pub type Mar6R = crate::FieldReader;
#[doc = "Field `MAR6` writer - "]
pub type Mar6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAR5` reader - Multicast Register 5"]
pub type Mar5R = crate::FieldReader;
#[doc = "Field `MAR5` writer - Multicast Register 5"]
pub type Mar5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAR4` reader - Multicast Register 4"]
pub type Mar4R = crate::FieldReader;
#[doc = "Field `MAR4` writer - Multicast Register 4"]
pub type Mar4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Multicast Register 7"]
    #[inline(always)]
    pub fn mar7(&self) -> Mar7R {
        Mar7R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mar6(&self) -> Mar6R {
        Mar6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Multicast Register 5"]
    #[inline(always)]
    pub fn mar5(&self) -> Mar5R {
        Mar5R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Multicast Register 4"]
    #[inline(always)]
    pub fn mar4(&self) -> Mar4R {
        Mar4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Multicast Register 7"]
    #[inline(always)]
    pub fn mar7(&mut self) -> Mar7W<'_, Mar4Spec> {
        Mar7W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mar6(&mut self) -> Mar6W<'_, Mar4Spec> {
        Mar6W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Multicast Register 5"]
    #[inline(always)]
    pub fn mar5(&mut self) -> Mar5W<'_, Mar4Spec> {
        Mar5W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Multicast Register 4"]
    #[inline(always)]
    pub fn mar4(&mut self) -> Mar4W<'_, Mar4Spec> {
        Mar4W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mar4Spec;
impl crate::RegisterSpec for Mar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mar4::R`](R) reader structure"]
impl crate::Readable for Mar4Spec {}
#[doc = "`write(|w| ..)` method takes [`mar4::W`](W) writer structure"]
impl crate::Writable for Mar4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAR4 to value 0"]
impl crate::Resettable for Mar4Spec {}
