#[doc = "Register `MAR0` reader"]
pub type R = crate::R<Mar0Spec>;
#[doc = "Register `MAR0` writer"]
pub type W = crate::W<Mar0Spec>;
#[doc = "Field `MAR3` reader - Multicast Register 3"]
pub type Mar3R = crate::FieldReader;
#[doc = "Field `MAR3` writer - Multicast Register 3"]
pub type Mar3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAR2` reader - Multicast Register 2"]
pub type Mar2R = crate::FieldReader;
#[doc = "Field `MAR2` writer - Multicast Register 2"]
pub type Mar2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAR1` reader - Multicast Register 1"]
pub type Mar1R = crate::FieldReader;
#[doc = "Field `MAR1` writer - Multicast Register 1"]
pub type Mar1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAR0` reader - Multicast Register 0, The MAR register0-7 are only permitted to write by 4-byte access. Read access can be byte, word, or double word access. Driver is responsible for initializing these registers. The MAR7-0 define 64 bits which is a bit wise index of the multicast function of multicast addresses. The hash function of multicast address is the upper 6 MSB's of the CRC32 of the address (destination). The index then is the numerical representation of those 6 bits in hex format."]
pub type Mar0R = crate::FieldReader;
#[doc = "Field `MAR0` writer - Multicast Register 0, The MAR register0-7 are only permitted to write by 4-byte access. Read access can be byte, word, or double word access. Driver is responsible for initializing these registers. The MAR7-0 define 64 bits which is a bit wise index of the multicast function of multicast addresses. The hash function of multicast address is the upper 6 MSB's of the CRC32 of the address (destination). The index then is the numerical representation of those 6 bits in hex format."]
pub type Mar0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Multicast Register 3"]
    #[inline(always)]
    pub fn mar3(&self) -> Mar3R {
        Mar3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Multicast Register 2"]
    #[inline(always)]
    pub fn mar2(&self) -> Mar2R {
        Mar2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Multicast Register 1"]
    #[inline(always)]
    pub fn mar1(&self) -> Mar1R {
        Mar1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Multicast Register 0, The MAR register0-7 are only permitted to write by 4-byte access. Read access can be byte, word, or double word access. Driver is responsible for initializing these registers. The MAR7-0 define 64 bits which is a bit wise index of the multicast function of multicast addresses. The hash function of multicast address is the upper 6 MSB's of the CRC32 of the address (destination). The index then is the numerical representation of those 6 bits in hex format."]
    #[inline(always)]
    pub fn mar0(&self) -> Mar0R {
        Mar0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Multicast Register 3"]
    #[inline(always)]
    pub fn mar3(&mut self) -> Mar3W<'_, Mar0Spec> {
        Mar3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Multicast Register 2"]
    #[inline(always)]
    pub fn mar2(&mut self) -> Mar2W<'_, Mar0Spec> {
        Mar2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Multicast Register 1"]
    #[inline(always)]
    pub fn mar1(&mut self) -> Mar1W<'_, Mar0Spec> {
        Mar1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Multicast Register 0, The MAR register0-7 are only permitted to write by 4-byte access. Read access can be byte, word, or double word access. Driver is responsible for initializing these registers. The MAR7-0 define 64 bits which is a bit wise index of the multicast function of multicast addresses. The hash function of multicast address is the upper 6 MSB's of the CRC32 of the address (destination). The index then is the numerical representation of those 6 bits in hex format."]
    #[inline(always)]
    pub fn mar0(&mut self) -> Mar0W<'_, Mar0Spec> {
        Mar0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mar0Spec;
impl crate::RegisterSpec for Mar0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mar0::R`](R) reader structure"]
impl crate::Readable for Mar0Spec {}
#[doc = "`write(|w| ..)` method takes [`mar0::W`](W) writer structure"]
impl crate::Writable for Mar0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAR0 to value 0"]
impl crate::Resettable for Mar0Spec {}
