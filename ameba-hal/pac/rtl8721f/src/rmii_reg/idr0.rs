#[doc = "Register `IDR0` reader"]
pub type R = crate::R<Idr0Spec>;
#[doc = "Register `IDR0` writer"]
pub type W = crate::W<Idr0Spec>;
#[doc = "Field `IDR3` reader - "]
pub type Idr3R = crate::FieldReader;
#[doc = "Field `IDR3` writer - "]
pub type Idr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDR2` reader - "]
pub type Idr2R = crate::FieldReader;
#[doc = "Field `IDR2` writer - "]
pub type Idr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDR1` reader - "]
pub type Idr1R = crate::FieldReader;
#[doc = "Field `IDR1` writer - "]
pub type Idr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDR0` reader - The ID register0-5 are only permitted to write by 4-byte access. Read access can be byte, word, or double word access."]
pub type Idr0R = crate::FieldReader;
#[doc = "Field `IDR0` writer - The ID register0-5 are only permitted to write by 4-byte access. Read access can be byte, word, or double word access."]
pub type Idr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn idr3(&self) -> Idr3R {
        Idr3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn idr2(&self) -> Idr2R {
        Idr2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn idr1(&self) -> Idr1R {
        Idr1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The ID register0-5 are only permitted to write by 4-byte access. Read access can be byte, word, or double word access."]
    #[inline(always)]
    pub fn idr0(&self) -> Idr0R {
        Idr0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn idr3(&mut self) -> Idr3W<'_, Idr0Spec> {
        Idr3W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn idr2(&mut self) -> Idr2W<'_, Idr0Spec> {
        Idr2W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn idr1(&mut self) -> Idr1W<'_, Idr0Spec> {
        Idr1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - The ID register0-5 are only permitted to write by 4-byte access. Read access can be byte, word, or double word access."]
    #[inline(always)]
    pub fn idr0(&mut self) -> Idr0W<'_, Idr0Spec> {
        Idr0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`idr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idr0Spec;
impl crate::RegisterSpec for Idr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr0::R`](R) reader structure"]
impl crate::Readable for Idr0Spec {}
#[doc = "`write(|w| ..)` method takes [`idr0::W`](W) writer structure"]
impl crate::Writable for Idr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR0 to value 0"]
impl crate::Resettable for Idr0Spec {}
